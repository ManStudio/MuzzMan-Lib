use std::ops::Range;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

use bytes_kman::TBytes;

#[derive(Clone, Debug, Serialize, Deserialize, bytes_kman::Bytes)]
pub enum SessionError {
    InvalidSession,
    ElementDoNotExist,
    InsufficientPermissions,
    InvalidLocation,
    ServerTimeOut,
    CannotConnectToServer,
    ServerInvalidIndentification,
    InvalidElementStatus,
    LocationAllreadyExist,
    InvalidModule,
    CannotInstallModule(String),
    AlreadySubscribed,
    AlreadyUnsubscribed,
    IsNotElement,
    IsNotLocation,
    RawModule(RawLibraryError),
    Custom(String),
}

pub type Actions = Vec<(String, MRef, Vec<(String, Value)>)>;

// TODO: all the functions should be async
// because is possible to be performed on the network
// or should have a asnyc version
// like TSessionAsync
pub trait TSession {
    //
    // Module
    //

    fn load_module(&self, path: PathBuf) -> Result<MRef, SessionError>;
    fn remove_module(&self, id: ModuleId) -> Result<MRow, SessionError>;

    fn register_action(
        &self,
        module_id: &ModuleId,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError>;
    fn remove_action(&self, module_id: &ModuleId, name: String) -> Result<(), SessionError>;
    fn get_actions(&self, range: Range<usize>) -> Result<Actions, SessionError>;
    fn get_actions_len(&self) -> Result<usize, SessionError>;
    fn run_action(
        &self,
        module_id: &ModuleId,
        name: String,
        data: Vec<Type>,
    ) -> Result<(), SessionError>;

    fn get_modules_len(&self) -> Result<usize, SessionError>;
    fn get_modules(&self, range: Range<usize>) -> Result<Vec<MRef>, SessionError>;

    fn module_get_name(&self, module_id: &ModuleId) -> Result<String, SessionError>;
    fn module_set_name(&self, module_id: &ModuleId, name: String) -> Result<(), SessionError>;
    fn module_get_default_name(&self, module_id: &ModuleId) -> Result<String, SessionError>;

    fn module_get_desc(&self, module_id: &ModuleId) -> Result<String, SessionError>;
    fn module_set_desc(&self, module_id: &ModuleId, desc: String) -> Result<(), SessionError>;
    fn module_get_default_desc(&self, module_id: &ModuleId) -> Result<String, SessionError>;

    fn module_get_proxy(&self, module_id: &ModuleId) -> Result<usize, SessionError>;
    fn module_set_proxy(&self, module_id: &ModuleId, proxy: usize) -> Result<(), SessionError>;

    fn module_get_settings(&self, module_id: &ModuleId) -> Result<Data, SessionError>;
    fn module_set_settings(&self, module_id: &ModuleId, data: Data) -> Result<(), SessionError>;

    fn module_get_element_settings(&self, module_id: &ModuleId) -> Result<Data, SessionError>;
    fn module_set_element_settings(
        &self,
        module_id: &ModuleId,
        data: Data,
    ) -> Result<(), SessionError>;

    fn module_init_location(
        &self,
        module_id: &ModuleId,
        location_id: &LocationId,
        data: FileOrData,
    ) -> Result<(), SessionError>;

    fn module_init_element(
        &self,
        module_id: &ModuleId,
        element_id: &ElementId,
    ) -> Result<(), SessionError>;

    fn module_accept_url(&self, module_id: &ModuleId, url: String) -> Result<bool, SessionError>;

    fn module_accept_extension(
        &self,
        module_id: &ModuleId,
        filename: &str,
    ) -> Result<bool, SessionError>;

    fn module_accepted_protocols(&self, module_id: &ModuleId) -> Result<Vec<String>, SessionError>;

    fn module_step_element(
        &self,
        module_id: &ModuleId,
        element_id: &ElementId,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError>;

    fn module_step_location(
        &self,
        module_id: &ModuleId,
        location_id: &LocationId,
        control_flow: ControlFlow,
        storage: Storage,
    ) -> Result<(ControlFlow, Storage), SessionError>;

    //
    // End Module
    //

    //
    // Element
    //

    fn create_element(&self, name: &str, location_id: &LocationId) -> Result<ERef, SessionError>;
    fn move_element(
        &self,
        element: &ElementId,
        location_id: &LocationId,
    ) -> Result<(), SessionError>;
    fn destroy_element(&self, element_id: ElementId) -> Result<ERow, SessionError>;

    fn element_get_name(&self, element_id: &ElementId) -> Result<String, SessionError>;
    fn element_set_name(&self, element_id: &ElementId, name: &str) -> Result<(), SessionError>;

    fn element_get_desc(&self, element_id: &ElementId) -> Result<String, SessionError>;
    fn element_set_desc(&self, element_id: &ElementId, desc: &str) -> Result<(), SessionError>;

    fn element_get_meta(&self, element_id: &ElementId) -> Result<String, SessionError>;
    fn element_set_meta(&self, element_id: &ElementId, meta: &str) -> Result<(), SessionError>;

    fn element_get_url(&self, element_id: &ElementId) -> Result<Option<String>, SessionError>;
    fn element_set_url(
        &self,
        element_id: &ElementId,
        url: Option<String>,
    ) -> Result<(), SessionError>;

    fn element_get_element_data(&self, element_id: &ElementId) -> Result<Data, SessionError>;
    fn element_set_element_data(
        &self,
        element_id: &ElementId,
        data: Data,
    ) -> Result<(), SessionError>;

    fn element_get_module_data(&self, element_id: &ElementId) -> Result<Data, SessionError>;
    fn element_set_module_data(
        &self,
        element_id: &ElementId,
        data: Data,
    ) -> Result<(), SessionError>;

    fn element_get_module(&self, element_id: &ElementId) -> Result<Option<MRef>, SessionError>;
    fn element_set_module(
        &self,
        element: &ElementId,
        module: Option<ModuleId>,
    ) -> Result<(), SessionError>;

    fn element_get_statuses(&self, element_id: &ElementId) -> Result<Vec<String>, SessionError>;
    fn element_set_statuses(
        &self,
        element: &ElementId,
        statuses: Vec<String>,
    ) -> Result<(), SessionError>;

    fn element_get_status(&self, element_id: &ElementId) -> Result<usize, SessionError>;
    fn element_set_status(&self, element_id: &ElementId, status: usize)
        -> Result<(), SessionError>;

    fn element_get_data(&self, element_id: &ElementId) -> Result<FileOrData, SessionError>;
    fn element_set_data(
        &self,
        element_id: &ElementId,
        data: FileOrData,
    ) -> Result<(), SessionError>;

    fn element_get_progress(&self, element_id: &ElementId) -> Result<f32, SessionError>;
    fn element_set_progress(
        &self,
        element_id: &ElementId,
        progress: f32,
    ) -> Result<(), SessionError>;

    fn element_get_should_save(&self, element_id: &ElementId) -> Result<bool, SessionError>;
    fn element_set_should_save(
        &self,
        element: &ElementId,
        should_save: bool,
    ) -> Result<(), SessionError>;

    fn element_get_enabled(&self, element_id: &ElementId) -> Result<bool, SessionError>;
    fn element_set_enabled(
        &self,
        element_id: &ElementId,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError>;

    fn element_resolv_module(&self, element_id: &ElementId) -> Result<bool, SessionError>;

    /// Blocking the current thread until is done!
    fn element_wait(&self, element_id: &ElementId) -> Result<(), SessionError>;

    fn element_get_element_info(&self, element_id: &ElementId)
        -> Result<ElementInfo, SessionError>;
    fn element_notify(&self, element_id: &ElementId, event: Event) -> Result<(), SessionError>;

    fn element_emit(&self, element_id: &ElementId, event: Event) -> Result<(), SessionError>;
    fn element_subscribe(&self, element_id: &ElementId, _ref: ID) -> Result<(), SessionError>;
    fn element_unsubscribe(&self, element_id: &ElementId, _ref: ID) -> Result<(), SessionError>;

    //
    // End Element
    //

    //
    // Location
    //

    fn create_location(&self, name: &str, location_id: &LocationId) -> Result<LRef, SessionError>;
    fn get_locations_len(&self, location_id: &LocationId) -> Result<usize, SessionError>;
    fn get_locations(
        &self,
        location_id: &LocationId,
        range: Range<usize>,
    ) -> Result<Vec<LRef>, SessionError>;
    fn destroy_location(&self, location_id: LocationId) -> Result<LRow, SessionError>;
    fn get_default_location(&self) -> Result<LRef, SessionError>;
    fn move_location(&self, location_id: &LocationId, to: &LocationId) -> Result<(), SessionError>;

    fn location_get_name(&self, location_id: &LocationId) -> Result<String, SessionError>;
    fn location_set_name(&self, location_id: &LocationId, name: &str) -> Result<(), SessionError>;

    fn location_get_desc(&self, location_id: &LocationId) -> Result<String, SessionError>;
    fn location_set_desc(&self, location_id: &LocationId, desc: &str) -> Result<(), SessionError>;

    fn location_get_path(&self, location_id: &LocationId) -> Result<PathBuf, SessionError>;
    fn location_set_path(
        &self,
        location_id: &LocationId,
        path: PathBuf,
    ) -> Result<(), SessionError>;

    fn location_get_where_is(
        &self,
        location_id: &LocationId,
    ) -> Result<WhereIsLocation, SessionError>;
    fn location_set_where_is(
        &self,
        location_id: &LocationId,
        where_is: WhereIsLocation,
    ) -> Result<(), SessionError>;

    fn location_get_should_save(&self, location_id: &LocationId) -> Result<bool, SessionError>;
    fn location_set_should_save(
        &self,
        location_id: &LocationId,
        should_save: bool,
    ) -> Result<(), SessionError>;

    fn location_get_elements_len(&self, location_id: &LocationId) -> Result<usize, SessionError>;
    fn location_get_elements(
        &self,
        location_id: &LocationId,
        range: Range<usize>,
    ) -> Result<Vec<ERef>, SessionError>;

    fn location_get_location_info(
        &self,
        location_id: &LocationId,
    ) -> Result<LocationInfo, SessionError>;
    fn location_notify(&self, location_id: &LocationId, event: Event) -> Result<(), SessionError>;

    fn location_emit(&self, location_id: &LocationId, event: Event) -> Result<(), SessionError>;
    fn location_subscribe(&self, location_id: &LocationId, _ref: ID) -> Result<(), SessionError>;
    fn location_unsubscribe(&self, location_id: &LocationId, _ref: ID) -> Result<(), SessionError>;

    //
    // End Location
    //

    fn get_module_ref(&self, id: &ModuleId) -> Result<MRef, SessionError>;
    fn get_element_ref(&self, id: &ElementId) -> Result<ERef, SessionError>;
    fn get_location_ref(&self, id: &LocationId) -> Result<LRef, SessionError>;

    fn c(&self) -> Box<dyn TSession>;
}
