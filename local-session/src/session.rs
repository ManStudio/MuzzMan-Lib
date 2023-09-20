use std::{
    path::PathBuf,
    sync::{Arc, RwLock, Weak},
};

use muzzman_lib::prelude::*;

use crate::{
    module::RawModule, ElementWraper, LocationWraper, ModuleWraper, Path, UIDPath, Wraper,
};

pub struct LocalSession {
    pub location: LocationWraper,
    pub refs: Vec<Path>,
    pub modules: Vec<ModuleWraper>,
    pub runtime: Arc<tokio::runtime::Runtime>,
}

impl LocalSession {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<dyn TLocalSession> {
        let path = Arc::new(RwLock::new(crate::UIDPath::Location(vec![])));
        let location = LocationWraper {
            location: Arc::new(RwLock::new(Location {
                name: "Default Location".into(),
                desc: "This is the default location of the muzzman session".into(),
                data: Default::default(),
                path: PathBuf::new(),
                settings: Default::default(),
                module: None,
                id: LocationId {
                    uid: 0,
                    session: None,
                },
                parent: None,
                locations: vec![],
                elements: vec![],
                buffer: vec![],
                status: 0,
                statuses: vec![],
                progress: 0.0,
                download_speed: 0,
                upload_speed: 0,
                download_speed_counter: 0,
                upload_speed_counter: 0,
                total_download: 0,
                total_upload: 0,
                enabled: false,
                is_error: false,
                is_completed: false,
            })),
            locations: Default::default(),
            elements: Default::default(),
            path: path.clone(),
            storage: Default::default(),
            thread: Default::default(),
            sender: Default::default(),
            events: Default::default(),
        };
        let s = Box::new(Arc::new(RwLock::new(Self {
            location,
            refs: vec![path],
            modules: vec![],
            runtime: Arc::new(
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap(),
            ),
        })));

        s.write().unwrap().location.location.write().unwrap().id = LocationId {
            uid: 0,
            session: Some(Session::from(Box::new(s.weak_clone()) as Box<dyn TSession>)),
        };

        s
    }

    pub(crate) fn register_path(&mut self, path: Path) -> UID {
        let id = self.refs.len();
        self.refs.push(path);
        id as UID
    }
}

pub trait TLocalSession: Send + Sync {
    /// create or get
    fn create_location(&self, name: String, path: &[usize]) -> LocationWraper;
    /// create or get
    fn create_element(&self, name: String, path: &[usize]) -> ElementWraper;

    fn get(&self, uid: UID) -> SessionResult<Wraper>;

    fn location(&self, uid: UID) -> SessionResult<LocationWraper>;
    fn element(&self, uid: UID) -> SessionResult<ElementWraper>;
    fn module(&self, uid: UID) -> SessionResult<ModuleWraper>;

    fn add_module(&self, source: ModuleSource) -> SessionResult<ModuleId>;

    fn default_location(&self) -> SessionResult<LocationId>;
    fn runtime(&self) -> Arc<tokio::runtime::Runtime>;

    fn weak_clone(&self) -> Box<dyn TLocalSession>;
}

impl TLocalSession for Arc<RwLock<LocalSession>> {
    fn create_location(&self, name: String, path: &[usize]) -> LocationWraper {
        let session = Session::from(Box::new(self.weak_clone()) as Box<dyn TSession>);
        let mut location = self.read().unwrap().location.clone();

        let mut traversed_path = Vec::<usize>::with_capacity(path.len());

        for index in path {
            let res = {
                let location = location.locations.read().unwrap();
                location.get(*index).cloned()
            };
            if let Some(tmp_location) = res {
                location = tmp_location;
            } else {
                let tmp_location = {
                    let mut s = self.write().unwrap();
                    let mut locations = location.locations.write().unwrap();
                    let mut location = location.location.write().unwrap();
                    let path = {
                        let mut path = traversed_path.clone();
                        path.push(locations.len());
                        path
                    };
                    let path = Arc::new(RwLock::new(crate::UIDPath::Location(path)));
                    let uid = s.register_path(path.clone());
                    let tmp_location = LocationWraper {
                        location: Arc::new(RwLock::new(Location {
                            name: name.clone(),
                            desc: Default::default(),
                            data: Default::default(),
                            path: Default::default(),
                            settings: Default::default(),
                            module: None,
                            id: LocationId {
                                uid,
                                session: Some(session.clone()),
                            },
                            parent: Some(location.id.clone()),
                            locations: Vec::new(),
                            elements: Vec::new(),
                            buffer: vec![],
                            status: 0,
                            statuses: vec![],
                            progress: 0.0,
                            download_speed: 0,
                            upload_speed: 0,
                            download_speed_counter: 0,
                            upload_speed_counter: 0,
                            total_download: 0,
                            total_upload: 0,
                            enabled: false,
                            is_error: false,
                            is_completed: false,
                        })),
                        locations: Default::default(),
                        elements: Default::default(),
                        path: path.clone(),
                        storage: Default::default(),
                        thread: Default::default(),
                        sender: Default::default(),
                        events: Default::default(),
                    };
                    locations.push(tmp_location.clone());
                    location.locations.push(LocationId {
                        uid,
                        session: Some(session.clone()),
                    });
                    tmp_location
                };
                location = tmp_location;
            }
            traversed_path.push(*index);
        }

        location
    }

