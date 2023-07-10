use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct ModuleId {
    pub uid: UID,
    pub session: Option<Session>,
}

impl PartialEq for ModuleId {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl ModuleId {
    pub fn get_session(&self) -> SessionResult<Session> {
        self.session
            .clone()
            .map_or_else(|| Err(SessionError::NoSession), Ok)
    }
}

pub trait TModuleHelper: TCommonHelper {
    fn get_element_settings(&self) -> SessionResult<Settings>;
    fn set_element_settings(&self, settings: Settings) -> SessionResult<()>;

    fn get_location_settings(&self) -> SessionResult<Settings>;
    fn set_location_settings(&self, settings: Settings) -> SessionResult<()>;

    /// Should be like "http:, https:"
    fn supports_protocols(&self) -> SessionResult<Vec<String>>;
    /// Should be like "html, exe"
    fn supports_extensions(&self) -> SessionResult<Vec<String>>;

    fn path(&self) -> SessionResult<usize>;

    fn id(&self) -> SessionResult<u64>;
    fn destroy(self) -> SessionResult<()>;
}

impl TModuleHelper for ModuleId {
    fn get_element_settings(&self) -> SessionResult<Settings> {
        self.get_session()?
            .module_get_element_settings(self.clone())
    }

    fn set_element_settings(&self, settings: Settings) -> SessionResult<()> {
        self.get_session()?
            .module_set_element_settings(self.clone(), settings)
    }

    fn get_location_settings(&self) -> SessionResult<Settings> {
        self.get_session()?
            .module_get_location_settings(self.clone())
    }

    fn set_location_settings(&self, settings: Settings) -> SessionResult<()> {
        self.get_session()?
            .module_set_location_settings(self.clone(), settings)
    }

    fn supports_protocols(&self) -> SessionResult<Vec<String>> {
        self.get_session()?.module_supports_protocols(self.clone())
    }

    fn supports_extensions(&self) -> SessionResult<Vec<String>> {
        self.get_session()?.module_supports_extensions(self.clone())
    }

    fn path(&self) -> SessionResult<usize> {
        self.get_session()?.module_path(self.clone())
    }

    fn id(&self) -> SessionResult<u64> {
        self.get_session()?.module_id(self.clone())
    }

    fn destroy(self) -> SessionResult<()> {
        self.get_session()?.destroy_module(self)
    }
}
