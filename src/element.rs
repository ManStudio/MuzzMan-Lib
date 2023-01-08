use std::{fmt::Debug, thread::JoinHandle};

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ElementNotify {
    Complited,
    ModuleChanged(#[serde(skip)] Option<MInfo>),
    StatusChanged(usize),
    Progress(f32),
    Error(String),
    Warning(String),
    Custom(String),
}

impl_get_ref!(ElementNotify);

#[derive(Serialize, Deserialize)]
pub struct Element {
    #[serde(skip)]
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
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError>;

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
    fn set_enabled(&self, enabled: bool, storage: Option<Storage>) -> Result<(), SessionError>;

    fn wait(&self) -> Result<(), SessionError>;

    fn destroy(self) -> Result<ERow, SessionError>;
}

#[derive(Serialize, Deserialize)]
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
    #[serde(skip)]
    pub thread: Option<JoinHandle<()>>,
    pub info: EInfo,
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
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError> {
        if let Some(session) = &self.read().unwrap().session {
            return Ok(session.c());
        }
        Err(SessionError::InvalidSession)
    }

    fn get_name(&self) -> Result<String, SessionError> {
        self.get_session()?.element_get_name(self)
    }

    fn set_name(&self, name: &str) -> Result<(), SessionError> {
        self.get_session()?.element_set_name(self, name)
    }

    fn get_desc(&self) -> Result<String, SessionError> {
        self.get_session()?.element_get_desc(self)
    }

    fn set_desc(&self, desc: &str) -> Result<(), SessionError> {
        self.get_session()?.element_set_desc(self, desc)
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

    fn get_module(&self) -> Result<Option<MInfo>, SessionError> {
        self.get_session()?.element_get_module(self)
    }

    fn set_module(&self, module: Option<MInfo>) -> Result<(), SessionError> {
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

    fn wait(&self) -> Result<(), SessionError> {
        self.get_session()?.element_wait(self)
    }

    fn destroy(self) -> Result<ERow, SessionError> {
        self.get_session()?.destroy_element(self)
    }
}
