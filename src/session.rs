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
    UnRegisteredElement,
    UnRegisteredLocation,
    UnRegisteredModule,
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
    IsNotModule,
    InvalidUID,
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

    fn load_module(&self, path: PathBuf) -> Result<ModuleId, SessionError>;
    fn remove_module(&self, id: UID) -> Result<MRow, SessionError>;
    fn load_module_info(&self, info: ModuleInfo) -> Result<ModuleId, SessionError>;
    fn find_module(&self, info: ModuleInfo) -> Result<ModuleId, SessionError>;

    fn register_action(
        &self,
        module_id: UID,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError>;
    fn remove_action(&self, module_id: UID, name: String) -> Result<(), SessionError>;
    fn get_actions(&self, range: Range<usize>) -> Result<Actions, SessionError>;
    fn get_actions_len(&self) -> Result<usize, SessionError>;
    fn run_action(&self, module_id: UID, name: String, data: Vec<Type>)
        -> Result<(), SessionError>;

    fn get_modules_len(&self) -> Result<usize, SessionError>;
    fn get_modules(&self, range: Range<usize>) -> Result<Vec<ModuleId>, SessionError>;

    fn module_get_name(&self, module_id: UID) -> Result<String, SessionError>;
    fn module_set_name(&self, module_id: UID, name: String) -> Result<(), SessionError>;
    fn module_get_default_name(&self, module_id: UID) -> Result<String, SessionError>;

    fn module_get_uid(&self, module_id: UID) -> Result<UID, SessionError>;
    fn module_get_version(&self, module_id: UID) -> Result<String, SessionError>;
    fn module_supported_versions(&self, module_id: UID) -> Result<Range<u64>, SessionError>;

    fn module_get_desc(&self, module_id: UID) -> Result<String, SessionError>;
    fn module_set_desc(&self, module_id: UID, desc: String) -> Result<(), SessionError>;
    fn module_get_default_desc(&self, module_id: UID) -> Result<String, SessionError>;

    fn module_get_proxy(&self, module_id: UID) -> Result<usize, SessionError>;
    fn module_set_proxy(&self, module_id: UID, proxy: usize) -> Result<(), SessionError>;

    fn module_get_settings(&self, module_id: UID) -> Result<Values, SessionError>;
    fn module_set_settings(&self, module_id: UID, data: Values) -> Result<(), SessionError>;

    fn module_get_element_settings(&self, module_id: UID) -> Result<Values, SessionError>;
    fn module_set_element_settings(&self, module_id: UID, data: Values)
        -> Result<(), SessionError>;

    fn module_get_location_settings(&self, module_id: UID) -> Result<Values, SessionError>;
    fn module_set_location_settings(
        &self,
        module_id: UID,
        data: Values,
    ) -> Result<(), SessionError>;

    fn module_init_location(&self, module_id: UID, location_id: UID) -> Result<(), SessionError>;

    fn module_init_element(&self, module_id: UID, element_id: UID) -> Result<(), SessionError>;

    fn module_accept_url(&self, module_id: UID, url: String) -> Result<bool, SessionError>;

    fn module_accept_extension(&self, module_id: UID, filename: &str)
        -> Result<bool, SessionError>;

    fn module_accepted_protocols(&self, module_id: UID) -> Result<Vec<String>, SessionError>;
    fn module_accepted_extensions(&self, module_id: UID) -> Result<Vec<String>, SessionError>;

    fn module_step_element(
        &self,
        module_id: UID,
        element_id: UID,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError>;

    fn module_step_location(
        &self,
        module_id: UID,
        location_id: UID,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError>;

    //
    // End Module
    //

    //
    // Element
    //

    fn create_element(&self, name: &str, location_id: UID) -> Result<ERef, SessionError>;
    fn load_element_info(&self, info: ElementInfo) -> Result<ERef, SessionError>;
    fn move_element(&self, element: UID, location_id: UID) -> Result<(), SessionError>;
    fn destroy_element(&self, element_id: UID) -> Result<ERow, SessionError>;

    fn element_get_name(&self, element_id: UID) -> Result<String, SessionError>;
    fn element_set_name(&self, element_id: UID, name: &str) -> Result<(), SessionError>;

    fn element_get_desc(&self, element_id: UID) -> Result<String, SessionError>;
    fn element_set_desc(&self, element_id: UID, desc: &str) -> Result<(), SessionError>;

    fn element_get_meta(&self, element_id: UID) -> Result<String, SessionError>;
    fn element_set_meta(&self, element_id: UID, meta: &str) -> Result<(), SessionError>;

    fn element_get_url(&self, element_id: UID) -> Result<Option<String>, SessionError>;
    fn element_set_url(&self, element_id: UID, url: Option<String>) -> Result<(), SessionError>;

    fn element_get_element_data(&self, element_id: UID) -> Result<Values, SessionError>;
    fn element_set_element_data(&self, element_id: UID, data: Values) -> Result<(), SessionError>;

    fn element_get_module_data(&self, element_id: UID) -> Result<Values, SessionError>;
    fn element_set_module_data(&self, element_id: UID, data: Values) -> Result<(), SessionError>;

    fn element_get_module(&self, element_id: UID) -> Result<Option<ModuleId>, SessionError>;
    fn element_set_module(&self, element: UID, module: Option<UID>) -> Result<(), SessionError>;

    fn element_get_statuses(&self, element_id: UID) -> Result<Vec<String>, SessionError>;
    fn element_set_statuses(&self, element: UID, statuses: Vec<String>)
        -> Result<(), SessionError>;

    fn element_get_status(&self, element_id: UID) -> Result<usize, SessionError>;
    fn element_set_status(&self, element_id: UID, status: usize) -> Result<(), SessionError>;

    fn element_get_data(&self, element_id: UID) -> Result<Data, SessionError>;
    fn element_set_data(&self, element_id: UID, data: Data) -> Result<(), SessionError>;

    fn element_get_progress(&self, element_id: UID) -> Result<f32, SessionError>;
    fn element_set_progress(&self, element_id: UID, progress: f32) -> Result<(), SessionError>;

    fn element_get_should_save(&self, element_id: UID) -> Result<bool, SessionError>;
    fn element_set_should_save(&self, element: UID, should_save: bool) -> Result<(), SessionError>;

    fn element_get_enabled(&self, element_id: UID) -> Result<bool, SessionError>;
    fn element_set_enabled(
        &self,
        element_id: UID,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError>;

    fn element_is_error(&self, element_id: UID) -> Result<bool, SessionError>;

    fn element_resolv_module(&self, element_id: UID) -> Result<bool, SessionError>;

    /// Blocking the current thread until is done!
    fn element_wait(&self, element_id: UID) -> Result<(), SessionError>;

    fn element_get_element_info(&self, element_id: UID) -> Result<ElementInfo, SessionError>;

    //
    // End Element
    //

    //
    // Location
    //

    fn create_location(&self, name: &str, location_id: UID) -> Result<LRef, SessionError>;
    fn load_location_info(&self, info: LocationInfo) -> Result<LRef, SessionError>;
    fn get_locations_len(&self, location_id: UID) -> Result<usize, SessionError>;
    fn get_locations(
        &self,
        location_id: UID,
        range: Range<usize>,
    ) -> Result<Vec<LRef>, SessionError>;
    fn destroy_location(&self, location_id: UID) -> Result<LRow, SessionError>;
    fn get_default_location(&self) -> Result<LocationId, SessionError>;
    fn move_location(&self, location_id: UID, to: UID) -> Result<(), SessionError>;

    fn location_get_name(&self, location_id: UID) -> Result<String, SessionError>;
    fn location_set_name(&self, location_id: UID, name: &str) -> Result<(), SessionError>;

    fn location_get_desc(&self, location_id: UID) -> Result<String, SessionError>;
    fn location_set_desc(&self, location_id: UID, desc: &str) -> Result<(), SessionError>;

    fn location_get_path(&self, location_id: UID) -> Result<PathBuf, SessionError>;
    fn location_set_path(&self, location_id: UID, path: PathBuf) -> Result<(), SessionError>;

    fn location_get_where_is(&self, location_id: UID) -> Result<WhereIsLocation, SessionError>;
    fn location_set_where_is(
        &self,
        location_id: UID,
        where_is: WhereIsLocation,
    ) -> Result<(), SessionError>;

    fn location_get_should_save(&self, location_id: UID) -> Result<bool, SessionError>;
    fn location_set_should_save(
        &self,
        location_id: UID,
        should_save: bool,
    ) -> Result<(), SessionError>;

    fn location_get_elements_len(&self, location_id: UID) -> Result<usize, SessionError>;
    fn location_get_elements(
        &self,
        location_id: UID,
        range: Range<usize>,
    ) -> Result<Vec<ERef>, SessionError>;

    fn location_get_module(&self, location_id: UID) -> Result<Option<ModuleId>, SessionError>;
    fn location_set_module(
        &self,
        location_id: UID,
        module_id: Option<UID>,
    ) -> Result<(), SessionError>;

    fn location_get_settings(&self, location_id: UID) -> Result<Values, SessionError>;
    fn location_set_settings(&self, location_id: UID, data: Values) -> Result<(), SessionError>;

    fn location_get_module_settings(&self, location_id: UID) -> Result<Values, SessionError>;
    fn location_set_module_settings(
        &self,
        location_id: UID,
        data: Values,
    ) -> Result<(), SessionError>;

    fn location_get_statuses(&self, location_id: UID) -> Result<Vec<String>, SessionError>;
    fn location_set_statuses(
        &self,
        location_id: UID,
        statuses: Vec<String>,
    ) -> Result<(), SessionError>;

    fn location_get_status(&self, location_id: UID) -> Result<usize, SessionError>;
    fn location_set_status(&self, location_id: UID, status: usize) -> Result<(), SessionError>;

    fn location_get_progress(&self, location_id: UID) -> Result<f32, SessionError>;
    fn location_set_progress(&self, location_id: UID, progress: f32) -> Result<(), SessionError>;

    fn location_is_enabled(&self, location_id: UID) -> Result<bool, SessionError>;
    fn location_set_enabled(
        &self,
        location_id: UID,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError>;

    fn location_is_error(&self, location_id: UID) -> Result<bool, SessionError>;

    fn location_get_location_info(&self, location_id: UID) -> Result<LocationInfo, SessionError>;

    fn notify(&self, uid: UID, event: Event) -> Result<(), SessionError>;
    fn emit(&self, who_emited: UID, event: Event) -> Result<(), SessionError>;
    fn subscribe(&self, uid: UID, to: UID) -> Result<(), SessionError>;
    fn unsubscribe(&self, uid: UID, to: UID) -> Result<(), SessionError>;

    //
    // End Location
    //

    fn get_module_id(&self, uid: UID) -> Result<ModuleId, SessionError>;
    fn get_element_id(&self, uid: UID) -> Result<ElementId, SessionError>;
    fn get_location_id(&self, uid: UID) -> Result<LocationId, SessionError>;
    fn get_id(&self, uid: UID) -> Result<ID, SessionError>;

    fn get_ref(&self, uid: UID) -> Result<Ref, SessionError>;

    fn get_module_from_path(&self, path: ModulePath) -> Result<ModuleId, SessionError>;
    fn get_element_from_path(&self, path: ElementPath) -> Result<ElementId, SessionError>;
    fn get_location_from_path(&self, path: LocationPath) -> Result<LocationId, SessionError>;

    //
    // Session
    //

    fn get_version(&self) -> Result<u64, SessionError>;
    fn get_version_text(&self) -> Result<String, SessionError>;

    fn c(&self) -> Box<dyn TSession>;
}
