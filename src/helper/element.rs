use std::{collections::HashMap, path::PathBuf};

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct ElementId {
    pub uid: UID,
    pub session: Option<Session>,
}

pub trait TElementHelper: TCommonHelper {
    fn _move(&self, location_uid: UID) -> SessionResult<()>;
    fn path(&self) -> SessionResult<Vec<usize>>;

    fn get_parent(&self) -> SessionResult<LocationId>;

    fn get_enabled(&self) -> SessionResult<bool>;
    fn set_enabled(&self, enabled: bool) -> SessionResult<()>;

    fn get_path(&self) -> SessionResult<PathBuf>;
    fn set_path(&self, path: PathBuf) -> SessionResult<()>;

    fn is_completed(&self) -> SessionResult<bool>;
    fn is_error(&self) -> SessionResult<bool>;

    fn get_statuses(&self) -> SessionResult<Vec<String>>;
    fn set_statuses(&self, statuses: Vec<String>) -> SessionResult<()>;

    fn get_status(&self) -> SessionResult<usize>;
    fn set_status(&self, status: usize) -> SessionResult<()>;

    fn get_status_str(&self) -> SessionResult<String>;

    fn get_progress(&self) -> SessionResult<f32>;
    fn get_download_speed(&self) -> SessionResult<usize>;
    fn get_upload_speed(&self) -> SessionResult<usize>;
    fn get_download_total(&self) -> SessionResult<usize>;
    fn get_upload_total(&self) -> SessionResult<usize>;

    fn get_data(&self) -> SessionResult<HashMap<String, Atom>>;
    fn set_data(&self, data: HashMap<String, Atom>) -> SessionResult<()>;

    fn get_settings(&self) -> SessionResult<Settings>;
    fn set_settings(&self, settings: Settings) -> SessionResult<()>;

    fn get_module(&self) -> SessionResult<Option<ModuleId>>;
    fn set_module(&self, module_id: Option<ModuleId>) -> SessionResult<()>;

    fn wait(&self) -> SessionResult<()>;

    fn destroy(self) -> SessionResult<()>;
}
