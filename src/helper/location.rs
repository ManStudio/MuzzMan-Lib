use std::{collections::HashMap, path::PathBuf};

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct LocationId {
    pub uid: UID,
    pub session: Option<Session>,
}

impl PartialEq for LocationId {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl LocationId {
    pub fn get_session(&self) -> SessionResult<Session> {
        self.session
            .clone()
            .map_or_else(|| Err(SessionError::NoSession), Ok)
    }
}

pub trait TLocationHelper: TCommonHelper {
    fn create_location(&self, name: String) -> SessionResult<LocationId>;
    fn create_element(&self, name: String) -> SessionResult<ElementId>;

    fn get_parent(&self) -> SessionResult<LocationId>;

    fn get_locations_len(&self) -> SessionResult<usize>;
    /// Range Inclusive
    /// That means if we pass start: 0, end: 2 will return 0, 1, 2
    fn get_locations(&self, start: usize, end: usize) -> SessionResult<Vec<LocationId>>;

    fn get_elements_len(&self) -> SessionResult<usize>;
    /// Range Inclusive
    /// That means if we pass start: 0, end: 2 will return 0, 1, 2
    fn get_elements(&self, start: usize, end: usize) -> SessionResult<Vec<ElementId>>;

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

    fn _move(&self, to: LocationId) -> SessionResult<()>;
    fn path(&self) -> SessionResult<Vec<usize>>;

    fn wait(&self) -> SessionResult<()>;

    fn destroy(self) -> SessionResult<()>;
}

impl TLocationHelper for LocationId {
    fn create_location(&self, name: String) -> SessionResult<LocationId> {
        self.get_session()?.create_location(self.clone(), name)
    }

    fn create_element(&self, name: String) -> SessionResult<ElementId> {
        self.get_session()?.create_element(self.clone(), name)
    }

    fn get_parent(&self) -> SessionResult<LocationId> {
        self.get_session()?.location_get_parent(self.clone())
    }

    fn get_locations_len(&self) -> SessionResult<usize> {
        self.get_session()?.location_get_locations_len(self.clone())
    }

    fn get_locations(&self, start: usize, end: usize) -> SessionResult<Vec<LocationId>> {
        self.get_session()?
            .location_get_locations(self.clone(), start, end)
    }

    fn get_elements_len(&self) -> SessionResult<usize> {
        self.get_session()?.location_get_elements_len(self.clone())
    }

    fn get_elements(&self, start: usize, end: usize) -> SessionResult<Vec<ElementId>> {
        self.get_session()?
            .location_get_elements(self.clone(), start, end)
    }

    fn get_enabled(&self) -> SessionResult<bool> {
        self.get_session()?.location_get_enabled(self.clone())
    }

    fn set_enabled(&self, enabled: bool) -> SessionResult<()> {
        self.get_session()?
            .location_set_enabled(self.clone(), enabled)
    }

    fn get_path(&self) -> SessionResult<PathBuf> {
        self.get_session()?.location_get_path(self.clone())
    }

    fn set_path(&self, path: PathBuf) -> SessionResult<()> {
        self.get_session()?.location_set_path(self.clone(), path)
    }

    fn is_completed(&self) -> SessionResult<bool> {
        self.get_session()?.location_is_completed(self.clone())
    }

    fn is_error(&self) -> SessionResult<bool> {
        self.get_session()?.location_is_error(self.clone())
    }

    fn get_statuses(&self) -> SessionResult<Vec<String>> {
        self.get_session()?.location_get_statuses(self.clone())
    }

    fn set_statuses(&self, statuses: Vec<String>) -> SessionResult<()> {
        self.get_session()?
            .location_set_statuses(self.clone(), statuses)
    }

    fn get_status(&self) -> SessionResult<usize> {
        self.get_session()?.location_get_status(self.clone())
    }

    fn set_status(&self, status: usize) -> SessionResult<()> {
        self.get_session()?
            .location_set_status(self.clone(), status)
    }

    fn get_status_str(&self) -> SessionResult<String> {
        self.get_session()?.location_get_status_str(self.clone())
    }

    fn get_progress(&self) -> SessionResult<f32> {
        self.get_session()?.location_get_progress(self.clone())
    }

    fn get_download_speed(&self) -> SessionResult<usize> {
        self.get_session()?
            .location_get_download_speed(self.clone())
    }

    fn get_upload_speed(&self) -> SessionResult<usize> {
        self.get_session()?.location_get_upload_speed(self.clone())
    }

    fn get_download_total(&self) -> SessionResult<usize> {
        self.get_session()?
            .location_get_download_total(self.clone())
    }

    fn get_upload_total(&self) -> SessionResult<usize> {
        self.get_session()?.location_get_upload_total(self.clone())
    }

    fn get_data(&self) -> SessionResult<HashMap<String, Atom>> {
        self.get_session()?.location_get_data(self.clone())
    }

    fn set_data(&self, data: HashMap<String, Atom>) -> SessionResult<()> {
        self.get_session()?.location_set_data(self.clone(), data)
    }

    fn get_settings(&self) -> SessionResult<Settings> {
        self.get_session()?.location_get_settings(self.clone())
    }

    fn set_settings(&self, settings: Settings) -> SessionResult<()> {
        self.get_session()?
            .location_set_settings(self.clone(), settings)
    }

    fn get_module(&self) -> SessionResult<Option<ModuleId>> {
        self.get_session()?.location_get_module(self.clone())
    }

    fn set_module(&self, module_id: Option<ModuleId>) -> SessionResult<()> {
        self.get_session()?
            .location_set_module(self.clone(), module_id)
    }

    fn wait(&self) -> SessionResult<()> {
        self.get_session()?.location_wait(self.clone())
    }

    fn destroy(self) -> SessionResult<()> {
        self.get_session()?.destroy_location(self)
    }

    fn _move(&self, to: LocationId) -> SessionResult<()> {
        self.get_session()?.move_location(self.clone(), to.clone())
    }

    fn path(&self) -> SessionResult<Vec<usize>> {
        self.get_session()?.location_path(self.clone())
    }
}
