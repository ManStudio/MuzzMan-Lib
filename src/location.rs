use bytes_kman::TBytes;
use std::fmt::Debug;
use std::hash::Hash;
use std::net::{IpAddr, TcpStream};
use std::ops::Range;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::JoinHandle;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, bytes_kman::Bytes)]
pub enum LocationNotify {
    ElementNotify(usize, ElementNotify),
    ModuleChanged(Option<ModulePath>),
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

impl TBytes for ServerLocation {
    fn size(&self) -> usize {
        self.ip.to_string().size()
            + self.port.size()
            + self.indentification.size()
            + self.server_cert.size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::with_capacity(self.size());

        buff.append(&mut self.ip.to_string().to_bytes());
        buff.append(&mut self.port.to_bytes());
        buff.append(&mut self.indentification.to_bytes());
        buff.append(&mut self.server_cert.to_bytes());

        buff
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let ip = String::from_bytes(buffer)?;
        let port = u16::from_bytes(buffer)?;
        let indentification = String::from_bytes(buffer)?;
        let server_cert = <Option<String>>::from_bytes(buffer)?;

        let ip = IpAddr::from_str(&ip).unwrap();

        Some(Self {
            ip,
            port,
            indentification,
            server_cert,
            conn: None,
        })
    }
}

impl Hash for ServerLocation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ip.hash(state);
        self.port.hash(state);
        self.indentification.hash(state);
        self.server_cert.hash(state);
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Hash, bytes_kman::Bytes)]
pub enum WhereIsLocation {
    Server(ServerLocation),
    #[default]
    Local,
}

#[derive(
    Default,
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    bytes_kman::Bytes,
)]
pub struct LocationPath(pub Vec<u64>);

#[derive(Serialize, Deserialize)]
pub struct LocationId {
    pub uid: UID,
    #[serde(skip)]
    pub session: Option<Box<dyn TSession>>,
}

impl Clone for LocationId {
    fn clone(&self) -> Self {
        Self {
            uid: self.uid,
            session: match &self.session {
                Some(session) => Some(session.c()),
                None => None,
            },
        }
    }
}

impl bytes_kman::TBytes for LocationId {
    fn size(&self) -> usize {
        self.uid.size()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.uid.to_bytes()
    }

    fn from_bytes(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self {
            uid: UID::from_bytes(buffer)?,
            session: None,
        })
    }
}

impl PartialEq for LocationId {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Debug for LocationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocationId")
            .field("uid", &self.uid)
            .finish()
    }
}

impl std::ops::Deref for LocationPath {
    type Target = Vec<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LocationPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::iter::IntoIterator for LocationPath {
    type Item = u64;

    type IntoIter = std::vec::IntoIter<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl LocationPath {
    pub fn into_ref(self, session: Box<dyn TSession>) -> RefLocationPath {
        RefLocationPath {
            session: Some(session),
            id: self,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefLocationPath {
    #[serde(skip)]
    pub session: Option<Box<dyn TSession>>,
    pub id: LocationPath,
}

impl Debug for RefLocationPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocatioInfo")
            .field("uid", &self.id)
            .finish()
    }
}

impl Clone for RefLocationPath {
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

impl PartialEq for RefLocationPath {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

pub struct Location {
    pub name: String,
    pub desc: String,
    pub where_is: WhereIsLocation,
    pub should_save: bool,
    pub module_settings: Values,
    pub settings: Values,
    pub elements: Vec<Option<ERow>>,
    pub locations: Vec<Option<LRow>>,
    pub ref_id: LRef,
    pub path: PathBuf,
    pub thread: Option<JoinHandle<()>>,
    pub module: Option<MRef>,
    pub events: Arc<RwLock<Events>>,

    pub progress: f32,
    pub statuses: Vec<String>,
    pub status: usize,
    pub is_error: bool,

    pub enabled: bool,
}

impl TLocation for LocationId {
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError> {
        if let Some(session) = &self.session {
            return Ok(session.c());
        }

        Err(SessionError::InvalidSession)
    }

    fn get_path(&self) -> Result<PathBuf, SessionError> {
        self.get_session()?.location_get_path(self.uid)
    }

    fn set_path(&self, path: PathBuf) -> Result<(), SessionError> {
        self.get_session()?.location_set_path(self.uid, path)
    }

    fn get_where_is(&self) -> Result<WhereIsLocation, SessionError> {
        self.get_session()?.location_get_where_is(self.uid)
    }

    fn set_where_is(&self, where_is: WhereIsLocation) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_where_is(self.uid, where_is)
    }

    fn get_should_save(&self) -> Result<bool, SessionError> {
        self.get_session()?.location_get_should_save(self.uid)
    }

    fn set_should_save(&self, should_save: bool) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_should_save(self.uid, should_save)
    }

    fn get_module(&self) -> Result<Option<ModuleId>, SessionError> {
        self.get_session()?.location_get_module(self.uid)
    }

    fn set_module(&self, module: Option<ModuleId>) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_module(self.uid, module.map(|m| m.uid))
    }

    fn get_settings(&self) -> Result<Values, SessionError> {
        self.get_session()?.location_get_settings(self.uid)
    }

    fn set_settings(&self, data: Values) -> Result<(), SessionError> {
        self.get_session()?.location_set_settings(self.uid, data)
    }

    fn get_module_settings(&self) -> Result<Values, SessionError> {
        self.get_session()?.location_get_module_settings(self.uid)
    }

    fn set_module_settings(&self, data: Values) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_module_settings(self.uid, data)
    }

    fn get_elements(&self, range: Range<usize>) -> Result<Vec<ERef>, SessionError> {
        self.get_session()?.location_get_elements(self.uid, range)
    }

    fn get_elements_len(&self) -> Result<usize, SessionError> {
        self.get_session()?.location_get_elements_len(self.uid)
    }

    fn get_locations(&self, range: Range<usize>) -> Result<Vec<LRef>, SessionError> {
        self.get_session()?.get_locations(self.uid, range)
    }

    fn get_locations_len(&self) -> Result<usize, SessionError> {
        self.get_session()?.get_locations_len(self.uid)
    }

    fn get_location_info(&self) -> Result<LocationInfo, SessionError> {
        self.get_session()?.location_get_location_info(self.uid)
    }

    fn create_element(&self, name: &str) -> Result<ERef, SessionError> {
        self.get_session()?.create_element(name, self.uid)
    }

    fn create_location(&self, name: &str) -> Result<LRef, SessionError> {
        self.get_session()?.create_location(name, self.uid)
    }

    fn get_statuses(&self) -> Result<Vec<String>, SessionError> {
        self.get_session()?.location_get_statuses(self.uid)
    }

    fn set_statuses(&self, statuses: Vec<String>) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_statuses(self.uid, statuses)
    }

