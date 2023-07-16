use std::{collections::HashMap, path::PathBuf};

use crate::prelude::*;

pub trait TSessionElement {
    fn create_element(&self, location: LocationId, name: String) -> SessionResult<ElementId>;
    fn get_element(&self, path: Vec<usize>) -> SessionResult<ElementId>;

    fn move_element(&self, element: ElementId, location: LocationId) -> SessionResult<()>;
    fn element_path(&self, element: ElementId) -> SessionResult<Vec<usize>>;

    fn element_get_parent(&self, element: ElementId) -> SessionResult<LocationId>;

    fn element_get_enabled(&self, element: ElementId) -> SessionResult<bool>;
    fn element_set_enabled(&self, element: ElementId, enabled: bool) -> SessionResult<()>;

    fn element_get_path(&self, element: ElementId) -> SessionResult<PathBuf>;
    fn element_set_path(&self, element: ElementId, path: PathBuf) -> SessionResult<()>;

    fn element_is_completed(&self, element: ElementId) -> SessionResult<bool>;
    fn element_is_error(&self, element: ElementId) -> SessionResult<bool>;

    fn element_get_statuses(&self, element: ElementId) -> SessionResult<Vec<String>>;
    fn element_set_statuses(&self, element: ElementId, statuses: Vec<String>) -> SessionResult<()>;

    fn element_get_status(&self, element: ElementId) -> SessionResult<usize>;
    fn element_set_status(&self, element: ElementId, status: usize) -> SessionResult<()>;

    fn element_get_status_str(&self, element: ElementId) -> SessionResult<String>;

    fn element_get_url(&self, element: ElementId) -> SessionResult<String>;
    fn element_set_url(&self, element: ElementId, url: String) -> SessionResult<()>;

    fn element_get_progress(&self, element: ElementId) -> SessionResult<f32>;
    fn element_get_download_speed(&self, element: ElementId) -> SessionResult<usize>;
    fn element_get_upload_speed(&self, element: ElementId) -> SessionResult<usize>;
    fn element_get_download_total(&self, element: ElementId) -> SessionResult<usize>;
    fn element_get_upload_total(&self, element: ElementId) -> SessionResult<usize>;

    fn element_get_data(&self, element: ElementId) -> SessionResult<HashMap<String, Atom>>;
    fn element_set_data(
        &self,
        element: ElementId,
        data: HashMap<String, Atom>,
    ) -> SessionResult<()>;

    fn element_get_settings(&self, element: ElementId) -> SessionResult<Settings>;
    fn element_set_settings(&self, element: ElementId, settings: Settings) -> SessionResult<()>;

    fn element_get_module(&self, element: ElementId) -> SessionResult<Option<ModuleId>>;
    fn element_set_module(
        &self,
        element: ElementId,
        module_id: Option<ModuleId>,
    ) -> SessionResult<()>;

    fn element_wait(&self, element: ElementId) -> SessionResult<()>;

    fn destroy_element(&self, element: ElementId) -> SessionResult<()>;
}
