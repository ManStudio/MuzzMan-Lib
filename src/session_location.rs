use std::{collections::HashMap, path::PathBuf};

use crate::prelude::*;

pub trait TSessionLocation {
    fn create_location(&self, location: LocationId, name: String) -> SessionResult<LocationId>;
    fn get_location(&self, path: Vec<usize>) -> SessionResult<LocationId>;
    fn get_default_location(&self) -> SessionResult<LocationId>;

    fn location_get_parent(&self, location: LocationId) -> SessionResult<LocationId>;

    fn location_get_locations_len(&self, location: LocationId) -> SessionResult<usize>;
    /// Range Inclusive
    /// That means if we pass start: 0, end: 2 will return 0, 1, 2
    fn location_get_locations(
        &self,
        location: LocationId,
        start: usize,
        end: usize,
    ) -> SessionResult<Vec<LocationId>>;

    fn location_get_elements_len(&self, location: LocationId) -> SessionResult<usize>;
    /// Range Inclusive
    /// That means if we pass start: 0, end: 2 will return 0, 1, 2
    fn location_get_elements(
        &self,
        location: LocationId,
        start: usize,
        end: usize,
    ) -> SessionResult<Vec<ElementId>>;

    fn location_get_enabled(&self, location: LocationId) -> SessionResult<bool>;
    fn location_set_enabled(&self, location: LocationId, enabled: bool) -> SessionResult<()>;

    fn location_get_path(&self, location: LocationId) -> SessionResult<PathBuf>;
    fn location_set_path(&self, location: LocationId, path: PathBuf) -> SessionResult<()>;

    fn location_is_completed(&self, location: LocationId) -> SessionResult<bool>;
    fn location_is_error(&self, location: LocationId) -> SessionResult<bool>;

    fn location_get_statuses(&self, location: LocationId) -> SessionResult<Vec<String>>;
    fn location_set_statuses(
        &self,
        location: LocationId,
        statuses: Vec<String>,
    ) -> SessionResult<()>;

    fn location_get_status(&self, location: LocationId) -> SessionResult<usize>;
    fn location_set_status(&self, location: LocationId, status: usize) -> SessionResult<()>;

    fn location_get_status_str(&self, location: LocationId) -> SessionResult<String>;

    fn location_get_progress(&self, location: LocationId) -> SessionResult<f32>;
    fn location_get_download_speed(&self, location: LocationId) -> SessionResult<usize>;
    fn location_get_upload_speed(&self, location: LocationId) -> SessionResult<usize>;
    fn location_get_download_total(&self, location: LocationId) -> SessionResult<usize>;
    fn location_get_upload_total(&self, location: LocationId) -> SessionResult<usize>;

    fn location_get_data(&self, location: LocationId) -> SessionResult<HashMap<String, Atom>>;
    fn location_set_data(
        &self,
        location: LocationId,
        data: HashMap<String, Atom>,
    ) -> SessionResult<()>;

    fn location_get_settings(&self, location: LocationId) -> SessionResult<Settings>;
    fn location_set_settings(&self, location: LocationId, settings: Settings) -> SessionResult<()>;

    fn location_get_module(&self, location: LocationId) -> SessionResult<Option<ModuleId>>;
    fn location_set_module(
        &self,
        location: LocationId,
        module_id: Option<ModuleId>,
    ) -> SessionResult<()>;

    fn move_location(
        &self,
        location: LocationId,
        location_location: LocationId,
    ) -> SessionResult<()>;
    fn location_path(&self, location: LocationId) -> SessionResult<Vec<usize>>;

    fn location_wait(&self, location: LocationId) -> SessionResult<()>;

    fn destroy_location(&self, location: LocationId) -> SessionResult<()>;
}
