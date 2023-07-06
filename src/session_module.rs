use crate::prelude::*;

pub trait TSessionModule {
    fn add_module(&self, source: ModuleSource) -> SessionResult<ModuleId>;
    fn get_module(&self, path: usize) -> SessionResult<ModuleId>;

    fn module_get_element_settings(&self, module: ModuleId) -> SessionResult<Settings>;
    fn module_set_element_settings(
        &self,
        module: ModuleId,
        settings: Settings,
    ) -> SessionResult<()>;

    fn module_get_location_settings(&self, module: ModuleId) -> SessionResult<Settings>;
    fn module_set_location_settings(
        &self,
        module: ModuleId,
        settings: Settings,
    ) -> SessionResult<()>;

    /// Should be like "http:, https:"
    fn module_supports_protocols(&self, module: ModuleId) -> SessionResult<Vec<String>>;
    /// Should be like "html, exe"
    fn module_supports_extensions(&self, module: ModuleId) -> SessionResult<Vec<String>>;

    fn module_path(&self, module: ModuleId) -> SessionResult<usize>;

    fn module_id(&self, module: ModuleId) -> SessionResult<u64>;
    fn destroy_module(&self, module: ModuleId) -> SessionResult<()>;
}
