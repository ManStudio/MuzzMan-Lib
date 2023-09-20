use muzzman_lib::prelude::*;

use crate::{TLocalSession, UIDPath};

impl TSessionElement for Box<dyn TLocalSession> {
    fn create_element(&self, location: LocationId, name: String) -> SessionResult<ElementId> {
        let inner = move || {
            let parent = self.as_ref().location(location.uid)?;
            let UIDPath::Location(mut path) = parent.path.read().unwrap().clone() else {
                return Err(SessionError::UIDIsNotALocation);
            };
            let index = parent.location.read().unwrap().elements.len();
            path.push(index);

            let id = self
                .as_ref()
                .create_element(name, &path)
                .element
                .read()
                .unwrap()
                .id
                .clone();
            Ok(id)
        };

        inner().map_err(|e| SessionError::CreateElement(Box::new(e)))
    }

    fn get_element(&self, path: Vec<usize>) -> SessionResult<ElementId> {
        todo!()
    }

    fn move_element(&self, element: ElementId, location: LocationId) -> SessionResult<()> {
        todo!()
    }

    fn element_path(&self, element: ElementId) -> SessionResult<Vec<usize>> {
        let inner = move || {
            let element = self.as_ref().element(element.uid)?;
            let UIDPath::Element(mut element, index) = element.path.read().unwrap().clone() else {
                return Err(SessionError::UIDIsNotAElement);
            };
            element.push(index);
            Ok(element)
        };
        inner().map_err(|e| SessionError::CreateElement(Box::new(e)))
    }

    fn element_get_parent(&self, element: ElementId) -> SessionResult<LocationId> {
        Ok(self
            .as_ref()
            .element(element.uid)
            .map_err(|e| SessionError::ElementGetParent(Box::new(e)))?
            .element
            .read()
            .unwrap()
            .parent
            .clone())
    }

    fn element_get_enabled(&self, element: ElementId) -> SessionResult<bool> {
        let inner = move || {
            let element = self.as_ref().element(element.uid)?;
            let enabled = element.element.read().unwrap().enabled;
            Ok(enabled)
        };
        inner().map_err(|e| SessionError::ElementGetEnabled(Box::new(e)))
    }

    fn element_set_enabled(&self, element: ElementId, enabled: bool) -> SessionResult<()> {
        let inner = move || {
            let element = self.as_ref().element(element.uid)?;
            if element.element.read().unwrap().enabled == enabled {
                return Ok(());
            }
            if !enabled {
                element.thread.write().unwrap().take().unwrap();
                element.element.write().unwrap().enabled = false;
            } else {
                let Some(module_id) = element.element.read().unwrap().module.clone() else {
                    return Err(SessionError::NoModule);
                };
                let module = self.as_ref().module(module_id.uid)?;
                element.element.write().unwrap().enabled = true;
                let errors = element.element.read().unwrap().settings.validate();
                if !errors.is_empty() {
                    return Err(SessionError::InvalidSettings(errors));
                }
                let thread = element.thread.clone();
                let runtime = self.runtime();
                *thread.write().unwrap() = Some(std::thread::spawn(move || {
                    unimplemented!();

                    // When ends
                    element.element.write().unwrap().enabled = false;
                }));
            }
            Ok(())
        };
        inner().map_err(|e| SessionError::ElementSetEnabled(Box::new(e)))
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

    fn element_get_url(&self, element: ElementId) -> SessionResult<String> {
        let inner = move || {
            let element = self.as_ref().element(element.uid)?;
            let url = element.element.read().unwrap().url.clone();
            Ok(url)
        };
        inner().map_err(|e| SessionError::ElementGetUrl(Box::new(e)))
    }

    fn element_set_url(&self, element: ElementId, url: String) -> SessionResult<()> {
        let inner = move || {
            let element = self.as_ref().element(element.uid)?;
            element.element.write().unwrap().url = url;
            Ok(())
        };
        inner().map_err(|e| SessionError::ElementSetUrl(Box::new(e)))
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
        self.as_ref()
            .element(element.uid)?
            .element
            .write()
            .unwrap()
            .module = module_id;
        Ok(())
    }

    fn element_wait(&self, element: ElementId) -> SessionResult<()> {
        todo!()
    }

    fn destroy_element(&self, element: ElementId) -> SessionResult<()> {
        todo!()
    }
}
