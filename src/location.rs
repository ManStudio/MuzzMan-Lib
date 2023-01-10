use std::fmt::Debug;
use std::hash::Hash;
use std::net::{IpAddr, TcpStream};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::JoinHandle;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum LocationNotify {
    ElementNotify(usize, ElementNotify),
    ModuleChanged(Option<MRef>),
    ElementsAllCompleted,
    Completed,
}

impl_get_ref!(LocationNotify);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerLocation {
    pub ip: IpAddr,
    pub port: u16,
    pub indentification: String,
    pub server_cert: Option<String>,
    #[serde(skip)]
    pub conn: Option<Arc<Mutex<TcpStream>>>,
}

impl Hash for ServerLocation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ip.hash(state);
        self.port.hash(state);
        self.indentification.hash(state);
        self.server_cert.hash(state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct LocalLocation {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub enum WhereIsLocation {
    Server(ServerLocation),
    Local(LocalLocation),
}

#[derive(Serialize, Deserialize)]
pub struct RefLocation {
    #[serde(skip)]
    pub session: Option<Box<dyn TSession>>,
    pub uid: Vec<usize>,
}

impl Debug for RefLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocatioInfo")
            .field("uid", &self.uid)
            .finish()
    }
}

impl Clone for RefLocation {
    fn clone(&self) -> Self {
        Self {
            uid: self.uid.clone(),
            session: if let Some(session) = &self.session {
                Some(session.c())
            } else {
                None
            },
        }
    }
}

impl PartialEq for RefLocation {
    fn eq(&self, other: &Self) -> bool {
        self.uid.eq(&other.uid)
    }
}

pub struct Location {
    pub name: String,
    pub desc: String,
    pub where_is: WhereIsLocation,
    pub shoud_save: bool,
    pub elements: Vec<ERow>,
    pub locations: Vec<LRow>,
    pub info: LRef,
    pub path: PathBuf,
    pub thread: Option<JoinHandle<()>>,
    pub module: Option<MRef>,
    pub events: Arc<RwLock<Events>>,
}

impl TLocation for LRef {
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError> {
        if let Some(session) = &self.read().unwrap().session {
            return Ok(session.c());
        }

        Err(SessionError::InvalidSession)
    }

    fn get_path(&self) -> Result<PathBuf, SessionError> {
        self.get_session()?.location_get_path(self)
    }

    fn set_path(&self, path: PathBuf) -> Result<(), SessionError> {
        self.get_session()?.location_set_path(self, path)
    }

    fn get_where_is(&self) -> Result<WhereIsLocation, SessionError> {
        self.get_session()?.location_get_where_is(self)
    }

    fn set_where_is(&self, where_is: WhereIsLocation) -> Result<(), SessionError> {
        self.get_session()?.location_set_where_is(self, where_is)
    }

    fn get_should_save(&self) -> Result<bool, SessionError> {
        self.get_session()?.location_get_should_save(self)
    }

    fn set_should_save(&self, should_save: bool) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_should_save(self, should_save)
    }

    fn get_elements(&self, range: Range<usize>) -> Result<Vec<ERef>, SessionError> {
        self.get_session()?.location_get_elements(self, range)
    }

    fn get_elements_len(&self) -> Result<usize, SessionError> {
        self.get_session()?.location_get_elements_len(self)
    }

    fn get_locations(&self, range: Range<usize>) -> Result<Vec<LRef>, SessionError> {
        self.get_session()?.get_locations(self, range)
    }

    fn get_locations_len(&self) -> Result<usize, SessionError> {
        self.get_session()?.get_locations_len(self)
    }

    fn get_location_info(&self) -> Result<LocationInfo, SessionError> {
        self.get_session()?.location_get_location_info(self)
    }

    fn create_element(&self, name: &str) -> Result<ERef, SessionError> {
        self.get_session()?.create_element(name, self)
    }

    fn create_location(&self, name: &str) -> Result<LRef, SessionError> {
        self.get_session()?.create_location(name, self)
    }

    fn destroy(self) -> Result<LRow, SessionError> {
        self.get_session()?.destroy_location(self)
    }

    fn _move(&self, to: &LRef) -> Result<(), SessionError> {
        self.get_session()?.move_location(self, to)
    }
}

impl Common for LRef {
    fn get_name(&self) -> Result<String, SessionError> {
        self.get_session()?.location_get_name(self)
    }

    fn set_name(&self, name: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?.location_set_name(self, &name.into())
    }

    fn get_desc(&self) -> Result<String, SessionError> {
        self.get_session()?.location_get_desc(self)
    }

    fn set_desc(&self, desc: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?.location_set_desc(self, &desc.into())
    }

    fn notify(&self, event: Event) -> Result<(), SessionError> {
        self.get_session()?.location_notify(self, event)
    }

    fn emit(&self, event: Event) -> Result<(), SessionError> {
        self.get_session()?.location_emit(self, event)
    }

    fn subscribe(&self, _ref: Ref) -> Result<(), SessionError> {
        self.get_session()?.location_subscribe(self, _ref)
    }

    fn unsubscribe(&self, _ref: Ref) -> Result<(), SessionError> {
        self.get_session()?.location_unsubscribe(self, _ref)
    }
}

pub trait TLocation {
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError>;

    fn get_path(&self) -> Result<PathBuf, SessionError>;
    fn set_path(&self, path: PathBuf) -> Result<(), SessionError>;

    fn get_where_is(&self) -> Result<WhereIsLocation, SessionError>;
    fn set_where_is(&self, where_is: WhereIsLocation) -> Result<(), SessionError>;

    fn get_should_save(&self) -> Result<bool, SessionError>;
    fn set_should_save(&self, should_save: bool) -> Result<(), SessionError>;

    fn get_elements(&self, range: Range<usize>) -> Result<Vec<ERef>, SessionError>;
    fn get_elements_len(&self) -> Result<usize, SessionError>;
    fn get_locations(&self, range: Range<usize>) -> Result<Vec<LRef>, SessionError>;
    fn get_locations_len(&self) -> Result<usize, SessionError>;

    fn get_location_info(&self) -> Result<LocationInfo, SessionError>;

    fn create_element(&self, name: &str) -> Result<ERef, SessionError>;
    fn create_location(&self, name: &str) -> Result<LRef, SessionError>;

    fn destroy(self) -> Result<LRow, SessionError>;
    fn _move(&self, to: &LRef) -> Result<(), SessionError>;
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash)]
pub struct LocationInfo {
    pub name: String,
    pub desc: String,
    // hash
    pub id: u64,
    pub where_is: WhereIsLocation,
    pub shoud_save: bool,
    pub elements: Vec<ElementInfo>,
    pub locations: Vec<LocationInfo>,
    pub path: PathBuf,
    pub module: Option<ModuleInfo>,
}
