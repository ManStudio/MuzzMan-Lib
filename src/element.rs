use std::{fmt::Debug, sync::RwLock, thread::JoinHandle};

use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum ElementNotify {
    Complited,
    ModuleChanged(Option<MInfo>),
    StatusChanged(usize),
    Progress(f32),
    Error(String),
    Warning(String),
    Custom(String),
}

impl_get_ref!(ElementNotify);

pub struct Element {
    pub session: Option<Box<dyn TSession>>,
    pub location: LInfo,
    pub uid: usize,
}

unsafe impl Sync for Element {}
unsafe impl Send for Element {}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.uid.eq(&other.uid)
            && self
                .location
                .read()
                .unwrap()
                .eq(&other.location.read().unwrap())
    }
}

pub trait TElement {
    fn get_session(&self) -> Option<Box<dyn TSession>>;

    fn get_name(&self) -> Result<String, SessionError>;
    fn set_name(&self, name: &str) -> Result<(), SessionError>;

    fn get_desc(&self) -> Result<String, SessionError>;
    fn set_desc(&self, desc: &str) -> Result<(), SessionError>;

    fn get_meta(&self) -> Result<String, SessionError>;
    fn set_meta(&self, meta: &str) -> Result<(), SessionError>;

    fn get_element_data(&self) -> Result<Data, SessionError>;
    fn set_element_data(&self, data: Data) -> Result<(), SessionError>;

    fn get_module_data(&self) -> Result<Data, SessionError>;
    fn set_module_data(&self, data: Data) -> Result<(), SessionError>;

    fn get_module(&self) -> Result<Option<MInfo>, SessionError>;
    fn set_module(&self, module: Option<MInfo>) -> Result<(), SessionError>;

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
    fn set_enabled(&self, enabled: bool) -> Result<(), SessionError>;

    fn get_notify(&self) -> Result<Arc<RwLock<AdvancedSignal<ElementNotify, ()>>>, SessionError>;

    fn wait(&self) -> Result<(), SessionError>;

    fn destroy(self) -> Result<ERow, SessionError>;
}

pub struct RowElement {
    pub name: String,
    pub desc: String,
    pub meta: String,
    pub element_data: Data,
    pub module_data: Data,
    pub module: Option<MInfo>,
    pub statuses: Vec<String>,
    pub status: usize,
    pub data: FileOrData,
    pub progress: f32,
    pub should_save: bool,
    pub enabled: bool,
    pub thread: JoinHandle<()>,
    pub info: EInfo,
    pub signal_notify: Arc<RwLock<AdvancedSignal<ElementNotify, ()>>>,
}

unsafe impl Sync for RowElement {}
unsafe impl Send for RowElement {}

pub trait TRowElement {
    fn set_status(&self, status: usize);
}

impl TRowElement for ERow {
    fn set_status(&self, status: usize) {
        {
            let mut element = self.write().unwrap();
            element.status = status;
        }
        let signal = self.read().unwrap().signal_notify.clone();
        let signal = signal.read().unwrap();
        signal.call(ElementNotify::StatusChanged(status));
    }
}

impl Debug for RowElement {
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

impl TElement for EInfo {
    fn get_session(&self) -> Option<Box<dyn TSession>> {
        let mut s = None;
        if let Some(session) = &self.read().unwrap().session {
            s = Some(session.c())
        }
        s
    }

    fn get_name(&self) -> Result<String, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_name(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_name(&self, name: &str) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_name(self, name);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_desc(&self) -> Result<String, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_desc(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_desc(&self, desc: &str) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_desc(self, desc);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_meta(&self) -> Result<String, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_meta(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_meta(&self, meta: &str) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_meta(self, meta);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_element_data(&self) -> Result<Data, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_element_data(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_element_data(&self, data: Data) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_element_data(self, data);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_module_data(&self) -> Result<Data, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_module_data(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_module_data(&self, data: Data) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_module_data(self, data);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_module(&self) -> Result<Option<MInfo>, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_module(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_module(&self, module: Option<MInfo>) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_module(self, module);
        }
        Err(SessionError::InvalidSession)
    }

    fn resolv_module(&self) -> Result<bool, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_resolv_module(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn init(&self) -> Result<bool, SessionError> {
        if let Some(session) = self.get_session() {
            if let Some(module) = self.get_module()? {
                session.module_init_element(&module, self)?;
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
        Err(SessionError::InvalidSession)
    }

    fn get_statuses(&self) -> Result<Vec<String>, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_statuses(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_statuses(&self, statuses: Vec<String>) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_statuses(self, statuses);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_status(&self) -> Result<usize, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_status(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_status_msg(&self) -> Result<String, SessionError> {
        let index = self.get_status()?;
        if let Some(status) = self.get_statuses()?.get(index) {
            Ok(status.clone())
        } else {
            Ok(String::from("None"))
        }
    }

    fn set_status(&self, status: usize) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_status(self, status);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_data(&self) -> Result<FileOrData, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_data(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_data(&self, data: FileOrData) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_data(self, data);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_progress(&self) -> Result<f32, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_progress(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_progress(&self, progress: f32) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_progress(self, progress);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_should_save(&self) -> Result<bool, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_should_save(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_should_save(&self, should_save: bool) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_should_save(self, should_save);
        }
        Err(SessionError::InvalidSession)
    }

    fn is_enabled(&self) -> Result<bool, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_enabled(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn set_enabled(&self, enabled: bool) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_set_enabled(self, enabled);
        }
        Err(SessionError::InvalidSession)
    }

    fn get_notify(&self) -> Result<Arc<RwLock<AdvancedSignal<ElementNotify, ()>>>, SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_get_notify(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn wait(&self) -> Result<(), SessionError> {
        if let Some(session) = self.get_session() {
            return session.element_wait(self);
        }
        Err(SessionError::InvalidSession)
    }

    fn destroy(self) -> Result<ERow, SessionError> {
        if let Some(session) = self.get_session() {
            return session.destroy_element(self);
        }
        Err(SessionError::InvalidSession)
    }
}
