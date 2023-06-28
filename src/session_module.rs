use std::path::PathBuf;

use crate::prelude::*;

pub trait TSessionModule {
    fn add_module(&self, source: ModuleSource) -> SessionResult<ModuleId>;
    fn get_module(&self, path: usize) -> SessionResult<ModuleId>;

    fn module_get_element_settings(&self, uid: UID) -> SessionResult<Settings>;
    fn module_set_element_settings(&self, uid: UID, settings: Settings) -> SessionResult<()>;

    fn module_get_location_settings(&self, uid: UID) -> SessionResult<Settings>;
    fn module_set_location_settings(&self, uid: UID, settings: Settings) -> SessionResult<()>;

    fn module_path(&self, uid: UID) -> SessionResult<usize>;

    fn module_id(&self, uid: UID) -> SessionResult<u64>;
    fn destroy_module(&self, uid: UID) -> SessionResult<()>;
}
