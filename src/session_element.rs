use std::{collections::HashMap, path::PathBuf};

use crate::prelude::*;

pub trait TSessionElement {
    fn create_element(&self, location_uid: UID, name: String) -> SessionResult<ElementId>;
    fn get_element(&self, path: Vec<usize>) -> SessionResult<ElementId>;

    fn move_element(&self, uid: UID, location_uid: UID) -> SessionResult<()>;
    fn element_path(&self, uid: UID) -> SessionResult<Vec<usize>>;

    fn element_get_enabled(&self, uid: UID) -> SessionResult<bool>;
    fn element_set_enabled(&self, uid: UID, enabled: bool) -> SessionResult<()>;

    fn element_get_path(&self, uid: UID) -> SessionResult<PathBuf>;
    fn element_set_path(&self, uid: UID, path: PathBuf) -> SessionResult<()>;

    fn element_is_completed(&self, uid: UID) -> SessionResult<bool>;
    fn element_is_error(&self, uid: UID) -> SessionResult<bool>;

    fn element_get_statuses(&self, uid: UID) -> SessionResult<Vec<String>>;
    fn element_set_statuses(&self, uid: UID, statuses: Vec<String>) -> SessionResult<()>;

    fn element_get_status(&self, uid: UID) -> SessionResult<usize>;
    fn element_set_status(&self, uid: UID, status: usize) -> SessionResult<()>;

    fn element_get_status_str(&self, uid: UID) -> SessionResult<String>;

    fn element_get_progress(&self, uid: UID) -> SessionResult<f32>;
    fn element_get_download_speed(&self, uid: UID) -> SessionResult<usize>;
    fn element_get_upload_speed(&self, uid: UID) -> SessionResult<usize>;
    fn element_get_download_total(&self, uid: UID) -> SessionResult<usize>;
    fn element_get_upload_total(&self, uid: UID) -> SessionResult<usize>;

    fn element_get_data(&self, uid: UID) -> SessionResult<HashMap<String, Atom>>;
    fn element_set_data(&self, uid: UID, data: HashMap<String, Atom>) -> SessionResult<()>;

    fn element_get_settings(&self, uid: UID) -> SessionResult<Settings>;
    fn element_set_settings(&self, uid: UID, settings: Settings) -> SessionResult<()>;

    fn element_wait(&self, uid: UID) -> SessionResult<()>;

    fn destroy_element(&self, uid: UID) -> SessionResult<()>;
}
