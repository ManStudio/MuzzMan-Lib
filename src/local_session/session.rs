use std::{
    ops::Range,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use crate::{prelude::*, VERSION};

pub const LOCAL_SESSION_VERSION: u64 = 1;

#[derive(Default)]
pub struct LocalSession {
    pub location: Option<LRow>,
    pub modules: Vec<Option<MRow>>,
    pub actions: Vec<Arc<RwLock<Action>>>,
    pub callback: Option<Box<dyn Fn(SessionEvent)>>,

    pub refs: Vec<Ref>,
}

unsafe impl Send for LocalSession {}
unsafe impl Sync for LocalSession {}

impl LocalSession {
    pub fn new_session(self) -> Box<Arc<RwLock<LocalSession>>> {
        let session = self;

        let session = Arc::new(RwLock::new(session));

        let location_info = Arc::new(RwLock::new(Path::UnRLocation(LocationPath(Vec::new()))));

        session.register_location(location_info);

        let location = Arc::new(RwLock::new(Location {
            name: String::from("Default Location"),
            desc: String::from("Default Location"),
            where_is: WhereIsLocation::Local,
            should_save: false,
            module_settings: Values::new(),
            settings: Values::new(),
            elements: Vec::new(),
            locations: Vec::new(),
            ref_id: location_info,
            path: PathBuf::from("."),
            thread: None,
            module: None,
            events: Arc::new(RwLock::new(Events::default())),

            progress: 0.0,
            statuses: vec![],
            status: usize::MAX,
            is_error: false,
            enabled: false,
        }));
        session.write().unwrap().location = Some(location);
        Box::new(session)
    }
}

pub trait TLocalSession: TSession {
    fn get_location(&self, location_id: &LocationPath) -> Result<LRow, SessionError>;
    fn get_element(&self, element_id: &ElementPath) -> Result<ERow, SessionError>;
    fn get_module(&self, module_id: &ModulePath) -> Result<MRow, SessionError>;

    fn add_module(
        &self,
        module: Box<dyn TModule>,
        path: Option<PathBuf>,
        info: Option<ModuleInfo>,
    ) -> Result<MRef, SessionError>;

    fn _register(&self, _ref: Ref) -> Result<UID, SessionError>;
    fn register(&self, _ref: Ref) -> Result<ID, SessionError>;
    fn register_module(&self, _ref: Ref) -> Result<ModuleId, SessionError>;
    fn register_element(&self, _ref: Ref) -> Result<ElementId, SessionError>;
    fn register_location(&self, _ref: Ref) -> Result<LocationId, SessionError>;

    fn notify_all(&self, events: Vec<SessionEvent>) -> Result<(), SessionError>;
}

impl TLocalSession for Arc<RwLock<LocalSession>> {
    fn get_location(&self, location_id: &LocationPath) -> Result<LRow, SessionError> {
        if let Some(location) = &self.read()?.location {
            let mut loc = location.clone();
            for i in location_id.0.clone() {
                let tmp_loc;
                if let Some(Some(location)) = loc.read()?.locations.get(i as usize) {
                    tmp_loc = location.clone()
                } else {
                    return Err(SessionError::InvalidLocation);
                }
                loc = tmp_loc
            }

            return Ok(loc);
        }
        Err(SessionError::InvalidLocation)
    }

    fn get_element(&self, element_id: &ElementPath) -> Result<ERow, SessionError> {
        if let Some(Some(element)) = self
            .get_location(&element_id.location_id)?
            .read()?
            .elements
            .get(element_id.index as usize)
        {
            Ok(element.clone())
        } else {
            Err(SessionError::ElementDoNotExist)
        }
    }

    fn get_module(&self, module_id: &ModulePath) -> Result<MRow, SessionError> {
        if let Some(Some(module)) = self.read()?.modules.get(module_id.0 as usize) {
            Ok(module.clone())
        } else {
            Err(SessionError::InvalidModule)
        }
    }

    fn add_module(
        &self,
        module: Box<dyn TModule>,
        path: Option<PathBuf>,
        info: Option<ModuleInfo>,
    ) -> Result<ModuleId, SessionError> {
        enum Res {
            ModulePath(Ref),
            ModuleId(ModuleId),
            None,
        }
        let result = Res::None;
        let mut notifications = Vec::new();

        'install_module: {
            if let Some(info) = info {
                let ref_ = Arc::new(RwLock::new(info.id.into()));
                let module = Arc::new(RwLock::new(Module {
                    name: info.name.clone(),
                    desc: info.desc.clone(),
                    module,
                    proxy: info.proxy,
                    settings: info.settings.clone(),
                    element_settings: info.element_settings.clone(),
                    ref_id: ref_.clone(),
                    path,
                    location_settings: info.location_settings.clone(),
                }));
                let module_old = self
                    .read()?
                    .modules
                    .get(info.id.0 as usize)
                    .map(std::clone::Clone::clone);
                if let Some(module_old) = module_old {
                    if let Some(module_old) = module_old {
                        if module_old.read()?.module.get_uid() == info.uid {
                            // if we had to correct module at the correct location
                            {
                                let mut module = module_old.write()?;
                                module.name = info.name;
                                module.desc = info.desc;
                                module.proxy = info.proxy;
                                module.settings = info.settings;
                                module.element_settings = info.element_settings;
                            }
                            result = Res::ModulePath(module.read()?.ref_id.clone());
                            break 'install_module;
                        } else {
                            // if we not have the correct module
                            // that means we need to move to the top
                            // and add our module at the module id
                            let mut lock = self.write()?;
                            let len = lock.modules.len();
                            lock.modules.push(Some(module));
                            let old = lock.modules.swap_remove(info.id.0 as usize);
                            lock.modules.push(old);
                            let (last_id, new_id) = {
                                let old = module_old.read()?;
                                let mut id = old.ref_id.write()?;
                                let id = id.module_mut()?;
                                let last = id.0 .0;
                                id.0 .0 = len as u64;
                                (last, id.0 .0)
                            };

                            notifications.push(SessionEvent::ModuleIdChanged(
                                ModulePath(last_id),
                                ModulePath(new_id),
                            ));

                            result = Res::ModulePath(ref_);
                            break 'install_module;
                        }
                    } else {
                        // if has a avalibile slot but no module in it
                        let mut s = self.write()?;
                        s.modules.push(Some(module));
                        s.modules.swap_remove(info.id.0 as usize);

                        result = Res::ModulePath(ref_);
                        break 'install_module;
                    }
                } else {
                    // if we don't have enouch modules
                    // we need to add empty modules until the disire id
                    let mut s = self.write()?;
                    loop {
                        if s.modules.len() == info.id.0 as usize {
                            s.modules.push(Some(module));

                            notifications.push(SessionEvent::NewModule(info.id));

                            result = Res::ModulePath(ref_);
                            break 'install_module;
                        }
                        s.modules.push(None);
                    }
                }
            }
            // if other module has the uid with the new module will be replaced and will
            // be returned as the new module

            {
                let len = self.get_modules_len()?;
                let uid = module.get_uid();
                for m in self.get_modules(0..len)? {
                    if m.uid()? == uid {
                        self.get_module(&m)?.write()?.module = module;
                        result = Res::ModuleId(m);
                        break 'install_module;
                    }
                }
            }
            // find if is a empty module id
            // that should be replaced by the new module
            let mut new_id = self.get_modules_len()?;
            let mut finded = None;
            for (i, modu) in self.write()?.modules.iter().enumerate() {
                if modu.is_none() {
                    new_id = i;
                    finded = Some(i);
                    break;
                }
            }

            let info = Arc::new(RwLock::new(ModulePath(new_id as u64).into()));

            let mut settings = Values::new();
            let mut element_settings = Values::new();
            let mut location_settings = Values::new();

            module.init_settings(&mut settings)?;
            module.init_element_settings(&mut element_settings)?;
            module.init_location_settings(&mut location_settings)?;

            let module = Module {
                name: module.get_name(),
                desc: module.get_desc(),
                module,
                path,
                proxy: 0,
                settings,
                element_settings,
                location_settings,
                ref_id: info.clone(),
            };

            // we need to remove and add in a single Atomic action
            // because if a other thread trying to get a module after the removed module
            // will have problems
            let module = Arc::new(RwLock::new(module));
            {
                let mut s = self.write()?;
                s.modules.push(Some(module.clone()));
                if let Some(finded) = finded {
                    s.modules.swap_remove(finded);
                }
            }

            result = Res::ModulePath(info);
        }

        let _ = self.notify_all(notifications);

        let result = match result {
            Res::ModulePath(path) => {
                let id = self.register_module(path)?;

                notifications.push(SessionEvent::NewModule(id.clone()));

                if let Err(error) = module.init(id.clone()) {
                    return Err(SessionError::CannotInstallModule(Box::new(error)));
                }
                Ok(id)
            }
            Res::ModuleId(id) => Ok(id),
            Res::None => panic!("This should not happen!"),
        };

        result
    }

    fn notify_all(&self, events: Vec<SessionEvent>) -> Result<(), SessionError> {
        let mut locations = vec![self.get_default_location()?];
        let mut new_range = 0..1;

        while !new_range.is_empty() {
            let start_pos = locations.len();
            let locat = locations[new_range.clone()].to_vec();
            for location in locat {
                let len = location.get_locations_len()?;
                for loc in location.get_locations(0..len)? {
                    locations.push(loc);
                }
            }

            new_range = start_pos..locations.len();
        }

        for event in events {
            for location in locations.iter() {
                let _ = location.notify(Event::SessionEvent(event.clone()));
                let len = location.get_elements_len()?;
                for element in location.get_elements(0..len)? {
                    let _ = element.notify(Event::SessionEvent(event.clone()));
                }
            }

            if let Some(callback) = &self.read()?.callback {
                callback(event)
            }
        }

        Ok(())
    }

    fn _register(&self, _ref: Ref) -> Result<UID, SessionError> {
        let id;
        {
            let mut s = self.write()?;
            id = s.refs.len() as u64;
            s.refs.push(_ref);
        }
        Ok(id)
    }

    fn register(&self, _ref: Ref) -> Result<ID, SessionError> {
        let c = _ref.clone();
        let mut _ref = _ref.write()?;
        match &*_ref {
            Path::UnRElement(path) => {
                let id = self._register(c)?;
                std::mem::replace(&mut *_ref, Path::Element(path.clone(), id));
            }
            Path::UnRLocation(path) => {
                let id = self._register(c)?;
                std::mem::replace(&mut *_ref, Path::Location(path.clone(), id));
            }
            Path::UnRModule(path) => {
                let id = self._register(c)?;
                std::mem::replace(&mut *_ref, Path::Module(path.clone(), id));
            }
            _ => {
                panic!()
            }
        }

        todo!()
    }

    fn register_module(&self, _ref: Ref) -> Result<ModuleId, SessionError> {
        todo!()
    }

    fn register_element(&self, _ref: Ref) -> Result<ElementId, SessionError> {
        todo!()
    }

    fn register_location(&self, _ref: Ref) -> Result<LocationId, SessionError> {
        todo!()
    }
}