    fn create_element(&self, name: String, path: &[usize]) -> ElementWraper {
        let element_index = path.last().unwrap();
        let location = self.create_location("Need an element".into(), &path[..path.len() - 1]);
        let mut elements = location.elements.write().unwrap();

        if let Some(element) = elements.get(*element_index) {
            element.clone()
        } else {
            let session = Session::from(Box::new(self.weak_clone()) as Box<dyn TSession>);
            let mut s = self.write().unwrap();
            let mut location = location.location.write().unwrap();
            let mut path = path.to_vec();
            path.pop();
            let path = Arc::new(RwLock::new(crate::UIDPath::Element(path, elements.len())));
            let uid = s.register_path(path.clone());
            let id = ElementId {
                uid,
                session: Some(session),
            };
            let element = ElementWraper {
                element: Arc::new(RwLock::new(Element {
                    name: name.clone(),
                    desc: Default::default(),
                    data: Default::default(),
                    settings: Default::default(),
                    path: location.path.clone().join(name),
                    module: None,
                    id: id.clone(),
                    parent: location.id.clone(),
                    stream: Stream::None,
                    buffer: vec![],
                    status: 0,
                    statuses: vec![],
                    progress: 0.0,
                    download_speed: 0,
                    upload_speed: 0,
                    download_speed_counter: 0,
                    upload_speed_counter: 0,
                    total_download: 0,
                    total_upload: 0,
                    enabled: false,
                    is_error: false,
                    is_completed: false,
                    url: String::default(),
                })),
                path: path.clone(),
                storage: Default::default(),
                thread: Default::default(),
                sender: Default::default(),
                events: Default::default(),
            };

            elements.push(element.clone());
            location.elements.push(id);
            element
        }
    }

    fn get(&self, uid: UID) -> SessionResult<Wraper> {
        let res = self.read().unwrap().refs.get(uid as usize).cloned();
        if let Some(path) = res {
            let path = &*path.read().unwrap();
            match path {
                UIDPath::Element(path, index) => {
                    let location = self.create_location("Trying to find element".into(), path);
                    let res = location.elements.read().unwrap().get(*index).cloned();
                    if let Some(element) = res {
                        return Ok(Wraper::Element(element));
                    }
                }
                UIDPath::Location(path) => {
                    let mut location = self.read().unwrap().location.clone();
                    for index in path {
                        let tmp_location = location.locations.read().unwrap().get(*index).cloned();
                        if let Some(tmp_location) = tmp_location {
                            location = tmp_location.clone()
                        } else {
                            return Err(SessionError::UIDIsNotALocation);
                        }
                    }
                    return Ok(Wraper::Location(location));
                }
                UIDPath::Module(index) => {
                    let s = self.read().unwrap();
                    if let Some(module) = s.modules.get(*index) {
                        return Ok(Wraper::Module(module.clone()));
                    }
                }
                UIDPath::None => return Err(SessionError::UIDWasDestroyed),
            }
        }
        Err(SessionError::InvalidUID)
    }

    fn location(&self, uid: UID) -> SessionResult<LocationWraper> {
        let Wraper::Location(location) = self.get(uid)? else {
            return Err(SessionError::UIDIsNotALocation);
        };
        Ok(location)
    }

