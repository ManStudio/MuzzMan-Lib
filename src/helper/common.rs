use crate::prelude::*;

pub trait TCommonHelper {
    fn get_name(&self) -> SessionResult<String>;
    fn set_name(&self, name: String) -> SessionResult<()>;

    fn get_desc(&self) -> SessionResult<String>;
    fn set_desc(&self, desc: String) -> SessionResult<()>;

    fn emit(&self, event: Event) -> SessionResult<()>;
    fn notify(&self, to: UID, event: Event) -> SessionResult<()>;

    fn subscribe(&self, to: UID) -> SessionResult<()>;
    fn unsubscribe(&self, from: UID) -> SessionResult<()>;

    fn events(&self, consume: bool) -> SessionResult<Vec<Event>>;
    fn push_event(&self, event: Event) -> SessionResult<()>;
}

impl TCommonHelper for LocationId {
    fn get_name(&self) -> SessionResult<String> {
        let session = self.get_session()?;
        session.get_name(self.uid)
    }

    fn set_name(&self, name: String) -> SessionResult<()> {
        let session = self.get_session()?;
        session.set_name(self.uid, name)
    }

    fn get_desc(&self) -> SessionResult<String> {
        let session = self.get_session()?;
        session.get_desc(self.uid)
    }

    fn set_desc(&self, desc: String) -> SessionResult<()> {
        let session = self.get_session()?;
        session.set_desc(self.uid, desc)
    }

    fn emit(&self, event: Event) -> SessionResult<()> {
        let session = self.get_session()?;
        session.emit(self.uid, event)
    }

    fn notify(&self, to: UID, event: Event) -> SessionResult<()> {
        let session = self.get_session()?;
        session.notify(self.uid, to, event)
    }

    fn subscribe(&self, to: UID) -> SessionResult<()> {
        let session = self.get_session()?;
        session.subscribe(self.uid, to)
    }

    fn unsubscribe(&self, from: UID) -> SessionResult<()> {
        let session = self.get_session()?;
        session.unsubscribe(self.uid, from)
    }

    fn events(&self, consume: bool) -> SessionResult<Vec<Event>> {
        let session = self.get_session()?;
        session.events(self.uid, consume)
    }

    fn push_event(&self, event: Event) -> SessionResult<()> {
        let session = self.get_session()?;
        session.push_event(self.uid, event)
    }
}

impl TCommonHelper for ElementId {
    fn get_name(&self) -> SessionResult<String> {
        let session = self.get_session()?;
        session.get_name(self.uid)
    }

    fn set_name(&self, name: String) -> SessionResult<()> {
        let session = self.get_session()?;
        session.set_name(self.uid, name)
    }

    fn get_desc(&self) -> SessionResult<String> {
        let session = self.get_session()?;
        session.get_desc(self.uid)
    }

    fn set_desc(&self, desc: String) -> SessionResult<()> {
        let session = self.get_session()?;
        session.set_desc(self.uid, desc)
    }

    fn emit(&self, event: Event) -> SessionResult<()> {
        let session = self.get_session()?;
        session.emit(self.uid, event)
    }

    fn notify(&self, to: UID, event: Event) -> SessionResult<()> {
        let session = self.get_session()?;
        session.notify(self.uid, to, event)
    }

    fn subscribe(&self, to: UID) -> SessionResult<()> {
        let session = self.get_session()?;
        session.subscribe(self.uid, to)
    }

    fn unsubscribe(&self, from: UID) -> SessionResult<()> {
        let session = self.get_session()?;
        session.unsubscribe(self.uid, from)
    }

    fn events(&self, consume: bool) -> SessionResult<Vec<Event>> {
        let session = self.get_session()?;
        session.events(self.uid, consume)
    }

    fn push_event(&self, event: Event) -> SessionResult<()> {
        let session = self.get_session()?;
        session.push_event(self.uid, event)
    }
}

impl TCommonHelper for ModuleId {
    fn get_name(&self) -> SessionResult<String> {
        let session = self.get_session()?;
        session.get_name(self.uid)
    }

    fn set_name(&self, name: String) -> SessionResult<()> {
        let session = self.get_session()?;
        session.set_name(self.uid, name)
    }

    fn get_desc(&self) -> SessionResult<String> {
        let session = self.get_session()?;
        session.get_desc(self.uid)
    }

    fn set_desc(&self, desc: String) -> SessionResult<()> {
        let session = self.get_session()?;
        session.set_desc(self.uid, desc)
    }

    fn emit(&self, event: Event) -> SessionResult<()> {
        let session = self.get_session()?;
        session.emit(self.uid, event)
    }

    fn notify(&self, to: UID, event: Event) -> SessionResult<()> {
        let session = self.get_session()?;
        session.notify(self.uid, to, event)
    }

    fn subscribe(&self, to: UID) -> SessionResult<()> {
        let session = self.get_session()?;
        session.subscribe(self.uid, to)
    }

    fn unsubscribe(&self, from: UID) -> SessionResult<()> {
        let session = self.get_session()?;
        session.unsubscribe(self.uid, from)
    }

    fn events(&self, consume: bool) -> SessionResult<Vec<Event>> {
        let session = self.get_session()?;
        session.events(self.uid, consume)
    }

    fn push_event(&self, event: Event) -> SessionResult<()> {
        let session = self.get_session()?;
        session.push_event(self.uid, event)
    }
}
