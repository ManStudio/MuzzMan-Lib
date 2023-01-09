use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::Range,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::prelude::*;

#[derive(Default)]
pub struct LocalSession {
    pub location: Option<LRow>,
    pub modules: Vec<MRow>,
    pub actions: Vec<Arc<RwLock<Action>>>,
}

impl LocalSession {
    pub fn new_session() -> Box<dyn TSession> {
        let session = Self::default();

        let session = Arc::new(RwLock::new(session));

        let location_info = Arc::new(RwLock::new(RefLocation {
            session: Some(session.c()),
            uid: vec![],
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
        }));
        session.write().unwrap().location = Some(location);
        session.c()
    }
}

trait TLocalSession {
    fn get_location(&self, info: &LRef) -> Result<LRow, SessionError>;
    fn get_element(&self, info: &ERef) -> Result<ERow, SessionError>;
    fn get_module(&self, info: &MRef) -> Result<MRow, SessionError>;
}

impl TLocalSession for Arc<RwLock<LocalSession>> {
    fn get_location(&self, info: &LRef) -> Result<LRow, SessionError> {
        if let Some(location) = &mut self.write().unwrap().location {
            let mut loc = location.clone();
            for i in info.read().unwrap().uid.clone() {
                let tmp_loc;
                if let Some(location) = loc.read().unwrap().locations.get(i) {
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

    fn get_element(&self, info: &ERef) -> Result<ERow, SessionError> {
        let info = info.read().unwrap();
        if let Some(element) = self
            .get_location(&info.location)?
            .read()
            .unwrap()
            .elements
            .get(info.uid)
        {
            Ok(element.clone())
        } else {
            Err(SessionError::ElementDoNotExist)
        }
    }

    fn get_module(&self, info: &MRef) -> Result<MRow, SessionError> {
        if let Some(uid) = info.read().unwrap().uid {
            Ok(self.read().unwrap().modules[uid].clone())
        } else {
            Err(SessionError::InvalidModule)
        }
    }
}

impl TSession for Arc<RwLock<LocalSession>> {
    fn add_module(&self, module: Box<dyn TModule>) -> Result<MRef, SessionError> {
        let mut module = Module {
            name: module.get_name(),
            desc: module.get_desc(),
            module,
            proxy: 0,
            settings: Data::new(),
            element_data: Data::new(),
            info: None,
        };

        let info = Arc::new(RwLock::new(RefModule {
            uid: Some(self.get_modules_len()?),
            session: Some(self.c()),
        }));

        if let Err(error) = module.module.init(info.clone()) {
            return Err(SessionError::CannotInstallModule(error));
        }

        module.info = Some(info.clone());

        self.write()
            .unwrap()
            .modules
            .push(Arc::new(RwLock::new(module)));

        Ok(info)
    }

    fn remove_module(&self, info: MRef) -> Result<MRow, SessionError> {
        if let Some(index) = info.read().unwrap().uid {
            {
                let module = self.read().unwrap().modules[index].clone();
                let module = module.read().unwrap();
                if let Some(info) = module.info.clone() {
                    self.write().unwrap().actions.retain(|e| {
                        *e.read().unwrap().owner.read().unwrap() != *info.read().unwrap()
                    });
                }
            }
            let module = self.write().unwrap().modules.remove(index);

            module.write().unwrap().info = None;

            {
                let mut info = info.write().unwrap();
                info.session = None;
                info.uid = None;
            }

            for (i, module) in self.write().unwrap().modules.iter().enumerate() {
                if let Some(info) = &module.write().unwrap().info {
                    info.write().unwrap().uid = Some(i);
                }
            }

            return Ok(module);
        }
        Err(SessionError::InvalidModule)
    }

    fn register_action(
        &self,
        module: &MRef,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError> {
        self.write()
            .unwrap()
            .actions
            .push(Arc::new(RwLock::new(Action {
                name,
                owner: module.clone(),
                input: values,
                callback,
            })));
        Ok(())
    }

    fn remove_action(&self, owner: &MRef, name: String) -> Result<(), SessionError> {
        let mut finded = None;
        for (i, action) in self.read().unwrap().actions.iter().enumerate() {
            let action = action.read().unwrap();
            if *action.owner.read().unwrap() == *owner.read().unwrap() && action.name == name {
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

    fn run_action(&self, owner: MRef, name: String, data: Vec<Type>) -> Result<(), SessionError> {
        let mut finded = None;
        for (i, action) in self.read().unwrap().actions.iter().enumerate() {
            let action = action.read().unwrap();
            if *action.owner.read().unwrap() == *owner.read().unwrap() && action.name == name {
                finded = Some(i);
                break;
            }
        }
        if let Some(finded) = finded {
            let action = self.read().unwrap().actions[finded].clone();
            (action.read().unwrap().callback)(owner, data);
        }
        Ok(())
    }

    fn get_modules_len(&self) -> Result<usize, SessionError> {
        Ok(self.read().unwrap().modules.len())
    }

    fn get_modules(&self, range: Range<usize>) -> Result<Vec<MRef>, SessionError> {
        let mut modules = Vec::new();

        for module in self.read().unwrap().modules[range].iter() {
            if let Some(info) = &module.write().unwrap().info {
                modules.push(info.clone())
            }
        }

        Ok(modules)
    }

    fn get_module_name(&self, info: &MRef) -> Result<String, SessionError> {
        if let Some(uid) = info.read().unwrap().uid {
            if let Some(module) = self.read().unwrap().modules.get(uid) {
                return Ok(module.read().unwrap().name.clone());
            }
        }

        Err(SessionError::InvalidModule)
    }

    fn set_module_name(&self, info: &MRef, name: String) -> Result<(), SessionError> {
        if let Some(uid) = info.read().unwrap().uid {
            if let Some(module) = self.read().unwrap().modules.get(uid) {
                module.write().unwrap().name = name;
                return Ok(());
            }
        }

        Err(SessionError::InvalidModule)
    }

    fn default_module_name(&self, info: &MRef) -> Result<(), SessionError> {
        if let Some(uid) = info.read().unwrap().uid {
            if let Some(module) = self.read().unwrap().modules.get(uid) {
                module.write().unwrap().name = module.read().unwrap().module.get_name();
                return Ok(());
            }
        }

        Err(SessionError::InvalidModule)
    }

    fn get_module_desc(&self, info: &MRef) -> Result<String, SessionError> {
        if let Some(uid) = info.read().unwrap().uid {
            if let Some(module) = self.read().unwrap().modules.get(uid) {
                return Ok(module.read().unwrap().desc.clone());
            }
        }

        Err(SessionError::InvalidModule)
    }

    fn set_module_desc(&self, info: &MRef, desc: String) -> Result<(), SessionError> {
        if let Some(uid) = info.read().unwrap().uid {
            if let Some(module) = self.read().unwrap().modules.get(uid) {
                module.write().unwrap().desc = desc;
                return Ok(());
            }
        }

        Err(SessionError::InvalidModule)
    }

    fn default_module_desc(&self, info: &MRef) -> Result<(), SessionError> {
        if let Some(uid) = info.read().unwrap().uid {
            if let Some(module) = self.read().unwrap().modules.get(uid) {
                module.write().unwrap().desc = module.read().unwrap().module.get_desc();
                return Ok(());
            }
        }

        Err(SessionError::InvalidModule)
    }

    fn get_module_proxy(&self, info: &MRef) -> Result<usize, SessionError> {
        if let Some(uid) = info.read().unwrap().uid {
            if let Some(module) = self.read().unwrap().modules.get(uid) {
                return Ok(module.read().unwrap().proxy);
            }
        }
        Err(SessionError::InvalidModule)
    }

    fn set_module_proxy(&self, info: &MRef, proxy: usize) -> Result<(), SessionError> {
        if let Some(uid) = info.read().unwrap().uid {
            if let Some(module) = self.read().unwrap().modules.get(uid) {
                module.write().unwrap().proxy = proxy;
                return Ok(());
            }
        }
        Err(SessionError::InvalidModule)
    }

    fn get_module_settings(&self, module_info: &MRef) -> Result<Data, SessionError> {
        Ok(self
            .get_module(module_info)?
            .read()
            .unwrap()
            .settings
            .clone())
    }

    fn set_module_settings(&self, module_info: &MRef, data: Data) -> Result<(), SessionError> {
        self.get_module(module_info)?.write().unwrap().settings = data;
        Ok(())
    }

    fn get_module_element_settings(&self, module_info: &MRef) -> Result<Data, SessionError> {
        Ok(self
            .get_module(module_info)?
            .read()
            .unwrap()
            .element_data
            .clone())
    }

    fn set_module_element_settings(
        &self,
        module_info: &MRef,
        data: Data,
    ) -> Result<(), SessionError> {
        self.get_module(module_info)?.write().unwrap().element_data = data;
        Ok(())
    }

    fn module_init_location(
        &self,
        module_info: &MRef,
        location_info: &LRef,
        data: crate::data::FileOrData,
    ) -> Result<(), SessionError> {
        if let Some(uid) = module_info.read().unwrap().uid {
            if let Some(module) = self.read().unwrap().modules.get(uid) {
                module
                    .read()
                    .unwrap()
                    .module
                    .init_location(location_info.clone(), data);
                return Ok(());
            }
        }
        Err(SessionError::InvalidModule)
    }

    fn module_init_element(
        &self,
        module_info: &MRef,
        element_info: &ERef,
    ) -> Result<(), SessionError> {
        if let Some(uid) = module_info.read().unwrap().uid {
            let mut module = None;
            if let Some(m) = self.read().unwrap().modules.get(uid) {
                module = Some(m.clone());
            }
            if let Some(module) = module {
                let module = module.read().unwrap();
                let element = self.get_element(element_info)?;
                module
                    .module
                    .init_settings(&mut element.write().unwrap().module_data);
                module
                    .module
                    .init_element_settings(&mut element.write().unwrap().element_data);
                module.module.init_element(element);
                return Ok(());
            }
        }
        Err(SessionError::InvalidModule)
    }

    fn moduie_accept_url(&self, module_info: &MRef, url: url::Url) -> Result<bool, SessionError> {
        if let Some(uid) = module_info.read().unwrap().uid {
            if let Some(module) = self.read().unwrap().modules.get(uid) {
                return Ok(module.read().unwrap().module.accept_url(url));
            }
        }
        Err(SessionError::InvalidModule)
    }

    fn module_accept_extension(
        &self,
        module_info: &MRef,
        filename: &str,
    ) -> Result<bool, SessionError> {
        if let Some(uid) = module_info.read().unwrap().uid {
            if let Some(module) = self.read().unwrap().modules.get(uid) {
                return Ok(module.read().unwrap().module.accept_extension(filename));
            }
        }
        Err(SessionError::InvalidModule)
    }

    fn module_step_element(
        &self,
        module_info: &MRef,
        element_info: &ERef,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError> {
        let module;
        {
            let m = self.get_module(module_info)?;
            module = m.read().unwrap().module.c();
        }

        let element = self.get_element(element_info)?;

        module.step_element(element, control_flow, storage);
        Ok(())
    }

    fn module_step_location(
        &self,
        _module_info: &MRef,
        _location_info: &LRef,
    ) -> Result<(), SessionError> {
        todo!()
    }

    fn create_element(&self, name: &str, location: &LRef) -> Result<ERef, SessionError> {
        let element_uid = self.get_location(location)?.read().unwrap().elements.len();
        let element_info = Arc::new(RwLock::new(RefElement {
            session: Some(self.c()),
            location: location.clone(),
            uid: element_uid,
        }));

        let path = location.get_path()?.join(name);

        let element = Arc::new(RwLock::new(Element {
            name: name.to_owned(),
            desc: String::new(),
            meta: String::new(),
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
        }));

        self.get_location(location)?
            .write()
            .unwrap()
            .elements
            .push(element);

        Ok(element_info)
    }

    fn move_element(&self, element: &ERef, location: &LRef) -> Result<(), SessionError> {
        let elem = self.destroy_element(element.clone())?;
        let new_uid = self.get_location(location)?.read().unwrap().elements.len();
        elem.read().unwrap().info.write().unwrap().uid = new_uid;
        elem.read().unwrap().info.write().unwrap().location = location.clone();
        self.get_location(location)?
            .write()
            .unwrap()
            .elements
            .push(elem);
        Ok(())
    }

    fn destroy_element(&self, element: ERef) -> Result<ERow, SessionError> {
        let element = self
            .get_location(&element.read().unwrap().location)?
            .write()
            .unwrap()
            .elements
            .remove(element.read().unwrap().uid);
        for (i, element) in self
            .get_location(&element.read().unwrap().info.read().unwrap().location)?
            .read()
            .unwrap()
            .elements
            .iter()
            .enumerate()
        {
            element.read().unwrap().info.write().unwrap().uid = i
        }
        Ok(element)
    }

    fn element_get_name(&self, element: &ERef) -> Result<String, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().name.clone())
    }

    fn element_set_name(&self, element: &ERef, name: &str) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().name = name.to_owned();
        Ok(())
    }

    fn element_get_desc(&self, element: &ERef) -> Result<String, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().desc.clone())
    }

    fn element_set_desc(&self, element: &ERef, desc: &str) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().desc = desc.to_owned();
        Ok(())
    }

    fn element_get_meta(&self, element: &ERef) -> Result<String, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().meta.clone())
    }

    fn element_set_meta(&self, element: &ERef, meta: &str) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().meta = meta.to_owned();
        Ok(())
    }

