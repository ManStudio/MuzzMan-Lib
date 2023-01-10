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
    fn remove_module(&self, info: MRef) -> Result<MRow, SessionError>;

    fn register_action(
        &self,
        module: &MRef,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MRef, values: Vec<Type>),
    ) -> Result<(), SessionError>;
    fn remove_action(&self, owner: &MRef, name: String) -> Result<(), SessionError>;
    fn get_actions(&self, range: Range<usize>) -> Result<Actions, SessionError>;
    fn get_actions_len(&self) -> Result<usize, SessionError>;
    fn run_action(&self, owner: MRef, name: String, data: Vec<Type>) -> Result<(), SessionError>;

    fn get_modules_len(&self) -> Result<usize, SessionError>;
    fn get_modules(&self, range: Range<usize>) -> Result<Vec<MRef>, SessionError>;

    fn get_module_name(&self, info: &MRef) -> Result<String, SessionError>;
    fn set_module_name(&self, info: &MRef, name: String) -> Result<(), SessionError>;
    fn default_module_name(&self, info: &MRef) -> Result<(), SessionError>;

    fn get_module_desc(&self, info: &MRef) -> Result<String, SessionError>;
    fn set_module_desc(&self, info: &MRef, desc: String) -> Result<(), SessionError>;
    fn default_module_desc(&self, info: &MRef) -> Result<(), SessionError>;

    fn get_module_proxy(&self, info: &MRef) -> Result<usize, SessionError>;
    fn set_module_proxy(&self, info: &MRef, proxy: usize) -> Result<(), SessionError>;

    fn get_module_settings(&self, module_info: &MRef) -> Result<Data, SessionError>;
    fn set_module_settings(&self, module_info: &MRef, data: Data) -> Result<(), SessionError>;

    fn get_module_element_settings(&self, module_info: &MRef) -> Result<Data, SessionError>;
    fn set_module_element_settings(
        &self,
        module_info: &MRef,
        data: Data,
    ) -> Result<(), SessionError>;

    fn module_init_location(
        &self,
        module_info: &MRef,
        location_info: &LRef,
        data: FileOrData,
    ) -> Result<(), SessionError>;

    fn module_init_element(
        &self,
        module_info: &MRef,
        element_info: &ERef,
    ) -> Result<(), SessionError>;

    fn moduie_accept_url(&self, module_info: &MRef, url: Url) -> Result<bool, SessionError>;

    fn module_accept_extension(
        &self,
        module_info: &MRef,
        filename: &str,
    ) -> Result<bool, SessionError>;

    fn module_step_element(
        &self,
        module_info: &MRef,
        element_info: &ERef,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError>;

    fn module_step_location(
        &self,
        module_info: &MRef,
        location_info: &LRef,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError>;

    //
    // End Module
    //

    //
    // Element
    //

    fn create_element(&self, name: &str, location: &LRef) -> Result<ERef, SessionError>;
    fn move_element(&self, element: &ERef, location: &LRef) -> Result<(), SessionError>;
    fn destroy_element(&self, element: ERef) -> Result<ERow, SessionError>;

    fn element_get_name(&self, element: &ERef) -> Result<String, SessionError>;
    fn element_set_name(&self, element: &ERef, name: &str) -> Result<(), SessionError>;

    fn element_get_desc(&self, element: &ERef) -> Result<String, SessionError>;
    fn element_set_desc(&self, element: &ERef, desc: &str) -> Result<(), SessionError>;

    fn element_get_meta(&self, element: &ERef) -> Result<String, SessionError>;
    fn element_set_meta(&self, element: &ERef, meta: &str) -> Result<(), SessionError>;

    fn element_get_element_data(&self, element: &ERef) -> Result<Data, SessionError>;
    fn element_set_element_data(&self, element: &ERef, data: Data) -> Result<(), SessionError>;

    fn element_get_module_data(&self, element: &ERef) -> Result<Data, SessionError>;
    fn element_set_module_data(&self, element: &ERef, data: Data) -> Result<(), SessionError>;

    fn element_get_module(&self, element: &ERef) -> Result<Option<MRef>, SessionError>;
    fn element_set_module(&self, element: &ERef, module: Option<MRef>) -> Result<(), SessionError>;

    fn element_get_statuses(&self, element: &ERef) -> Result<Vec<String>, SessionError>;
    fn element_set_statuses(
        &self,
        element: &ERef,
        statuses: Vec<String>,
    ) -> Result<(), SessionError>;

    fn element_get_status(&self, element: &ERef) -> Result<usize, SessionError>;
    fn element_set_status(&self, element: &ERef, status: usize) -> Result<(), SessionError>;

    fn element_get_data(&self, element: &ERef) -> Result<FileOrData, SessionError>;
    fn element_set_data(&self, element: &ERef, data: FileOrData) -> Result<(), SessionError>;

    fn element_get_progress(&self, element: &ERef) -> Result<f32, SessionError>;
    fn element_set_progress(&self, element: &ERef, progress: f32) -> Result<(), SessionError>;

    fn element_get_should_save(&self, element: &ERef) -> Result<bool, SessionError>;
    fn element_set_should_save(
        &self,
        element: &ERef,
        should_save: bool,
    ) -> Result<(), SessionError>;

    fn element_get_enabled(&self, element: &ERef) -> Result<bool, SessionError>;
    fn element_set_enabled(
        &self,
        element: &ERef,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError>;

    fn element_resolv_module(&self, element_info: &ERef) -> Result<bool, SessionError>;

    /// Blocking the current thread until is done!
    fn element_wait(&self, element: &ERef) -> Result<(), SessionError>;

    fn element_get_element_info(&self, element: &ERef) -> Result<ElementInfo, SessionError>;
    fn element_notify(&self, element: &ERef, event: Event) -> Result<(), SessionError>;

    fn element_emit(&self, element: &ERef, event: Event) -> Result<(), SessionError>;
    fn element_subscribe(&self, element: &ERef, _ref: Ref) -> Result<(), SessionError>;
    fn element_unsubscribe(&self, element: &ERef, _ref: Ref) -> Result<(), SessionError>;

    //
    // End Element
    //

    //
    // Location
    //

    fn create_location(&self, name: &str, location: &LRef) -> Result<LRef, SessionError>;
    fn get_locations_len(&self, location: &LRef) -> Result<usize, SessionError>;
    fn get_locations(
        &self,
        location: &LRef,
        range: Range<usize>,
    ) -> Result<Vec<LRef>, SessionError>;
    fn destroy_location(&self, location: LRef) -> Result<LRow, SessionError>;
    fn get_default_location(&self) -> Result<LRef, SessionError>;
    fn move_location(&self, location: &LRef, to: &LRef) -> Result<(), SessionError>;

    fn location_get_name(&self, location: &LRef) -> Result<String, SessionError>;
    fn location_set_name(&self, location: &LRef, name: &str) -> Result<(), SessionError>;

    fn location_get_desc(&self, location: &LRef) -> Result<String, SessionError>;
    fn location_set_desc(&self, location: &LRef, desc: &str) -> Result<(), SessionError>;

    fn location_get_path(&self, location: &LRef) -> Result<PathBuf, SessionError>;
    fn location_set_path(&self, location: &LRef, path: PathBuf) -> Result<(), SessionError>;

    fn location_get_where_is(&self, location: &LRef) -> Result<WhereIsLocation, SessionError>;
    fn location_set_where_is(
        &self,
        location: &LRef,
        where_is: WhereIsLocation,
    ) -> Result<(), SessionError>;

    fn location_get_should_save(&self, location: &LRef) -> Result<bool, SessionError>;
    fn location_set_should_save(
        &self,
        location: &LRef,
        should_save: bool,
    ) -> Result<(), SessionError>;

    fn location_get_elements_len(&self, location: &LRef) -> Result<usize, SessionError>;
    fn location_get_elements(
        &self,
        location: &LRef,
        range: Range<usize>,
    ) -> Result<Vec<ERef>, SessionError>;

    fn location_get_location_info(&self, location: &LRef) -> Result<LocationInfo, SessionError>;
    fn location_notify(&self, location: &LRef, event: Event) -> Result<(), SessionError>;

    fn location_emit(&self, location: &LRef, event: Event) -> Result<(), SessionError>;
    fn location_subscribe(&self, location: &LRef, _ref: Ref) -> Result<(), SessionError>;
    fn location_unsubscribe(&self, location: &LRef, _ref: Ref) -> Result<(), SessionError>;

    //
    // End Location
    //

    fn c(&self) -> Box<dyn TSession>;
}
