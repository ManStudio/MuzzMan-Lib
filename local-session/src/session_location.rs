use muzzman_lib::prelude::*;

use crate::{TLocalSession, UIDPath, Wraper};

impl TSessionLocation for Box<dyn TLocalSession> {
    fn create_location(&self, location: LocationId, name: String) -> SessionResult<LocationId> {
        let inner = move || {
            let location_parent = self.as_ref().get_location(location.uid)?;
            let UIDPath::Location(mut path) = location_parent.path.read().unwrap().clone() else {return Err(SessionError::UIDIsNotALocation)};
            path.push(usize::MAX);
            let location = self.as_ref().create_location(name, &path);
            let id = location.location.read().unwrap().id.clone();
            Ok(id)
        };
        inner().map_err(|e| SessionError::CreateLocation(Box::new(e)))
    }

    fn get_location(&self, path: Vec<usize>) -> SessionResult<LocationId> {
        let location = self.as_ref().create_location("Get Location".into(), &path);
        let id = location.location.read().unwrap().id.clone();
        Ok(id)
    }

    fn get_default_location(&self) -> SessionResult<LocationId> {
        self.as_ref().get_default_location()
    }

    fn location_get_parent(&self, location: LocationId) -> SessionResult<Option<LocationId>> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            let parent = location.location.read().unwrap().parent.clone();
            Ok(parent)
        };
        inner().map_err(|e| SessionError::LocationGetParent(Box::new(e)))
    }

    fn location_get_locations_len(&self, location: LocationId) -> SessionResult<usize> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            let len = location.locations.read().unwrap().len();
            Ok(len)
        };
        inner().map_err(|e| SessionError::LocationGetLocationsLen(Box::new(e)))
    }

    fn location_get_locations(
        &self,
        location: LocationId,
        start: usize,
        end: usize,
    ) -> SessionResult<Vec<LocationId>> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            let location = location.location.read().unwrap();
            if start < end && location.locations.len() <= end {
                Err(SessionError::LocationGetLocations(Box::new(
                    SessionError::ThereAreLessLocations,
                )))
            } else {
                let locations = &location.locations[start..=end];
                Ok(locations.to_vec())
            }
        };
        inner().map_err(|e| SessionError::LocationGetLocations(Box::new(e)))
    }

    fn location_get_elements_len(&self, location: LocationId) -> SessionResult<usize> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            let len = location.elements.read().unwrap().len();
            Ok(len)
        };
        inner().map_err(|e| SessionError::LocationGetElementsLen(Box::new(e)))
    }

    fn location_get_elements(
        &self,
        location: LocationId,
        start: usize,
        end: usize,
    ) -> SessionResult<Vec<ElementId>> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;

            let location = location.location.read().unwrap();

            if start < end && location.locations.len() <= end {
                Err(SessionError::LocationGetLocations(Box::new(
                    SessionError::ThereAreLessLocations,
                )))
            } else {
                let elements = &location.elements[start..=end];
                Ok(elements.to_vec())
            }
        };

        inner().map_err(|e| SessionError::LocationGetElements(Box::new(e)))
    }

    fn location_get_enabled(&self, location: LocationId) -> SessionResult<bool> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            let enabled = location.location.read().unwrap().enabled;
            Ok(enabled)
        };
        inner().map_err(|e| SessionError::LocationGetEnabled(Box::new(e)))
    }

    fn location_set_enabled(&self, location: LocationId, enabled: bool) -> SessionResult<()> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            // TODO: LocalSession::location_set_enabled
            // We need to start the location on a separate thread
            // The thread when will finish will set the location enabled to false
            // If the module location_poll panics will set the location enabled to false and is_error to true,
            // and add the panic message on status usize::MAX and set status to usize::MAX
            eprintln!("TODO: LocalSession::location_set_enabled");
            Ok(())
        };
        inner().map_err(|e| SessionError::LocationGetEnabled(Box::new(e)))
    }

    fn location_get_path(&self, location: LocationId) -> SessionResult<std::path::PathBuf> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            let path = location.location.read().unwrap().path.clone();
            Ok(path)
        };
        inner().map_err(|e| SessionError::LocationGetPath(Box::new(e)))
    }

    fn location_set_path(
        &self,
        location: LocationId,
        path: std::path::PathBuf,
    ) -> SessionResult<()> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            location.location.write().unwrap().path = path;
            Ok(())
        };
        inner().map_err(|e| SessionError::LocationSetPath(Box::new(e)))
    }

    fn location_is_completed(&self, location: LocationId) -> SessionResult<bool> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            let is_completed = location.location.read().unwrap().is_completed.clone();
            Ok(is_completed)
        };
        inner().map_err(|e| SessionError::LocationIsCompleted(Box::new(e)))
    }

    fn location_is_error(&self, location: LocationId) -> SessionResult<bool> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            let is_error = location.location.read().unwrap().is_error.clone();
            Ok(is_error)
        };
        inner().map_err(|e| SessionError::LocationIsError(Box::new(e)))
    }

    fn location_get_statuses(&self, location: LocationId) -> SessionResult<Vec<String>> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            let statuses = location.location.read().unwrap().statuses.clone();
            Ok(statuses)
        };
        inner().map_err(|e| SessionError::LocationGetStatuses(Box::new(e)))
    }

    fn location_set_statuses(
        &self,
        location: LocationId,
        statuses: Vec<String>,
    ) -> SessionResult<()> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            location.location.write().unwrap().statuses = statuses;
            Ok(())
        };
        inner().map_err(|e| SessionError::LocationSetStatuses(Box::new(e)))
    }

    fn location_get_status(&self, location: LocationId) -> SessionResult<usize> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            let status = location.location.read().unwrap().status.clone();
            Ok(status)
        };
        inner().map_err(|e| SessionError::LocationGetStatus(Box::new(e)))
    }

    fn location_set_status(&self, location: LocationId, status: usize) -> SessionResult<()> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            location.location.write().unwrap().status = status;
            Ok(())
        };
        inner().map_err(|e| SessionError::LocationSetStatus(Box::new(e)))
    }

    fn location_get_status_str(&self, location: LocationId) -> SessionResult<String> {
        let inner = move || {
            let location = self.as_ref().get_location(location.uid)?;
            let location = location.location.read().unwrap();
            if let Some(status) = location.statuses.get(location.status) {
                Ok(status.clone())
            } else {
                Err(SessionError::InvalidStatus)
            }
        };
        inner().map_err(|e| SessionError::LocationGetStatuses(Box::new(e)))
    }

    fn location_get_progress(&self, location: LocationId) -> SessionResult<f32> {
        todo!()
    }

    fn location_get_download_speed(&self, location: LocationId) -> SessionResult<usize> {
        todo!()
    }

    fn location_get_upload_speed(&self, location: LocationId) -> SessionResult<usize> {
        todo!()
    }

    fn location_get_download_total(&self, location: LocationId) -> SessionResult<usize> {
        todo!()
    }

    fn location_get_upload_total(&self, location: LocationId) -> SessionResult<usize> {
        todo!()
    }

    fn location_get_data(
        &self,
        location: LocationId,
    ) -> SessionResult<std::collections::HashMap<String, Atom>> {
        todo!()
    }

    fn location_set_data(
        &self,
        location: LocationId,
        data: std::collections::HashMap<String, Atom>,
    ) -> SessionResult<()> {
        todo!()
    }

    fn location_get_settings(&self, location: LocationId) -> SessionResult<Settings> {
        todo!()
    }

    fn location_set_settings(&self, location: LocationId, settings: Settings) -> SessionResult<()> {
        todo!()
    }

    fn location_get_module(&self, location: LocationId) -> SessionResult<Option<ModuleId>> {
        todo!()
    }

    fn location_set_module(
        &self,
        location: LocationId,
        module_id: Option<ModuleId>,
    ) -> SessionResult<()> {
        todo!()
    }

    fn move_location(
        &self,
        location: LocationId,
        location_location: LocationId,
    ) -> SessionResult<()> {
        todo!()
    }

    fn location_path(&self, location: LocationId) -> SessionResult<Vec<usize>> {
        todo!()
    }

    fn destroy_location(&self, location: LocationId) -> SessionResult<()> {
        todo!()
    }
}
