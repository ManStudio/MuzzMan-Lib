use std::{
    fmt::Debug,
    hash::Hash,
    sync::{Arc, RwLock},
    thread::JoinHandle,
};

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use bytes_kman::TBytes;

#[derive(Debug, Clone, Serialize, Deserialize, bytes_kman::Bytes)]
pub enum ElementNotify {
    Complited,
    ModuleChanged(Option<ModulePath>),
    StatusChanged(usize),
    Progress(f32),
}

impl_get_ref!(ElementNotify);

#[derive(
    Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, bytes_kman::Bytes,
)]
pub struct ElementPath {
    pub index: u64,
    pub location_id: LocationPath,
}

#[derive(Serialize, Deserialize)]
pub struct ElementId {
    pub uid: UID,
    #[serde(skip)]
    pub session: Option<Box<dyn TSession>>,
}

impl Clone for ElementId {
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

impl bytes_kman::TBytes for ElementId {
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

impl PartialEq for ElementId {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Debug for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElementId").field("uid", &self.uid).finish()
    }
}

impl ElementPath {
    pub fn into_ref(self, session: Box<dyn TSession>) -> RefElementPath {
        RefElementPath {
            session: Some(session),
            id: self,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefElementPath {
    #[serde(skip)]
    pub session: Option<Box<dyn TSession>>,
    pub id: ElementPath,
}

unsafe impl Sync for RefElementPath {}
unsafe impl Send for RefElementPath {}

impl Debug for RefElementPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefElement").field("id", &self.id).finish()
    }
}

impl PartialEq for RefElementPath {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.id.location_id.eq(&other.id.location_id)
    }
}

pub trait TElement {
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError>;

    fn get_meta(&self) -> Result<String, SessionError>;
    fn set_meta(&self, meta: &str) -> Result<(), SessionError>;

    fn get_settings(&self) -> Result<Values, SessionError>;
    fn set_settings(&self, data: Values) -> Result<(), SessionError>;

    fn get_module_settings(&self) -> Result<Values, SessionError>;
    fn set_module_settings(&self, data: Values) -> Result<(), SessionError>;

    fn get_module(&self) -> Result<Option<ModuleId>, SessionError>;
    fn set_module(&self, module: Option<ModuleId>) -> Result<(), SessionError>;

    fn resolv_module(&self) -> Result<bool, SessionError>;
    fn init(&self) -> Result<bool, SessionError>;

    fn get_url(&self) -> Result<Option<String>, SessionError>;
    fn set_url(&self, url: Option<String>) -> Result<(), SessionError>;

    fn get_statuses(&self) -> Result<Vec<String>, SessionError>;
    fn set_statuses(&self, statuses: Vec<String>) -> Result<(), SessionError>;

    fn get_status(&self) -> Result<usize, SessionError>;
    fn get_status_msg(&self) -> Result<String, SessionError>;
    fn set_status(&self, status: usize) -> Result<(), SessionError>;

    fn get_data(&self) -> Result<Data, SessionError>;
    fn set_data(&self, data: Data) -> Result<(), SessionError>;

    fn get_progress(&self) -> Result<f32, SessionError>;
    fn set_progress(&self, progress: f32) -> Result<(), SessionError>;

    fn get_should_save(&self) -> Result<bool, SessionError>;
    fn set_should_save(&self, should_save: bool) -> Result<(), SessionError>;

    fn is_enabled(&self) -> Result<bool, SessionError>;
    fn set_enabled(&self, enabled: bool, storage: Option<Storage>) -> Result<(), SessionError>;

    fn get_element_info(&self) -> Result<ElementInfo, SessionError>;

    fn wait(&self) -> Result<(), SessionError>;
    fn is_error(&self) -> Result<bool, SessionError>;

    fn destroy(self) -> Result<ERow, SessionError>;
}

pub struct Element {
    pub name: String,
    pub desc: String,
    pub meta: String,
    pub url: Option<String>,
    pub element_data: Values,
    pub settings: Values,
    pub module: Option<MRef>,
    pub data: Data,
    pub should_save: bool,
    pub enabled: bool,
    pub thread: Option<JoinHandle<()>>,
    pub events: Arc<RwLock<Events>>,
    pub ref_id: ERef,

    pub progress: f32,
    pub statuses: Vec<String>,
    pub status: usize,

    pub is_error: bool,
}

unsafe impl Sync for Element {}
unsafe impl Send for Element {}

pub trait TRowElement {
    fn set_status(&self, status: usize);
}

impl TRowElement for ERow {
    fn set_status(&self, status: usize) {
        {
            let mut element = self.write().unwrap();
            element.status = status;
        }
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RowElement")
            .field("name", &self.name)
            .field("desc", &self.desc)
            .field("meta", &self.meta)
            .field("element_data", &self.element_data)
            .field("module_data", &self.settings)
            .field("statuses", &self.statuses)
            .field("data", &self.data)
            .field("progress", &self.progress)
            .field("should_save", &self.should_save)
            .field("enable", &self.enabled)
            .field("thread", &self.thread)
            .finish()
    }
}

impl TElement for ElementId {
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError> {
        if let Some(session) = &self.session {
            return Ok(session.c());
        }
        Err(SessionError::InvalidSession)
    }

    fn get_meta(&self) -> Result<String, SessionError> {
        self.get_session()?.element_get_meta(self.uid)
    }

