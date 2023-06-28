use crate::prelude::*;

pub trait TCommonHelper {
    fn get_name(&self) -> SessionResult<String>;
    fn set_name(&self, name: String) -> SessionResult<()>;

    fn get_desc(&self, uid: UID) -> SessionResult<String>;
    fn set_desc(&self, name: String) -> SessionResult<()>;

    fn emit(&self, event: Event) -> SessionResult<()>;
    fn notify(&self, to: UID, event: Event) -> SessionResult<()>;

    fn subscribe(&self, to: UID) -> SessionResult<()>;
    fn unsubscribe(&self, from: UID) -> SessionResult<()>;

    fn events(&self, consume: bool) -> SessionResult<Vec<Event>>;
    fn push_event(&self, event: Event) -> SessionResult<()>;
}
