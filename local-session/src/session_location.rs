use muzzman_lib::prelude::*;

use crate::{TLocalSession, UIDPath, Wraper};

impl TSessionLocation for Box<dyn TLocalSession> {
    fn create_location(&self, location: LocationId, name: String) -> SessionResult<LocationId> {
        let inner = move || {
            let Wraper::Location(location_parent) = self.as_ref().get(location.uid)? else {return Err(SessionError::UIDIsNotALocation)};
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
        let Wraper::Location(location) = self
            .as_ref()
            .get(location.uid)
            .map_err(|e| SessionError::LocationGetParent(Box::new(e)))? else {return Err(SessionError::UIDIsNotALocation)};
        let parent = location.location.read().unwrap().parent.clone();
        Ok(parent)
    }

    fn location_get_locations_len(&self, location: LocationId) -> SessionResult<usize> {
        let Wraper::Location(location) = self
            .as_ref()
            .get(location.uid)
            .map_err(|e| SessionError::LocationGetLocationsLen(Box::new(e)))? else {return Err(SessionError::UIDIsNotALocation)};
        let len = location.locations.read().unwrap().len();
        Ok(len)
    }

    fn location_get_locations(
        &self,
        location: LocationId,
        start: usize,
        end: usize,
    ) -> SessionResult<Vec<LocationId>> {
        let Wraper::Location(location) = self
            .as_ref()
            .get(location.uid)
            .map_err(|e| SessionError::LocationGetLocations(Box::new(e)))?else {return Err(SessionError::UIDIsNotALocation)};
        let location = location.location.read().unwrap();
        if start < end && location.locations.len() <= end {
            Err(SessionError::LocationGetLocations(Box::new(
                SessionError::ThereAreLessLocations,
            )))
        } else {
            let locations = &location.locations[start..=end];
            Ok(locations.to_vec())
        }
    }

    fn location_get_elements_len(&self, location: LocationId) -> SessionResult<usize> {
        let Wraper::Location(location) = self
            .as_ref()
            .get(location.uid)
            .map_err(|e| SessionError::LocationGetElementsLen(Box::new(e)))?else {return Err(SessionError::UIDIsNotALocation)};
        let len = location.elements.read().unwrap().len();
        Ok(len)
    }

    fn location_get_elements(
        &self,
        location: LocationId,
        start: usize,
        end: usize,
    ) -> SessionResult<Vec<ElementId>> {
        let Wraper::Location(location) = self.as_ref().get(location.uid)?else {return Err(SessionError::UIDIsNotALocation)};
        let location = location.location.read().unwrap();

        if start < end && location.locations.len() <= end {
            Err(SessionError::LocationGetLocations(Box::new(
                SessionError::ThereAreLessLocations,
            )))
        } else {
            let elements = &location.elements[start..=end];
            Ok(elements.to_vec())
        }
    }

    fn location_get_enabled(&self, location: LocationId) -> SessionResult<bool> {
        todo!()
    }

    fn location_set_enabled(&self, location: LocationId, enabled: bool) -> SessionResult<()> {
        todo!()
    }

    fn location_get_path(&self, location: LocationId) -> SessionResult<std::path::PathBuf> {
        todo!()
    }

    fn location_set_path(
        &self,
        location: LocationId,
        path: std::path::PathBuf,
    ) -> SessionResult<()> {
        todo!()
    }

    fn location_is_completed(&self, location: LocationId) -> SessionResult<bool> {
        todo!()
    }

    fn location_is_error(&self, location: LocationId) -> SessionResult<bool> {
        todo!()
    }

    fn location_get_statuses(&self, location: LocationId) -> SessionResult<Vec<String>> {
        todo!()
    }

    fn location_set_statuses(
        &self,
        location: LocationId,
        statuses: Vec<String>,
    ) -> SessionResult<()> {
        todo!()
    }

    fn location_get_status(&self, location: LocationId) -> SessionResult<usize> {
        todo!()
    }

    fn location_set_status(&self, location: LocationId, status: usize) -> SessionResult<()> {
        todo!()
    }

    fn location_get_status_str(&self, location: LocationId) -> SessionResult<String> {
        todo!()
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