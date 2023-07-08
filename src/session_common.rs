use crate::prelude::*;

pub trait TSessionCommon {
    fn get_name(&self, uid: UID) -> SessionResult<String>;
    fn set_name(&self, uid: UID, name: String) -> SessionResult<()>;

    fn get_desc(&self, uid: UID) -> SessionResult<String>;
    fn set_desc(&self, uid: UID, desc: String) -> SessionResult<()>;

    fn emit(&self, uid: UID, event: Event) -> SessionResult<()>;
    fn notify(&self, uid: UID, to: UID, event: Event) -> SessionResult<()>;

    fn subscribe(&self, uid: UID, to: UID) -> SessionResult<()>;
    fn unsubscribe(&self, uid: UID, from: UID) -> SessionResult<()>;

    fn events(&self, uid: UID, consume: bool) -> SessionResult<Vec<Event>>;
    fn push_event(&self, uid: UID, event: Event) -> SessionResult<()>;
}
