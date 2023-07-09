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

    fn get_buffer_size(&self) -> SessionResult<usize>;
    fn set_buffer_size(&self, size: usize) -> SessionResult<()>;

    fn remaining(&self) -> SessionResult<usize>;
    fn read(&self, len: usize) -> SessionResult<Vec<u8>>;
    fn write(&self, data: &[u8]) -> SessionResult<usize>;
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

    fn get_buffer_size(&self) -> SessionResult<usize> {
        let session = self.get_session()?;
        session.get_buffer_size(self.uid)
    }

    fn set_buffer_size(&self, size: usize) -> SessionResult<()> {
        let session = self.get_session()?;
        session.set_buffer_size(self.uid, size)
    }

    fn remaining(&self) -> SessionResult<usize> {
        let session = self.get_session()?;
        session.remaining(self.uid)
    }

    fn read(&self, len: usize) -> SessionResult<Vec<u8>> {
        let session = self.get_session()?;
        session.read(self.uid, len)
    }

    fn write(&self, data: &[u8]) -> SessionResult<usize> {
        let session = self.get_session()?;
        session.write(self.uid, data)
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

    fn get_buffer_size(&self) -> SessionResult<usize> {
        let session = self.get_session()?;
        session.get_buffer_size(self.uid)
    }

    fn set_buffer_size(&self, size: usize) -> SessionResult<()> {
        let session = self.get_session()?;
        session.set_buffer_size(self.uid, size)
    }

    fn remaining(&self) -> SessionResult<usize> {
        let session = self.get_session()?;
        session.remaining(self.uid)
    }

    fn read(&self, len: usize) -> SessionResult<Vec<u8>> {
        let session = self.get_session()?;
        session.read(self.uid, len)
    }

    fn write(&self, data: &[u8]) -> SessionResult<usize> {
        let session = self.get_session()?;
        session.write(self.uid, data)
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

    fn get_buffer_size(&self) -> SessionResult<usize> {
        let session = self.get_session()?;
        session.get_buffer_size(self.uid)
    }

    fn set_buffer_size(&self, size: usize) -> SessionResult<()> {
        let session = self.get_session()?;
        session.set_buffer_size(self.uid, size)
    }

    fn remaining(&self) -> SessionResult<usize> {
        let session = self.get_session()?;
        session.remaining(self.uid)
    }

    fn read(&self, len: usize) -> SessionResult<Vec<u8>> {
        let session = self.get_session()?;
        session.read(self.uid, len)
    }

    fn write(&self, data: &[u8]) -> SessionResult<usize> {
        let session = self.get_session()?;
        session.write(self.uid, data)
    }
}