    fn get_status(&self) -> Result<usize, SessionError> {
        self.get_session()?.location_get_status(self.uid)
    }

    fn get_status_msg(&self) -> Result<String, SessionError> {
        todo!()
    }

    fn set_status(&self, status: usize) -> Result<(), SessionError> {
        self.get_session()?.location_set_status(self.uid, status)
    }

    fn get_progress(&self) -> Result<f32, SessionError> {
        self.get_session()?.location_get_progress(self.uid)
    }

    fn set_progress(&self, progress: f32) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_progress(self.uid, progress)
    }

    fn is_enabled(&self) -> Result<bool, SessionError> {
        self.get_session()?.location_is_enabled(self.uid)
    }

    fn set_enabled(&self, enabled: bool, storage: Option<Storage>) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_enabled(self.uid, enabled, storage)
    }

    fn destroy(self) -> Result<LRow, SessionError> {
        self.get_session()?.destroy_location(self.uid)
    }

    fn _move(&self, to: LocationId) -> Result<(), SessionError> {
        self.get_session()?.move_location(self.uid, to.uid)
    }

    fn is_error(&self) -> Result<bool, SessionError> {
        self.get_session()?.location_is_error(self.uid)
    }
}

impl Common for LocationId {
    fn get_name(&self) -> Result<String, SessionError> {
        self.get_session()?.location_get_name(self.uid)
    }

    fn set_name(&self, name: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_name(self.uid, &name.into())
    }

    fn get_desc(&self) -> Result<String, SessionError> {
        self.get_session()?.location_get_desc(self.uid)
    }

    fn set_desc(&self, desc: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?
            .location_set_desc(self.uid, &desc.into())
    }

    fn notify(&self, event: Event) -> Result<(), SessionError> {
        self.get_session()?.location_notify(self.uid, event)
    }

    fn emit(&self, event: Event) -> Result<(), SessionError> {
        self.get_session()?.location_emit(self.uid, event)
    }

    fn subscribe(&self, _ref: ID) -> Result<(), SessionError> {
        self.get_session()?.location_subscribe(self.uid, _ref)
    }

    fn unsubscribe(&self, _ref: ID) -> Result<(), SessionError> {
        self.get_session()?.location_unsubscribe(self.uid, _ref)
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

    fn get_module(&self) -> Result<Option<ModuleId>, SessionError>;
    fn set_module(&self, module: Option<ModuleId>) -> Result<(), SessionError>;

    fn get_settings(&self) -> Result<Values, SessionError>;
    fn set_settings(&self, data: Values) -> Result<(), SessionError>;

    fn get_module_settings(&self) -> Result<Values, SessionError>;
    fn set_module_settings(&self, data: Values) -> Result<(), SessionError>;

    fn get_elements(&self, range: Range<usize>) -> Result<Vec<ERef>, SessionError>;
    fn get_elements_len(&self) -> Result<usize, SessionError>;
    fn get_locations(&self, range: Range<usize>) -> Result<Vec<LRef>, SessionError>;
    fn get_locations_len(&self) -> Result<usize, SessionError>;

    fn get_location_info(&self) -> Result<LocationInfo, SessionError>;

    fn create_element(&self, name: &str) -> Result<ERef, SessionError>;
    fn create_location(&self, name: &str) -> Result<LRef, SessionError>;

    fn get_statuses(&self) -> Result<Vec<String>, SessionError>;
    fn set_statuses(&self, statuses: Vec<String>) -> Result<(), SessionError>;

    fn get_status(&self) -> Result<usize, SessionError>;
    fn get_status_msg(&self) -> Result<String, SessionError>;
    fn set_status(&self, status: usize) -> Result<(), SessionError>;

    fn get_progress(&self) -> Result<f32, SessionError>;
    fn set_progress(&self, progress: f32) -> Result<(), SessionError>;

    fn is_enabled(&self) -> Result<bool, SessionError>;
    fn set_enabled(&self, enabled: bool, storage: Option<Storage>) -> Result<(), SessionError>;

    fn destroy(self) -> Result<LRow, SessionError>;
    fn _move(&self, to: LocationId) -> Result<(), SessionError>;

    fn is_error(&self) -> Result<bool, SessionError>;
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Hash, bytes_kman::Bytes)]
pub struct LocationInfo {
    pub name: String,
    pub desc: String,
    pub id: LocationPath,
    pub where_is: WhereIsLocation,
    pub shoud_save: bool,
    pub elements: Vec<ElementInfo>,
    pub locations: Vec<LocationInfo>,
    pub path: PathBuf,
    pub module: Option<ModuleInfo>,
}
