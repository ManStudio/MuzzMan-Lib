use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use muzzman_lib::prelude::*;

use crate::{ElementWraper, LocationWraper, ModuleWraper, Path, UIDPath, Wraper};

pub struct LocalSession {
    pub location: LocationWraper,
    pub refs: Vec<Path>,
}

impl LocalSession {
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
        };
        let s = Box::new(Arc::new(RwLock::new(Self {
            location,
            refs: vec![path],
        })));

        s.write().unwrap().location.location.write().unwrap().id = LocationId {
            uid: 0,
            session: Some(Session::from(Box::new(s.c()) as Box<dyn TSession>)),
        };

        s.c()
    }

    pub(crate) fn register_path(&mut self, path: Path) -> UID {
        let id = self.refs.len();
        self.refs.push(path);
        id as UID
    }
}

pub trait TLocalSession {
    /// create or get
    fn create_location(&self, name: String, path: &[usize]) -> LocationWraper;
    /// create or get
    fn create_element(&self, name: String, path: &[usize]) -> ElementWraper;

    fn get(&self, uid: UID) -> SessionResult<Wraper>;

    fn get_location(&self, uid: UID) -> SessionResult<LocationWraper>;
    fn get_element(&self, uid: UID) -> SessionResult<ElementWraper>;
    fn get_module(&self, uid: UID) -> SessionResult<ModuleWraper>;

    fn get_default_location(&self) -> SessionResult<LocationId>;

    fn c(&self) -> Box<dyn TLocalSession>;
}

impl TLocalSession for Box<Arc<RwLock<LocalSession>>> {
    fn create_location(&self, name: String, path: &[usize]) -> LocationWraper {
        let session = Session::from(Box::new(self.c()) as Box<dyn TSession>);
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
            let session = Session::from(Box::new(self.c()) as Box<dyn TSession>);
            let mut s = self.write().unwrap();
            let mut location = location.location.write().unwrap();
            let mut path = path.to_vec();
            path.pop();
            let path = Arc::new(RwLock::new(crate::UIDPath::Element(path, elements.len())));
            let uid = s.register_path(path.clone());
            let id = ElementId {
                uid: uid,
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
                })),
                path: path.clone(),
                storage: Default::default(),
                thread: Default::default(),
                sender: Default::default(),
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
                    unimplemented!()
                }
                UIDPath::None => return Err(SessionError::UIDWasDestroyed),
            }
        }
        Err(SessionError::InvalidUID)
    }

    fn get_location(&self, uid: UID) -> SessionResult<LocationWraper> {
        let Wraper::Location(location) = self.get(uid)?else {return Err(SessionError::UIDIsNotALocation)};
        Ok(location)
    }

    fn get_element(&self, uid: UID) -> SessionResult<ElementWraper> {
        let Wraper::Element(element) = self.get(uid)?else {return Err(SessionError::UIDIsNotAElement)};
        Ok(element)
    }

    fn get_module(&self, uid: UID) -> SessionResult<ModuleWraper> {
        let Wraper::Module(module) = self.get(uid)?else {return Err(SessionError::UIDIsNotAModule)};
        Ok(module)
    }

    fn get_default_location(&self) -> SessionResult<LocationId> {
        let location = self.read().unwrap().location.clone();
        let id = location.location.read().unwrap().id.clone();
        Ok(id)
    }

    fn c(&self) -> Box<dyn TLocalSession> {
        Box::new(self.clone())
    }
}

impl TSession for Box<dyn TLocalSession> {
    fn clone_box(&self) -> Box<dyn TSession> {
        Box::new(self.c())
    }

    fn version(&self) -> SessionResult<u64> {
        Ok(1)
    }

    fn version_str(&self) -> SessionResult<String> {
        Ok("LocalSession: 1".to_string())
    }
}