    fn element_get_element_data(&self, element: &ERef) -> Result<Data, SessionError> {
        Ok(self
            .get_element(element)?
            .read()
            .unwrap()
            .element_data
            .clone())
    }

    fn element_set_element_data(&self, element: &ERef, data: Data) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().element_data = data;
        Ok(())
    }

    fn element_get_module_data(&self, element: &ERef) -> Result<Data, SessionError> {
        Ok(self
            .get_element(element)?
            .read()
            .unwrap()
            .module_data
            .clone())
    }

    fn element_set_module_data(&self, element: &ERef, data: Data) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().module_data = data;
        Ok(())
    }

    fn element_get_module(&self, element: &ERef) -> Result<Option<MRef>, SessionError> {
        if let Some(module) = &self.get_element(element)?.read().unwrap().module {
            Ok(Some(module.clone()))
        } else {
            Ok(None)
        }
    }

    fn element_set_module(&self, element: &ERef, module: Option<MRef>) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().module = module;
        Ok(())
    }

    fn element_get_statuses(&self, element: &ERef) -> Result<Vec<String>, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().statuses.clone())
    }

    fn element_set_statuses(
        &self,
        element: &ERef,
        statuses: Vec<String>,
    ) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().statuses = statuses;
        Ok(())
    }

    fn element_get_status(&self, element: &ERef) -> Result<usize, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().status)
    }

    fn element_set_status(&self, element: &ERef, status: usize) -> Result<(), SessionError> {
        let statuses = self.get_element(element)?.read().unwrap().statuses.len();
        if status < statuses {
            self.get_element(element)?.write().unwrap().status = status;
            Ok(())
        } else {
            Err(SessionError::InvalidElementStatus)
        }
    }

    fn element_get_data(&self, element: &ERef) -> Result<FileOrData, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().data.clone())
    }

    fn element_set_data(&self, element: &ERef, data: FileOrData) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().data = data;
        Ok(())
    }

    fn element_get_progress(&self, element: &ERef) -> Result<f32, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().progress)
    }

    fn element_set_progress(&self, element: &ERef, progress: f32) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().progress = progress;
        Ok(())
    }

    fn element_get_should_save(&self, element: &ERef) -> Result<bool, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().should_save)
    }

    fn element_set_should_save(
        &self,
        element: &ERef,
        should_save: bool,
    ) -> Result<(), SessionError> {
        self.get_element(element)?.write().unwrap().should_save = should_save;
        Ok(())
    }

    fn element_get_enabled(&self, element: &ERef) -> Result<bool, SessionError> {
        Ok(self.get_element(element)?.read().unwrap().enabled)
    }

    fn element_set_enabled(
        &self,
        element: &ERef,
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
                            panic!("Is inposibile!")
                        }
                    } else {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        continue;
                    }
                    if let Some(module) = module {
                        module
                            .step_element(&element_info, &mut control_flow, &mut storage)
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

    fn element_resolv_module(&self, element_info: &ERef) -> Result<bool, SessionError> {
        let len = self.get_modules_len()?;

        let mut module = None;

        if let Some(Type::String(url)) = element_info.get_element_data().unwrap().get("url") {
            for tmp_module in self.get_modules(0..len)? {
                if self
                    .get_module(&tmp_module)?
                    .read()
                    .unwrap()
                    .module
                    .accept_url(Url::parse(url).unwrap())
                {
                    module = Some(tmp_module);
                    break;
                }
            }
        }

        if let Some(module) = module {
            self.element_set_module(element_info, Some(module))?;
            return Ok(true);
        }

        Ok(false)
    }

    fn element_wait(&self, element: &ERef) -> Result<(), SessionError> {
        let thread = self.get_element(element)?.write().unwrap().thread.take();
        if let Some(thread) = thread {
            thread.join().unwrap();
        }
        Ok(())
    }

    fn element_get_element_info(&self, element: &ERef) -> Result<ElementInfo, SessionError> {
        let element = self.get_element(element)?;
        let mut module = None;
        {
            let __module;
            {
                __module = element.read().unwrap().module.clone();
            }

            if let Some(__module) = __module {
                let __module = self.get_module(&__module)?;
                let __module = __module.read().unwrap();

                let mut hasher = DefaultHasher::new();
                let hasher = &mut hasher;
                __module.name.hash(hasher);
                __module.desc.hash(hasher);
                let id = hasher.finish();

                module = Some(ModuleInfo {
                    name: __module.name.clone(),
                    desc: __module.desc.clone(),
                    module: id,
                    proxy: __module.proxy,
                    settings: __module.settings.clone(),
                    element_data: __module.element_data.clone(),
                });
            }
        }
        let element = element.read().unwrap();
        Ok(ElementInfo {
            name: element.name.clone(),
            desc: element.desc.clone(),
            meta: element.meta.clone(),
            element_data: element.element_data.clone(),
            module_data: element.module_data.clone(),
            module,
            statuses: element.statuses.clone(),
            status: element.status,
            data: element.data.clone(),
            progress: element.progress,
            should_save: element.should_save,
            enabled: element.enabled,
        })
    }

    fn create_location(&self, name: &str, location: &LRef) -> Result<LRef, SessionError> {
        let mut location_uid = location.read().unwrap().uid.clone();
        location_uid.push(self.get_locations_len(location)?);
        let location_info = Arc::new(RwLock::new(RefLocation {
            session: Some(self.c()),
            uid: location_uid,
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
        };

        let dest = self.get_location(location)?;
        dest.write()
            .unwrap()
            .locations
            .push(Arc::new(RwLock::new(loc)));

        Ok(location_info)
    }

    fn get_locations_len(&self, location: &LRef) -> Result<usize, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().locations.len())
    }

    fn get_locations(
        &self,
        location: &LRef,
        range: Range<usize>,
    ) -> Result<Vec<LRef>, SessionError> {
        let mut location_infos = Vec::new();
        let location = self.get_location(location)?;
        for loc in location.read().unwrap().locations[range].iter() {
            location_infos.push(loc.read().unwrap().info.clone());
        }
        Ok(location_infos)
    }

    fn destroy_location(&self, location: LRef) -> Result<LRow, SessionError> {
        let mut location_uid = location.read().unwrap().uid.clone();
        if let Some(location_index) = location_uid.pop() {
            let parent_location = Arc::new(RwLock::new(RefLocation {
                session: None,
                uid: location_uid,
            }));
            let parent_location = self.get_location(&parent_location)?;

            let removed_location = parent_location
                .write()
                .unwrap()
                .locations
                .remove(location_index);
            for (i, location) in parent_location.read().unwrap().locations.iter().enumerate() {
                *location
                    .read()
                    .unwrap()
                    .info
                    .write()
                    .unwrap()
                    .uid
                    .last_mut()
                    .unwrap() = i
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

    fn move_location(&self, location: &LRef, to: &LRef) -> Result<(), SessionError> {
        let location = self.destroy_location(location.clone())?;
        let mut location_uid = to.read().unwrap().uid.clone();
        location_uid.push(self.get_locations_len(to)?);
        let location_info = Arc::new(RwLock::new(RefLocation {
            session: Some(self.c()),
            uid: location_uid,
        }));
        location.write().unwrap().info = location_info;
        self.get_location(to)?
            .write()
            .unwrap()
            .locations
            .push(location);

        Ok(())
    }

    fn location_get_name(&self, location: &LRef) -> Result<String, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().name.clone())
    }

    fn location_set_name(&self, location: &LRef, name: &str) -> Result<(), SessionError> {
        self.get_location(location)?.write().unwrap().name = name.to_owned();
        Ok(())
    }

    fn location_get_desc(&self, location: &LRef) -> Result<String, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().desc.clone())
    }

    fn location_set_desc(&self, location: &LRef, desc: &str) -> Result<(), SessionError> {
        self.get_location(location)?.write().unwrap().desc = desc.to_owned();
        Ok(())
    }

    fn location_get_path(&self, location: &LRef) -> Result<PathBuf, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().path.clone())
    }

    fn location_set_path(&self, location: &LRef, path: PathBuf) -> Result<(), SessionError> {
        self.get_location(location)?.write().unwrap().path = path;
        Ok(())
    }

    fn location_get_where_is(&self, location: &LRef) -> Result<WhereIsLocation, SessionError> {
        Ok(self
            .get_location(location)?
            .read()
            .unwrap()
            .where_is
            .clone())
    }

    fn location_set_where_is(
        &self,
        location: &LRef,
        where_is: WhereIsLocation,
    ) -> Result<(), SessionError> {
        self.get_location(location)?.write().unwrap().where_is = where_is;
        Ok(())
    }

    fn location_get_should_save(&self, location: &LRef) -> Result<bool, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().shoud_save)
    }

    fn location_set_should_save(
        &self,
        location: &LRef,
        should_save: bool,
    ) -> Result<(), SessionError> {
        self.get_location(location)?.write().unwrap().shoud_save = should_save;
        Ok(())
    }

    fn location_get_elements_len(&self, location: &LRef) -> Result<usize, SessionError> {
        Ok(self.get_location(location)?.read().unwrap().elements.len())
    }

    fn location_get_elements(
        &self,
        location: &LRef,
        range: Range<usize>,
    ) -> Result<Vec<ERef>, SessionError> {
        let mut element_infos = Vec::new();
        for element in self.get_location(location)?.read().unwrap().elements[range].iter() {
            element_infos.push(element.read().unwrap().info.clone())
        }
        Ok(element_infos)
    }

    fn location_get_location_info(&self, location: &LRef) -> Result<LocationInfo, SessionError> {
        let mut module = None;
        {
            let __module;
            {
                let location = self.get_location(location)?;
                __module = location.read().unwrap().module.clone();
            }

            if let Some(__module) = __module {
                let __module = self.get_module(&__module)?;
                let __module = __module.read().unwrap();

                let mut hasher = DefaultHasher::new();
                let hasher = &mut hasher;
                __module.name.hash(hasher);
                __module.desc.hash(hasher);
                let id = hasher.finish();

                module = Some(ModuleInfo {
                    name: __module.name.clone(),
                    desc: __module.desc.clone(),
                    module: id,
                    proxy: __module.proxy,
                    settings: __module.settings.clone(),
                    element_data: __module.element_data.clone(),
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

        let mut hasher = DefaultHasher::new();
        let hasher = &mut hasher;
        location.name.hash(hasher);
        location.desc.hash(hasher);

        let id = hasher.finish();

        Ok(LocationInfo {
            name: location.name.clone(),
            desc: location.desc.clone(),
            id,
            where_is: location.where_is.clone(),
            shoud_save: location.shoud_save,
            elements,
            locations,
            path: location.path.clone(),
            module,
        })
    }

    fn c(&self) -> Box<dyn TSession> {
        Box::new(self.clone())
    }
}
