use std::fmt::Debug;
use std::net::{IpAddr, TcpStream};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum LocationNotify {
    ElementNotify(usize, ElementNotify),
    ModuleChanged(Option<MInfo>),
    ElementsAllCompleted,
    Completed,
    Custom(String),
}

impl_get_ref!(LocationNotify);

#[derive(Clone)]
pub struct ServerLocation {
    pub ip: IpAddr,
    pub port: u16,
    pub indentification: String,
    pub server_cert: Option<String>,
    pub conn: Option<Arc<Mutex<TcpStream>>>,
}

#[derive(Clone)]
pub struct LocalLocation {
    pub path: PathBuf,
}

#[derive(Clone)]
pub enum WhereIsLocation {
    Server(ServerLocation),
    Local(LocalLocation),
}

pub struct LocationInfo {
    pub session: Option<Box<dyn TSession>>,
    pub uid: Vec<usize>,
}

impl Debug for LocationInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocatioInfo")
            .field("uid", &self.uid)
            .finish()
    }
}

impl Clone for LocationInfo {
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

impl PartialEq for LocationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.uid.eq(&other.uid)
    }
}

#[derive(Clone)]
pub struct Location {
    pub name: String,
    pub desc: String,
    pub where_is: WhereIsLocation,
    pub shoud_save: bool,
    pub elements: Vec<ERow>,
    pub locations: Vec<LRow>,
    pub info: LInfo,
    pub module: Option<Box<dyn TModule>>,
    pub signal_notify: Arc<RwLock<AdvancedSignal<LocationNotify, ()>>>,
}

impl TLocation for LInfo {
    fn get_session(&self) -> Option<Box<dyn TSession>> {
        let mut s = None;
        if let Some(session) = &self.read().unwrap().session {
            s = Some(session.c())
        }
        s
    }

    fn get_name(&self) -> Result<String, SessionError> {
        if let Some(session) = self.get_session() {
            return session.location_get_name(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_name(&self, name: &str) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.location_set_name(self, name);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_desc(&self) -> Result<String, SessionError> {
        if let Some(session) = self.get_session() {
            return session.location_get_desc(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_desc(&self, desc: &str) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.location_set_desc(self, desc);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_where_is(&self) -> Result<WhereIsLocation, SessionError> {
        if let Some(session) = self.get_session() {
            return session.location_get_where_is(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_where_is(&self, where_is: WhereIsLocation) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.location_set_where_is(self, where_is);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_should_save(&self) -> Result<bool, SessionError> {
        if let Some(session) = self.get_session() {
            return session.location_get_should_save(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_should_save(&self, should_save: bool) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.location_set_should_save(self, should_save);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_elements(&self, range: Range<usize>) -> Result<Vec<EInfo>, SessionError> {
        if let Some(session) = self.get_session() {
            return session.location_get_elements(self, range);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_elements_len(&self) -> Result<usize, SessionError> {
        if let Some(session) = self.get_session() {
            return session.location_get_elements_len(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_locations(&self, range: Range<usize>) -> Result<Vec<LInfo>, SessionError> {
        if let Some(session) = self.get_session() {
            return session.get_locations(self, range);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_locations_len(&self) -> Result<usize, SessionError> {
        if let Some(session) = self.get_session() {
            return session.get_locations_len(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn create_element(&self, name: &str) -> Result<EInfo, SessionError> {
        if let Some(session) = self.get_session() {
            return session.create_element(name, self);
        }
        Err(SessionError::InvalidSession)
    }

    fn create_location(&self, name: &str) -> Result<LInfo, SessionError> {
        if let Some(session) = self.get_session() {
            return session.create_location(name, self);
        }
        Err(SessionError::InvalidSession)
    }

    fn destroy(self) -> Result<LRow, SessionError> {
        if let Some(session) = self.get_session() {
            return session.destroy_location(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn _move(&self, to: &LInfo) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.move_location(self, to);
        }
        Err(SessionError::InvalidSession)
    }
}

pub trait TLocation {
    fn get_session(&self) -> Option<Box<dyn TSession>>;

    fn get_name(&self) -> Result<String, SessionError>;
    fn set_name(&self, name: &str) -> Result<(), SessionError>;

    fn get_desc(&self) -> Result<String, SessionError>;
    fn set_desc(&self, desc: &str) -> Result<(), SessionError>;

    fn get_where_is(&self) -> Result<WhereIsLocation, SessionError>;
    fn set_where_is(&self, where_is: WhereIsLocation) -> Result<(), SessionError>;

    fn get_should_save(&self) -> Result<bool, SessionError>;
    fn set_should_save(&self, should_save: bool) -> Result<(), SessionError>;

    fn get_elements(&self, range: Range<usize>) -> Result<Vec<EInfo>, SessionError>;
    fn get_elements_len(&self) -> Result<usize, SessionError>;
    fn get_locations(&self, range: Range<usize>) -> Result<Vec<LInfo>, SessionError>;
    fn get_locations_len(&self) -> Result<usize, SessionError>;

    fn create_element(&self, name: &str) -> Result<EInfo, SessionError>;
    fn create_location(&self, name: &str) -> Result<LInfo, SessionError>;

    fn destroy(self) -> Result<LRow, SessionError>;
    fn _move(&self, to: &LInfo) -> Result<(), SessionError>;
}
