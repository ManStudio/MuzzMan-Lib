use std::{collections::HashMap, path::PathBuf};

use crate::prelude::*;

pub trait TSessionLocation {
    fn create_location(&self, location_uid: UID, name: String) -> SessionResult<LocationId>;
    fn get_location(&self, path: Vec<usize>) -> SessionResult<LocationId>;

    fn location_get_locations_len(&self, uid: UID) -> SessionResult<usize>;
    /// Range Inclusive
    /// That means if we pass start: 0, end: 2 will return 0, 1, 2
    fn location_get_locations(
        &self,
        uid: UID,
        start: usize,
        end: usize,
    ) -> SessionResult<Vec<LocationId>>;

    fn location_get_elements_len(&self, uid: UID) -> SessionResult<usize>;
    /// Range Inclusive
    /// That means if we pass start: 0, end: 2 will return 0, 1, 2
    fn location_get_elements(
        &self,
        uid: UID,
        start: usize,
        end: usize,
    ) -> SessionResult<Vec<ElementId>>;

    fn location_get_enabled(&self, uid: UID) -> SessionResult<bool>;
    fn location_set_enabled(&self, uid: UID, enabled: bool) -> SessionResult<()>;

    fn location_get_path(&self, uid: UID) -> SessionResult<PathBuf>;
    fn location_set_path(&self, uid: UID, path: PathBuf) -> SessionResult<()>;

    fn location_is_completed(&self, uid: UID) -> SessionResult<bool>;
    fn location_is_error(&self, uid: UID) -> SessionResult<bool>;

    fn location_get_statuses(&self, uid: UID) -> SessionResult<Vec<String>>;
    fn location_set_statuses(&self, uid: UID, statuses: Vec<String>) -> SessionResult<()>;

    fn location_get_status(&self, uid: UID) -> SessionResult<usize>;
    fn location_set_status(&self, uid: UID, status: usize) -> SessionResult<()>;

    fn location_get_status_str(&self, uid: UID) -> SessionResult<String>;

    fn location_get_progress(&self, uid: UID) -> SessionResult<f32>;
    fn location_get_download_speed(&self, uid: UID) -> SessionResult<usize>;
    fn location_get_upload_speed(&self, uid: UID) -> SessionResult<usize>;
    fn location_get_download_total(&self, uid: UID) -> SessionResult<usize>;
    fn location_get_upload_total(&self, uid: UID) -> SessionResult<usize>;

    fn location_get_data(&self, uid: UID) -> SessionResult<HashMap<String, Atom>>;
    fn location_set_data(&self, uid: UID, data: HashMap<String, Atom>) -> SessionResult<()>;

    fn location_get_settings(&self, uid: UID) -> SessionResult<Settings>;
    fn location_set_settings(&self, uid: UID, settings: Settings) -> SessionResult<()>;

    fn move_location(&self, uid: UID, location_uid: UID) -> SessionResult<()>;
    fn location_path(&self, uid: UID) -> SessionResult<Vec<usize>>;
    fn destroy_location(&self, uid: UID) -> SessionResult<()>;
}
