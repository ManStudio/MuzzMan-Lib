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
    prelude::{Ref, SessionError},
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

impl Common for Ref {
    fn get_name(&self) -> Result<String, crate::prelude::SessionError> {
        match self {
            Ref::Element(e) => e.get_name(),
            Ref::Location(l) => l.get_name(),
        }
    }

    fn set_name(&self, name: impl Into<String>) -> Result<(), crate::prelude::SessionError> {
        match self {
            Ref::Element(e) => e.set_name(name),
            Ref::Location(l) => l.set_name(name),
        }
    }

    fn get_desc(&self) -> Result<String, crate::prelude::SessionError> {
        match self {
            Ref::Element(e) => e.get_desc(),
            Ref::Location(l) => l.get_desc(),
        }
    }

    fn set_desc(&self, desc: impl Into<String>) -> Result<(), crate::prelude::SessionError> {
        match self {
            Ref::Element(e) => e.set_desc(desc),
            Ref::Location(l) => l.set_desc(desc),
        }
    }

    fn notify(&self, event: crate::events::Event) -> Result<(), crate::prelude::SessionError> {
        match self {
            Ref::Element(e) => e.notify(event),
            Ref::Location(l) => l.notify(event),
        }
    }

    fn emit(&self, event: Event) -> Result<(), SessionError> {
        match self {
            Ref::Element(e) => e.emit(event),
            Ref::Location(l) => l.emit(event),
        }
    }

    fn subscribe(&self, _ref: ID) -> Result<(), SessionError> {
        match self {
            Ref::Element(e) => e.subscribe(_ref),
            Ref::Location(l) => l.subscribe(_ref),
        }
    }

    fn unsubscribe(&self, _ref: ID) -> Result<(), SessionError> {
        match self {
            Ref::Element(e) => e.unsubscribe(_ref),
            Ref::Location(l) => l.unsubscribe(_ref),
        }
    }
}
