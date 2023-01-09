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
    Custom(String),
}

pub type Actions = Vec<(String, MInfo, Vec<(String, Value)>)>;

pub trait TSession {
    //
    // Module
    //

    fn add_module(&self, module: Box<dyn TModule>) -> Result<MInfo, SessionError>;
    fn remove_module(&self, info: MInfo) -> Result<MRow, SessionError>;

    fn register_action(
        &self,
        module: &MInfo,
        name: String,
        values: Vec<(String, Value)>,
        callback: fn(MInfo, values: Vec<Type>),
    ) -> Result<(), SessionError>;
    fn remove_action(&self, owner: &MInfo, name: String) -> Result<(), SessionError>;
    fn get_actions(&self, range: Range<usize>) -> Result<Actions, SessionError>;
    fn get_actions_len(&self) -> Result<usize, SessionError>;
    fn run_action(&self, owner: MInfo, name: String, data: Vec<Type>) -> Result<(), SessionError>;

    fn get_modules_len(&self) -> Result<usize, SessionError>;
    fn get_modules(&self, range: Range<usize>) -> Result<Vec<MInfo>, SessionError>;

    fn get_module_name(&self, info: &MInfo) -> Result<String, SessionError>;
    fn set_module_name(&self, info: &MInfo, name: String) -> Result<(), SessionError>;
    fn default_module_name(&self, info: &MInfo) -> Result<(), SessionError>;

    fn get_module_desc(&self, info: &MInfo) -> Result<String, SessionError>;
    fn set_module_desc(&self, info: &MInfo, desc: String) -> Result<(), SessionError>;
    fn default_module_desc(&self, info: &MInfo) -> Result<(), SessionError>;

    fn get_module_proxy(&self, info: &MInfo) -> Result<usize, SessionError>;
    fn set_module_proxy(&self, info: &MInfo, proxy: usize) -> Result<(), SessionError>;

    fn get_module_settings(&self, module_info: &MInfo) -> Result<Data, SessionError>;
    fn set_module_settings(&self, module_info: &MInfo, data: Data) -> Result<(), SessionError>;

    fn get_module_element_settings(&self, module_info: &MInfo) -> Result<Data, SessionError>;
    fn set_module_element_settings(
        &self,
        module_info: &MInfo,
        data: Data,
    ) -> Result<(), SessionError>;

    fn module_init_location(
        &self,
        module_info: &MInfo,
        location_info: &LInfo,
        data: FileOrData,
    ) -> Result<(), SessionError>;

    fn module_init_element(
        &self,
        module_info: &MInfo,
        element_info: &EInfo,
    ) -> Result<(), SessionError>;

    fn moduie_accept_url(&self, module_info: &MInfo, url: Url) -> Result<bool, SessionError>;

    fn module_accept_extension(
        &self,
        module_info: &MInfo,
        filename: &str,
    ) -> Result<bool, SessionError>;

