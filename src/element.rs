use std::{
    fmt::Debug,
    hash::Hash,
    sync::{Arc, RwLock},
    thread::JoinHandle,
};

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum ElementNotify {
    Complited,
    ModuleChanged(Option<MRef>),
    StatusChanged(usize),
    Progress(f32),
}

impl_get_ref!(ElementNotify);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElementId {
    pub uid: usize,
    pub location_id: LocationId,
}

impl ElementId {
    pub fn into_ref(self, session: Box<dyn TSession>) -> RefElement {
        RefElement {
            session: Some(session),
            id: self,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefElement {
    #[serde(skip)]
    pub session: Option<Box<dyn TSession>>,
    pub id: ElementId,
}

unsafe impl Sync for RefElement {}
unsafe impl Send for RefElement {}

impl Debug for RefElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefElement").field("id", &self.id).finish()
    }
}

impl PartialEq for RefElement {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.id.location_id.eq(&other.id.location_id)
    }
}

pub trait TElement {
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError>;

    fn get_meta(&self) -> Result<String, SessionError>;
    fn set_meta(&self, meta: &str) -> Result<(), SessionError>;

    fn get_element_data(&self) -> Result<Data, SessionError>;
    fn set_element_data(&self, data: Data) -> Result<(), SessionError>;

    fn get_module_data(&self) -> Result<Data, SessionError>;
    fn set_module_data(&self, data: Data) -> Result<(), SessionError>;

    fn get_module(&self) -> Result<Option<MRef>, SessionError>;
    fn set_module(&self, module: Option<MRef>) -> Result<(), SessionError>;

    fn resolv_module(&self) -> Result<bool, SessionError>;
    fn init(&self) -> Result<bool, SessionError>;

    fn get_statuses(&self) -> Result<Vec<String>, SessionError>;
    fn set_statuses(&self, statuses: Vec<String>) -> Result<(), SessionError>;

    fn get_status(&self) -> Result<usize, SessionError>;
    fn get_status_msg(&self) -> Result<String, SessionError>;
    fn set_status(&self, status: usize) -> Result<(), SessionError>;

    fn get_data(&self) -> Result<FileOrData, SessionError>;
    fn set_data(&self, data: FileOrData) -> Result<(), SessionError>;

    fn get_progress(&self) -> Result<f32, SessionError>;
    fn set_progress(&self, progress: f32) -> Result<(), SessionError>;

    fn get_should_save(&self) -> Result<bool, SessionError>;
    fn set_should_save(&self, should_save: bool) -> Result<(), SessionError>;

    fn is_enabled(&self) -> Result<bool, SessionError>;
    fn set_enabled(&self, enabled: bool, storage: Option<Storage>) -> Result<(), SessionError>;

    fn get_element_info(&self) -> Result<ElementInfo, SessionError>;

    fn wait(&self) -> Result<(), SessionError>;

    fn destroy(self) -> Result<ERow, SessionError>;
}

pub struct Element {
    pub name: String,
    pub desc: String,
    pub meta: String,
    pub element_data: Data,
    pub module_data: Data,
    pub module: Option<MRef>,
    pub statuses: Vec<String>,
    pub status: usize,
    pub data: FileOrData,
    pub progress: f32,
    pub should_save: bool,
    pub enabled: bool,
    pub thread: Option<JoinHandle<()>>,
    pub events: Arc<RwLock<Events>>,
    pub info: ERef,
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
            .field("module_data", &self.module_data)
            .field("statuses", &self.statuses)
            .field("data", &self.data)
            .field("progress", &self.progress)
            .field("should_save", &self.should_save)
            .field("enable", &self.enabled)
            .field("thread", &self.thread)
            .finish()
    }
}

impl TElement for ERef {
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError> {
        if let Some(session) = &self.read().unwrap().session {
            return Ok(session.c());
        }
        Err(SessionError::InvalidSession)
    }

    fn get_meta(&self) -> Result<String, SessionError> {
        self.get_session()?.element_get_meta(self)
    }

    fn set_meta(&self, meta: &str) -> Result<(), SessionError> {
        self.get_session()?.element_set_meta(self, meta)
    }

    fn get_element_data(&self) -> Result<Data, SessionError> {
        self.get_session()?.element_get_element_data(self)
    }

    fn set_element_data(&self, data: Data) -> Result<(), SessionError> {
        self.get_session()?.element_set_element_data(self, data)
    }

    fn get_module_data(&self) -> Result<Data, SessionError> {
        self.get_session()?.element_get_module_data(self)
    }

    fn set_module_data(&self, data: Data) -> Result<(), SessionError> {
        self.get_session()?.element_set_module_data(self, data)
    }

    fn get_module(&self) -> Result<Option<MRef>, SessionError> {
        self.get_session()?.element_get_module(self)
    }

