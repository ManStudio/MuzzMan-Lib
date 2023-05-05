use std::ops::Range;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

use bytes_kman::TBytes;

#[derive(Clone, Debug, Serialize, Deserialize, bytes_kman::Bytes)]
pub enum SessionError {
    InvalidSession,
    EmptyElement,
    EmptyLocation,
    EmptyModule,
    ElementDoNotExist,
    InsufficientPermissions,
    InvalidLocation,
    ServerTimeOut,
    CannotConnectToServer,
    ServerInvalidIndentification,
    InvalidElementStatus,
    LocationAllreadyExist,
    InvalidModule,
    CannotInstallModule(Box<SessionError>),
    AlreadySubscribed,
    AlreadyUnsubscribed,
    IsNotElement,
    IsNotLocation,
    CannotLoadModuleInfo,
    CannotFindModule,
    CannotLoadElementInfo,
    CannotLoadLocationInfo,
    DefaultLocationDoNotExist,
    PosionErrorCannotLockForRead,
    PosionErrorCannotLocakForWrite,
    StepOnElementPaniced,
    RawModule(RawLibraryError),
    Custom(String),
}

impl<'a, T> From<std::sync::PoisonError<std::sync::RwLockReadGuard<'a, T>>> for SessionError {
    fn from(_value: std::sync::PoisonError<std::sync::RwLockReadGuard<T>>) -> Self {
        Self::PosionErrorCannotLockForRead
    }
}

impl<'a, T> From<std::sync::PoisonError<std::sync::RwLockWriteGuard<'a, T>>> for SessionError {
    fn from(_value: std::sync::PoisonError<std::sync::RwLockWriteGuard<T>>) -> Self {
        Self::PosionErrorCannotLocakForWrite
    }
}

impl From<String> for SessionError {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

pub type Actions = Vec<(String, MRef, Vec<(String, Value)>)>;

// TODO: all the functions should be async
// because is possible to be performed on the network
// or should have a asnyc version
// like TSessionAsync
pub trait TSession: Send + Sync {
    //
    // Module
    //

    fn load_module(&self, path: PathBuf) -> Result<MRef, SessionError>;
    fn remove_module(&self, id: ModulePath) -> Result<MRow, SessionError>;
    fn load_module_info(&self, info: ModuleInfo) -> Result<MRef, SessionError>;
    fn find_module(&self, info: ModuleInfo) -> Result<MRef, SessionError>;