    fn set_meta(&self, meta: &str) -> Result<(), SessionError> {
        self.get_session()?.element_set_meta(self.uid, meta)
    }

    fn get_settings(&self) -> Result<Values, SessionError> {
        self.get_session()?.element_get_element_data(self.uid)
    }

    fn set_settings(&self, data: Values) -> Result<(), SessionError> {
        self.get_session()?.element_set_element_data(self.uid, data)
    }

    fn get_module_settings(&self) -> Result<Values, SessionError> {
        self.get_session()?.element_get_module_data(self.uid)
    }

    fn set_module_settings(&self, data: Values) -> Result<(), SessionError> {
        self.get_session()?.element_set_module_data(self.uid, data)
    }

    fn get_module(&self) -> Result<Option<ModuleId>, SessionError> {
        self.get_session()?.element_get_module(self.uid)
    }

    fn set_module(&self, module: Option<ModuleId>) -> Result<(), SessionError> {
        self.get_session()?
            .element_set_module(self.uid, module.map(|m| m.uid))
    }

    fn resolv_module(&self) -> Result<bool, SessionError> {
        self.get_session()?.element_resolv_module(self.uid)
    }

    fn init(&self) -> Result<bool, SessionError> {
        if let Some(module) = &self.get_module()? {
            self.get_session()?
                .module_init_element(module.uid, self.uid)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_statuses(&self) -> Result<Vec<String>, SessionError> {
        self.get_session()?.element_get_statuses(self.uid)
    }

    fn set_statuses(&self, statuses: Vec<String>) -> Result<(), SessionError> {
        self.get_session()?.element_set_statuses(self.uid, statuses)
    }

    fn get_status(&self) -> Result<usize, SessionError> {
        self.get_session()?.element_get_status(self.uid)
    }

    fn get_status_msg(&self) -> Result<String, SessionError> {
        if let Some(status) = self.get_statuses()?.get(self.get_status()?) {
            Ok(status.clone())
        } else {
            Ok(String::from("None"))
        }
    }

    fn set_status(&self, status: usize) -> Result<(), SessionError> {
        self.get_session()?.element_set_status(self.uid, status)
    }

    fn get_data(&self) -> Result<Data, SessionError> {
        self.get_session()?.element_get_data(self.uid)
    }

    fn set_data(&self, data: Data) -> Result<(), SessionError> {
        self.get_session()?.element_set_data(self.uid, data)
    }

    fn get_progress(&self) -> Result<f32, SessionError> {
        self.get_session()?.element_get_progress(self.uid)
    }

    fn set_progress(&self, progress: f32) -> Result<(), SessionError> {
        self.get_session()?.element_set_progress(self.uid, progress)
    }

    fn get_should_save(&self) -> Result<bool, SessionError> {
        self.get_session()?.element_get_should_save(self.uid)
    }

    fn set_should_save(&self, should_save: bool) -> Result<(), SessionError> {
        self.get_session()?
            .element_set_should_save(self.uid, should_save)
    }

    fn is_enabled(&self) -> Result<bool, SessionError> {
        self.get_session()?.element_get_enabled(self.uid)
    }

    fn set_enabled(&self, enabled: bool, storage: Option<Storage>) -> Result<(), SessionError> {
        self.get_session()?
            .element_set_enabled(self.uid, enabled, storage)
    }

    fn get_element_info(&self) -> Result<ElementInfo, SessionError> {
        self.get_session()?.element_get_element_info(self.uid)
    }

    fn wait(&self) -> Result<(), SessionError> {
        self.get_session()?.element_wait(self.uid)
    }

    fn destroy(self) -> Result<ERow, SessionError> {
        self.get_session()?.destroy_element(self.uid)
    }

    fn get_url(&self) -> Result<Option<String>, SessionError> {
        self.get_session()?.element_get_url(self.uid)
    }

    fn set_url(&self, url: Option<String>) -> Result<(), SessionError> {
        self.get_session()?.element_set_url(self.uid, url)
    }

    fn is_error(&self) -> Result<bool, SessionError> {
        self.get_session()?.element_is_error(self.uid)
    }
}

impl Common for ElementId {
    fn get_name(&self) -> Result<String, SessionError> {
        self.get_session()?.element_get_name(self.uid)
    }

    fn set_name(&self, name: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?.element_set_name(self.uid, &name.into())
    }

    fn get_desc(&self) -> Result<String, SessionError> {
        self.get_session()?.element_get_desc(self.uid)
    }

    fn set_desc(&self, desc: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?.element_set_desc(self.uid, &desc.into())
    }

    fn notify(&self, event: Event) -> Result<(), SessionError> {
        self.get_session()?.element_notify(self.uid, event)
    }

    fn emit(&self, event: Event) -> Result<(), SessionError> {
        self.get_session()?.element_emit(self.uid, event)
    }

    fn subscribe(&self, _ref: ID) -> Result<(), SessionError> {
        self.get_session()?.element_subscribe(self.uid, _ref)
    }

    fn unsubscribe(&self, _ref: ID) -> Result<(), SessionError> {
        self.get_session()?.element_unsubscribe(self.uid, _ref)
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, bytes_kman::Bytes)]
pub struct ElementInfo {
    pub name: String,
    pub desc: String,
    pub meta: String,
    pub url: Option<String>,
    pub element_data: Values,
    pub module_data: Values,
    pub module: Option<ModuleInfo>,
    pub data: Data,
    pub should_save: bool,
    pub enabled: bool,
    pub id: ElementPath,
}