impl TSession for Arc<RwLock<LocalSession>> {
    fn load_module(&self, path: PathBuf) -> Result<MRef, SessionError> {
        let module = RawModule::new_module(&path);

        match module {
            Ok(module) => Ok(self.add_module(module, Some(path.clone()), None)?),
            Err(err) => Err(SessionError::RawModule(err)),
        }
    }

    fn remove_module(&self, module_id: ModulePath) -> Result<MRow, SessionError> {
        let _ = self.notify_all(vec![SessionEvent::DestroyedModule(module_id)]);

        let index = module_id.0;
        {
            let module = self.read()?.modules[index as usize].clone();
            let Some(module) = module else{return Err(SessionError::InvalidModule)};
            let module = module.read()?;
            let info = module.ref_id.clone();
            self.write()?
                .actions
                .retain(|e| *e.read().unwrap().owner.read().unwrap() != *info.read().unwrap());
        }
        let mut module = self.write()?.modules.remove(index as usize);
        let module = module.take().unwrap();

        let mut notifications = Vec::new();

        for (i, module) in self.write()?.modules.iter().enumerate() {
            if let Some(module) = module {
                let info = &module.write()?.ref_id;
                let last = info.read()?.index;
                let mut new = last;
                new.0 = i as u64;
                info.write()?.index = new;
                notifications.push(SessionEvent::ModuleIdChanged(last, new));
            }
        }

        let _ = self.notify_all(notifications);

        Ok(module)
    }

    fn load_module_info(&self, info: ModuleInfo) -> Result<MRef, SessionError> {
        let mut path = info.path.clone();
        let module = if let Some(path) = &path {
            RawModule::new_module(path)?
        } else {
            let modules = get_modules();
            let modules = modules
                .iter()
                .filter_map(|m| m.to_str())
                .filter(|m| m.contains(&info.name))
                .collect::<Vec<&str>>();
            let Some(module_path) = modules.first() else{return Err(SessionError::CannotLoadModuleInfo)};
            let p = PathBuf::from(module_path);
            path = Some(p.clone());
            let module = RawModule::new_module(&p)?;
            let protocols = module.accepted_protocols();
            // !TODO: for filetypes
            if info.supports_protocols.len()
                == info
                    .supports_protocols
                    .iter()
                    .filter(|proto| protocols.contains(proto))
                    .collect::<Vec<&String>>()
                    .len()
            {
                module
            } else {
                return Err(SessionError::CannotLoadModuleInfo);
            }
        };

        self.add_module(module, path, Some(info))
    }

    fn find_module(&self, info: ModuleInfo) -> Result<MRef, SessionError> {
        let modules_len = self.get_modules_len()?;
        let modules = self.get_modules(0..modules_len)?;

        let modules: Vec<&MRef> = modules
            .iter()
            .filter(|module| {
                let Ok(accepted_protocols) = module.accepted_protocols() else {return false};
                let len = accepted_protocols
                    .iter()
                    .filter(|accepted| info.supports_protocols.contains(accepted))
                    .collect::<Vec<&String>>()
                    .len();
                len == info.supports_protocols.len()
            })
            .collect();

        if let Some(module) = modules.first() {
            Ok((*module).clone())
        } else {
            Err(SessionError::CannotFindModule)
        }
    }