    fn register_action(
        &self,
        module_id: &ModulePath,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError>;
    fn remove_action(&self, module_id: &ModulePath, name: String) -> Result<(), SessionError>;
    fn get_actions(&self, range: Range<usize>) -> Result<Actions, SessionError>;
    fn get_actions_len(&self) -> Result<usize, SessionError>;
    fn run_action(
        &self,
        module_id: &ModulePath,
        name: String,
        data: Vec<Type>,
    ) -> Result<(), SessionError>;

    fn get_modules_len(&self) -> Result<usize, SessionError>;
    fn get_modules(&self, range: Range<usize>) -> Result<Vec<MRef>, SessionError>;

    fn module_get_name(&self, module_id: &ModulePath) -> Result<String, SessionError>;
    fn module_set_name(&self, module_id: &ModulePath, name: String) -> Result<(), SessionError>;
    fn module_get_default_name(&self, module_id: &ModulePath) -> Result<String, SessionError>;

    fn module_get_uid(&self, module_id: &ModulePath) -> Result<UID, SessionError>;
    fn module_get_version(&self, module_id: &ModulePath) -> Result<String, SessionError>;
    fn module_supported_versions(&self, module_id: &ModulePath)
        -> Result<Range<u64>, SessionError>;

    fn module_get_desc(&self, module_id: &ModulePath) -> Result<String, SessionError>;
    fn module_set_desc(&self, module_id: &ModulePath, desc: String) -> Result<(), SessionError>;
    fn module_get_default_desc(&self, module_id: &ModulePath) -> Result<String, SessionError>;

    fn module_get_proxy(&self, module_id: &ModulePath) -> Result<usize, SessionError>;
    fn module_set_proxy(&self, module_id: &ModulePath, proxy: usize) -> Result<(), SessionError>;

    fn module_get_settings(&self, module_id: &ModulePath) -> Result<Values, SessionError>;
    fn module_set_settings(&self, module_id: &ModulePath, data: Values)
        -> Result<(), SessionError>;

    fn module_get_element_settings(&self, module_id: &ModulePath) -> Result<Values, SessionError>;
    fn module_set_element_settings(
        &self,
        module_id: &ModulePath,
        data: Values,
    ) -> Result<(), SessionError>;

    fn module_get_location_settings(&self, module_id: &ModulePath) -> Result<Values, SessionError>;
    fn module_set_location_settings(
        &self,
        module_id: &ModulePath,
        data: Values,
    ) -> Result<(), SessionError>;

    fn module_init_location(
        &self,
        module_id: &ModulePath,
        location_id: &LocationPath,
    ) -> Result<(), SessionError>;

    fn module_init_element(
        &self,
        module_id: &ModulePath,
        element_id: &ElementPath,
    ) -> Result<(), SessionError>;

    fn module_accept_url(&self, module_id: &ModulePath, url: String) -> Result<bool, SessionError>;

    fn module_accept_extension(
        &self,
        module_id: &ModulePath,
        filename: &str,
    ) -> Result<bool, SessionError>;

    fn module_accepted_protocols(
        &self,
        module_id: &ModulePath,
    ) -> Result<Vec<String>, SessionError>;
    fn module_accepted_extensions(
        &self,
        module_id: &ModulePath,
    ) -> Result<Vec<String>, SessionError>;

    fn module_step_element(
        &self,
        module_id: &ModulePath,
        element_id: &ElementPath,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError>;

    fn module_step_location(
        &self,
        module_id: &ModulePath,
        location_id: &LocationPath,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError>;

    //
    // End Module
    //

    //
    // Element
    //

    fn create_element(&self, name: &str, location_id: &LocationPath) -> Result<ERef, SessionError>;
    fn load_element_info(&self, info: ElementInfo) -> Result<ERef, SessionError>;
    fn move_element(
        &self,
        element: &ElementPath,
        location_id: &LocationPath,
    ) -> Result<(), SessionError>;
    fn destroy_element(&self, element_id: ElementPath) -> Result<ERow, SessionError>;

    fn element_get_name(&self, element_id: &ElementPath) -> Result<String, SessionError>;
    fn element_set_name(&self, element_id: &ElementPath, name: &str) -> Result<(), SessionError>;

    fn element_get_desc(&self, element_id: &ElementPath) -> Result<String, SessionError>;
    fn element_set_desc(&self, element_id: &ElementPath, desc: &str) -> Result<(), SessionError>;

    fn element_get_meta(&self, element_id: &ElementPath) -> Result<String, SessionError>;
    fn element_set_meta(&self, element_id: &ElementPath, meta: &str) -> Result<(), SessionError>;

    fn element_get_url(&self, element_id: &ElementPath) -> Result<Option<String>, SessionError>;
    fn element_set_url(
        &self,
        element_id: &ElementPath,
        url: Option<String>,
    ) -> Result<(), SessionError>;

    fn element_get_element_data(&self, element_id: &ElementPath) -> Result<Values, SessionError>;
    fn element_set_element_data(
        &self,
        element_id: &ElementPath,
        data: Values,
    ) -> Result<(), SessionError>;

    fn element_get_module_data(&self, element_id: &ElementPath) -> Result<Values, SessionError>;
    fn element_set_module_data(
        &self,
        element_id: &ElementPath,
        data: Values,
    ) -> Result<(), SessionError>;

    fn element_get_module(&self, element_id: &ElementPath) -> Result<Option<MRef>, SessionError>;
    fn element_set_module(
        &self,
        element: &ElementPath,
        module: Option<ModulePath>,
    ) -> Result<(), SessionError>;

    fn element_get_statuses(&self, element_id: &ElementPath) -> Result<Vec<String>, SessionError>;
    fn element_set_statuses(
        &self,
        element: &ElementPath,
        statuses: Vec<String>,
    ) -> Result<(), SessionError>;

    fn element_get_status(&self, element_id: &ElementPath) -> Result<usize, SessionError>;
    fn element_set_status(
        &self,
        element_id: &ElementPath,
        status: usize,
    ) -> Result<(), SessionError>;

    fn element_get_data(&self, element_id: &ElementPath) -> Result<Data, SessionError>;
    fn element_set_data(&self, element_id: &ElementPath, data: Data) -> Result<(), SessionError>;

    fn element_get_progress(&self, element_id: &ElementPath) -> Result<f32, SessionError>;
    fn element_set_progress(
        &self,
        element_id: &ElementPath,
        progress: f32,
    ) -> Result<(), SessionError>;

    fn element_get_should_save(&self, element_id: &ElementPath) -> Result<bool, SessionError>;
    fn element_set_should_save(
        &self,
        element: &ElementPath,
        should_save: bool,
    ) -> Result<(), SessionError>;

    fn element_get_enabled(&self, element_id: &ElementPath) -> Result<bool, SessionError>;
    fn element_set_enabled(
        &self,
        element_id: &ElementPath,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError>;

    fn element_is_error(&self, element_id: &ElementPath) -> Result<bool, SessionError>;

    fn element_resolv_module(&self, element_id: &ElementPath) -> Result<bool, SessionError>;

    /// Blocking the current thread until is done!
    fn element_wait(&self, element_id: &ElementPath) -> Result<(), SessionError>;

    fn element_get_element_info(
        &self,
        element_id: &ElementPath,
    ) -> Result<ElementInfo, SessionError>;
    fn element_notify(&self, element_id: &ElementPath, event: Event) -> Result<(), SessionError>;

    fn element_emit(&self, element_id: &ElementPath, event: Event) -> Result<(), SessionError>;
    fn element_subscribe(&self, element_id: &ElementPath, _ref: ID) -> Result<(), SessionError>;
    fn element_unsubscribe(&self, element_id: &ElementPath, _ref: ID) -> Result<(), SessionError>;

    //
    // End Element
    //

    //
    // Location
    //

    fn create_location(&self, name: &str, location_id: &LocationPath)
        -> Result<LRef, SessionError>;
    fn load_location_info(&self, info: LocationInfo) -> Result<LRef, SessionError>;
    fn get_locations_len(&self, location_id: &LocationPath) -> Result<usize, SessionError>;
    fn get_locations(
        &self,
        location_id: &LocationPath,
        range: Range<usize>,
    ) -> Result<Vec<LRef>, SessionError>;
    fn destroy_location(&self, location_id: LocationPath) -> Result<LRow, SessionError>;
    fn get_default_location(&self) -> Result<LRef, SessionError>;
    fn move_location(
        &self,
        location_id: &LocationPath,
        to: &LocationPath,
    ) -> Result<(), SessionError>;

    fn location_get_name(&self, location_id: &LocationPath) -> Result<String, SessionError>;
    fn location_set_name(&self, location_id: &LocationPath, name: &str)
        -> Result<(), SessionError>;

    fn location_get_desc(&self, location_id: &LocationPath) -> Result<String, SessionError>;
    fn location_set_desc(&self, location_id: &LocationPath, desc: &str)
        -> Result<(), SessionError>;

    fn location_get_path(&self, location_id: &LocationPath) -> Result<PathBuf, SessionError>;
    fn location_set_path(
        &self,
        location_id: &LocationPath,
        path: PathBuf,
    ) -> Result<(), SessionError>;

    fn location_get_where_is(
        &self,
        location_id: &LocationPath,
    ) -> Result<WhereIsLocation, SessionError>;
    fn location_set_where_is(
        &self,
        location_id: &LocationPath,
        where_is: WhereIsLocation,
    ) -> Result<(), SessionError>;

    fn location_get_should_save(&self, location_id: &LocationPath) -> Result<bool, SessionError>;
    fn location_set_should_save(
        &self,
        location_id: &LocationPath,
        should_save: bool,
    ) -> Result<(), SessionError>;

    fn location_get_elements_len(&self, location_id: &LocationPath) -> Result<usize, SessionError>;
    fn location_get_elements(
        &self,
        location_id: &LocationPath,
        range: Range<usize>,
    ) -> Result<Vec<ERef>, SessionError>;

    fn location_get_module(&self, location_id: &LocationPath)
        -> Result<Option<MRef>, SessionError>;
    fn location_set_module(
        &self,
        location_id: &LocationPath,
        module_id: Option<ModulePath>,
    ) -> Result<(), SessionError>;

    fn location_get_settings(&self, location_id: &LocationPath) -> Result<Values, SessionError>;
    fn location_set_settings(
        &self,
        location_id: &LocationPath,
        data: Values,
    ) -> Result<(), SessionError>;

    fn location_get_module_settings(
        &self,
        location_id: &LocationPath,
    ) -> Result<Values, SessionError>;
    fn location_set_module_settings(
        &self,
        location_id: &LocationPath,
        data: Values,
    ) -> Result<(), SessionError>;

    fn location_get_statuses(
        &self,
        location_id: &LocationPath,
    ) -> Result<Vec<String>, SessionError>;
    fn location_set_statuses(
        &self,
        location_id: &LocationPath,
        statuses: Vec<String>,
    ) -> Result<(), SessionError>;

    fn location_get_status(&self, location_id: &LocationPath) -> Result<usize, SessionError>;
    fn location_set_status(
        &self,
        location_id: &LocationPath,
        status: usize,
    ) -> Result<(), SessionError>;

    fn location_get_progress(&self, location_id: &LocationPath) -> Result<f32, SessionError>;
    fn location_set_progress(
        &self,
        location_id: &LocationPath,
        progress: f32,
    ) -> Result<(), SessionError>;

    fn location_is_enabled(&self, location_id: &LocationPath) -> Result<bool, SessionError>;
    fn location_set_enabled(
        &self,
        location_id: &LocationPath,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError>;

    fn location_is_error(&self, location_id: &LocationPath) -> Result<bool, SessionError>;

    fn location_get_location_info(
        &self,
        location_id: &LocationPath,
    ) -> Result<LocationInfo, SessionError>;
    fn location_notify(&self, location_id: &LocationPath, event: Event)
        -> Result<(), SessionError>;

    fn location_emit(&self, location_id: &LocationPath, event: Event) -> Result<(), SessionError>;
    fn location_subscribe(&self, location_id: &LocationPath, _ref: ID) -> Result<(), SessionError>;
    fn location_unsubscribe(
        &self,
        location_id: &LocationPath,
        _ref: ID,
    ) -> Result<(), SessionError>;

    //
    // End Location
    //

    fn get_module_ref(&self, id: &ModulePath) -> Result<MRef, SessionError>;
    fn get_element_ref(&self, id: &ElementPath) -> Result<ERef, SessionError>;
    fn get_location_ref(&self, id: &LocationPath) -> Result<LRef, SessionError>;

    //
    // Session
    //

    fn get_version(&self) -> Result<u64, SessionError>;
    fn get_version_text(&self) -> Result<String, SessionError>;

    fn c(&self) -> Box<dyn TSession>;
}