    fn module_step_element(
        &self,
        module_info: &MInfo,
        element_info: &EInfo,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError>;

    fn module_step_location(
        &self,
        module_info: &MInfo,
        location_info: &LInfo,
    ) -> Result<(), SessionError>;

    //
    // End Module
    //

    //
    // Element
    //

    fn create_element(&self, name: &str, location: &LInfo) -> Result<EInfo, SessionError>;
    fn move_element(&self, element: &EInfo, location: &LInfo) -> Result<(), SessionError>;
    fn destroy_element(&self, element: EInfo) -> Result<ERow, SessionError>;

    fn element_get_name(&self, element: &EInfo) -> Result<String, SessionError>;
    fn element_set_name(&self, element: &EInfo, name: &str) -> Result<(), SessionError>;

    fn element_get_desc(&self, element: &EInfo) -> Result<String, SessionError>;
    fn element_set_desc(&self, element: &EInfo, desc: &str) -> Result<(), SessionError>;

    fn element_get_meta(&self, element: &EInfo) -> Result<String, SessionError>;
    fn element_set_meta(&self, element: &EInfo, meta: &str) -> Result<(), SessionError>;

    fn element_get_element_data(&self, element: &EInfo) -> Result<Data, SessionError>;
    fn element_set_element_data(&self, element: &EInfo, data: Data) -> Result<(), SessionError>;

    fn element_get_module_data(&self, element: &EInfo) -> Result<Data, SessionError>;
    fn element_set_module_data(&self, element: &EInfo, data: Data) -> Result<(), SessionError>;

    fn element_get_module(&self, element: &EInfo) -> Result<Option<MInfo>, SessionError>;
    fn element_set_module(
        &self,
        element: &EInfo,
        module: Option<MInfo>,
    ) -> Result<(), SessionError>;

    fn element_get_statuses(&self, element: &EInfo) -> Result<Vec<String>, SessionError>;
    fn element_set_statuses(
        &self,
        element: &EInfo,
        statuses: Vec<String>,
    ) -> Result<(), SessionError>;

    fn element_get_status(&self, element: &EInfo) -> Result<usize, SessionError>;
    fn element_set_status(&self, element: &EInfo, status: usize) -> Result<(), SessionError>;

    fn element_get_data(&self, element: &EInfo) -> Result<FileOrData, SessionError>;
    fn element_set_data(&self, element: &EInfo, data: FileOrData) -> Result<(), SessionError>;

    fn element_get_progress(&self, element: &EInfo) -> Result<f32, SessionError>;
    fn element_set_progress(&self, element: &EInfo, progress: f32) -> Result<(), SessionError>;

    fn element_get_should_save(&self, element: &EInfo) -> Result<bool, SessionError>;
    fn element_set_should_save(
        &self,
        element: &EInfo,
        should_save: bool,
    ) -> Result<(), SessionError>;

    fn element_get_enabled(&self, element: &EInfo) -> Result<bool, SessionError>;
    fn element_set_enabled(
        &self,
        element: &EInfo,
        enabled: bool,
        storage: Option<Storage>,
    ) -> Result<(), SessionError>;

    fn element_resolv_module(&self, element_info: &EInfo) -> Result<bool, SessionError>;

    /// Blocking the current thread until is done!
    fn element_wait(&self, element: &EInfo) -> Result<(), SessionError>;

    fn element_get_element_info(&self, element: &EInfo) -> Result<ElementInfo, SessionError>;

    //
    // End Element
    //

    //
    // Location
    //

    fn create_location(&self, name: &str, location: &LInfo) -> Result<LInfo, SessionError>;
    fn get_locations_len(&self, location: &LInfo) -> Result<usize, SessionError>;
    fn get_locations(
        &self,
        location: &LInfo,
        range: Range<usize>,
    ) -> Result<Vec<LInfo>, SessionError>;
    fn destroy_location(&self, location: LInfo) -> Result<LRow, SessionError>;
    fn get_default_location(&self) -> Result<LInfo, SessionError>;
    fn move_location(&self, location: &LInfo, to: &LInfo) -> Result<(), SessionError>;

    fn location_get_name(&self, location: &LInfo) -> Result<String, SessionError>;
    fn location_set_name(&self, location: &LInfo, name: &str) -> Result<(), SessionError>;

    fn location_get_desc(&self, location: &LInfo) -> Result<String, SessionError>;
    fn location_set_desc(&self, location: &LInfo, desc: &str) -> Result<(), SessionError>;

    fn location_get_path(&self, location: &LInfo) -> Result<PathBuf, SessionError>;
    fn location_set_path(&self, location: &LInfo, path: PathBuf) -> Result<(), SessionError>;

    fn location_get_where_is(&self, location: &LInfo) -> Result<WhereIsLocation, SessionError>;
    fn location_set_where_is(
        &self,
        location: &LInfo,
        where_is: WhereIsLocation,
    ) -> Result<(), SessionError>;

    fn location_get_should_save(&self, location: &LInfo) -> Result<bool, SessionError>;
    fn location_set_should_save(
        &self,
        location: &LInfo,
        should_save: bool,
    ) -> Result<(), SessionError>;

    fn location_get_elements_len(&self, location: &LInfo) -> Result<usize, SessionError>;
    fn location_get_elements(
        &self,
        location: &LInfo,
        range: Range<usize>,
    ) -> Result<Vec<EInfo>, SessionError>;

    fn location_get_location_info(&self, location: &LInfo) -> Result<LocationInfo, SessionError>;

    //
    // End Location
    //

    fn c(&self) -> Box<dyn TSession>;
}
