use std::{
    ops::DerefMut,
    path::Path,
    sync::{Arc, RwLock},
};

use libloading::{Library, Symbol};
use muzzman_lib::{prelude::*, storage::Storage};
use once_cell::sync::Lazy;

#[allow(clippy::type_complexity)]
pub struct RawModule {
    lib: &'static Library,

    fn_name: Symbol<'static, fn() -> &'static str>,
    fn_desc: Symbol<'static, fn() -> &'static str>,

    fn_id: Symbol<'static, fn() -> u64>,
    fn_version: Symbol<'static, fn() -> u64>,
    fn_supported_versions: Symbol<'static, fn() -> Vec<u64>>,

    fn_poll_element: Symbol<
        'static,
        fn(&mut std::task::Context<'_>, Arc<RwLock<Element>>, &mut Storage) -> SessionResult<()>,
    >,
    fn_poll_location: Symbol<
        'static,
        fn(&mut std::task::Context<'_>, Arc<RwLock<Location>>, &mut Storage) -> SessionResult<()>,
    >,

    fn_element_on_event:
        Symbol<'static, fn(Arc<RwLock<Element>>, event: Event, &mut Storage) -> SessionResult<()>>,
    fn_location_on_event:
        Symbol<'static, fn(Arc<RwLock<Location>>, event: Event, &mut Storage) -> SessionResult<()>>,

    fn_default_element_settings: Symbol<'static, fn() -> Settings>,
    fn_default_location_settings: Symbol<'static, fn() -> Settings>,

    fn_supports_protocols: Symbol<'static, fn() -> Vec<String>>,
    fn_supports_extensions: Symbol<'static, fn() -> Vec<String>>,
}

impl RawModule {
    pub fn new_module(path: &Path) -> Result<Box<dyn TModule>, RawLibraryError> {
        match Self::new(path) {
            Ok(module) => Ok(Box::new(module)),
            Err(err) => Err(err),
        }
    }
    pub fn new(path: &Path) -> Result<Self, RawLibraryError> {
        let lib = unsafe { Library::new(path) };
        if lib.is_err() {
            return Err(RawLibraryError::NotFound);
        }
        let lib = Box::leak(Box::new(lib.unwrap()));

        let fn_name = if let Ok(func) = unsafe { lib.get(b"name\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolName);
        };

        let fn_desc = if let Ok(func) = unsafe { lib.get(b"desc\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolDesc);
        };

        let fn_id = if let Ok(func) = unsafe { lib.get(b"id\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolId);
        };

        let fn_version = if let Ok(func) = unsafe { lib.get(b"version\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolVersion);
        };

        let fn_supported_versions = if let Ok(func) = unsafe { lib.get(b"supported_version\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolSupportedVersions);
        };

        let fn_poll_element = if let Ok(func) = unsafe { lib.get(b"poll_element\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolPollElement);
        };

        let fn_poll_location = if let Ok(func) = unsafe { lib.get(b"poll_location\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolPollLocation);
        };

        let fn_element_on_event = if let Ok(func) = unsafe { lib.get(b"element_on_event\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolElementOnEvent);
        };

        let fn_location_on_event = if let Ok(func) = unsafe { lib.get(b"location_on_event\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolLocationOnEvent);
        };

        let fn_default_element_settings =
            if let Ok(func) = unsafe { lib.get(b"default_element_settings\0") } {
                func
            } else {
                return Err(RawLibraryError::DontHaveSymbolDefaultElementSettings);
            };

        let fn_default_location_settings =
            if let Ok(func) = unsafe { lib.get(b"default_location_settings\0") } {
                func
            } else {
                return Err(RawLibraryError::DontHaveSymbolDefaultLocationSettings);
            };

        let fn_supports_protocols = if let Ok(func) = unsafe { lib.get(b"supports_protocols\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolSupportsProtocols);
        };
        let fn_supports_extensions = if let Ok(func) = unsafe { lib.get(b"supports_extensions\0") }
        {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolSupportsExtensions);
        };

        if let Ok(logger_state) = unsafe {
            lib.get::<*mut Lazy<std::sync::Arc<std::sync::RwLock<muzzman_lib::logger::State>>>>(
                b"LOGGER_STATE\0",
            )
        } {
            let state = muzzman_lib::logger::LOGGER_STATE.clone();
            unsafe {
                let dylib_state = logger_state.as_mut().unwrap().deref_mut();

                let _ = std::mem::replace(dylib_state, state);
            }
        }

        if let Ok(logger_who_iam) = unsafe {
            lib.get::<*mut Lazy<Arc<std::thread::LocalKey<RwLock<muzzman_lib::logger::Iam>>>>>(
                b"LOGGER_WHO_IAM\0",
            )
        } {
            let state = muzzman_lib::logger::LOGGER_WHO_IAM.clone();
            unsafe {
                let dylib_who_iam = logger_who_iam.as_mut().unwrap().deref_mut();

                let _ = std::mem::replace(dylib_who_iam, state);
            }
        }

        Ok(Self {
            lib,
            fn_name,
            fn_desc,
            fn_id,
            fn_version,
            fn_supported_versions,
            fn_poll_element,
            fn_poll_location,
            fn_element_on_event,
            fn_location_on_event,
            fn_default_element_settings,
            fn_default_location_settings,
            fn_supports_protocols,
            fn_supports_extensions,
        })
    }
}

impl Drop for RawModule {
    fn drop(&mut self) {
        let lib = unsafe { Box::from_raw((self.lib as *const _) as *mut Library) };
        drop(lib)
    }
}

impl TModule for RawModule {
    fn name(&self) -> &str {
        (*self.fn_name)()
    }

    fn desc(&self) -> &str {
        (*self.fn_desc)()
    }

    fn id(&self) -> u64 {
        (*self.fn_id)()
    }

    fn version(&self) -> u64 {
        (*self.fn_version)()
    }

    fn supported_versions(&self) -> Vec<u64> {
        (*self.fn_supported_versions)()
    }

    fn poll_element(
        &self,
        ctx: &mut std::task::Context<'_>,
        element: Arc<RwLock<Element>>,
        storage: &mut Storage,
    ) -> SessionResult<()> {
        (*self.fn_poll_element)(ctx, element, storage)
    }

    fn poll_location(
        &self,
        ctx: &mut std::task::Context<'_>,
        location: Arc<RwLock<Location>>,
        storage: &mut Storage,
    ) -> SessionResult<()> {
        (*self.fn_poll_location)(ctx, location, storage)
    }

    fn element_on_event(
        &self,
        element: Arc<RwLock<Element>>,
        event: Event,
        storage: &mut Storage,
    ) -> SessionResult<()> {
        (*self.fn_element_on_event)(element, event, storage)
    }

    fn location_on_event(
        &self,
        location: Arc<RwLock<Location>>,
        event: Event,
        storage: &mut Storage,
    ) -> SessionResult<()> {
        (*self.fn_location_on_event)(location, event, storage)
    }

    fn default_element_settings(&self) -> Settings {
        (*self.fn_default_element_settings)()
    }

    fn default_location_settings(&self) -> Settings {
        (*self.fn_default_location_settings)()
    }

    fn supports_protocols(&self) -> Vec<String> {
        (*self.fn_supports_protocols)()
    }

    fn supports_extensions(&self) -> Vec<String> {
        (*self.fn_supports_extensions)()
    }
}
