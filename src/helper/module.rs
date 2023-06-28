use crate::prelude::*;

pub struct ModuleId {
    pub uid: UID,
    pub session: Box<dyn TSession>,
}

pub trait TModuleHelper: TCommonHelper {
    fn get_element_settings(&self) -> SessionResult<Settings>;
    fn set_element_settings(&self, settings: Settings) -> SessionResult<()>;

    fn get_location_settings(&self) -> SessionResult<Settings>;
    fn set_location_settings(&self, settings: Settings) -> SessionResult<()>;

    fn path(&self) -> SessionResult<usize>;

    fn id(&self) -> SessionResult<u64>;
    fn destroy(self) -> SessionResult<()>;
}
