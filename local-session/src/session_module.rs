use muzzman_lib::prelude::*;

use crate::TLocalSession;

impl TSessionModule for Box<dyn TLocalSession> {
    fn add_module(&self, source: ModuleSource) -> SessionResult<ModuleId> {
        todo!()
    }

    fn get_module(&self, path: usize) -> SessionResult<ModuleId> {
        todo!()
    }

    fn module_get_element_settings(&self, module: ModuleId) -> SessionResult<Settings> {
        todo!()
    }

    fn module_set_element_settings(
        &self,
        module: ModuleId,
        settings: Settings,
    ) -> SessionResult<()> {
        todo!()
    }

    fn module_get_location_settings(&self, module: ModuleId) -> SessionResult<Settings> {
        todo!()
    }

    fn module_set_location_settings(
        &self,
        module: ModuleId,
        settings: Settings,
    ) -> SessionResult<()> {
        todo!()
    }

    fn module_supports_protocols(&self, module: ModuleId) -> SessionResult<Vec<String>> {
        todo!()
    }

    fn module_supports_extensions(&self, module: ModuleId) -> SessionResult<Vec<String>> {
        todo!()
    }

    fn module_path(&self, module: ModuleId) -> SessionResult<usize> {
        todo!()
    }

    fn module_id(&self, module: ModuleId) -> SessionResult<u64> {
        todo!()
    }

    fn destroy_module(&self, module: ModuleId) -> SessionResult<()> {
        todo!()
    }
}