    fn element(&self, uid: UID) -> SessionResult<ElementWraper> {
        let Wraper::Element(element) = self.get(uid)? else {
            return Err(SessionError::UIDIsNotAElement);
        };
        Ok(element)
    }

    fn module(&self, uid: UID) -> SessionResult<ModuleWraper> {
        let Wraper::Module(module) = self.get(uid)? else {
            return Err(SessionError::UIDIsNotAModule);
        };
        Ok(module)
    }

    fn default_location(&self) -> SessionResult<LocationId> {
        let location = self.read().unwrap().location.clone();
        let id = location.location.read().unwrap().id.clone();
        Ok(id)
    }

    fn weak_clone(&self) -> Box<dyn TLocalSession> {
        Box::new(Arc::downgrade(self))
    }

    fn add_module(&self, source: ModuleSource) -> SessionResult<ModuleId> {
        let uid = {
            let module = match source {
                ModuleSource::Wasm(_) => unimplemented!(),
                ModuleSource::Dynamic(path) => RawModule::new_module(&path)?,
                ModuleSource::Box(module) => module,
            };
            let id = module.id();
            let mut s = self.write().unwrap();
            let mut index = s.modules.len();
            for (i, module) in s.modules.iter().enumerate() {
                if id == module.module.read().unwrap().module.id() {
                    index = i;
                    break;
                }
            }

            let path = Arc::new(RwLock::new(UIDPath::Module(index)));
            let uid = s.register_path(path.clone());

            let module = ModuleWraper {
                module: Arc::new(RwLock::new(Module {
                    name: module.name().to_string(),
                    desc: module.desc().to_string(),
                    proxy: 0,
                    element_settings: module.default_element_settings(),
                    location_settings: module.default_location_settings(),
                    module,
                })),
                path,
            };

            s.modules.push(module);
            uid
        };
        Ok(ModuleId {
            uid,
            session: Some((Box::new(self.weak_clone()) as Box<dyn TSession>).into()),
        })
    }

    fn runtime(&self) -> Arc<tokio::runtime::Runtime> {
        self.read().unwrap().runtime.clone()
    }
}

const UPGRADE_ERROR: &str = "LocalSession Was Freed!";
impl TLocalSession for Weak<RwLock<LocalSession>> {
    fn create_location(&self, name: String, path: &[usize]) -> LocationWraper {
        self.upgrade()
            .expect(UPGRADE_ERROR)
            .create_location(name, path)
    }

    fn create_element(&self, name: String, path: &[usize]) -> ElementWraper {
        self.upgrade()
            .expect(UPGRADE_ERROR)
            .create_element(name, path)
    }

    fn get(&self, uid: UID) -> SessionResult<Wraper> {
        self.upgrade().expect(UPGRADE_ERROR).get(uid)
    }

    fn location(&self, uid: UID) -> SessionResult<LocationWraper> {
        self.upgrade().expect(UPGRADE_ERROR).location(uid)
    }

    fn element(&self, uid: UID) -> SessionResult<ElementWraper> {
        self.upgrade().expect(UPGRADE_ERROR).element(uid)
    }

    fn module(&self, uid: UID) -> SessionResult<ModuleWraper> {
        self.upgrade().expect(UPGRADE_ERROR).module(uid)
    }

    fn add_module(&self, source: ModuleSource) -> SessionResult<ModuleId> {
        self.upgrade().expect(UPGRADE_ERROR).add_module(source)
    }

    fn default_location(&self) -> SessionResult<LocationId> {
        self.upgrade().expect(UPGRADE_ERROR).default_location()
    }

    fn weak_clone(&self) -> Box<dyn TLocalSession> {
        Box::new(self.clone())
    }

    fn runtime(&self) -> Arc<tokio::runtime::Runtime> {
        self.upgrade().expect(UPGRADE_ERROR).runtime()
    }
}

impl TSession for Box<dyn TLocalSession> {
    fn weak_box(&self) -> Box<dyn TSession> {
        Box::new(self.weak_clone())
    }

    fn version(&self) -> SessionResult<u64> {
        Ok(1)
    }

    fn version_str(&self) -> SessionResult<String> {
        Ok("LocalSession: 1".to_string())
    }
}
