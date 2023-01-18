use std::ops::Range;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SessionError {
    InvalidSession,
    ElementDoNotExist,
    InsufficientPermissions,
    InvalidLocation,
    ServerTimeOut(RefLocation),
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
    Custom(String),
}

pub type Actions = Vec<(String, MRef, Vec<(String, Value)>)>;

pub trait TSession {
    //
    // Module
    //

    fn add_module(&self, module: Box<dyn TModule>) -> Result<MRef, SessionError>;
    fn remove_module(&self, info: ModuleId) -> Result<MRow, SessionError>;

    fn register_action(
        &self,
        module: &ModuleId,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError>;
    fn remove_action(&self, owner: &ModuleId, name: String) -> Result<(), SessionError>;
    fn get_actions(&self, range: Range<usize>) -> Result<Actions, SessionError>;
    fn get_actions_len(&self) -> Result<usize, SessionError>;
    fn run_action(
        &self,
        owner: &ModuleId,
        name: String,
        data: Vec<Type>,
    ) -> Result<(), SessionError>;

    fn get_modules_len(&self) -> Result<usize, SessionError>;
    fn get_modules(&self, range: Range<usize>) -> Result<Vec<MRef>, SessionError>;

    fn get_module_name(&self, info: &ModuleId) -> Result<String, SessionError>;
    fn set_module_name(&self, info: &ModuleId, name: String) -> Result<(), SessionError>;
    fn default_module_name(&self, info: &ModuleId) -> Result<(), SessionError>;

    fn get_module_desc(&self, info: &ModuleId) -> Result<String, SessionError>;
    fn set_module_desc(&self, info: &ModuleId, desc: String) -> Result<(), SessionError>;
    fn default_module_desc(&self, info: &ModuleId) -> Result<(), SessionError>;

    fn get_module_proxy(&self, info: &ModuleId) -> Result<usize, SessionError>;
    fn set_module_proxy(&self, info: &ModuleId, proxy: usize) -> Result<(), SessionError>;

    fn get_module_settings(&self, module_info: &ModuleId) -> Result<Data, SessionError>;
    fn set_module_settings(&self, module_info: &ModuleId, data: Data) -> Result<(), SessionError>;

    fn get_module_element_settings(&self, module_info: &ModuleId) -> Result<Data, SessionError>;
    fn set_module_element_settings(
        &self,
        module_info: &ModuleId,
        data: Data,
    ) -> Result<(), SessionError>;

    fn module_init_location(
        &self,
        module_info: &ModuleId,
        location_info: &LocationId,
        data: FileOrData,
    ) -> Result<(), SessionError>;

    fn module_init_element(
        &self,
        module_info: &ModuleId,
        element_info: &ElementId,
    ) -> Result<(), SessionError>;

    fn moduie_accept_url(&self, module_info: &ModuleId, url: Url) -> Result<bool, SessionError>;

    fn module_accept_extension(
        &self,
        module_info: &ModuleId,
        filename: &str,
    ) -> Result<bool, SessionError>;

    fn module_step_element(
        &self,
        module_info: &ModuleId,
        element_info: &ElementId,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError>;

    fn module_step_location(
        &self,
        module_info: &ModuleId,
        location_info: &LocationId,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError>;

    //
    // End Module
    //

    //
    // Element
    //

    fn create_element(&self, name: &str, location: &LocationId) -> Result<ERef, SessionError>;
    fn move_element(&self, element: &ElementId, location: &LocationId) -> Result<(), SessionError>;
    fn destroy_element(&self, element: ElementId) -> Result<ERow, SessionError>;

    fn element_get_name(&self, element: &ElementId) -> Result<String, SessionError>;
    fn element_set_name(&self, element: &ElementId, name: &str) -> Result<(), SessionError>;

    fn element_get_desc(&self, element: &ElementId) -> Result<String, SessionError>;
    fn element_set_desc(&self, element: &ElementId, desc: &str) -> Result<(), SessionError>;

    fn element_get_meta(&self, element: &ElementId) -> Result<String, SessionError>;
    fn element_set_meta(&self, element: &ElementId, meta: &str) -> Result<(), SessionError>;

    fn element_get_element_data(&self, element: &ElementId) -> Result<Data, SessionError>;
    fn element_set_element_data(&self, element: &ElementId, data: Data)
        -> Result<(), SessionError>;

    fn element_get_module_data(&self, element: &ElementId) -> Result<Data, SessionError>;
    fn element_set_module_data(&self, element: &ElementId, data: Data) -> Result<(), SessionError>;

    fn element_get_module(&self, element: &ElementId) -> Result<Option<MRef>, SessionError>;
    fn element_set_module(
        &self,
        element: &ElementId,
        module: Option<MRef>,
    ) -> Result<(), SessionError>;

    fn element_get_statuses(&self, element: &ElementId) -> Result<Vec<String>, SessionError>;
    fn element_set_statuses(
        &self,
        element: &ElementId,
        statuses: Vec<String>,
    ) -> Result<(), SessionError>;

    fn element_get_status(&self, element: &ElementId) -> Result<usize, SessionError>;
    fn element_set_status(&self, element: &ElementId, status: usize) -> Result<(), SessionError>;

    fn element_get_data(&self, element: &ElementId) -> Result<FileOrData, SessionError>;
    fn element_set_data(&self, element: &ElementId, data: FileOrData) -> Result<(), SessionError>;

    fn element_get_progress(&self, element: &ElementId) -> Result<f32, SessionError>;
    fn element_set_progress(&self, element: &ElementId, progress: f32) -> Result<(), SessionError>;

    fn element_get_should_save(&self, element: &ElementId) -> Result<bool, SessionError>;
    fn element_set_should_save(
        &self,
        element: &ElementId,
        should_save: bool,
    ) -> Result<(), SessionError>;

    fn element_get_enabled(&self, element: &ElementId) -> Result<bool, SessionError>;
    fn element_set_enabled(
        &self,
        element: &ElementId,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError>;

    fn element_resolv_module(&self, element_info: &ElementId) -> Result<bool, SessionError>;

    /// Blocking the current thread until is done!
    fn element_wait(&self, element: &ElementId) -> Result<(), SessionError>;

    fn element_get_element_info(&self, element: &ElementId) -> Result<ElementInfo, SessionError>;
    fn element_notify(&self, element: &ElementId, event: Event) -> Result<(), SessionError>;

    fn element_emit(&self, element: &ElementId, event: Event) -> Result<(), SessionError>;
    fn element_subscribe(&self, element: &ElementId, _ref: Ref) -> Result<(), SessionError>;
    fn element_unsubscribe(&self, element: &ElementId, _ref: Ref) -> Result<(), SessionError>;

    //
    // End Element
    //

    //
    // Location
    //

    fn create_location(&self, name: &str, location: &LocationId) -> Result<LRef, SessionError>;
    fn get_locations_len(&self, location: &LocationId) -> Result<usize, SessionError>;
    fn get_locations(
        &self,
        location: &LocationId,
        range: Range<usize>,
    ) -> Result<Vec<LRef>, SessionError>;
    fn destroy_location(&self, location: LocationId) -> Result<LRow, SessionError>;
    fn get_default_location(&self) -> Result<LRef, SessionError>;
    fn move_location(&self, location: &LocationId, to: &LocationId) -> Result<(), SessionError>;

    fn location_get_name(&self, location: &LocationId) -> Result<String, SessionError>;
    fn location_set_name(&self, location: &LocationId, name: &str) -> Result<(), SessionError>;

    fn location_get_desc(&self, location: &LocationId) -> Result<String, SessionError>;
    fn location_set_desc(&self, location: &LocationId, desc: &str) -> Result<(), SessionError>;

    fn location_get_path(&self, location: &LocationId) -> Result<PathBuf, SessionError>;
    fn location_set_path(&self, location: &LocationId, path: PathBuf) -> Result<(), SessionError>;

    fn location_get_where_is(&self, location: &LocationId)
        -> Result<WhereIsLocation, SessionError>;
    fn location_set_where_is(
        &self,
        location: &LocationId,
        where_is: WhereIsLocation,
    ) -> Result<(), SessionError>;

    fn location_get_should_save(&self, location: &LocationId) -> Result<bool, SessionError>;
    fn location_set_should_save(
        &self,
        location: &LocationId,
        should_save: bool,
    ) -> Result<(), SessionError>;

    fn location_get_elements_len(&self, location: &LocationId) -> Result<usize, SessionError>;
    fn location_get_elements(
        &self,
        location: &LocationId,
        range: Range<usize>,
    ) -> Result<Vec<ERef>, SessionError>;

    fn location_get_location_info(
        &self,
        location: &LocationId,
    ) -> Result<LocationInfo, SessionError>;
    fn location_notify(&self, location: &LocationId, event: Event) -> Result<(), SessionError>;

    fn location_emit(&self, location: &LocationId, event: Event) -> Result<(), SessionError>;
    fn location_subscribe(&self, location: &LocationId, _ref: Ref) -> Result<(), SessionError>;
    fn location_unsubscribe(&self, location: &LocationId, _ref: Ref) -> Result<(), SessionError>;

    //
    // End Location
    //

    fn c(&self) -> Box<dyn TSession>;
}