    fn register_action(
        &self,
        module_id: &ModulePath,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError> {
        self.write()?.actions.push(Arc::new(RwLock::new(Action {
            name,
            owner: Arc::new(RwLock::new(RefModulePath {
                session: Some(self.c()),
                index: *module_id,
            })),
            input: values,
            callback,
        })));
        Ok(())
    }

    fn remove_action(&self, owner: &ModulePath, name: String) -> Result<(), SessionError> {
        let mut finded = None;
        for (i, action) in self.read()?.actions.iter().enumerate() {
            let action = action.read()?;
            if action.owner.read()?.index == *owner && action.name == name {
                finded = Some(i);
                break;
            }
        }
        if let Some(finded) = finded {
            self.write()?.actions.remove(finded);
        }
        Ok(())
    }

    fn get_actions(
        &self,
        range: Range<usize>,
    ) -> Result<Vec<(String, MRef, Vec<(String, Value)>)>, SessionError> {
        let mut res = Vec::new();
        for action in self.read()?.actions[range].iter() {
            let action = action.read()?;
            res.push((
                action.name.clone(),
                action.owner.clone(),
                action.input.clone(),
            ));
        }

        Ok(res)
    }

    fn get_actions_len(&self) -> Result<usize, SessionError> {
        Ok(self.read()?.actions.len())
    }

    fn run_action(
        &self,
        owner: &ModulePath,
        name: String,
        data: Vec<Type>,
    ) -> Result<(), SessionError> {
        let mut finded = None;
        for (i, action) in self.read()?.actions.iter().enumerate() {
            let action = action.read()?;
            if action.owner.read()?.index == *owner && action.name == name {
                finded = Some(i);
                break;
            }
        }

        let info = self.get_module(owner)?.read()?.ref_id.clone();

        if let Some(finded) = finded {
            let action = self.read()?.actions[finded].clone();
            (action.read()?.callback)(info, data);
        }
        Ok(())
    }

    fn get_modules_len(&self) -> Result<usize, SessionError> {
        Ok(self.read()?.modules.len())
    }

    fn get_modules(&self, range: Range<usize>) -> Result<Vec<ModuleId>, SessionError> {
        let mut modules = Vec::new();

        for module in self.read()?.modules[range].iter().flatten() {
            let info = &module.write()?.ref_id;
            let info = info.read()?.module()?.clone();
            modules.push(info)
        }

        Ok(modules)
    }

    fn module_get_name(&self, module_id: &ModulePath) -> Result<String, SessionError> {
        Ok(self.get_module(module_id)?.read()?.name.clone())
    }

    fn module_set_name(&self, module_id: &ModulePath, name: String) -> Result<(), SessionError> {
        self.get_module(module_id)?.write()?.name = name;
        Ok(())
    }

    fn module_get_default_name(&self, module_id: &ModulePath) -> Result<String, SessionError> {
        let module = self.get_module(module_id)?;
        let name = module.read()?.module.get_name();
        Ok(name)
    }

    fn module_get_uid(&self, module_id: &ModulePath) -> Result<UID, SessionError> {
        let m = self.get_module(module_id)?;
        let uid = m.read()?.module.get_uid();
        Ok(uid)
    }

    fn module_get_version(&self, module_id: &ModulePath) -> Result<String, SessionError> {
        let m = self.get_module(module_id)?;
        let version = m.read()?.module.get_version();
        Ok(version)
    }

    fn module_supported_versions(
        &self,
        module_id: &ModulePath,
    ) -> Result<Range<u64>, SessionError> {
        let m = self.get_module(module_id)?;
        let versions = m.read()?.module.supported_versions();
        Ok(versions)
    }

    fn module_get_desc(&self, module_id: &ModulePath) -> Result<String, SessionError> {
        Ok(self.get_module(module_id)?.read()?.desc.clone())
    }

    fn module_set_desc(&self, module_id: &ModulePath, desc: String) -> Result<(), SessionError> {
        self.get_module(module_id)?.write()?.desc = desc;
        Ok(())
    }

    fn module_get_default_desc(&self, module_id: &ModulePath) -> Result<String, SessionError> {
        let module = self.get_module(module_id)?;
        let desc = module.read()?.module.get_desc();
        Ok(desc)
    }

    fn module_get_proxy(&self, module_id: &ModulePath) -> Result<usize, SessionError> {
        Ok(self.get_module(module_id)?.read()?.proxy)
    }

    fn module_set_proxy(&self, module_id: &ModulePath, proxy: usize) -> Result<(), SessionError> {
        self.get_module(module_id)?.write()?.proxy = proxy;
        Err(SessionError::InvalidModule)
    }

    fn module_get_settings(&self, module_id: &ModulePath) -> Result<Values, SessionError> {
        Ok(self.get_module(module_id)?.read()?.settings.clone())
    }

    fn module_set_settings(
        &self,
        module_id: &ModulePath,
        data: Values,
    ) -> Result<(), SessionError> {
        self.get_module(module_id)?.write()?.settings = data;
        Ok(())
    }

    fn module_get_element_settings(&self, module_id: &ModulePath) -> Result<Values, SessionError> {
        Ok(self.get_module(module_id)?.read()?.element_settings.clone())
    }

    fn module_set_element_settings(
        &self,
        module_id: &ModulePath,
        data: Values,
    ) -> Result<(), SessionError> {
        self.get_module(module_id)?.write()?.element_settings = data;
        Ok(())
    }

    fn module_get_location_settings(&self, module_id: &ModulePath) -> Result<Values, SessionError> {
        Ok(self
            .get_module(module_id)?
            .read()?
            .location_settings
            .clone())
    }

    fn module_set_location_settings(
        &self,
        module_id: &ModulePath,
        data: Values,
    ) -> Result<(), SessionError> {
        self.get_module(module_id)?.write()?.location_settings = data;
        Ok(())
    }

    fn module_init_location(
        &self,
        module_id: &ModulePath,
        location_id: &LocationPath,
    ) -> Result<(), SessionError> {
        let location_info = self.get_location(location_id)?.read()?.ref_id.clone();
        self.get_module(module_id)?
            .read()?
            .module
            .init_location(location_info)?;
        Ok(())
    }

    fn module_init_element(
        &self,
        module_id: &ModulePath,
        element_id: &ElementPath,
    ) -> Result<(), SessionError> {
        let module = self.get_module(module_id)?;
        let module = module.read()?;
        let element = self.get_element(element_id)?;

        {
            let mut element = element.write()?;
            element.element_data = module.element_settings.clone();
            element.settings = module.settings.clone();
        }
        module.module.init_element(element)?;

        Ok(())
    }

    fn module_accept_url(&self, module_id: &ModulePath, url: String) -> Result<bool, SessionError> {
        Ok(self.get_module(module_id)?.read()?.module.accept_url(url))
    }

    fn module_accept_extension(
        &self,
        module_id: &ModulePath,
        filename: &str,
    ) -> Result<bool, SessionError> {
        Ok(self
            .get_module(module_id)?
            .read()?
            .module
            .accept_extension(filename))
    }

    fn module_accepted_protocols(
        &self,
        module_id: &ModulePath,
    ) -> Result<Vec<String>, SessionError> {
        Ok(self
            .get_module(module_id)?
            .read()?
            .module
            .accepted_protocols())
    }

    fn module_accepted_extensions(
        &self,
        module_id: &ModulePath,
    ) -> Result<Vec<String>, SessionError> {
        Ok(self
            .get_module(module_id)?
            .read()?
            .module
            .accepted_extensions())
    }

    fn module_step_element(
        &self,
        module_id: &ModulePath,
        element_id: &ElementPath,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError> {
        let module;
        {
            let m = self.get_module(module_id)?;
            module = m.read()?.module.c();
        }

        let element = self.get_element(element_id)?;

        let control_flow = Mutex::new(control_flow);
        let storage = Mutex::new(storage);

        let result = {
            let mut control_flow = control_flow.lock().unwrap();
            let mut storage = storage.lock().unwrap();
            std::panic::catch_unwind(move || {
                module.step_element(element, &mut control_flow, &mut storage)
            })
        };

        let control_flow = control_flow.into_inner().unwrap();
        let storage = storage.into_inner().unwrap();

        match result {
            Ok(res) => {
                if let Err(err) = res {
                    log::error!("{err:?}");
                }
                Ok((control_flow, storage))
            }
            Err(_) => Err(SessionError::StepOnElementPaniced),
        }
    }

    fn module_step_location(
        &self,
        module_id: &ModulePath,
        location_id: &LocationPath,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError> {
        let module;
        {
            let m = self.get_module(module_id)?;
            module = m.read()?.module.c();
        }

        let location = self.get_location(location_id)?;

        let mut control_flow = control_flow;
        let mut storage = storage;

        module.step_location(location, &mut control_flow, &mut storage)?;
        Ok((control_flow, storage))
    }

    fn create_element(&self, name: &str, location_id: &LocationPath) -> Result<ERef, SessionError> {
        let mut element_uid = self.get_location(location_id)?.read()?.elements.len();
        let mut should_replace = None;
        for (i, element) in self
            .get_location(location_id)?
            .read()?
            .elements
            .iter()
            .enumerate()
        {
            if element.is_none() {
                element_uid = i;
                should_replace = Some(i);
                break;
            }
        }

        let element_info = Arc::new(RwLock::new(RefElementPath {
            session: Some(self.c()),
            id: ElementPath {
                location_id: location_id.clone(),
                index: element_uid as u64,
            },
        }));

        let path = self.get_location(location_id)?.read()?.path.join(name);

        let element = Arc::new(RwLock::new(Element {
            name: name.to_owned(),
            desc: String::new(),
            meta: String::new(),
            url: None,
            element_data: Values::new(),
            settings: Values::new(),
            module: None,
            statuses: Vec::new(),
            status: usize::MAX,
            data: Data::File(path, None),
            progress: 0.0,
            should_save: false,
            enabled: false,
            thread: None,
            ref_id: element_info.clone(),
            events: Arc::new(RwLock::new(Events::default())),
            is_error: false,
        }));

        self.get_location(location_id)?
            .write()?
            .elements
            .push(Some(element));
        if let Some(should_replace) = should_replace {
            self.get_location(location_id)?
                .write()?
                .elements
                .swap_remove(should_replace);
        }

        let _ = self.notify_all(vec![SessionEvent::NewElement(
            element_info.read()?.id.clone(),
        )]);

        Ok(element_info)
    }

    fn load_element_info(&self, info: ElementInfo) -> Result<ERef, SessionError> {
        let location = self.get_location(&info.id.location_id)?;
        let location_ref = self.get_location_id(&info.id.location_id)?;
        let location_id = info.id.location_id;

        let element_id = Arc::new(RwLock::new(RefElementPath {
            session: Some(self.c()),
            id: ElementPath {
                location_id,
                index: info.id.index,
            },
        }));

        // TODO: better path system
        let path = if let Data::File(path, _) = info.data {
            path
        } else {
            location_ref.get_path()?.join(&info.name)
        };

        let module = if let Some(module) = info.module {
            Some(self.find_module(module)?)
        } else {
            None
        };

        let element = Arc::new(RwLock::new(Element {
            name: info.name,
            desc: info.desc,
            meta: info.meta,
            url: info.url,
            element_data: info.element_data,
            settings: info.module_data,
            module,
            data: Data::File(path, None),
            should_save: info.should_save,
            enabled: false,
            thread: None,
            ref_id: element_id.clone(),
            events: Arc::new(RwLock::new(Events::default())),

            progress: 0.0,
            statuses: vec![],
            status: usize::MAX,
            is_error: false,
        }));

        let mut notifications = Vec::new();

        {
            let mut location = location.write()?;
            if let Some(other_element) = location.elements.get(info.id.index as usize) {
                if let Some(other_element) = other_element.clone() {
                    let last_id = other_element.read()?.ref_id.read()?.id.clone();
                    let other_element_new_id = location.elements.len();
                    location.elements.push(Some(other_element.clone()));
                    let new_id = {
                        let other_element = other_element.read()?;
                        let mut uid = other_element.ref_id.write()?;
                        uid.id.index = other_element_new_id as u64;
                        uid.id.clone()
                    };
                    notifications.push(SessionEvent::ElementIdChanged(last_id, new_id));
                }
                location.elements.push(Some(element));
                location.elements.swap_remove(info.id.index as usize);
            } else {
                loop {
                    if location.elements.len() == info.id.index as usize {
                        location.elements.push(Some(element));
                        break;
                    } else {
                        location.elements.push(None)
                    }
                }
            }
        }

        notifications.push(SessionEvent::NewElement(element_id.read()?.id.clone()));
        let _ = self.notify_all(notifications);

        Ok(element_id)
    }

    fn move_element(
        &self,
        element_id: &ElementPath,
        location_id: &LocationPath,
    ) -> Result<(), SessionError> {
        let elem = self.destroy_element(element_id.clone())?;
        let info = elem.read()?.ref_id.clone();
        let new_uid = self.get_location(location_id)?.read()?.elements.len();
        let last = info.read()?.id.clone();
        let new = ElementPath {
            index: new_uid as u64,
            location_id: location_id.clone(),
        };

        elem.read()?.ref_id.write()?.id = new.clone();
        self.get_location(location_id)?
            .write()?
            .elements
            .push(Some(elem));

        let _ = self.notify_all(vec![SessionEvent::ElementIdChanged(last, new)]);
        Ok(())
    }

    fn destroy_element(&self, element_id: ElementPath) -> Result<ERow, SessionError> {
        let _ = self.get_element_id(&element_id)?;

        let _ = self.notify_all(vec![SessionEvent::DestroyedElement(element_id.clone())]);

        let Some(element) = self
            .get_location(&element_id.location_id)?
            .write()
            ?.elements
            .remove(element_id.index as usize)else{return Err(SessionError::EmptyElement)};

        let mut notifications = Vec::new();
        for (i, element) in self
            .get_location(&element.read()?.ref_id.read()?.id.location_id)?
            .read()?
            .elements
            .iter()
            .enumerate()
        {
            let Some(element) = element else {continue};
            let info = element.read()?.ref_id.clone();
            let last = info.read()?.id.clone();
            info.write()?.id.index = i as u64;

            notifications.push(SessionEvent::ElementIdChanged(
                last,
                ElementPath {
                    index: i as u64,
                    location_id: info.read()?.id.location_id.clone(),
                },
            ));
        }
        for notification in notifications {
            let _ = self.notify_all(vec![notification]);
        }
        Ok(element)
    }

    fn element_get_name(&self, element_id: &ElementPath) -> Result<String, SessionError> {
        Ok(self.get_element(element_id)?.read()?.name.clone())
    }

    fn element_set_name(&self, element_id: &ElementPath, name: &str) -> Result<(), SessionError> {
        self.get_element(element_id)?.write()?.name = name.to_owned();
        Ok(())
    }

    fn element_get_desc(&self, element_id: &ElementPath) -> Result<String, SessionError> {
        Ok(self.get_element(element_id)?.read()?.desc.clone())
    }

    fn element_set_desc(&self, element_id: &ElementPath, desc: &str) -> Result<(), SessionError> {
        self.get_element(element_id)?.write()?.desc = desc.to_owned();
        Ok(())
    }

    fn element_get_meta(&self, element_id: &ElementPath) -> Result<String, SessionError> {
        Ok(self.get_element(element_id)?.read()?.meta.clone())
    }

    fn element_set_meta(&self, element_id: &ElementPath, meta: &str) -> Result<(), SessionError> {
        self.get_element(element_id)?.write()?.meta = meta.to_owned();
        Ok(())
    }

    fn element_get_url(&self, element_id: &ElementPath) -> Result<Option<String>, SessionError> {
        Ok(self.get_element(element_id)?.read()?.url.clone())
    }

    fn element_set_url(
        &self,
        element_id: &ElementPath,
        url: Option<String>,
    ) -> Result<(), SessionError> {
        self.get_element(element_id)?.write()?.url = url;
        Ok(())
    }

    fn element_get_element_data(&self, element_id: &ElementPath) -> Result<Values, SessionError> {
        Ok(self.get_element(element_id)?.read()?.element_data.clone())
    }

    fn element_set_element_data(
        &self,
        element_id: &ElementPath,
        data: Values,
    ) -> Result<(), SessionError> {
        self.get_element(element_id)?.write()?.element_data = data;
        Ok(())
    }

    fn element_get_module_data(&self, element_id: &ElementPath) -> Result<Values, SessionError> {
        Ok(self.get_element(element_id)?.read()?.settings.clone())
    }

    fn element_set_module_data(
        &self,
        element_id: &ElementPath,
        data: Values,
    ) -> Result<(), SessionError> {
        self.get_element(element_id)?.write()?.settings = data;
        Ok(())
    }

    fn element_get_module(&self, element_id: &ElementPath) -> Result<Option<MRef>, SessionError> {
        if let Some(module) = &self.get_element(element_id)?.read()?.module {
            Ok(Some(module.clone()))
        } else {
            Ok(None)
        }
    }

    fn element_set_module(
        &self,
        element_id: &ElementPath,
        module: Option<ModulePath>,
    ) -> Result<(), SessionError> {
        self.get_element(element_id)?.write()?.module = match &module {
            Some(module_id) => Some(self.get_module_id(module_id)?),
            None => None,
        };
        Ok(())
    }

    fn element_get_statuses(&self, element_id: &ElementPath) -> Result<Vec<String>, SessionError> {
        Ok(self.get_element(element_id)?.read()?.statuses.clone())
    }

    fn element_set_statuses(
        &self,
        element_id: &ElementPath,
        statuses: Vec<String>,
    ) -> Result<(), SessionError> {
        self.get_element(element_id)?.write()?.statuses = statuses;
        Ok(())
    }

    fn element_get_status(&self, element_id: &ElementPath) -> Result<usize, SessionError> {
        Ok(self.get_element(element_id)?.read()?.status)
    }

    fn element_set_status(
        &self,
        element_id: &ElementPath,
        status: usize,
    ) -> Result<(), SessionError> {
        let statuses = self.get_element(element_id)?.read()?.statuses.len();
        if status < statuses {
            self.get_element(element_id)?.write()?.status = status;
            Ok(())
        } else {
            Err(SessionError::InvalidElementStatus)
        }
    }

    fn element_get_data(&self, element_id: &ElementPath) -> Result<Data, SessionError> {
        Ok(self.get_element(element_id)?.read()?.data.clone())
    }

    fn element_set_data(&self, element_id: &ElementPath, data: Data) -> Result<(), SessionError> {
        self.get_element(element_id)?.write()?.data = data;
        Ok(())
    }

    fn element_get_progress(&self, element_id: &ElementPath) -> Result<f32, SessionError> {
        Ok(self.get_element(element_id)?.read()?.progress)
    }

    fn element_set_progress(
        &self,
        element_id: &ElementPath,
        progress: f32,
    ) -> Result<(), SessionError> {
        self.get_element(element_id)?.write()?.progress = progress;
        Ok(())
    }

    fn element_get_should_save(&self, element_id: &ElementPath) -> Result<bool, SessionError> {
        Ok(self.get_element(element_id)?.read()?.should_save)
    }

    fn element_set_should_save(
        &self,
        element_id: &ElementPath,
        should_save: bool,
    ) -> Result<(), SessionError> {
        self.get_element(element_id)?.write()?.should_save = should_save;
        Ok(())
    }

    fn element_get_enabled(&self, element_id: &ElementPath) -> Result<bool, SessionError> {
        Ok(self.get_element(element_id)?.read()?.enabled)
    }

    fn element_set_enabled(
        &self,
        element_id: &ElementPath,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError> {
        let element = self.get_element(element_id)?;
        element.write()?.enabled = enabled;
        if !enabled {
            return Ok(());
        }

        let tmp_element = element.clone();
        element.write()?.thread = Some(std::thread::spawn(move || {
            let element = tmp_element.clone();
            let element_info = element.read().unwrap().ref_id.clone();
            let mut control_flow = ControlFlow::Run;
            let mut storage = if let Some(storage) = storage {
                storage
            } else {
                Storage::default()
            };

            loop {
                let id = element.read().unwrap().ref_id.read().unwrap().id.clone();
                logger::LOGGER_WHO_IAM
                    .with(|w| *w.write().unwrap() = logger::Iam::Element { uid: 0, id });
                // TODO: Change to be async spawn
                if let ControlFlow::Break = control_flow {
                    break;
                }
                let enabled = element.read().unwrap().enabled;
                if enabled {
                    let module;
                    let has_module = element.read().unwrap().module.is_some();
                    if has_module {
                        if let Some(m) = &element.read().unwrap().module {
                            module = Some(m.clone());
                        } else {
                            // this should not happend but is possibile because is multi threaded
                            // is not problem if this happend
                            module = None;
                        }
                    } else {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        continue;
                    }
                    if let Some(module) = module {
                        match module.step_element(
                            &element_info.read().unwrap().id.clone(),
                            control_flow,
                            storage,
                        ) {
                            Ok((cf, s)) => {
                                control_flow = cf;
                                storage = s;
                            }
                            Err(error) => {
                                log::error!(
                                    "Module crashed: {}, with error: {error:?}",
                                    module.get_name().unwrap(),
                                );

                                break;
                            }
                        }
                    }
                } else {
                    break;
                }
            }

            element.write().unwrap().enabled = false;
        }));

        Ok(())
    }

    fn element_is_error(&self, element_id: &ElementPath) -> Result<bool, SessionError> {
        Ok(self.get_element(element_id)?.read()?.is_error)
    }

    fn element_resolv_module(&self, element_id: &ElementPath) -> Result<bool, SessionError> {
        let len = self.get_modules_len()?;

        let mut module = None;

        if let Some(url) = self.get_element(element_id)?.read()?.url.clone() {
            for tmp_module in self.get_modules(0..len)? {
                if self
                    .get_module(&tmp_module.read()?.index)?
                    .read()?
                    .module
                    .accept_url(url.clone())
                {
                    module = Some(tmp_module);
                    break;
                }
            }
        }

        if let Some(module) = module {
            self.element_set_module(element_id, Some(module.id()))?;
            return Ok(true);
        }

        Ok(false)
    }

    fn element_wait(&self, element_id: &ElementPath) -> Result<(), SessionError> {
        let thread = self.get_element(element_id)?.write()?.thread.take();
        if let Some(thread) = thread {
            thread.join().unwrap();
        }
        Ok(())
    }

    fn element_get_element_info(
        &self,
        element_id: &ElementPath,
    ) -> Result<ElementInfo, SessionError> {
        let element = self.get_element(element_id)?;
        let mut module = None;
        {
            let settings;
            let element_settings;

            let __module;
            {
                let element = element.read()?;
                __module = element.module.clone();
                element_settings = element.element_data.clone();
                settings = element.settings.clone();
            }

            if let Some(__module) = __module {
                let __module = self.get_module(&__module.read()?.index)?;
                let __module = __module.read()?;

                module = Some(ModuleInfo {
                    name: __module.name.clone(),
                    desc: __module.desc.clone(),
                    proxy: __module.proxy,
                    settings,
                    element_settings,
                    id: __module.ref_id.id(),
                    path: __module.path.clone(),
                    uid: __module.module.get_uid(),
                    version: __module.module.get_version(),
                    supports_protocols: __module.module.accepted_protocols(),
                    supports_file_types: Vec::new(),
                    location_settings: __module.location_settings.clone(), // !TODO: supports file types
                });
            }
        }
        let element = element.read()?;
        Ok(ElementInfo {
            name: element.name.clone(),
            desc: element.desc.clone(),
            meta: element.meta.clone(),
            url: element.url.clone(),
            element_data: element.element_data.clone(),
            module_data: element.settings.clone(),
            module,
            data: element.data.clone(),
            should_save: element.should_save,
            enabled: element.enabled,
            id: element.ref_id.id(),
        })
    }

    fn create_location(
        &self,
        name: &str,
        location_id: &LocationPath,
    ) -> Result<LRef, SessionError> {
        let mut location_uid = location_id.clone();
        let mut uid = self.get_locations_len(location_id)?;
        let mut should_swap = None;
        for (i, location) in self
            .get_location(location_id)?
            .write()?
            .locations
            .iter()
            .enumerate()
        {
            if location.is_none() {
                uid = i;
                should_swap = Some(i);
                break;
            }
        }
        location_uid.push(uid as u64);
        let location_info = Arc::new(RwLock::new(RefLocationPath {
            session: Some(self.c()),
            id: location_uid,
        }));

        let path = self.get_location(location_id)?.read()?.path.join(name);

        let loc = Location {
            name: name.to_owned(),
            desc: String::new(),
            where_is: WhereIsLocation::Local,
            should_save: false,
            elements: Vec::new(),
            locations: Vec::new(),
            ref_id: location_info.clone(),
            module: None,
            path,
            thread: None,
            events: Arc::new(RwLock::new(Events::default())),
            module_settings: Values::new(),
            settings: Values::new(),

            progress: 0.0,
            statuses: vec![],
            status: usize::MAX,
            is_error: false,
            enabled: false,
        };

        let dest = self.get_location(location_id)?;
        dest.write()?
            .locations
            .push(Some(Arc::new(RwLock::new(loc))));

        if let Some(should_swap) = should_swap {
            self.get_location(location_id)?
                .write()?
                .locations
                .swap_remove(should_swap);
        }

        let _ = self.notify_all(vec![SessionEvent::NewLocation(
            location_info.read()?.id.clone(),
        )]);

        Ok(location_info)
    }

    fn load_location_info(&self, info: LocationInfo) -> Result<LRef, SessionError> {
        // TODO: Is a really big problem some were when loading a location can have a self reference to it self and when notify for events will be stuck in a loop until the machine remains without memory
        let location_id = info.id.clone();

        let location_ref = Arc::new(RwLock::new(RefLocationPath {
            session: Some(self.c()),
            id: location_id.clone(),
        }));

        let location_data;
        let module_data;

        let module = if let Some(module_info) = info.module {
            location_data = module_info.element_settings.clone();
            module_data = module_info.settings.clone();
            Some(self.find_module(module_info)?)
        } else {
            location_data = Values::new();
            module_data = Values::new();
            None
        };

        let new_location = Location {
            name: info.name,
            desc: info.desc,
            where_is: info.where_is,
            should_save: info.shoud_save,
            module_settings: location_data,
            settings: module_data,
            elements: Vec::with_capacity(info.elements.len()),
            locations: Vec::with_capacity(info.locations.len()),
            ref_id: location_ref.clone(),
            path: info.path,
            thread: None,
            module,
            events: Arc::new(RwLock::new(Events::default())),

            progress: 0.0,
            statuses: vec![],
            status: usize::MAX,
            is_error: false,
            enabled: false,
        };

        let new_location = Arc::new(RwLock::new(new_location));
        let mut notifications = Vec::new();

        // We should iterate for every location until we find a location with the same id with the
        // new one and replaced only if is empty if not will put the old one inside the new one
        //
        // If don't have as many locations as is needed will create empty locations wintil the
        // location we need to put inside of and create a empty location

        'set_location: {
            let mut location;
            if let Some(tmp_location) = &self.read()?.location {
                location = tmp_location.clone();
            } else {
                return Err(SessionError::DefaultLocationDoNotExist);
            }

            for i in location_id.clone() {
                let tmp_loc;
                let tmp_location = location.clone();
                let tmp_location = tmp_location.read()?;
                // if the contition is false the lock is not droped!
                if let Some(Some(location)) = tmp_location.locations.get(i as usize) {
                    tmp_loc = location.clone()
                } else {
                    // fixed with manual drop
                    drop(tmp_location);
                    let mut new_id = location.read()?.ref_id.read()?.id.clone();
                    new_id.push(i);

                    let where_is;
                    let should_save;

                    {
                        let location = location.read()?;
                        where_is = location.where_is.clone();
                        should_save = location.should_save;
                    }

                    let new_location = Location {
                        name: String::new(),
                        desc: String::new(),
                        where_is,
                        should_save,
                        module_settings: Values::new(),
                        settings: Values::new(),
                        elements: Vec::new(),
                        locations: Vec::new(),
                        ref_id: Arc::new(RwLock::new(RefLocationPath {
                            id: new_id.clone(),
                            session: Some(self.c()),
                        })),
                        path: PathBuf::new(),
                        thread: None,
                        module: None,
                        events: Arc::default(),

                        progress: 0.0,
                        statuses: vec![],
                        status: usize::MAX,
                        is_error: false,
                        enabled: false,
                    };
                    tmp_loc = Arc::new(RwLock::new(new_location));

                    {
                        let mut location = location.write()?;
                        while location.locations.len() < i as usize {
                            location.locations.push(None);
                        }
                        let len = location.locations.len();
                        location.locations.push(Some(tmp_loc.clone()));
                        notifications.push(SessionEvent::NewLocation(new_id));
                        if len >= i as usize {
                            location.locations.swap(len, i as usize);
                            // now update the old location
                            let mut old = location.ref_id.read()?.id.clone();
                            let mut new = old.clone();
                            old.push(i);
                            new.push(len as u64);
                            if let Some(Some(location)) = location.locations.get(len) {
                                let _ref = location.read()?.ref_id.clone();
                                let mut _ref = _ref.write()?;
                                let tmp = _ref.id.last_mut().unwrap();
                                *tmp = len as u64;
                            }
                            notifications.push(SessionEvent::LocationIdChanged(old, new));
                        }
                    }
                }
                location = tmp_loc
            }

            let is_empty = {
                let location = location.read()?;
                location.locations.is_empty() && location.elements.is_empty()
            };
            if is_empty {
                std::mem::swap(&mut *location.write()?, &mut *new_location.write()?);
                notifications.push(SessionEvent::LocationIdChanged(
                    location_id.clone(),
                    location_id,
                ))
            } else {
                {
                    let mut old_location = location.write()?;
                    let mut new_location = new_location.write()?;
                    std::mem::swap(&mut *old_location, &mut new_location);
                }

                let new_id;
                {
                    let mut location = location.write()?;
                    new_id = location.locations.len() as u64;
                    location.locations.push(Some(new_location.clone()));
                }

                let old_id;

                let new_id = {
                    let location = new_location.read()?;
                    let mut info = location.ref_id.write()?;
                    old_id = info.id.clone();
                    info.id.push(new_id);

                    fn change_location_childs(
                        notifications: &mut Vec<SessionEvent>,
                        location: &Location,
                        new_id: u64,
                        depth: usize,
                    ) -> Result<(), SessionError> {
                        for location in location.locations.iter() {
                            let Some(location) = location else {continue};
                            let location = location.read()?;
                            let mut id = location.ref_id.write()?;
                            let old_id = id.id.clone();
                            let len = id.id.len();
                            id.id.insert(len - depth, new_id);
                            change_location_childs(notifications, &location, new_id, depth + 1)?;
                            let new_id = id.id.clone();
                            notifications.push(SessionEvent::LocationIdChanged(old_id, new_id));
                        }
                        Ok(())
                    }

                    change_location_childs(&mut notifications, &location, new_id, 1)?;

                    info.id.clone()
                };

                notifications.push(SessionEvent::LocationIdChanged(old_id, new_id));
            }
            break 'set_location;
        }

        for location in info.locations {
            self.load_location_info(location)?;
        }

        for element in info.elements {
            self.load_element_info(element)?;
        }

        let results: Vec<Result<(), SessionError>> = notifications
            .into_iter()
            .map(|notification| self.notify_all(vec![notification]))
            .collect();

        for result in results {
            // better error system
            result?;
        }

        Ok(location_ref)
    }

    fn get_locations_len(&self, location_id: &LocationPath) -> Result<usize, SessionError> {
        Ok(self.get_location(location_id)?.read()?.locations.len())
    }

    fn get_locations(
        &self,
        location_id: &LocationPath,
        range: Range<usize>,
    ) -> Result<Vec<LRef>, SessionError> {
        let mut location_infos = Vec::new();
        let location = self.get_location(location_id)?;
        for loc in location.read()?.locations[range].iter().flatten() {
            location_infos.push(loc.read()?.ref_id.clone());
        }
        Ok(location_infos)
    }

    fn destroy_location(&self, location_id: LocationPath) -> Result<LRow, SessionError> {
        let _ = self.get_location_id(&location_id)?;
        let _ = self.notify_all(vec![SessionEvent::DestroyedLocation(location_id.clone())]);

        let mut location_uid = location_id;
        if let Some(location_index) = location_uid.pop() {
            let parent_location = self.get_location(&location_uid)?;

            let Some(removed_location) = parent_location
                .write()
                ?
                .locations
                .remove(location_index as usize)else{return Err(SessionError::EmptyLocation)};

            let mut notifications = Vec::new();
            for (i, location) in parent_location.read()?.locations.iter().enumerate() {
                let Some(location) = location else{continue};
                let info = location.read()?.ref_id.clone();
                let last = info.id();
                let mut new = last.clone();
                *new.last_mut().unwrap() = i as u64;

                info.write()?.id = new.clone();

                notifications.push(SessionEvent::LocationIdChanged(last, new));
            }

            let _ = self.notify_all(notifications);

            return Ok(removed_location);
        }
        Err(SessionError::InvalidLocation)
    }

    fn get_default_location(&self) -> Result<LocationId, SessionError> {
        if let Some(location) = &self.read()?.location {
            return Ok(LocationId {
                uid: *location.read()?.ref_id.read()?.location()?.1,
                session: Some(self.c()),
            });
        }
        Err(SessionError::InvalidLocation)
    }

    fn move_location(&self, location_id: UID, to: UID) -> Result<(), SessionError> {
        let location = self.destroy_location(location_id.clone())?;
        let mut new = to.clone();
        new.push(self.get_locations_len(to)? as u64);
        let info = location.read()?.ref_id.clone();
        let last = info.read()?.id.clone();
        location.write()?.ref_id.write()?.id = new.clone();
        self.get_location(to)?
            .write()?
            .locations
            .push(Some(location));

        let _ = self.notify_all(vec![SessionEvent::LocationIdChanged(last, new)]);

        Ok(())
    }

    fn location_get_name(&self, location_id: &LocationPath) -> Result<String, SessionError> {
        Ok(self.get_location(location_id)?.read()?.name.clone())
    }

    fn location_set_name(
        &self,
        location_id: &LocationPath,
        name: &str,
    ) -> Result<(), SessionError> {
        self.get_location(location_id)?.write()?.name = name.to_owned();
        Ok(())
    }

    fn location_get_desc(&self, location_id: &LocationPath) -> Result<String, SessionError> {
        Ok(self.get_location(location_id)?.read()?.desc.clone())
    }

    fn location_set_desc(
        &self,
        location_id: &LocationPath,
        desc: &str,
    ) -> Result<(), SessionError> {
        self.get_location(location_id)?.write()?.desc = desc.to_owned();
        Ok(())
    }

    fn location_get_path(&self, location_id: &LocationPath) -> Result<PathBuf, SessionError> {
        Ok(self.get_location(location_id)?.read()?.path.clone())
    }

    fn location_set_path(
        &self,
        location_id: &LocationPath,
        path: PathBuf,
    ) -> Result<(), SessionError> {
        self.get_location(location_id)?.write()?.path = path;
        Ok(())
    }

    fn location_get_where_is(
        &self,
        location_id: &LocationPath,
    ) -> Result<WhereIsLocation, SessionError> {
        Ok(self.get_location(location_id)?.read()?.where_is.clone())
    }

    fn location_set_where_is(
        &self,
        location_id: &LocationPath,
        where_is: WhereIsLocation,
    ) -> Result<(), SessionError> {
        self.get_location(location_id)?.write()?.where_is = where_is;
        Ok(())
    }

    fn location_get_should_save(&self, location_id: &LocationPath) -> Result<bool, SessionError> {
        Ok(self.get_location(location_id)?.read()?.should_save)
    }

    fn location_set_should_save(
        &self,
        location_id: &LocationPath,
        should_save: bool,
    ) -> Result<(), SessionError> {
        self.get_location(location_id)?.write()?.should_save = should_save;
        Ok(())
    }

    fn location_get_elements_len(&self, location_id: &LocationPath) -> Result<usize, SessionError> {
        Ok(self.get_location(location_id)?.read()?.elements.len())
    }

    fn location_get_elements(
        &self,
        location_id: &LocationPath,
        range: Range<usize>,
    ) -> Result<Vec<ERef>, SessionError> {
        let mut element_infos = Vec::new();
        for element in self.get_location(location_id)?.read()?.elements[range]
            .iter()
            .flatten()
        {
            element_infos.push(element.read()?.ref_id.clone())
        }
        Ok(element_infos)
    }

    fn location_get_module(
        &self,
        location_id: &LocationPath,
    ) -> Result<Option<MRef>, SessionError> {
        Ok(self.get_location(location_id)?.read()?.module.clone())
    }

    fn location_set_module(
        &self,
        location_id: &LocationPath,
        module_id: Option<ModulePath>,
    ) -> Result<(), SessionError> {
        self.get_location(location_id)?.write()?.module = if let Some(module_id) = module_id {
            Some(self.get_module_id(&module_id)?)
        } else {
            None
        };

        Ok(())
    }

    fn location_get_settings(&self, location_id: &LocationPath) -> Result<Values, SessionError> {
        Ok(self.get_location(location_id)?.read()?.settings.clone())
    }

    fn location_set_settings(
        &self,
        location_id: &LocationPath,
        data: Values,
    ) -> Result<(), SessionError> {
        self.get_location(location_id)?.write()?.settings = data;
        Ok(())
    }

    fn location_get_module_settings(
        &self,
        location_id: &LocationPath,
    ) -> Result<Values, SessionError> {
        Ok(self
            .get_location(location_id)?
            .read()?
            .module_settings
            .clone())
    }

    //
    // Session
    //

    fn location_set_module_settings(
        &self,
        location_id: &LocationPath,
        data: Values,
    ) -> Result<(), SessionError> {
        self.get_location(location_id)?.write()?.module_settings = data;
        Ok(())
    }

    fn location_get_statuses(
        &self,
        location_id: &LocationPath,
    ) -> Result<Vec<String>, SessionError> {
        Ok(self.get_location(location_id)?.read()?.statuses.clone())
    }

    fn location_set_statuses(
        &self,
        location_id: &LocationPath,
        statuses: Vec<String>,
    ) -> Result<(), SessionError> {
        self.get_location(location_id)?.write()?.statuses = statuses;
        Ok(())
    }

    fn location_get_status(&self, location_id: &LocationPath) -> Result<usize, SessionError> {
        Ok(self.get_location(location_id)?.read()?.status)
    }

    fn location_set_status(
        &self,
        location_id: &LocationPath,
        status: usize,
    ) -> Result<(), SessionError> {
        self.get_location(location_id)?.write()?.status = status;
        Ok(())
    }

    fn location_get_progress(&self, location_id: &LocationPath) -> Result<f32, SessionError> {
        Ok(self.get_location(location_id)?.read()?.progress)
    }

    fn location_set_progress(
        &self,
        location_id: &LocationPath,
        progress: f32,
    ) -> Result<(), SessionError> {
        self.get_location(location_id)?.write()?.progress = progress;
        Ok(())
    }

    fn location_is_enabled(&self, location_id: &LocationPath) -> Result<bool, SessionError> {
        Ok(self.get_location(location_id)?.read()?.enabled)
    }

    fn location_set_enabled(
        &self,
        location_id: &LocationPath,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn location_is_error(&self, location_id: &LocationPath) -> Result<bool, SessionError> {
        Ok(self.get_location(location_id)?.read()?.is_error)
    }

    fn location_get_location_info(
        &self,
        location_id: &LocationPath,
    ) -> Result<LocationInfo, SessionError> {
        let mut module = None;
        {
            let settings;
            let location_settings;

            let __module;
            {
                let location = self.get_location(location_id)?;
                let location = location.read()?;
                __module = location.module.clone();
                location_settings = location.module_settings.clone();
                settings = location.settings.clone();
            }

            if let Some(__module) = __module {
                let __module = self.get_module(&__module.read()?.index)?;
                let __module = __module.read()?;

                module = Some(ModuleInfo {
                    name: __module.name.clone(),
                    desc: __module.desc.clone(),
                    proxy: __module.proxy,
                    settings,
                    element_settings: __module.element_settings.clone(),
                    id: __module.ref_id.id(),
                    path: __module.path.clone(),
                    uid: __module.module.get_uid(),
                    version: __module.module.get_version(),
                    supports_protocols: __module.module.accepted_protocols(),
                    supports_file_types: Vec::new(),
                    location_settings,
                });
            }
        }

        let mut elements = Vec::new();
        {
            let len = self.location_get_elements_len(location_id)?;
            let __elements = self.location_get_elements(location_id, 0..len)?;
            for element in __elements {
                let info = element.get_element_info()?;
                elements.push(info)
            }
        }

        let mut locations = Vec::new();
        {
            let len = self.get_locations_len(location_id)?;
            let __locations = self.get_locations(location_id, 0..len)?;
            for location in __locations {
                let info = location.get_location_info()?;
                locations.push(info)
            }
        }

        let location = self.get_location(location_id)?;
        let location = location.read()?;

        Ok(LocationInfo {
            name: location.name.clone(),
            desc: location.desc.clone(),
            id: location.ref_id.id(),
            where_is: location.where_is.clone(),
            shoud_save: location.should_save,
            elements,
            locations,
            path: location.path.clone(),
            module,
        })
    }

    fn get_module_id(&self, uid: UID) -> Result<ModuleId, SessionError> {
        self.get_id(uid)
            .map_or(Err(SessionError::InvalidUID), |id| {
                if let UID::Module(id) = id {
                    Ok(id)
                } else {
                    Err(SessionError::IsNotModule)
                }
            })
    }

    fn get_element_id(&self, uid: UID) -> Result<ElementId, SessionError> {
        self.get_id(uid)
            .map_or(Err(SessionError::InvalidUID), |id| {
                if let UID::Element(id) = id {
                    Ok(id)
                } else {
                    Err(SessionError::IsNotElement)
                }
            })
    }

    fn get_location_id(&self, uid: UID) -> Result<LocationId, SessionError> {
        self.get_id(uid)
            .map_or(Err(SessionError::InvalidUID), |id| {
                if let UID::Location(id) = id {
                    Ok(id)
                } else {
                    Err(SessionError::IsNotLocation)
                }
            })
    }

    fn get_id(&self, uid: UID) -> Result<UID, SessionError> {
        let Some(_ref) = self.read()?.refs.get(uid as usize) else {return Err(SessionError::InvalidUID)};
        match _ref.read()? {
            Path::Element(_, uid) => Ok(UID::Element(ElementId {
                uid: *uid,
                session: Some(self.c()),
            })),
            Path::Location(_, uid) => Ok(UID::Location(LocationId {
                uid: *uid,
                session: Some(self.c()),
            })),
            Path::Module(_, uid) => Ok(UID::Module(ModuleId {
                uid: *uid,
                session: Some(self.c()),
            })),
            Path::None => Err(SessionError::InvalidUID),
        }
    }

    fn get_version(&self) -> Result<u64, SessionError> {
        Ok(VERSION)
    }

    fn get_version_text(&self) -> Result<String, SessionError> {
        Ok(format!(
            "MuzzManLib: {VERSION}, LocalSession: {LOCAL_SESSION_VERSION}"
        ))
    }

    fn c(&self) -> Box<dyn TSession> {
        Box::new(self.clone())
    }

    fn notify(&self, uid: UID, event: Event) -> Result<(), SessionError> {
        match TSession::get_ref(self, uid)?.read()? {
            Path::Element(element_id, _) | Path::UnRElement(element_id) => {
                let element = self.get_element(element_id)?;
                let module;
                {
                    let Some(__module) = &element.read()?.module else{
            return Err(SessionError::InvalidModule)
        };

                    module = __module.clone();
                }

                let raw_module;
                {
                    let __module = self.get_module(&module.read()?.index)?;
                    raw_module = __module.read()?.module.c();
                }

                let element_ref = self.get_element_id(element_id)?;
                raw_module.notify(Ref::Element(element_ref), event)?;
            }
            Path::Location(location_id, _) | Path::UnRLocation(location_id) => {
                let location = self.get_location(location_id)?;
                let module;
                {
                    let Some(__module) = &location.read()?.module else{
            return Err(SessionError::InvalidModule)
        };

                    module = __module.clone();
                }

                let raw_module;
                {
                    let __module = self.get_module(&module.read()?.index)?;
                    raw_module = __module.read()?.module.c();
                }

                let location_ref = self.get_location_id(location_id)?;
                raw_module.notify(Ref::Location(location_ref), event)?;
            }
            _ => return Err(SessionError::InvalidUID),
        }
        Ok(())
    }

    fn emit(&self, uid: UID, event: Event) -> Result<(), SessionError> {
        match TSession::get_ref(self, uid)?.read()? {
            Path::Element(element_id) | Path::UnRElement(element_id) => {
                let element = self.get_element(element_id)?;
                let events;
                {
                    events = element.read()?.events.clone()
                }

                let mut event = event;
                match &mut event {
                    Event::Element(info, _) => *info = element_id.clone(),
                    Event::Location(_, _) => return Err(SessionError::IsNotLocation),
                    Event::SessionEvent(_) => return Ok(()),
                }

                events.write()?.new_event(event, self.c());
            }
            Path::Location(location_id) | Path::UnRLocation(location_id) => {
                let location = self.get_location(location_id)?;
                let events;
                {
                    events = location.read()?.events.clone()
                }

                let mut event = event;
                match &mut event {
                    Event::Element(_, _) => return Err(SessionError::IsNotElement),
                    Event::Location(info, _) => *info = location_id.clone(),
                    Event::SessionEvent(_) => return Ok(()),
                }

                events.write()?.new_event(event, self.c());
            }
            _ => return Err(SessionError::InvalidUID),
        }
        Ok(())
    }

    fn subscribe(&self, uid: UID, to: UID) -> Result<(), SessionError> {
        match TSession::get_ref(self, uid)?.read()? {
            Path::Element(element_id) | Path::UnRElement(element_id) => {
                // validate if element is valid
                {
                    let _ = self.get_element(element_id)?;
                }

                let events = match to {
                    UID::Element(e_info) => self.get_element(&e_info)?.read()?.events.clone(),
                    UID::Location(l_info) => self.get_location(&l_info)?.read()?.events.clone(),
                };

                if !events.write()?.subscribe(UID::Element(element_id.clone())) {
                    return Err(SessionError::AlreadySubscribed);
                }
            }
            Path::Location(location_id) | Path::UnRLocation(location_id) => {
                // validate if element is valid
                {
                    let _ = self.get_location(location_id)?;
                }

                let events = match to {
                    UID::Element(e_info) => self.get_element(&e_info)?.read()?.events.clone(),
                    UID::Location(l_info) => self.get_location(&l_info)?.read()?.events.clone(),
                };

                if !events
                    .write()?
                    .subscribe(UID::Location(location_id.clone()))
                {
                    return Err(SessionError::AlreadySubscribed);
                }
            }
            _ => return Err(SessionError::InvalidUID),
        }
        Ok(())
    }

    fn unsubscribe(&self, uid: UID, to: UID) -> Result<(), SessionError> {
        match TSession::get_ref(self, uid)?.read()? {
            Path::Element(element_id) | Path::UnRElement(element_id) => {
                // validate if element is valid
                {
                    let _ = self.get_element(element_id)?;
                }

                let events = match to {
                    UID::Element(e_info) => self.get_element(&e_info)?.read()?.events.clone(),
                    UID::Location(l_info) => self.get_location(&l_info)?.read()?.events.clone(),
                };

                if !events
                    .write()?
                    .unsubscribe(UID::Element(element_id.clone()))
                {
                    return Err(SessionError::AlreadyUnsubscribed);
                }
            }
            Path::Location(location_id) | Path::UnRLocation(location_id) => {
                // validate if element is valid
                {
                    let _ = self.get_location(location_id)?;
                }

                let events = match to {
                    UID::Element(e_info) => self.get_element(&e_info)?.read()?.events.clone(),
                    UID::Location(l_info) => self.get_location(&l_info)?.read()?.events.clone(),
                };

                if !events
                    .write()?
                    .unsubscribe(UID::Location(location_id.clone()))
                {
                    return Err(SessionError::AlreadyUnsubscribed);
                }
            }
            _ => return Err(SessionError::InvalidUID),
        }
        Ok(())
    }

    fn get_ref(&self, uid: UID) -> Result<Ref, SessionError> {
        let Some(path) = self.read().unwrap().refs.get(uid as usize) else {return SessionError::InvalidUID};
        Ok(path.clone())
    }

    fn get_module_from_path(&self, path: ModulePath) -> Result<ModuleId, SessionError> {
        let row = self.get_module(&path)?;
        match row.read().unwrap().ref_id.read().unwrap() {
            Path::Module(_, uid) => {
                return Ok(ModuleId {
                    uid,
                    session: Some(self.c()),
                })
            }
            _ => return Err(SessionError::IsNotModule),
        }
    }

    fn get_element_from_path(&self, path: ElementPath) -> Result<ElementId, SessionError> {
        let row = self.get_element(&path)?;
        match row.read().unwrap().ref_id.read().unwrap() {
            Path::Element(_, uid) => {
                return Ok(ElementId {
                    uid,
                    session: Some(self.c()),
                })
            }
            _ => return Err(SessionError::IsNotModule),
        }
    }

    fn get_location_from_path(&self, path: LocationPath) -> Result<LocationId, SessionError> {
        let row = self.get_location(&path)?;
        match row.read().unwrap().ref_id.read().unwrap() {
            Path::Location(_, uid) => {
                return Ok(LocationId {
                    uid,
                    session: Some(self.c()),
                })
            }
            _ => return Err(SessionError::IsNotModule),
        }
    }
}
