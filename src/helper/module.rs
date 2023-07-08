use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct ModuleId {
    pub uid: UID,
    pub session: Option<Session>,
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
    fn module_supports_protocols(&self) -> SessionResult<Vec<String>>;
    /// Should be like "html, exe"
    fn module_supports_extensions(&self) -> SessionResult<Vec<String>>;

    fn path(&self) -> SessionResult<usize>;

    fn id(&self) -> SessionResult<u64>;
    fn destroy(self) -> SessionResult<()>;
}
