use std::{
    ops::Range,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::prelude::*;

#[derive(Default)]
pub struct LocalSession {
    pub location: Option<LRow>,
    pub modules: Vec<Option<MRow>>,
    pub actions: Vec<Arc<RwLock<Action>>>,
    pub callback: Option<Box<dyn Fn(SessionEvent)>>,
}

impl LocalSession {
    pub fn new_session(self) -> Box<dyn TLocalSession> {
        let session = self;

        let session = Arc::new(RwLock::new(session));

        let location_info = Arc::new(RwLock::new(RefLocation {
            session: Some(session.c()),
            id: Default::default(),
        }));

        let location = Arc::new(RwLock::new(Location {
            name: String::from("Default Location"),
            desc: String::from("Default Location"),
            where_is: WhereIsLocation::Local(LocalLocation {
                path: PathBuf::from("."),
            }),
            shoud_save: false,
            elements: Vec::new(),
            locations: Vec::new(),
            info: location_info,
            module: None,
            path: PathBuf::from("."),
            thread: None,
            events: Arc::new(RwLock::new(Events::default())),
        }));
        session.write().unwrap().location = Some(location);
        Box::new(session)
    }
}

pub trait TLocalSession: TSession {
    fn get_location(&self, info: &LocationId) -> Result<LRow, SessionError>;
    fn get_element(&self, info: &ElementId) -> Result<ERow, SessionError>;
    fn get_module(&self, info: &ModuleId) -> Result<MRow, SessionError>;

    fn add_module(
        &self,
        module: Box<dyn TModule>,
        path: Option<PathBuf>,
        info: Option<ModuleInfo>,
    ) -> Result<MRef, SessionError>;

    fn notify_all(&self, event: SessionEvent) -> Result<(), SessionError>;
}

impl TLocalSession for Arc<RwLock<LocalSession>> {
    fn get_location(&self, info: &LocationId) -> Result<LRow, SessionError> {
        if let Some(location) = &mut self.write().unwrap().location {
            let mut loc = location.clone();
            for i in info.0.clone() {
                let tmp_loc;
                if let Some(Some(location)) = loc.read().unwrap().locations.get(i as usize) {
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

    fn get_element(&self, info: &ElementId) -> Result<ERow, SessionError> {
        if let Some(Some(element)) = self
            .get_location(&info.location_id)?
            .read()
            .unwrap()
            .elements
            .get(info.uid as usize)
        {
            Ok(element.clone())
        } else {
            Err(SessionError::ElementDoNotExist)
        }
    }

    fn get_module(&self, info: &ModuleId) -> Result<MRow, SessionError> {
        if let Some(Some(module)) = self.read().unwrap().modules.get(info.0 as usize) {
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
    ) -> Result<MRef, SessionError> {
        let mut notifications = Vec::new();

        if let Some(info) = info {
            let ref_ = Arc::new(RwLock::new(RefModule {
                uid: info.id,
                session: Some(self.c()),
            }));

            let module = Arc::new(RwLock::new(Module {
                name: info.name.clone(),
                desc: info.desc.clone(),
                module,
                proxy: info.proxy,
                settings: info.settings.clone(),
                element_data: info.element_data.clone(),
                info: ref_.clone(),
                path,
            }));
            let module_old = self
                .read()
                .unwrap()
                .modules
                .get(info.id.0 as usize)
                .map(std::clone::Clone::clone);
            if let Some(module_old) = module_old {
                if let Some(module_old) = module_old {
                    if module_old.read().unwrap().path == info.path {
                        // if we had to correct module at the correct location
                        {
                            let mut module = module_old.write().unwrap();
                            module.name = info.name;
                            module.desc = info.desc;
                            module.proxy = info.proxy;
                            module.settings = info.settings;
                            module.element_data = info.element_data;
                        }
                        return self.get_module_ref(&info.id);
                    } else {
                        // if we not have the correct module
                        // that means we need to move to the top
                        // and add our module at the module id
                        let mut lock = self.write().unwrap();
                        let len = lock.modules.len();
                        lock.modules.push(Some(module));
                        let old = lock.modules.swap_remove(info.id.0 as usize);
                        lock.modules.push(old);
                        let (last_id, new_id) = {
                            let old = module_old.read().unwrap();
                            let mut id = old.info.write().unwrap();
                            let last = id.uid.clone();
                            id.uid.0 = len as u64;
                            (last, id.uid.clone())
                        };

                        notifications.push(SessionEvent::ModuleIdChanged(last_id, new_id));

                        return Ok(ref_);
                    }
                } else {
                    // if has a avalibile slot but no module in it
                    let mut s = self.write().unwrap();
                    s.modules.push(Some(module));
                    s.modules.swap_remove(info.id.0 as usize);
                    return Ok(ref_);
                }
            } else {
                // if we don't have enouch modules
                // we need to add empty modules until the disire id
                let mut s = self.write().unwrap();
                loop {
                    if s.modules.len() == info.id.0 as usize {
                        s.modules.push(Some(module));

                        return Ok(ref_);
                    }
                    s.modules.push(None);
                }
            }
        }
        // if other module has the same default name with the new module will be replaced and will
        // be returned as the new module

        {
            let len = self.get_modules_len()?;
            let name = module.get_name();
            for m in self.get_modules(0..len)? {
                if m.get_default_name()? == name {
                    self.get_module(&m.id())?.write().unwrap().module = module;
                    return Ok(m);
                }
            }
        }
        // find if is a empty module id
        // that should be replaced by the new module
        let mut new_id = self.get_modules_len()?;
        let mut finded = None;
        for (i, modu) in self.write().unwrap().modules.iter().enumerate() {
            if modu.is_none() {
                new_id = i;
                finded = Some(i);
                break;
            }
        }

        let info = Arc::new(RwLock::new(RefModule {
            uid: ModuleId(new_id as u64),
            session: Some(self.c()),
        }));

        let mut settings = Data::new();
        let mut element_data = Data::new();

        module.init_settings(&mut settings);
        module.init_element_settings(&mut element_data);

        let module = Module {
            name: module.get_name(),
            desc: module.get_desc(),
            module,
            proxy: 0,
            settings,
            element_data,
            info: info.clone(),
            path,
        };

        if let Err(error) = module.module.init(info.clone()) {
            return Err(SessionError::CannotInstallModule(error));
        }

        // we need to remove and add in a single Atomic action
        // because if a other thread trying to get a module after the removed module
        // will have problems
        {
            let mut s = self.write().unwrap();
            if let Some(finded) = finded {
                s.modules.remove(finded);
            }

            s.modules
                .insert(new_id, Some(Arc::new(RwLock::new(module))));
        }

        notifications.push(SessionEvent::NewModule(info.read().unwrap().uid));

        for notification in notifications {
            let _ = self.notify_all(notification);
        }

        Ok(info)
    }

    fn notify_all(&self, event: SessionEvent) -> Result<(), SessionError> {
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

        for location in locations {
            let _ = location.notify(Event::SessionEvent(event.clone()));
            let len = location.get_elements_len()?;
            for element in location.get_elements(0..len)? {
                let _ = element.notify(Event::SessionEvent(event.clone()));
            }
        }

        if let Some(callback) = &self.read().unwrap().callback {
            callback(event)
        }

        Ok(())
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

    fn remove_module(&self, info: ModuleId) -> Result<MRow, SessionError> {
        let _ = self.notify_all(SessionEvent::DestroyedModule(info));

        let index = info.0;
        {
            let module = self.read().unwrap().modules[index as usize].clone();
            let Some(module) = module else{return Err(SessionError::InvalidModule)};
            let module = module.read().unwrap();
            let info = module.info.clone();
            self.write()
                .unwrap()
                .actions
                .retain(|e| *e.read().unwrap().owner.read().unwrap() != *info.read().unwrap());
        }
        let mut module = self.write().unwrap().modules.remove(index as usize);
        let module = module.take().unwrap();

        let mut notifications = Vec::new();

        for (i, module) in self.write().unwrap().modules.iter().enumerate() {
            if let Some(module) = module {
                let info = &module.write().unwrap().info;
                let last = info.read().unwrap().uid;
                let mut new = last;
                new.0 = i as u64;
                info.write().unwrap().uid = new;
                notifications.push(SessionEvent::ModuleIdChanged(last, new));
            }
        }

        for notification in notifications {
            let _ = self.notify_all(notification);
        }

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
            RawModule::new_module(&p)?
        };

        self.add_module(module, path, Some(info))
    }

    fn register_action(
        &self,
        module: &ModuleId,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError> {
        self.write()
            .unwrap()
            .actions
            .push(Arc::new(RwLock::new(Action {
                name,
                owner: Arc::new(RwLock::new(RefModule {
                    session: Some(self.c()),
                    uid: *module,
                })),
                input: values,
                callback,
            })));
        Ok(())
    }

    fn remove_action(&self, owner: &ModuleId, name: String) -> Result<(), SessionError> {
        let mut finded = None;
        for (i, action) in self.read().unwrap().actions.iter().enumerate() {
            let action = action.read().unwrap();
            if action.owner.read().unwrap().uid == *owner && action.name == name {
                finded = Some(i);
                break;
            }
        }
        if let Some(finded) = finded {
            self.write().unwrap().actions.remove(finded);
        }
        Ok(())
    }

    fn get_actions(
        &self,
        range: Range<usize>,
    ) -> Result<Vec<(String, MRef, Vec<(String, Value)>)>, SessionError> {
        let mut res = Vec::new();
        for action in self.read().unwrap().actions[range].iter() {
            let action = action.read().unwrap();
            res.push((
                action.name.clone(),
                action.owner.clone(),
                action.input.clone(),
            ));
        }

        Ok(res)
    }

    fn get_actions_len(&self) -> Result<usize, SessionError> {
        Ok(self.read().unwrap().actions.len())
    }

    fn run_action(
        &self,
        owner: &ModuleId,
        name: String,
        data: Vec<Type>,
    ) -> Result<(), SessionError> {
        let mut finded = None;
        for (i, action) in self.read().unwrap().actions.iter().enumerate() {
            let action = action.read().unwrap();
            if action.owner.read().unwrap().uid == *owner && action.name == name {
                finded = Some(i);
                break;
            }
        }

        let info = self.get_module(owner)?.read().unwrap().info.clone();

        if let Some(finded) = finded {
            let action = self.read().unwrap().actions[finded].clone();
            (action.read().unwrap().callback)(info, data);
        }
        Ok(())
    }

    fn get_modules_len(&self) -> Result<usize, SessionError> {
        Ok(self.read().unwrap().modules.len())
    }

    fn get_modules(&self, range: Range<usize>) -> Result<Vec<MRef>, SessionError> {
        let mut modules = Vec::new();

        for module in self.read().unwrap().modules[range].iter().flatten() {
            let info = &module.write().unwrap().info;
            modules.push(info.clone())
        }

        Ok(modules)
    }

    fn module_get_name(&self, module_id: &ModuleId) -> Result<String, SessionError> {
        Ok(self.get_module(module_id)?.read().unwrap().name.clone())
    }

    fn module_set_name(&self, module_id: &ModuleId, name: String) -> Result<(), SessionError> {
        self.get_module(module_id)?.write().unwrap().name = name;
        Ok(())
    }

    fn module_get_default_name(&self, module_id: &ModuleId) -> Result<String, SessionError> {
        let module = self.get_module(module_id)?;
        let name = module.read().unwrap().module.get_name();
        Ok(name)
    }

    fn module_get_desc(&self, module_id: &ModuleId) -> Result<String, SessionError> {
        Ok(self.get_module(module_id)?.read().unwrap().desc.clone())
    }

    fn module_set_desc(&self, module_id: &ModuleId, desc: String) -> Result<(), SessionError> {
        self.get_module(module_id)?.write().unwrap().desc = desc;
        Ok(())
    }

    fn module_get_default_desc(&self, module_id: &ModuleId) -> Result<String, SessionError> {
        let module = self.get_module(module_id)?;
        let desc = module.read().unwrap().module.get_desc();
        Ok(desc)
    }

    fn module_get_proxy(&self, module_id: &ModuleId) -> Result<usize, SessionError> {
        Ok(self.get_module(module_id)?.read().unwrap().proxy)
    }

    fn module_set_proxy(&self, module_id: &ModuleId, proxy: usize) -> Result<(), SessionError> {
        self.get_module(module_id)?.write().unwrap().proxy = proxy;
        Err(SessionError::InvalidModule)
    }

    fn module_get_settings(&self, module_info: &ModuleId) -> Result<Data, SessionError> {
        Ok(self
            .get_module(module_info)?
            .read()
            .unwrap()
            .settings
            .clone())
    }

    fn module_set_settings(&self, module_info: &ModuleId, data: Data) -> Result<(), SessionError> {
        self.get_module(module_info)?.write().unwrap().settings = data;
        Ok(())
    }

    fn module_get_element_settings(&self, module_info: &ModuleId) -> Result<Data, SessionError> {
        Ok(self
            .get_module(module_info)?
            .read()
            .unwrap()
            .element_data
            .clone())
    }

    fn module_set_element_settings(
        &self,
        module_info: &ModuleId,
        data: Data,
    ) -> Result<(), SessionError> {
        self.get_module(module_info)?.write().unwrap().element_data = data;
        Ok(())
    }

    fn module_init_location(
        &self,
        module_info: &ModuleId,
        location_info: &LocationId,
        data: crate::data::FileOrData,
    ) -> Result<(), SessionError> {
        let location_info = self
            .get_location(location_info)?
            .read()
            .unwrap()
            .info
            .clone();
        self.get_module(module_info)?
            .read()
            .unwrap()
            .module
            .init_location(location_info, data);
        Ok(())
    }

    fn module_init_element(
        &self,
        module_info: &ModuleId,
        element_info: &ElementId,
    ) -> Result<(), SessionError> {
        let module = self.get_module(module_info)?;
        let module = module.read().unwrap();
        let element = self.get_element(element_info)?;

        {
            let mut element = element.write().unwrap();
            element.element_data = module.element_data.clone();
            element.module_data = module.settings.clone();
        }
        module.module.init_element(element);

        Ok(())
    }

    fn module_accept_url(&self, module_info: &ModuleId, url: String) -> Result<bool, SessionError> {
        Ok(self
            .get_module(module_info)?
            .read()
            .unwrap()
            .module
            .accept_url(url))
    }

    fn module_accept_extension(
        &self,
        module_info: &ModuleId,
        filename: &str,
    ) -> Result<bool, SessionError> {
        Ok(self
            .get_module(module_info)?
            .read()
            .unwrap()
            .module
            .accept_extension(filename))
    }

    fn module_accepted_protocols(&self, module_id: &ModuleId) -> Result<Vec<String>, SessionError> {
        Ok(self
            .get_module(module_id)?
            .read()
            .unwrap()
            .module
            .accepted_protocols())
    }

    fn module_step_element(
        &self,
        module_info: &ModuleId,
        element_info: &ElementId,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError> {
        let module;
        {
            let m = self.get_module(module_info)?;
            module = m.read().unwrap().module.c();
        }

        let element = self.get_element(element_info)?;

        let mut control_flow = control_flow;
        let mut storage = storage;

        module.step_element(element, &mut control_flow, &mut storage);
        Ok((control_flow, storage))
    }

    fn module_step_location(
        &self,
        module_info: &ModuleId,
        location_info: &LocationId,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError> {
        let module;
        {
            let m = self.get_module(module_info)?;
            module = m.read().unwrap().module.c();
        }

        let location = self.get_location(location_info)?;

        let mut control_flow = control_flow;
        let mut storage = storage;

        module.step_location(location, &mut control_flow, &mut storage);
        Ok((control_flow, storage))
    }

    fn create_element(&self, name: &str, location: &LocationId) -> Result<ERef, SessionError> {
        let mut element_uid = self.get_location(location)?.read().unwrap().elements.len();
        let mut should_replace = None;
        for (i, element) in self
            .get_location(location)?
            .read()
            .unwrap()
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

        let element_info = Arc::new(RwLock::new(RefElement {
            session: Some(self.c()),
            id: ElementId {
                location_id: location.clone(),
                uid: element_uid as u64,
            },
        }));

        let path = self.get_location(location)?.read().unwrap().path.join(name);

        let element = Arc::new(RwLock::new(Element {
            name: name.to_owned(),
            desc: String::new(),
            meta: String::new(),
            url: None,
            element_data: Data::new(),
            module_data: Data::new(),
            module: None,
            statuses: Vec::new(),
            status: usize::MAX,
            data: FileOrData::File(path, None),
            progress: 0.0,
            should_save: false,
            enabled: false,
            thread: None,
            info: element_info.clone(),
            events: Arc::new(RwLock::new(Events::default())),
        }));

        self.get_location(location)?
            .write()
            .unwrap()
            .elements
            .push(Some(element));
        if let Some(should_replace) = should_replace {
            self.get_location(location)?
                .write()
                .unwrap()
                .elements
                .swap_remove(should_replace);
        }

        let _ = self.notify_all(SessionEvent::NewElement(
            element_info.read().unwrap().id.clone(),
        ));

        Ok(element_info)
    }

    fn load_element_info(&self, info: ElementInfo) -> Result<ERef, SessionError> {
        let location = self.get_location(&info.id.location_id)?;
        let location_ref = self.get_location_ref(&info.id.location_id)?;
        let location_id = info.id.location_id;

        let element_id = Arc::new(RwLock::new(RefElement {
            session: Some(self.c()),
            id: ElementId {
                location_id,
                uid: info.id.uid,
            },
        }));

        // TODO: better path system
        let path = if let FileOrData::File(path, _) = info.data {
            path
        } else {
            location_ref.get_path()?.join(&info.name)
        };

        let module = if let Some(module) = info.module {
            Some(self.load_module_info(module)?)
        } else {
            None
        };

        let element = Arc::new(RwLock::new(Element {
            name: info.name,
            desc: info.desc,
            meta: info.meta,
            url: info.url,
            element_data: info.element_data,
            module_data: info.module_data,
            module,
            statuses: info.statuses,
            status: info.status,
            data: FileOrData::File(path, None),
            progress: info.progress,
            should_save: info.should_save,
            enabled: false,
            thread: None,
            info: element_id.clone(),
            events: Arc::new(RwLock::new(Events::default())),
        }));

        let mut notifications = Vec::new();

        {
            let mut location = location.write().unwrap();
            if let Some(other_element) = location.elements.get(info.id.uid as usize) {
                if let Some(other_element) = other_element.clone() {
                    let last_id = other_element
                        .read()
                        .unwrap()
                        .info
                        .read()
                        .unwrap()
                        .id
                        .clone();
                    let other_element_new_id = location.elements.len();
                    location.elements.push(Some(other_element.clone()));
                    let new_id = {
                        let other_element = other_element.read().unwrap();
                        let mut uid = other_element.info.write().unwrap();
                        uid.id.uid = other_element_new_id as u64;
                        uid.id.clone()
                    };
                    notifications.push(SessionEvent::ElementIdChanged(last_id, new_id));
                }
                location.elements.push(Some(element));
                location.elements.swap_remove(info.id.uid as usize);
            } else {
                loop {
                    if location.elements.len() == info.id.uid as usize {
                        location.elements.push(Some(element));
                        break;
                    } else {
                        location.elements.push(None)
                    }
                }
            }
        }

        notifications.push(SessionEvent::NewElement(
            element_id.read().unwrap().id.clone(),
        ));
        for notification in notifications {
            let _ = self.notify_all(notification);
        }

        Ok(element_id)
    }

    fn move_element(&self, element: &ElementId, location: &LocationId) -> Result<(), SessionError> {
        let elem = self.destroy_element(element.clone())?;
        let info = elem.read().unwrap().info.clone();
        let new_uid = self.get_location(location)?.read().unwrap().elements.len();
        let last = info.read().unwrap().id.clone();
        let new = ElementId {
            uid: new_uid as u64,
            location_id: location.clone(),
        };

        elem.read().unwrap().info.write().unwrap().id = new.clone();
        self.get_location(location)?
            .write()
            .unwrap()
            .elements
            .push(Some(elem));

        let _ = self.notify_all(SessionEvent::ElementIdChanged(last, new));
        Ok(())
    }

    fn destroy_element(&self, element: ElementId) -> Result<ERow, SessionError> {
        let _ = self.get_element_ref(&element)?;

        let _ = self.notify_all(SessionEvent::DestroyedElement(element.clone()));

        let Some(element) = self
            .get_location(&element.location_id)?
            .write()
            .unwrap()
            .elements
            .remove(element.uid as usize)else{return Err(SessionError::EmptyElement)};

        let mut notifications = Vec::new();
        for (i, element) in self
            .get_location(&element.read().unwrap().info.read().unwrap().id.location_id)?
            .read()
            .unwrap()
            .elements
            .iter()
            .enumerate()
        {
            let Some(element) = element else {continue};
            let info = element.read().unwrap().info.clone();
            let last = info.read().unwrap().id.clone();
            info.write().unwrap().id.uid = i as u64;

            notifications.push(SessionEvent::ElementIdChanged(
                last,
                ElementId {
                    uid: i as u64,
                    location_id: info.read().unwrap().id.location_id.clone(),
                },
            ));
        }
        for notification in notifications {
            let _ = self.notify_all(notification);
        }
        Ok(element)
    }

    fn element_get_name(&self, element: &ElementId) -> Result<String, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().name.clone())
    }

    fn element_set_name(&self, element: &ElementId, name: &str) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().name = name.to_owned();
        Ok(())
    }

    fn element_get_desc(&self, element: &ElementId) -> Result<String, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().desc.clone())
    }

    fn element_set_desc(&self, element: &ElementId, desc: &str) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().desc = desc.to_owned();
        Ok(())
    }

    fn element_get_meta(&self, element: &ElementId) -> Result<String, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().meta.clone())
    }

    fn element_set_meta(&self, element: &ElementId, meta: &str) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().meta = meta.to_owned();
        Ok(())
    }

    fn element_get_url(&self, element_id: &ElementId) -> Result<Option<String>, SessionError> {
        Ok(self.get_element(element_id)?.read().unwrap().url.clone())
    }

    fn element_set_url(
        &self,
        element_id: &ElementId,
        url: Option<String>,
    ) -> Result<(), SessionError> {
        self.get_element(element_id)?.write().unwrap().url = url;
        Ok(())
    }

    fn element_get_element_data(&self, element: &ElementId) -> Result<Data, SessionError> {
        Ok(self
            .get_element(element)?
            .read()
            .unwrap()
            .element_data
            .clone())
    }

    fn element_set_element_data(
        &self,
        element: &ElementId,
        data: Data,
    ) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().element_data = data;
        Ok(())
    }

    fn element_get_module_data(&self, element: &ElementId) -> Result<Data, SessionError> {
        Ok(self
            .get_element(element)?
            .read()
            .unwrap()
            .module_data
            .clone())
    }

    fn element_set_module_data(&self, element: &ElementId, data: Data) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().module_data = data;
        Ok(())
    }

    fn element_get_module(&self, element: &ElementId) -> Result<Option<MRef>, SessionError> {
        if let Some(module) = &self.get_element(element)?.read().unwrap().module {
            Ok(Some(module.clone()))
        } else {
            Ok(None)
        }
    }

    fn element_set_module(
        &self,
        element_id: &ElementId,
        module: Option<ModuleId>,
    ) -> Result<(), SessionError> {
        self.get_element(element_id)?.write().unwrap().module = match &module {
            Some(module_id) => Some(self.get_module_ref(module_id)?),
            None => None,
        };
        Ok(())
    }

    fn element_get_statuses(&self, element: &ElementId) -> Result<Vec<String>, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().statuses.clone())
    }

    fn element_set_statuses(
        &self,
        element: &ElementId,
        statuses: Vec<String>,
    ) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().statuses = statuses;
        Ok(())
    }

    fn element_get_status(&self, element: &ElementId) -> Result<usize, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().status)
    }

    fn element_set_status(&self, element: &ElementId, status: usize) -> Result<(), SessionError> {
        let statuses = self.get_element(element)?.read().unwrap().statuses.len();
        if status < statuses {
            self.get_element(element)?.write().unwrap().status = status;
            Ok(())
        } else {
            Err(SessionError::InvalidElementStatus)
        }
    }

    fn element_get_data(&self, element: &ElementId) -> Result<FileOrData, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().data.clone())
    }

    fn element_set_data(&self, element: &ElementId, data: FileOrData) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().data = data;
        Ok(())
    }

    fn element_get_progress(&self, element: &ElementId) -> Result<f32, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().progress)
    }

    fn element_set_progress(&self, element: &ElementId, progress: f32) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().progress = progress;
        Ok(())
    }

    fn element_get_should_save(&self, element: &ElementId) -> Result<bool, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().should_save)
    }

    fn element_set_should_save(
        &self,
        element: &ElementId,
        should_save: bool,
    ) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().should_save = should_save;
        Ok(())
    }

    fn element_get_enabled(&self, element: &ElementId) -> Result<bool, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().enabled)
    }

    fn element_set_enabled(
        &self,
        element: &ElementId,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError> {
        let element = self.get_element(element)?;
        element.write().unwrap().enabled = enabled;
        if !enabled {
            return Ok(());
        }

        let tmp_element = element.clone();
        element.write().unwrap().thread = Some(std::thread::spawn(move || {
            let element = tmp_element.clone();
            let element_info = element.read().unwrap().info.clone();
            let mut control_flow = ControlFlow::Run;
            let mut storage = if let Some(storage) = storage {
                storage
            } else {
                Storage::default()
            };

            loop {
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
                        (control_flow, storage) = module
                            .step_element(
                                &element_info.read().unwrap().id.clone(),
                                control_flow,
                                storage,
                            )
                            .unwrap();
                    }
                } else {
                    break;
                }
            }

            element.write().unwrap().enabled = false;
        }));

        Ok(())
    }

    fn element_resolv_module(&self, element_info: &ElementId) -> Result<bool, SessionError> {
        let len = self.get_modules_len()?;

        let mut module = None;

        if let Some(url) = self.get_element(element_info)?.read().unwrap().url.clone() {
            for tmp_module in self.get_modules(0..len)? {
                if self
                    .get_module(&tmp_module.read().unwrap().uid)?
                    .read()
                    .unwrap()
                    .module
                    .accept_url(url.clone())
                {
                    module = Some(tmp_module);
                    break;
                }
            }
        }

        if let Some(module) = module {
            self.element_set_module(element_info, Some(module.id()))?;
            return Ok(true);
        }

        Ok(false)
    }

    fn element_wait(&self, element: &ElementId) -> Result<(), SessionError> {
        let thread = self.get_element(element)?.write().unwrap().thread.take();
        if let Some(thread) = thread {
            thread.join().unwrap();
        }
        Ok(())
    }

    fn element_get_element_info(&self, element: &ElementId) -> Result<ElementInfo, SessionError> {
        let element = self.get_element(element)?;
        let mut module = None;
        {
            let __module;
            {
                __module = element.read().unwrap().module.clone();
            }

            if let Some(__module) = __module {
                let __module = self.get_module(&__module.read().unwrap().uid)?;
                let __module = __module.read().unwrap();

                module = Some(ModuleInfo {
                    name: __module.name.clone(),
                    desc: __module.desc.clone(),
                    proxy: __module.proxy,
                    settings: __module.settings.clone(),
                    element_data: __module.element_data.clone(),
                    id: __module.info.id(),
                    path: __module.path.clone(),
                });
            }
        }
        let element = element.read().unwrap();
        Ok(ElementInfo {
            name: element.name.clone(),
            desc: element.desc.clone(),
            meta: element.meta.clone(),
            url: element.url.clone(),
            element_data: element.element_data.clone(),
            module_data: element.module_data.clone(),
            module,
            statuses: element.statuses.clone(),
            status: element.status,
            data: element.data.clone(),
            progress: element.progress,
            should_save: element.should_save,
            enabled: element.enabled,
            id: element.info.id(),
        })
    }

    fn element_notify(&self, element_info: &ElementId, event: Event) -> Result<(), SessionError> {
        let element = self.get_element(element_info)?;
        let module;
        {
            let Some(__module) = &element.read().unwrap().module else{
            return Err(SessionError::InvalidModule)
        };

            module = __module.clone();
        }

        let raw_module;
        {
            let __module = self.get_module(&module.read().unwrap().uid)?;
            raw_module = __module.read().unwrap().module.c();
        }

        let element_ref = self.get_element_ref(element_info)?;
        raw_module.notify(Ref::Element(element_ref), event);

        Ok(())
    }

    fn element_emit(&self, element_info: &ElementId, event: Event) -> Result<(), SessionError> {
        let element = self.get_element(element_info)?;
        let events;
        {
            events = element.read().unwrap().events.clone()
        }

        let mut event = event;
        match &mut event {
            Event::Element(info, _) => *info = element_info.clone(),
            Event::Location(_, _) => return Err(SessionError::IsNotLocation),
            Event::Log(r, _) => *r = ID::Element(element_info.clone()),
            Event::SessionEvent(_) => return Ok(()),
        }

        events.write().unwrap().new_event(event, self.c());

        Ok(())
    }

    fn element_subscribe(&self, element_info: &ElementId, _ref: ID) -> Result<(), SessionError> {
        // validate if element is valid
        {
            let _ = self.get_element(element_info)?;
        }

        let events = match _ref {
            ID::Element(e_info) => {
                let e = self.get_element(&e_info)?;
                let d = e.read().unwrap().events.clone();
                d
            }
            ID::Location(l_info) => {
                let l = self.get_location(&l_info)?;
                let d = l.read().unwrap().events.clone();
                d
            }
        };

        if !events
            .write()
            .unwrap()
            .subscribe(ID::Element(element_info.clone()))
        {
            return Err(SessionError::AlreadySubscribed);
        }

        Ok(())
    }

    fn element_unsubscribe(&self, element_info: &ElementId, _ref: ID) -> Result<(), SessionError> {
        // validate if element is valid
        {
            let _ = self.get_element(element_info)?;
        }

        let events = match _ref {
            ID::Element(e_info) => {
                let e = self.get_element(&e_info)?;
                let d = e.read().unwrap().events.clone();
                d
            }
            ID::Location(l_info) => {
                let l = self.get_location(&l_info)?;
                let d = l.read().unwrap().events.clone();
                d
            }
        };

        if !events
            .write()
            .unwrap()
            .unsubscribe(ID::Element(element_info.clone()))
        {
            return Err(SessionError::AlreadyUnsubscribed);
        }

        Ok(())
    }

    fn create_location(&self, name: &str, location: &LocationId) -> Result<LRef, SessionError> {
        let mut location_uid = location.clone();
        let mut uid = self.get_locations_len(location)?;
        let mut should_swap = None;
        for (i, location) in self
            .get_location(location)
            .unwrap()
            .write()
            .unwrap()
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
        let location_info = Arc::new(RwLock::new(RefLocation {
            session: Some(self.c()),
            id: location_uid,
        }));

        let path = self.get_location(location)?.read().unwrap().path.join(name);

        let loc = Location {
            name: name.to_owned(),
            desc: String::new(),
            where_is: WhereIsLocation::Local(LocalLocation {
                path: PathBuf::from("."),
            }),
            shoud_save: false,
            elements: Vec::new(),
            locations: Vec::new(),
            info: location_info.clone(),
            module: None,
            path,
            thread: None,
            events: Arc::new(RwLock::new(Events::default())),
        };

        let dest = self.get_location(location)?;
        dest.write()
            .unwrap()
            .locations
            .push(Some(Arc::new(RwLock::new(loc))));

        if let Some(should_swap) = should_swap {
            self.get_location(location)
                .unwrap()
                .write()
                .unwrap()
                .locations
                .swap_remove(should_swap);
        }

        let _ = self.notify_all(SessionEvent::NewLocation(
            location_info.read().unwrap().id.clone(),
        ));

        Ok(location_info)
    }

    fn load_location_info(&self, info: LocationInfo) -> Result<LRef, SessionError> {
        let mut location_uid = info.id;

        let location_info = Arc::new(RwLock::new(RefLocation {
            session: Some(self.c()),
            id: location_uid,
        }));

        let module = if let Some(module_info) = info.module {
            Some(self.load_module_info(module_info)?)
        } else {
            None
        };

        let mut locations = Vec::with_capacity(info.locations.len());

        for location in info.locations {
            locations.push(Some(
                self.get_location(&self.load_location_info(location)?.id())?,
            ))
        }

        let mut elements = Vec::with_capacity(info.elements.len());

        for element in info.elements {
            elements.push(Some(
                self.get_element(&self.load_element_info(element)?.id())?,
            ))
        }

        let loc = Location {
            name: info.name,
            desc: info.desc,
            where_is: info.where_is,
            shoud_save: info.shoud_save,
            elements,
            locations,
            info: location_info.clone(),
            module,
            path: info.path,
            thread: None,
            events: Arc::new(RwLock::new(Events::default())),
        };

        // TODO: Implelemnt how will be added!
        todo!();

        let _ = self.notify_all(SessionEvent::NewLocation(
            location_info.read().unwrap().id.clone(),
        ));

        Ok(location_info)
    }

    fn get_locations_len(&self, location: &LocationId) -> Result<usize, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().locations.len())
    }

    fn get_locations(
        &self,
        location: &LocationId,
        range: Range<usize>,
    ) -> Result<Vec<LRef>, SessionError> {
        let mut location_infos = Vec::new();
        let location = self.get_location(location)?;
        for loc in location.read().unwrap().locations[range].iter().flatten() {
            location_infos.push(loc.read().unwrap().info.clone());
        }
        Ok(location_infos)
    }

    fn destroy_location(&self, location: LocationId) -> Result<LRow, SessionError> {
        let _ = self.get_location_ref(&location)?;
        let _ = self.notify_all(SessionEvent::DestroyedLocation(location.clone()));

        let mut location_uid = location;
        if let Some(location_index) = location_uid.pop() {
            let parent_location = self.get_location(&location_uid)?;

            let Some(removed_location) = parent_location
                .write()
                .unwrap()
                .locations
                .remove(location_index as usize)else{return Err(SessionError::EmptyLocation)};

            let mut notifications = Vec::new();
            for (i, location) in parent_location.read().unwrap().locations.iter().enumerate() {
                let Some(location) = location else{continue};
                let info = location.read().unwrap().info.clone();
                let last = info.id();
                let mut new = last.clone();
                *new.last_mut().unwrap() = i as u64;

                info.write().unwrap().id = new.clone();

                notifications.push(SessionEvent::LocationIdChanged(last, new));
            }

            for notification in notifications {
                let _ = self.notify_all(notification);
            }

            return Ok(removed_location);
        }
        Err(SessionError::InvalidLocation)
    }

    fn get_default_location(&self) -> Result<LRef, SessionError> {
        if let Some(location) = &self.read().unwrap().location {
            return Ok(location.read().unwrap().info.clone());
        }
        Err(SessionError::InvalidLocation)
    }

    fn move_location(&self, location: &LocationId, to: &LocationId) -> Result<(), SessionError> {
        let location = self.destroy_location(location.clone())?;
        let mut new = to.clone();
        new.push(self.get_locations_len(to)? as u64);
        let info = location.read().unwrap().info.clone();
        let last = info.read().unwrap().id.clone();
        location.write().unwrap().info.write().unwrap().id = new.clone();
        self.get_location(to)?
            .write()
            .unwrap()
            .locations
            .push(Some(location));

        let _ = self.notify_all(SessionEvent::LocationIdChanged(last, new));

        Ok(())
    }

    fn location_get_name(&self, location: &LocationId) -> Result<String, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().name.clone())
    }

    fn location_set_name(&self, location: &LocationId, name: &str) -> Result<(), SessionError> {
        self.get_location(location)?.write().unwrap().name = name.to_owned();
        Ok(())
    }

    fn location_get_desc(&self, location: &LocationId) -> Result<String, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().desc.clone())
    }

    fn location_set_desc(&self, location: &LocationId, desc: &str) -> Result<(), SessionError> {
        self.get_location(location)?.write().unwrap().desc = desc.to_owned();
        Ok(())
    }

    fn location_get_path(&self, location: &LocationId) -> Result<PathBuf, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().path.clone())
    }

    fn location_set_path(&self, location: &LocationId, path: PathBuf) -> Result<(), SessionError> {
        self.get_location(location)?.write().unwrap().path = path;
        Ok(())
    }

    fn location_get_where_is(
        &self,
        location: &LocationId,
    ) -> Result<WhereIsLocation, SessionError> {
        Ok(self
            .get_location(location)?
            .read()
            .unwrap()
            .where_is
            .clone())
    }

    fn location_set_where_is(
        &self,
        location: &LocationId,
        where_is: WhereIsLocation,
    ) -> Result<(), SessionError> {
        self.get_location(location)?.write().unwrap().where_is = where_is;
        Ok(())
    }

    fn location_get_should_save(&self, location: &LocationId) -> Result<bool, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().shoud_save)
    }

    fn location_set_should_save(
        &self,
        location: &LocationId,
        should_save: bool,
    ) -> Result<(), SessionError> {
        self.get_location(location)?.write().unwrap().shoud_save = should_save;
        Ok(())
    }

    fn location_get_elements_len(&self, location: &LocationId) -> Result<usize, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().elements.len())
    }

    fn location_get_elements(
        &self,
        location: &LocationId,
        range: Range<usize>,
    ) -> Result<Vec<ERef>, SessionError> {
        let mut element_infos = Vec::new();
        for element in self.get_location(location)?.read().unwrap().elements[range]
            .iter()
            .flatten()
        {
            element_infos.push(element.read().unwrap().info.clone())
        }
        Ok(element_infos)
    }

    fn location_get_location_info(
        &self,
        location: &LocationId,
    ) -> Result<LocationInfo, SessionError> {
        let mut module = None;
        {
            let __module;
            {
                let location = self.get_location(location)?;
                __module = location.read().unwrap().module.clone();
            }

            if let Some(__module) = __module {
                let __module = self.get_module(&__module.read().unwrap().uid)?;
                let __module = __module.read().unwrap();

                module = Some(ModuleInfo {
                    name: __module.name.clone(),
                    desc: __module.desc.clone(),
                    proxy: __module.proxy,
                    settings: __module.settings.clone(),
                    element_data: __module.element_data.clone(),
                    id: __module.info.id(),
                    path: __module.path.clone(),
                });
            }
        }

        let mut elements = Vec::new();
        {
            let len = self.location_get_elements_len(location)?;
            let __elements = self.location_get_elements(location, 0..len)?;
            for element in __elements {
                let info = element.get_element_info()?;
                elements.push(info)
            }
        }

        let mut locations = Vec::new();
        {
            let len = self.get_locations_len(location)?;
            let __locations = self.get_locations(location, 0..len)?;
            for location in __locations {
                let info = location.get_location_info()?;
                locations.push(info)
            }
        }

        let location = self.get_location(location)?;
        let location = location.read().unwrap();

        Ok(LocationInfo {
            name: location.name.clone(),
            desc: location.desc.clone(),
            id: location.info.id(),
            where_is: location.where_is.clone(),
            shoud_save: location.shoud_save,
            elements,
            locations,
            path: location.path.clone(),
            module,
        })
    }

    fn location_notify(
        &self,
        location_info: &LocationId,
        event: Event,
    ) -> Result<(), SessionError> {
        let location = self.get_location(location_info)?;
        let module;
        {
            let Some(__module) = &location.read().unwrap().module else{
            return Err(SessionError::InvalidModule)
        };

            module = __module.clone();
        }

        let raw_module;
        {
            let __module = self.get_module(&module.read().unwrap().uid)?;
            raw_module = __module.read().unwrap().module.c();
        }

        let location_ref = self.get_location_ref(location_info)?;
        raw_module.notify(Ref::Location(location_ref), event);

        Ok(())
    }

    fn location_emit(&self, location_info: &LocationId, event: Event) -> Result<(), SessionError> {
        let location = self.get_location(location_info)?;
        let events;
        {
            events = location.read().unwrap().events.clone()
        }

        let mut event = event;
        match &mut event {
            Event::Element(_, _) => return Err(SessionError::IsNotElement),
            Event::Location(info, _) => *info = location_info.clone(),
            Event::Log(r, _) => *r = ID::Location(location_info.clone()),
            Event::SessionEvent(_) => return Ok(()),
        }

        events.write().unwrap().new_event(event, self.c());

        Ok(())
    }

    fn location_subscribe(&self, location_info: &LocationId, _ref: ID) -> Result<(), SessionError> {
        // validate if element is valid
        {
            let _ = self.get_location(location_info)?;
        }

        let events = match _ref {
            ID::Element(e_info) => {
                let e = self.get_element(&e_info)?;
                let d = e.read().unwrap().events.clone();
                d
            }
            ID::Location(l_info) => {
                let l = self.get_location(&l_info)?;
                let d = l.read().unwrap().events.clone();
                d
            }
        };

        if !events
            .write()
            .unwrap()
            .subscribe(ID::Location(location_info.clone()))
        {
            return Err(SessionError::AlreadySubscribed);
        }

        Ok(())
    }

    fn location_unsubscribe(
        &self,
        location_info: &LocationId,
        _ref: ID,
    ) -> Result<(), SessionError> {
        // validate if element is valid
        {
            let _ = self.get_location(location_info)?;
        }

        let events = match _ref {
            ID::Element(e_info) => {
                let e = self.get_element(&e_info)?;
                let d = e.read().unwrap().events.clone();
                d
            }
            ID::Location(l_info) => {
                let l = self.get_location(&l_info)?;
                let d = l.read().unwrap().events.clone();
                d
            }
        };

        if !events
            .write()
            .unwrap()
            .unsubscribe(ID::Location(location_info.clone()))
        {
            return Err(SessionError::AlreadyUnsubscribed);
        }

        Ok(())
    }

    fn get_module_ref(&self, id: &ModuleId) -> Result<MRef, SessionError> {
        Ok(self.get_module(id)?.read().unwrap().info.clone())
    }

    fn get_element_ref(&self, id: &ElementId) -> Result<ERef, SessionError> {
        Ok(self.get_element(id)?.read().unwrap().info.clone())
    }

    fn get_location_ref(&self, id: &LocationId) -> Result<LRef, SessionError> {
        Ok(self.get_location(id)?.read().unwrap().info.clone())
    }

    fn c(&self) -> Box<dyn TSession> {
        Box::new(self.clone())
    }
}
