use std::{
    fmt::Debug,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::prelude::*;
use bytes_kman::TBytes;
use libloading::{Library, Symbol};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum ControlFlow {
    Run,
    Pause,
    Break,
}

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    bytes_kman::Bytes,
)]
pub struct ModuleId(pub u64);

impl From<MRef> for ModuleId {
    fn from(value: MRef) -> Self {
        value.read().unwrap().uid
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefModule {
    #[serde(skip)]
    pub session: Option<Box<dyn TSession>>,
    pub uid: ModuleId,
}

impl Debug for RefModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleInfo").finish()
    }
}

impl PartialEq for RefModule {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub desc: String,
    pub module: Box<dyn TModule>,
    /// this is the path of where the module get loaded
    pub path: Option<PathBuf>,
    pub proxy: usize,
    /// default module settings
    pub settings: Values,
    /// default data
    pub element_settings: Values,
    pub location_settings: Values,
    pub ref_id: MRef,
}

impl Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Module {}", self.name))
            .field("name", &self.name)
            .field("desc", &self.desc)
            .field("proxy", &self.proxy)
            .field("settings", &self.settings)
            .field("element_data", &self.element_settings)
            .finish()
    }
}

pub trait TModule: std::panic::UnwindSafe {
    fn init(&self, module_ref: MRef) -> Result<(), SessionError>;

    fn get_name(&self) -> String;
    fn get_desc(&self) -> String;

    fn get_uid(&self) -> UID;
    fn get_version(&self) -> String;
    fn supported_versions(&self) -> Range<u64>;

    fn init_settings(&self, data: &mut Values) -> Result<(), SessionError>;
    fn init_element_settings(&self, data: &mut Values) -> Result<(), SessionError>;
    fn init_location_settings(&self, data: &mut Values) -> Result<(), SessionError>;

