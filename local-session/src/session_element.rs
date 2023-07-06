use muzzman_lib::prelude::*;

use crate::TLocalSession;

impl TSessionElement for Box<dyn TLocalSession> {
    fn create_element(&self, location: LocationId, name: String) -> SessionResult<ElementId> {
        todo!()
    }

    fn get_element(&self, path: Vec<usize>) -> SessionResult<ElementId> {
        todo!()
    }

    fn move_element(&self, element: ElementId, location: LocationId) -> SessionResult<()> {
        todo!()
    }

    fn element_path(&self, element: ElementId) -> SessionResult<Vec<usize>> {
        todo!()
    }

    fn element_get_parent(&self, element: ElementId) -> SessionResult<LocationId> {
        todo!()
    }

    fn element_get_enabled(&self, element: ElementId) -> SessionResult<bool> {
        todo!()
    }

    fn element_set_enabled(&self, element: ElementId, enabled: bool) -> SessionResult<()> {
        todo!()
    }

    fn element_get_path(&self, element: ElementId) -> SessionResult<std::path::PathBuf> {
        todo!()
    }

    fn element_set_path(&self, element: ElementId, path: std::path::PathBuf) -> SessionResult<()> {
        todo!()
    }

    fn element_is_completed(&self, element: ElementId) -> SessionResult<bool> {
        todo!()
    }

    fn element_is_error(&self, element: ElementId) -> SessionResult<bool> {
        todo!()
    }

    fn element_get_statuses(&self, element: ElementId) -> SessionResult<Vec<String>> {
        todo!()
    }

    fn element_set_statuses(&self, element: ElementId, statuses: Vec<String>) -> SessionResult<()> {
        todo!()
    }

    fn element_get_status(&self, element: ElementId) -> SessionResult<usize> {
        todo!()
    }

    fn element_set_status(&self, element: ElementId, status: usize) -> SessionResult<()> {
        todo!()
    }

    fn element_get_status_str(&self, element: ElementId) -> SessionResult<String> {
        todo!()
    }

    fn element_get_progress(&self, element: ElementId) -> SessionResult<f32> {
        todo!()
    }

    fn element_get_download_speed(&self, element: ElementId) -> SessionResult<usize> {
        todo!()
    }

    fn element_get_upload_speed(&self, element: ElementId) -> SessionResult<usize> {
        todo!()
    }

    fn element_get_download_total(&self, element: ElementId) -> SessionResult<usize> {
        todo!()
    }

    fn element_get_upload_total(&self, element: ElementId) -> SessionResult<usize> {
        todo!()
    }

    fn element_get_data(
        &self,
        element: ElementId,
    ) -> SessionResult<std::collections::HashMap<String, Atom>> {
        todo!()
    }

    fn element_set_data(
        &self,
        element: ElementId,
        data: std::collections::HashMap<String, Atom>,
    ) -> SessionResult<()> {
        todo!()
    }

    fn element_get_settings(&self, element: ElementId) -> SessionResult<Settings> {
        todo!()
    }

    fn element_set_settings(&self, element: ElementId, settings: Settings) -> SessionResult<()> {
        todo!()
    }

    fn element_get_module(&self, element: ElementId) -> SessionResult<Option<ModuleId>> {
        todo!()
    }

    fn element_set_module(
        &self,
        element: ElementId,
        module_id: Option<ModuleId>,
    ) -> SessionResult<()> {
        todo!()
    }

    fn element_wait(&self, element: ElementId) -> SessionResult<()> {
        todo!()
    }

    fn destroy_element(&self, element: ElementId) -> SessionResult<()> {
        todo!()
    }
}