    fn set_module(&self, module: Option<MRef>) -> Result<(), SessionError> {
        self.get_session()?.element_set_module(self, module)
    }

    fn resolv_module(&self) -> Result<bool, SessionError> {
        self.get_session()?.element_resolv_module(self)
    }

    fn init(&self) -> Result<bool, SessionError> {
        if let Some(module) = &self.get_module()? {
            self.get_session()?.module_init_element(module, self)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_statuses(&self) -> Result<Vec<String>, SessionError> {
        self.get_session()?.element_get_statuses(self)
    }

    fn set_statuses(&self, statuses: Vec<String>) -> Result<(), SessionError> {
        self.get_session()?.element_set_statuses(self, statuses)
    }

    fn get_status(&self) -> Result<usize, SessionError> {
        self.get_session()?.element_get_status(self)
    }

    fn get_status_msg(&self) -> Result<String, SessionError> {
        if let Some(status) = self.get_statuses()?.get(self.get_status()?) {
            Ok(status.clone())
        } else {
            Ok(String::from("None"))
        }
    }

    fn set_status(&self, status: usize) -> Result<(), SessionError> {
        self.get_session()?.element_set_status(self, status)
    }

    fn get_data(&self) -> Result<FileOrData, SessionError> {
        self.get_session()?.element_get_data(self)
    }

    fn set_data(&self, data: FileOrData) -> Result<(), SessionError> {
        self.get_session()?.element_set_data(self, data)
    }

    fn get_progress(&self) -> Result<f32, SessionError> {
        self.get_session()?.element_get_progress(self)
    }

    fn set_progress(&self, progress: f32) -> Result<(), SessionError> {
        self.get_session()?.element_set_progress(self, progress)
    }

    fn get_should_save(&self) -> Result<bool, SessionError> {
        self.get_session()?.element_get_should_save(self)
    }

    fn set_should_save(&self, should_save: bool) -> Result<(), SessionError> {
        self.get_session()?
            .element_set_should_save(self, should_save)
    }

    fn is_enabled(&self) -> Result<bool, SessionError> {
        self.get_session()?.element_get_enabled(self)
    }

    fn set_enabled(&self, enabled: bool, storage: Option<Storage>) -> Result<(), SessionError> {
        self.get_session()?
            .element_set_enabled(self, enabled, storage)
    }

    fn get_element_info(&self) -> Result<ElementInfo, SessionError> {
        self.get_session()?.element_get_element_info(self)
    }

    fn wait(&self) -> Result<(), SessionError> {
        self.get_session()?.element_wait(self)
    }

    fn destroy(self) -> Result<ERow, SessionError> {
        self.get_session()?.destroy_element(self)
    }
}

impl Common for ERef {
    fn get_name(&self) -> Result<String, SessionError> {
        self.get_session()?.element_get_name(self)
    }

    fn set_name(&self, name: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?.element_set_name(self, &name.into())
    }

    fn get_desc(&self) -> Result<String, SessionError> {
        self.get_session()?.element_get_desc(self)
    }

    fn set_desc(&self, desc: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?.element_set_desc(self, &desc.into())
    }

    fn notify(&self, event: Event) -> Result<(), SessionError> {
        self.get_session()?.element_notify(self, event)
    }

    fn emit(&self, event: Event) -> Result<(), SessionError> {
        self.get_session()?.element_emit(self, event)
    }

    fn subscribe(&self, _ref: Ref) -> Result<(), SessionError> {
        self.get_session()?.element_subscribe(self, _ref)
    }

    fn unsubscribe(&self, _ref: Ref) -> Result<(), SessionError> {
        self.get_session()?.element_unsubscribe(self, _ref)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementInfo {
    pub name: String,
    pub desc: String,
    pub meta: String,
    pub element_data: Data,
    pub module_data: Data,
    pub module: Option<ModuleInfo>,
    pub statuses: Vec<String>,
    pub status: usize,
    pub data: FileOrData,
    pub progress: f32,
    pub should_save: bool,
    pub enabled: bool,
}

impl Hash for ElementInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.desc.hash(state);
        self.meta.hash(state);
        self.element_data.hash(state);
        self.module_data.hash(state);
        self.statuses.hash(state);
        self.status.hash(state);
        self.data.hash(state);
        self.enabled.hash(state);
        (self.progress as i32).hash(state)
    }
}

impl TGetLogger for ERef {
    fn get_logger(&self, dst: Option<Arc<std::sync::Mutex<std::fs::File>>>) -> Logger {
        Logger::for_element(dst, self.clone())
    }
}

impl TGetLogger for ERow {
    fn get_logger(&self, dst: Option<Arc<std::sync::Mutex<std::fs::File>>>) -> Logger {
        let info = self.read().unwrap().info.clone();
        Logger::for_element(dst, info)
    }
}
