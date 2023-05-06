#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
pub use windows::*;

pub fn get_modules() -> Vec<PathBuf> {
    let mut modules = Vec::new();
    for paths in get_muzzman_dir().join("modules").read_dir().unwrap() {
        let entry = paths.unwrap();
        let path = entry.path();
        if path
            .as_os_str()
            .to_string_lossy()
            .split('.')
            .last()
            .unwrap()
            == std::env::consts::DLL_EXTENSION
        {
            modules.push(path);
        }
    }
    modules
}

use crate::{
    events::Event,
    prelude::{Ref, SessionError, TModuleHelper},
    types::ID,
};

pub trait Common {
    fn get_name(&self) -> Result<String, SessionError>;
    fn set_name(&self, name: impl Into<String>) -> Result<(), SessionError>;

    fn get_desc(&self) -> Result<String, SessionError>;
    fn set_desc(&self, desc: impl Into<String>) -> Result<(), SessionError>;

    fn notify(&self, event: Event) -> Result<(), SessionError>;

    fn emit(&self, event: Event) -> Result<(), SessionError>;
    fn subscribe(&self, _ref: ID) -> Result<(), SessionError>;
    fn unsubscribe(&self, _ref: ID) -> Result<(), SessionError>;
}

impl Common for ID {
    fn get_name(&self) -> Result<String, crate::prelude::SessionError> {
        match self {
            ID::Element(e) => e.get_name(),
            ID::Location(l) => l.get_name(),
            ID::Module(m) => m.get_name(),
        }
    }

    fn set_name(&self, name: impl Into<String>) -> Result<(), crate::prelude::SessionError> {
        match self {
            ID::Element(e) => e.set_name(name),
            ID::Location(l) => l.set_name(name),
            ID::Module(m) => m.set_name(name),
        }
    }

    fn get_desc(&self) -> Result<String, crate::prelude::SessionError> {
        match self {
            ID::Element(e) => e.get_desc(),
            ID::Location(l) => l.get_desc(),
            ID::Module(m) => m.get_desc(),
        }
    }

    fn set_desc(&self, desc: impl Into<String>) -> Result<(), crate::prelude::SessionError> {
        match self {
            ID::Element(e) => e.set_desc(desc),
            ID::Location(l) => l.set_desc(desc),
            ID::Module(m) => m.set_desc(desc),
        }
    }

    fn notify(&self, event: crate::events::Event) -> Result<(), crate::prelude::SessionError> {
        match self {
            ID::Element(e) => e.notify(event),
            ID::Location(l) => l.notify(event),
            ID::Module(_) => Ok(()),
        }
    }

    fn emit(&self, event: Event) -> Result<(), SessionError> {
        match self {
            ID::Element(e) => e.emit(event),
            ID::Location(l) => l.emit(event),
            ID::Module(_) => Ok(()),
        }
    }

    fn subscribe(&self, _ref: ID) -> Result<(), SessionError> {
        match self {
            ID::Element(e) => e.subscribe(_ref),
            ID::Location(l) => l.subscribe(_ref),
            ID::Module(_) => Ok(()),
        }
    }

    fn unsubscribe(&self, _ref: ID) -> Result<(), SessionError> {
        match self {
            ID::Element(e) => e.unsubscribe(_ref),
            ID::Location(l) => l.unsubscribe(_ref),
            ID::Module(_) => Ok(()),
        }
    }
}
