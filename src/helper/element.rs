use std::{collections::HashMap, path::PathBuf};

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct ElementId {
    pub uid: UID,
    pub session: Option<Session>,
}

impl PartialEq for ElementId {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl ElementId {
    pub fn get_session(&self) -> SessionResult<Session> {
        self.session
            .clone()
            .map_or_else(|| Err(SessionError::NoSession), Ok)
    }
}

pub trait TElementHelper: TCommonHelper {
    fn _move(&self, location: LocationId) -> SessionResult<()>;
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

impl TElementHelper for ElementId {
    fn _move(&self, location: LocationId) -> SessionResult<()> {
        self.get_session()?.move_element(self.clone(), location)
    }

    fn path(&self) -> SessionResult<Vec<usize>> {
        self.get_session()?.element_path(self.clone())
    }

    fn get_parent(&self) -> SessionResult<LocationId> {
        self.get_session()?.element_get_parent(self.clone())
    }

    fn get_enabled(&self) -> SessionResult<bool> {
        self.get_session()?.element_get_enabled(self.clone())
    }

    fn set_enabled(&self, enabled: bool) -> SessionResult<()> {
        self.get_session()?
            .element_set_enabled(self.clone(), enabled)
    }

    fn get_path(&self) -> SessionResult<PathBuf> {
        self.get_session()?.element_get_path(self.clone())
    }

    fn set_path(&self, path: PathBuf) -> SessionResult<()> {
        self.get_session()?.element_set_path(self.clone(), path)
    }

    fn is_completed(&self) -> SessionResult<bool> {
        self.get_session()?.element_is_completed(self.clone())
    }

    fn is_error(&self) -> SessionResult<bool> {
        self.get_session()?.element_is_error(self.clone())
    }

    fn get_statuses(&self) -> SessionResult<Vec<String>> {
        self.get_session()?.element_get_statuses(self.clone())
    }

    fn set_statuses(&self, statuses: Vec<String>) -> SessionResult<()> {
        self.get_session()?
            .element_set_statuses(self.clone(), statuses)
    }

    fn get_status(&self) -> SessionResult<usize> {
        self.get_session()?.element_get_status(self.clone())
    }

    fn set_status(&self, status: usize) -> SessionResult<()> {
        self.get_session()?.element_set_status(self.clone(), status)
    }

    fn get_status_str(&self) -> SessionResult<String> {
        self.get_session()?.element_get_status_str(self.clone())
    }

    fn get_progress(&self) -> SessionResult<f32> {
        self.get_session()?.element_get_progress(self.clone())
    }

    fn get_download_speed(&self) -> SessionResult<usize> {
        self.get_session()?.element_get_download_speed(self.clone())
    }

    fn get_upload_speed(&self) -> SessionResult<usize> {
        self.get_session()?.element_get_upload_speed(self.clone())
    }

    fn get_download_total(&self) -> SessionResult<usize> {
        self.get_session()?.element_get_download_total(self.clone())
    }

    fn get_upload_total(&self) -> SessionResult<usize> {
        self.get_session()?.element_get_upload_total(self.clone())
    }

    fn get_data(&self) -> SessionResult<HashMap<String, Atom>> {
        self.get_session()?.element_get_data(self.clone())
    }

    fn set_data(&self, data: HashMap<String, Atom>) -> SessionResult<()> {
        self.get_session()?.element_set_data(self.clone(), data)
    }

    fn get_settings(&self) -> SessionResult<Settings> {
        self.get_session()?.element_get_settings(self.clone())
    }

    fn set_settings(&self, settings: Settings) -> SessionResult<()> {
        self.get_session()?
            .element_set_settings(self.clone(), settings)
    }

    fn get_module(&self) -> SessionResult<Option<ModuleId>> {
        self.get_session()?.element_get_module(self.clone())
    }

    fn set_module(&self, module_id: Option<ModuleId>) -> SessionResult<()> {
        self.get_session()?
            .element_set_module(self.clone(), module_id)
    }

    fn wait(&self) -> SessionResult<()> {
        self.get_session()?.element_wait(self.clone())
    }

    fn destroy(self) -> SessionResult<()> {
        self.get_session()?.destroy_element(self)
    }
}
