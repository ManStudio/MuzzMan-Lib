use std::fmt::Debug;

use crate::prelude::*;
use libloading::{Library, Symbol};

#[derive(Clone)]
pub enum ControlFlow {
    Run,
    Pause,
    Break,
}

pub struct ModuleInfo {
    pub uid: Option<usize>,
    pub session: Option<Box<dyn TSession>>,
}

impl Debug for ModuleInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleInfo").finish()
    }
}

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub desc: String,
    pub module: Box<dyn TModule>,
    pub proxy: usize,
    /// default module data/settings
    pub settings: Data,
    /// default element data/settings
    pub element_data: Data,
    pub info: Option<MInfo>,
}

impl Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Module {}", self.name))
            .field("name", &self.name)
            .field("desc", &self.desc)
            .field("proxy", &self.proxy)
            .field("settings", &self.settings)
            .field("element_data", &self.element_data)
            .finish()
    }
}

pub trait TModule: std::panic::UnwindSafe {
    fn init(&self, session: Box<dyn TSession>) -> Result<(), String>;

    fn get_name(&self) -> String;
    fn get_desc(&self) -> String;

    fn init_settings(&self, data: &mut Data);
    fn init_element_settings(&self, data: &mut Data);

    fn init_element(&self, element: ERow);
    fn step_element(&self, element: ERow, control_flow: &mut ControlFlow);

    fn accept_extension(&self, filename: &str) -> bool;
    fn accept_url(&self, uri: Url) -> bool;
    fn init_location(&self, location: LInfo, data: FileOrData);

    fn c(&self) -> Box<dyn TModule>;
}

impl Clone for Box<dyn TModule> {
    fn clone(&self) -> Self {
        self.c()
    }
}

#[derive(Debug)]
pub enum RawLibraryError {
    NotFound,
    DontHaveSymbolGetName,
    DontHaveSymbolGetDesc,
    DontHaveSymbolInit,
    DontHaveSymbolInitSettings,
    DontHaveSymbolInitElementSettings,
    DontHaveSymbolInitElement,
    DontHaveSymbolStepElement,
    DontHaveSymbolAcceptExtension,
    DontHaveSymbolAcceptUrl,
    DontHaveSymbolInitLocation,
}

pub struct RawModule {
    lib: &'static Library,

    fn_init: Symbol<'static, fn(Box<dyn TSession>) -> Result<(), String>>,

    fn_get_name: Symbol<'static, fn() -> String>,
    fn_get_desc: Symbol<'static, fn() -> String>,

    fn_init_settings: Symbol<'static, fn(&mut Data)>,
    fn_init_element_settings: Symbol<'static, fn(&mut Data)>,

    fn_init_element: Symbol<'static, fn(ERow)>,
    fn_init_location: Symbol<'static, fn(LInfo, FileOrData)>,

    fn_step_element: Symbol<'static, fn(ERow, &mut ControlFlow)>,

    fn_accept_extension: Symbol<'static, fn(&str) -> bool>,
    fn_accept_url: Symbol<'static, fn(Url) -> bool>,
}

impl RawModule {
    pub fn new(path: &str) -> Result<Box<dyn TModule>, RawLibraryError> {
        match Self::new_raw(path) {
            Ok(module) => Ok(Box::new(Arc::new(module))),
            Err(err) => Err(err),
        }
    }
    pub fn new_raw(path: &str) -> Result<Self, RawLibraryError> {
        let lib = unsafe { Library::new(path) };
        if lib.is_err() {
            return Err(RawLibraryError::NotFound);
        }
        let lib = Box::leak(Box::new(lib.unwrap()));

        let fn_init = if let Ok(func) = unsafe { lib.get(b"init\0").into() } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolInit);
        };

        let fn_get_name = if let Ok(func) = unsafe { lib.get(b"get_name\0").into() } {
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

        let fn_init_location = if let Ok(func) = unsafe { lib.get(b"init_location\0") } {
            func
        } else {
            return Err(RawLibraryError::DontHaveSymbolInitLocation);
        };

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
        })
    }
}

impl Drop for RawModule {
    fn drop(&mut self) {
        std::mem::forget(self.lib)
    }
}

impl TModule for Arc<RawModule> {
    fn init(&self, session: Box<dyn TSession>) -> Result<(), String> {
        (*self.fn_init)(session)
    }

    fn get_name(&self) -> String {
        (*self.fn_get_name)()
    }

    fn get_desc(&self) -> String {
        (*self.fn_get_desc)()
    }

    fn init_settings(&self, data: &mut Data) {
        (*self.fn_init_settings)(data)
    }

    fn init_element_settings(&self, data: &mut Data) {
        (*self.fn_init_element_settings)(data)
    }

    fn init_element(&self, element: ERow) {
        (*self.fn_init_element)(element)
    }

    fn step_element(&self, element: ERow, control_flow: &mut ControlFlow) {
        (*self.fn_step_element)(element, control_flow)
    }

    fn accept_extension(&self, filename: &str) -> bool {
        (*self.fn_accept_extension)(filename)
    }

    fn accept_url(&self, url: Url) -> bool {
        (*self.fn_accept_url)(url)
    }

    fn init_location(&self, location: LInfo, data: FileOrData) {
        (*self.fn_init_location)(location, data)
    }

    fn c(&self) -> Box<dyn TModule> {
        Box::new(self.clone())
    }
}