    fn init_element(&self, element_row: ERow) -> Result<(), SessionError>;
    fn step_element(
        &self,
        element_row: ERow,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError>;

    fn accept_extension(&self, filename: &str) -> bool;
    fn accept_url(&self, url: String) -> bool;
    fn accepted_extensions(&self) -> Vec<String>;
    fn accepted_protocols(&self) -> Vec<String>;

    fn init_location(&self, location_ref: LRef) -> Result<(), SessionError>;
    fn step_location(
        &self,
        location_row: LRow,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError>;

    fn notify(&self, _ref: Ref, event: Event) -> Result<(), SessionError>;

    fn c(&self) -> Box<dyn TModule>;
}

impl Clone for Box<dyn TModule> {
    fn clone(&self) -> Self {
        self.c()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, bytes_kman::Bytes)]
pub enum RawLibraryError {
    NotFound,
    DontHaveSymbolGetName,
    DontHaveSymbolGetDesc,
    DontHaveSymbolGetUID,
    DontHaveSymbolGetVersion,
    DontHaveSymbolSupportedVersions,
    DontHaveSymbolInit,
    DontHaveSymbolInitSettings,
    DontHaveSymbolInitElementSettings,
    DontHaveSymbolInitLocationSettings,
    DontHaveSymbolInitElement,
    DontHaveSymbolStepElement,
    DontHaveSymbolAcceptExtension,
    DontHaveSymbolAcceptUrl,
    DontHaveSymbolAcceptedExtensions,
    DontHaveSymbolAcceptedProtocols,
    DontHaveSymbolInitLocation,
    DontHaveSymbolStepLocation,
    DontHaveSymbolNotify,
}

impl From<RawLibraryError> for SessionError {
    fn from(value: RawLibraryError) -> Self {
        Self::RawModule(value)
    }
}

#[allow(clippy::type_complexity)]
pub struct RawModule {
    lib: &'static Library,

    fn_init: Symbol<'static, fn(MRef) -> Result<(), SessionError>>,

    fn_get_name: Symbol<'static, fn() -> String>,
    fn_get_desc: Symbol<'static, fn() -> String>,

    fn_get_uid: Symbol<'static, fn() -> UID>,
    fn_get_version: Symbol<'static, fn() -> String>,
    fn_supported_versions: Symbol<'static, fn() -> Range<u64>>,

    fn_init_settings: Symbol<'static, fn(&mut Values) -> Result<(), SessionError>>,
    fn_init_element_settings: Symbol<'static, fn(&mut Values) -> Result<(), SessionError>>,
    fn_init_location_settings: Symbol<'static, fn(&mut Values) -> Result<(), SessionError>>,

    fn_init_element: Symbol<'static, fn(ERow) -> Result<(), SessionError>>,
    fn_init_location: Symbol<'static, fn(LRef) -> Result<(), SessionError>>,

    fn_step_element:
        Symbol<'static, fn(ERow, &mut ControlFlow, &mut Storage) -> Result<(), SessionError>>,
    fn_step_location:
        Symbol<'static, fn(LRow, &mut ControlFlow, &mut Storage) -> Result<(), SessionError>>,

    fn_accept_extension: Symbol<'static, fn(&str) -> bool>,
    fn_accept_url: Symbol<'static, fn(String) -> bool>,
    fn_accepted_extensions: Symbol<'static, fn() -> Vec<String>>,
    fn_accepted_protocols: Symbol<'static, fn() -> Vec<String>>,

    fn_notify: Symbol<'static, fn(Ref, Event) -> Result<(), SessionError>>,
}

impl RawModule {
    pub fn new_module(path: &Path) -> Result<Box<dyn TModule>, RawLibraryError> {
        match Self::new(path) {
            Ok(module) => Ok(Box::new(Arc::new(module))),
            Err(err) => Err(err),
        }
    }
    pub fn new(path: &Path) -> Result<Self, RawLibraryError> {
        let lib = unsafe { Library::new(path) };
        if lib.is_err() {
            return Err(RawLibraryError::NotFound);
        }
        let lib = Box::leak(Box::new(lib.unwrap()));

        let fn_init = if let Ok(func) = unsafe { lib.get(b"init\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolInit);
        };

        let fn_get_name = if let Ok(func) = unsafe { lib.get(b"get_name\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolGetName);
        };

        let fn_get_desc = if let Ok(func) = unsafe { lib.get(b"get_desc\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolGetDesc);
        };

        let fn_init_settings = if let Ok(func) = unsafe { lib.get(b"init_settings\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolInitSettings);
        };

        let fn_init_element_settings =
            if let Ok(func) = unsafe { lib.get(b"init_element_settings\0") } {
                func
            } else {
                return Err(RawLibraryError::DontHaveSymbolInitElementSettings);
            };

        let fn_init_element = if let Ok(func) = unsafe { lib.get(b"init_element\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolInitElement);
        };

        let fn_step_element = if let Ok(func) = unsafe { lib.get(b"step_element\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolStepElement);
        };

        let fn_accept_extension = if let Ok(func) = unsafe { lib.get(b"accept_extension\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolAcceptExtension);
        };

        let fn_accept_url = if let Ok(func) = unsafe { lib.get(b"accept_url\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolAcceptUrl);
        };

        let fn_accepted_protocols = if let Ok(func) = unsafe { lib.get(b"accepted_protocols\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolAcceptedProtocols);
        };

        let fn_init_location = if let Ok(func) = unsafe { lib.get(b"init_location\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolInitLocation);
        };

        let fn_notify = if let Ok(func) = unsafe { lib.get(b"notify\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolNotify);
        };

        let fn_step_location = if let Ok(func) = unsafe { lib.get(b"step_location\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolStepLocation);
        };

        let fn_get_uid = if let Ok(func) = unsafe { lib.get(b"get_uid\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolGetUID);
        };

        let fn_get_version = if let Ok(func) = unsafe { lib.get(b"get_version\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolGetVersion);
        };

        let fn_accepted_extensions = if let Ok(func) = unsafe { lib.get(b"accepted_extensions\0") }
        {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolAcceptedExtensions);
        };

        let fn_supported_versions = if let Ok(func) = unsafe { lib.get(b"supported_versions\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolSupportedVersions);
        };

        let fn_init_location_settings =
            if let Ok(func) = unsafe { lib.get(b"init_location_settings\0") } {
                func
            } else {
                return Err(RawLibraryError::DontHaveSymbolInitLocationSettings);
            };

        if let Ok(mut logger_state) = unsafe {
            lib.get::<
                *mut once_cell::sync::Lazy<std::sync::Arc<std::sync::RwLock<crate::logger::State>>,
            >>(b"LOGGER_STATE\0")
        } {
            let state = logger::LOGGER_STATE.clone();
            unsafe {
                let dylib_state = once_cell::sync::Lazy::<
                    std::sync::Arc<std::sync::RwLock<crate::logger::State>>,
                    fn() -> std::sync::Arc<std::sync::RwLock<crate::logger::State>>,
                >::force_mut(logger_state.as_mut().unwrap());

                std::mem::replace(dylib_state, state);
            }
            println!("Logger state set for module!");
        } else {
            println!("Logger is not set");
        }

        Ok(Self {
            lib,
            fn_init,
            fn_get_name,
            fn_get_desc,
            fn_init_settings,
            fn_init_element_settings,
            fn_init_element,
            fn_init_location,
            fn_step_element,
            fn_accept_extension,
            fn_accept_url,
            fn_accepted_protocols,
            fn_notify,
            fn_step_location,
            fn_get_uid,
            fn_get_version,
            fn_supported_versions,
            fn_accepted_extensions,
            fn_init_location_settings,
        })
    }
}

impl Drop for RawModule {
    fn drop(&mut self) {
        let lib = unsafe { Box::from_raw((self.lib as *const _) as *mut Library) };
        drop(lib)
    }
}

impl TModule for Arc<RawModule> {
    fn init(&self, info: MRef) -> Result<(), SessionError> {
        (*self.fn_init)(info)
    }

    fn get_name(&self) -> String {
        (*self.fn_get_name)()
    }

    fn get_desc(&self) -> String {
        (*self.fn_get_desc)()
    }

    fn get_uid(&self) -> u64 {
        (*self.fn_get_uid)()
    }

    fn get_version(&self) -> String {
        (*self.fn_get_version)()
    }

    fn supported_versions(&self) -> Range<u64> {
        (*self.fn_supported_versions)()
    }

    fn init_settings(&self, data: &mut Values) -> Result<(), SessionError> {
        (*self.fn_init_settings)(data)
    }

    fn init_element_settings(&self, data: &mut Values) -> Result<(), SessionError> {
        (*self.fn_init_element_settings)(data)
    }

    fn init_location_settings(&self, data: &mut Values) -> Result<(), SessionError> {
        (*self.fn_init_location_settings)(data)
    }

    fn init_element(&self, element: ERow) -> Result<(), SessionError> {
        (*self.fn_init_element)(element)
    }

    fn step_element(
        &self,
        element: ERow,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError> {
        (*self.fn_step_element)(element, control_flow, storage)
    }

    fn accept_extension(&self, filename: &str) -> bool {
        (*self.fn_accept_extension)(filename)
    }

    fn accept_url(&self, url: String) -> bool {
        (*self.fn_accept_url)(url)
    }

    fn accepted_extensions(&self) -> Vec<String> {
        (*self.fn_accepted_extensions)()
    }

    fn accepted_protocols(&self) -> Vec<String> {
        (*self.fn_accepted_protocols)()
    }

    fn init_location(&self, location: LRef) -> Result<(), SessionError> {
        (*self.fn_init_location)(location)
    }

    fn step_location(
        &self,
        location: LRow,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError> {
        (*self.fn_step_location)(location, control_flow, storage)
    }

    fn notify(&self, info: Ref, event: Event) -> Result<(), SessionError> {
        (*self.fn_notify)(info, event)
    }

    fn c(&self) -> Box<dyn TModule> {
        Box::new(self.clone())
    }
}

pub trait TModuleInfo {
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError>;

    fn get_name(&self) -> Result<String, SessionError>;
    fn set_name(&self, name: impl Into<String>) -> Result<(), SessionError>;
    fn get_default_name(&self) -> Result<String, SessionError>;

    fn uid(&self) -> Result<UID, SessionError>;
    fn version(&self) -> Result<String, SessionError>;
    fn supported_versions(&self) -> Result<Range<u64>, SessionError>;

    fn get_desc(&self) -> Result<String, SessionError>;
    fn set_desc(&self, desc: impl Into<String>) -> Result<(), SessionError>;
    fn get_default_desc(&self) -> Result<String, SessionError>;

    fn get_proxy(&self) -> Result<usize, SessionError>;
    fn set_proxy(&self, proxy: usize) -> Result<(), SessionError>;

    fn get_settings(&self) -> Result<Values, SessionError>;
    fn set_settings(&self, settings: Values) -> Result<(), SessionError>;

    fn get_element_settings(&self) -> Result<Values, SessionError>;
    fn set_element_settings(&self, settings: Values) -> Result<(), SessionError>;

    fn register_action(
        &self,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError>;
    fn remove_action(&self, name: String) -> Result<(), SessionError>;
    fn run_action(&self, name: String, data: Vec<Type>) -> Result<(), SessionError>;

    fn step_element(
        &self,
        element_info: &ElementId,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError>;
    fn step_location(
        &self,
        location_info: &LocationId,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError>;

    fn accept_url(&self, url: String) -> Result<bool, SessionError>;
    fn accept_extension(&self, filename: impl Into<String>) -> Result<bool, SessionError>;
    fn accepted_protocols(&self) -> Result<Vec<String>, SessionError>;
    fn accepted_extensions(&self) -> Result<Vec<String>, SessionError>;

    fn init_element(&self, element_info: &ElementId) -> Result<(), SessionError>;
    fn init_location(&self, location_info: &LocationId) -> Result<(), SessionError>;

    fn notify(&self, info: ID, event: Event) -> Result<(), SessionError>;

    fn id(&self) -> ModuleId;
}

impl TModuleInfo for MRef {
    fn get_session(&self) -> Result<Box<dyn TSession>, SessionError> {
        if let Some(session) = &self.read().unwrap().session {
            return Ok(session.c());
        }
        Err(SessionError::InvalidSession)
    }

    fn get_name(&self) -> Result<String, SessionError> {
        self.get_session()?.module_get_name(&self.id())
    }

    fn set_name(&self, name: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?.module_set_name(&self.id(), name.into())
    }

    fn get_default_name(&self) -> Result<String, SessionError> {
        self.get_session()?.module_get_default_name(&self.id())
    }

    fn uid(&self) -> Result<UID, SessionError> {
        self.get_session()?.module_get_uid(&self.id())
    }

    fn version(&self) -> Result<String, SessionError> {
        self.get_session()?.module_get_version(&self.id())
    }

    fn supported_versions(&self) -> Result<Range<u64>, SessionError> {
        self.get_session()?.module_supported_versions(&self.id())
    }

    fn get_desc(&self) -> Result<String, SessionError> {
        self.get_session()?.module_get_desc(&self.id())
    }

    fn set_desc(&self, desc: impl Into<String>) -> Result<(), SessionError> {
        self.get_session()?.module_set_desc(&self.id(), desc.into())
    }

    fn get_default_desc(&self) -> Result<String, SessionError> {
        self.get_session()?.module_get_default_desc(&self.id())
    }

    fn get_proxy(&self) -> Result<usize, SessionError> {
        self.get_session()?.module_get_proxy(&self.id())
    }

    fn set_proxy(&self, proxy: usize) -> Result<(), SessionError> {
        self.get_session()?.module_set_proxy(&self.id(), proxy)
    }

    fn get_settings(&self) -> Result<Values, SessionError> {
        self.get_session()?.module_get_settings(&self.id())
    }

    fn set_settings(&self, settings: Values) -> Result<(), SessionError> {
        self.get_session()?
            .module_set_settings(&self.id(), settings)
    }

    fn get_element_settings(&self) -> Result<Values, SessionError> {
        self.get_session()?.module_get_element_settings(&self.id())
    }

    fn set_element_settings(&self, settings: Values) -> Result<(), SessionError> {
        self.get_session()?
            .module_set_element_settings(&self.id(), settings)
    }

    fn register_action(
        &self,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError> {
        self.get_session()?
            .register_action(&self.id(), name, values, callback)
    }

    fn remove_action(&self, name: String) -> Result<(), SessionError> {
        self.get_session()?.remove_action(&self.id(), name)
    }

    fn run_action(&self, name: String, data: Vec<Type>) -> Result<(), SessionError> {
        self.get_session()?.run_action(&self.id(), name, data)
    }

    fn step_element(
        &self,
        element_info: &ElementId,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError> {
        self.get_session()?
            .module_step_element(&self.id(), element_info, control_flow, storage)
    }

    fn step_location(
        &self,
        location_info: &LocationId,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError> {
        self.get_session()?
            .module_step_location(&self.id(), location_info, control_flow, storage)
    }

    fn accept_url(&self, url: String) -> Result<bool, SessionError> {
        self.get_session()?.module_accept_url(&self.id(), url)
    }

    fn accept_extension(&self, filename: impl Into<String>) -> Result<bool, SessionError> {
        self.get_session()?
            .module_accept_extension(&self.id(), &filename.into())
    }

    fn accepted_protocols(&self) -> Result<Vec<String>, SessionError> {
        self.get_session()?.module_accepted_protocols(&self.id())
    }

    fn accepted_extensions(&self) -> Result<Vec<String>, SessionError> {
        self.get_session()?.module_accepted_extensions(&self.id())
    }

    fn init_element(&self, element_info: &ElementId) -> Result<(), SessionError> {
        self.get_session()?
            .module_init_element(&self.id(), element_info)
    }

    fn init_location(&self, location_info: &LocationId) -> Result<(), SessionError> {
        self.get_session()?
            .module_init_location(&self.id(), location_info)
    }

    fn notify(&self, info: ID, event: Event) -> Result<(), SessionError> {
        let session = self.get_session()?;
        match info {
            ID::Element(e) => session.element_notify(&e, event),
            ID::Location(l) => session.location_notify(&l, event),
        }
    }

    fn id(&self) -> ModuleId {
        self.read().unwrap().uid
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, bytes_kman::Bytes)]
pub struct ModuleInfo {
    pub name: String,
    pub desc: String,
    pub path: Option<PathBuf>,
    pub id: ModuleId,
    pub uid: u64,
    pub version: String,
    pub proxy: usize,
    pub settings: Values,
    pub element_settings: Values,
    pub location_settings: Values,
    pub supports_protocols: Vec<String>,
    pub supports_file_types: Vec<PathBuf>,
}
