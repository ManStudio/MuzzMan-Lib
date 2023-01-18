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

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocationId(pub Vec<usize>);

impl std::ops::Deref for LocationId {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LocationId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::iter::IntoIterator for LocationId {
    type Item = usize;

    type IntoIter = std::vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl LocationId {
    pub fn into_ref(self, session: Box<dyn TSession>) -> RefLocation {
        RefLocation {
            session: Some(session),
            id: self,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefLocation {
    #[serde(skip)]
    pub session: Option<Box<dyn TSession>>,
    pub id: LocationId,
}

impl Debug for RefLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocatioInfo")
            .field("uid", &self.id)
            .finish()
    }
}

impl Clone for RefLocation {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
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
        self.id.eq(&other.id)
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
        self.get_session()?.location_get_path(&self.id())
    }

    fn set_path(&self, path: PathBuf) -> Result<(), SessionError> {
        self.get_session()?.location_set_path(&self.id(), path)
    }

    fn get_where_is(&self) -> Result<WhereIsLocation, SessionError> {
        self.get_session()?.location_get_where_is(&self.id())
    }

    fn set_where_is(&self, where_is: WhereIsLocation) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_where_is(&self.id(), where_is)
    }

    fn get_should_save(&self) -> Result<bool, SessionError> {
        self.get_session()?.location_get_should_save(&self.id())
    }

    fn set_should_save(&self, should_save: bool) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_should_save(&self.id(), should_save)
    }

    fn get_elements(&self, range: Range<usize>) -> Result<Vec<ERef>, SessionError> {
        self.get_session()?.location_get_elements(&self.id(), range)
    }

    fn get_elements_len(&self) -> Result<usize, SessionError> {
        self.get_session()?.location_get_elements_len(&self.id())
    }

    fn get_locations(&self, range: Range<usize>) -> Result<Vec<LRef>, SessionError> {
        self.get_session()?.get_locations(&self.id(), range)
    }

    fn get_locations_len(&self) -> Result<usize, SessionError> {
        self.get_session()?.get_locations_len(&self.id())
    }

    fn get_location_info(&self) -> Result<LocationInfo, SessionError> {
        self.get_session()?.location_get_location_info(&self.id())
    }

    fn create_element(&self, name: &str) -> Result<ERef, SessionError> {
        self.get_session()?.create_element(name, &self.id())
    }

    fn create_location(&self, name: &str) -> Result<LRef, SessionError> {
        self.get_session()?.create_location(name, &self.id())
    }

    fn destroy(self) -> Result<LRow, SessionError> {
        self.get_session()?.destroy_location(self.id())
    }

    fn _move(&self, to: &LocationId) -> Result<(), SessionError> {
        self.get_session()?.move_location(&self.id(), to)
    }

    fn id(&self) -> LocationId {
        self.read().unwrap().id.clone()
    }
}

impl Common for LRef {
    fn get_name(&self) -> Result<String, SessionError> {
        self.get_session()?.location_get_name(&self.id())
    }

    fn set_name(&self, name: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_name(&self.id(), &name.into())
    }

    fn get_desc(&self) -> Result<String, SessionError> {
        self.get_session()?.location_get_desc(&self.id())
    }

    fn set_desc(&self, desc: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_desc(&self.id(), &desc.into())
    }

    fn notify(&self, event: Event) -> Result<(), SessionError> {
        self.get_session()?.location_notify(&self.id(), event)
    }

    fn emit(&self, event: Event) -> Result<(), SessionError> {
        self.get_session()?.location_emit(&self.id(), event)
    }

    fn subscribe(&self, _ref: ID) -> Result<(), SessionError> {
        self.get_session()?.location_subscribe(&self.id(), _ref)
    }

    fn unsubscribe(&self, _ref: ID) -> Result<(), SessionError> {
        self.get_session()?.location_unsubscribe(&self.id(), _ref)
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
    fn _move(&self, to: &LocationId) -> Result<(), SessionError>;

    fn id(&self) -> LocationId;
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

impl TGetLogger for LRef {
    fn get_logger(&self, dst: Option<Arc<Mutex<std::fs::File>>>) -> Logger {
        Logger::for_location(dst, self.clone())
    }
}

impl TGetLogger for LRow {
    fn get_logger(&self, dst: Option<Arc<Mutex<std::fs::File>>>) -> Logger {
        let info = self.read().unwrap().info.clone();
        Logger::for_location(dst, info)
    }
}
