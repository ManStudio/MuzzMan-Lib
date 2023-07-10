use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use bytes_kman::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{prelude::*, storage::Storage};

pub trait TModule: std::panic::UnwindSafe {
    fn name(&self) -> &str;
    fn desc(&self) -> &str;
    fn id(&self) -> u64;
    fn version(&self) -> u64;
    fn supported_versions(&self) -> Vec<u64>;

    fn poll_element(
        &self,
        ctx: &mut std::task::Context<'_>,
        element: Arc<RwLock<Element>>,
        storage: &mut Storage,
    ) -> SessionResult<()>;

    fn poll_location(
        &self,
        ctx: &mut std::task::Context<'_>,
        location: Arc<RwLock<Location>>,
        storage: &mut Storage,
    ) -> SessionResult<()>;

    fn element_on_event(
        &self,
        element: Arc<RwLock<Element>>,
        event: Event,
        storage: &mut Storage,
    ) -> SessionResult<()>;
    fn location_on_event(
        &self,
        location: Arc<RwLock<Location>>,
        event: Event,
        storage: &mut Storage,
    ) -> SessionResult<()>;

    fn default_element_settings(&self) -> Settings;
    fn default_location_settings(&self) -> Settings;

    /// Should be like "http:, https:"
    fn supports_protocols(&self) -> Vec<String>;
    /// Should be like "html, exe"
    fn supports_extensions(&self) -> Vec<String>;
}

pub enum ModuleSource {
    Wasm(Vec<u8>),
    Dynamic(PathBuf),
    DynamicLoaded(PathBuf, Box<dyn TModule>),
    Box(Box<dyn TModule>),
}

impl std::fmt::Debug for ModuleSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleSource::Wasm(_) => f.write_str("ModuleSource::Wasm"),
            ModuleSource::Dynamic(_) => f.write_str("ModuleSource::Dynamic"),
            ModuleSource::DynamicLoaded(_, _) => f.write_str("ModuleSource::DynamicLoaded"),
            ModuleSource::Box(_) => f.write_str("ModuleSource::Box"),
        }
    }
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub desc: String,
    pub proxy: u32,
    pub source: ModuleSource,
    pub element_settings: Settings,
    pub location_settings: Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Bytes)]
pub enum RawLibraryError {
    NotFound,
    DontHaveSymbolName,
    DontHaveSymbolDesc,
    DontHaveSymbolId,
    DontHaveSymbolVersion,
    DontHaveSymbolSupportedVersions,
    DontHaveSymbolPollElement,
    DontHaveSymbolPollLocation,
    DontHaveSymbolElementOnEvent,
    DontHaveSymbolLocationOnEvent,
    DontHaveSymbolDefaultElementSettings,
    DontHaveSymbolDefaultLocationSettings,
    DontHaveSymbolSupportsProtocols,
    DontHaveSymbolSupportsExtensions,
}

impl From<RawLibraryError> for SessionError {
    fn from(value: RawLibraryError) -> Self {
        Self::RawModule(value)
    }
}
