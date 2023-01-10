use crate::{events::Event, prelude::SessionError, types::Ref};

pub trait Common {
    fn get_name(&self) -> Result<String, SessionError>;
    fn set_name(&self, name: impl Into<String>) -> Result<(), SessionError>;

    fn get_desc(&self) -> Result<String, SessionError>;
    fn set_desc(&self, desc: impl Into<String>) -> Result<(), SessionError>;

    fn notify(&self, event: Event) -> Result<(), SessionError>;

    fn emit(&self, event: Event) -> Result<(), SessionError>;
    fn subscribe(&self, _ref: Ref) -> Result<(), SessionError>;
    fn unsubscribe(&self, _ref: Ref) -> Result<(), SessionError>;
}

impl Common for Ref {
    fn get_name(&self) -> Result<String, crate::prelude::SessionError> {
        match self {
            Ref::Element(e) => e.get_name(),
            Ref::Location(l) => l.get_name(),
        }
    }

    fn set_name(&self, name: impl Into<String>) -> Result<(), crate::prelude::SessionError> {
        match self {
            Ref::Element(e) => e.set_name(name),
            Ref::Location(l) => l.set_name(name),
        }
    }

    fn get_desc(&self) -> Result<String, crate::prelude::SessionError> {
        match self {
            Ref::Element(e) => e.get_desc(),
            Ref::Location(l) => l.get_desc(),
        }
    }

    fn set_desc(&self, desc: impl Into<String>) -> Result<(), crate::prelude::SessionError> {
        match self {
            Ref::Element(e) => e.set_desc(desc),
            Ref::Location(l) => l.set_desc(desc),
        }
    }

    fn notify(&self, event: crate::events::Event) -> Result<(), crate::prelude::SessionError> {
        match self {
            Ref::Element(e) => e.notify(event),
            Ref::Location(l) => l.notify(event),
        }
    }

    fn emit(&self, event: Event) -> Result<(), SessionError> {
        match self {
            Ref::Element(e) => e.emit(event),
            Ref::Location(l) => l.emit(event),
        }
    }

    fn subscribe(&self, _ref: Ref) -> Result<(), SessionError> {
        match self {
            Ref::Element(e) => e.subscribe(_ref),
            Ref::Location(l) => l.subscribe(_ref),
        }
    }

    fn unsubscribe(&self, _ref: Ref) -> Result<(), SessionError> {
        match self {
            Ref::Element(e) => e.unsubscribe(_ref),
            Ref::Location(l) => l.unsubscribe(_ref),
        }
    }
}
