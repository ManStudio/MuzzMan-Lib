use std::fmt::Debug;
use std::net::{IpAddr, TcpStream};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

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

pub struct Location {
    pub name: String,
    pub desc: String,
    pub where_is: WhereIsLocation,
    pub shoud_save: bool,
    pub elements: Vec<ERow>,
    pub locations: Vec<LRow>,
    pub info: LInfo,
    pub path: PathBuf,
    pub thread: JoinHandle<()>,
    pub module: Option<Box<dyn TModule>>,
}

impl TLocation for LInfo {
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError> {
        if let Some(session) = &self.read().unwrap().session {
            return Ok(session.c());
        }

        Err(SessionError::InvalidSession)
    }

    fn get_name(&self) -> Result<String, SessionError> {
        self.get_session()?.location_get_name(self)
    }

    fn set_name(&self, name: &str) -> Result<(), SessionError> {
        self.get_session()?.location_set_name(self, name)
    }

    fn get_desc(&self) -> Result<String, SessionError> {
        self.get_session()?.location_get_desc(self)
    }

    fn set_desc(&self, desc: &str) -> Result<(), SessionError> {
        self.get_session()?.location_set_desc(self, desc)
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

    fn get_elements(&self, range: Range<usize>) -> Result<Vec<EInfo>, SessionError> {
        self.get_session()?.location_get_elements(self, range)
    }

    fn get_elements_len(&self) -> Result<usize, SessionError> {
        self.get_session()?.location_get_elements_len(self)
    }

    fn get_locations(&self, range: Range<usize>) -> Result<Vec<LInfo>, SessionError> {
        self.get_session()?.get_locations(self, range)
    }

    fn get_locations_len(&self) -> Result<usize, SessionError> {
        self.get_session()?.get_locations_len(self)
    }

    fn create_element(&self, name: &str) -> Result<EInfo, SessionError> {
        self.get_session()?.create_element(name, self)
    }

    fn create_location(&self, name: &str) -> Result<LInfo, SessionError> {
        self.get_session()?.create_location(name, self)
    }

    fn destroy(self) -> Result<LRow, SessionError> {
        self.get_session()?.destroy_location(self)
    }

    fn _move(&self, to: &LInfo) -> Result<(), SessionError> {
        self.get_session()?.move_location(self, to)
    }
}

pub trait TLocation {
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError>;

    fn get_name(&self) -> Result<String, SessionError>;
    fn set_name(&self, name: &str) -> Result<(), SessionError>;

    fn get_desc(&self) -> Result<String, SessionError>;
    fn set_desc(&self, desc: &str) -> Result<(), SessionError>;

    fn get_path(&self) -> Result<PathBuf, SessionError>;
    fn set_path(&self, path: PathBuf) -> Result<(), SessionError>;

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
