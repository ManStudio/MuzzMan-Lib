use muzzman_lib::prelude::*;

use crate::TLocalSession;

impl TSessionCommon for Box<dyn TLocalSession> {
    fn get_name(&self, uid: UID) -> SessionResult<String> {
        todo!()
    }

    fn set_name(&self, uid: UID, name: String) -> SessionResult<()> {
        todo!()
    }

    fn get_desc(&self, uid: UID) -> SessionResult<String> {
        todo!()
    }

    fn set_desc(&self, uid: UID, name: String) -> SessionResult<()> {
        todo!()
    }

    fn emit(&self, uid: UID, event: Event) -> SessionResult<()> {
        todo!()
    }

    fn notify(&self, uid: UID, to: UID, event: Event) -> SessionResult<()> {
        todo!()
    }

    fn subscribe(&self, uid: UID, to: UID) -> SessionResult<()> {
        todo!()
    }

    fn unsubscribe(&self, uid: UID, from: UID) -> SessionResult<()> {
        todo!()
    }

    fn events(&self, uid: UID, consume: bool) -> SessionResult<Vec<Event>> {
        todo!()
    }

    fn push_event(&self, uid: UID, event: Event) -> SessionResult<()> {
        todo!()
    }
}
