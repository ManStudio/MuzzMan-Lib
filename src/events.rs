use std::time::SystemTime;

use crate::prelude::*;
use bytes_kman::TBytes;

#[derive(Default, Clone, Debug)]
pub struct Events {
    pub events: [Option<(SystemTime, Event)>; 20],
    pub cursour: usize,

    pub subscribers: Vec<ID>,
}

#[derive(Clone, Debug, bytes_kman::Bytes)]
pub enum SessionEvent {
    NewElement(ElementPath),
    NewLocation(LocationPath),
    NewModule(ModulePath),

    DestroyedElement(ElementPath),
    DestroyedLocation(LocationPath),
    DestroyedModule(ModulePath),

    // old, new
    ElementIdChanged(ElementPath, ElementPath),
    LocationIdChanged(LocationPath, LocationPath),
    ModuleIdChanged(ModulePath, ModulePath),
}

#[derive(Clone, Debug, bytes_kman::Bytes)]
pub enum Event {
    Element(ElementPath, ElementNotify),
    Location(LocationPath, LocationNotify),
    SessionEvent(SessionEvent),
}

impl Events {
    pub fn new_event(&mut self, event: Event, session: Box<dyn TSession>) {
        self.events[self.cursour] = Some((SystemTime::now(), event.clone()));
        self.cursour += 1;
        if self.cursour > 19 {
            self.cursour = 0;
        }

        self.subscribers.retain(|subscriber| {
            if let Ok(subscriber) = &subscriber.get_ref(session.as_ref()) {
                let _ = subscriber.notify(event.clone());
                true
            } else {
                false
            }
        })
    }

    pub fn is_subscribed(&self, _ref: &ID) -> bool {
        for r in self.subscribers.iter() {
            if r == _ref {
                return true;
            }
        }
        false
    }

    pub fn subscribe(&mut self, _ref: ID) -> bool {
        if self.is_subscribed(&_ref) {
            return false;
        }

        self.subscribers.push(_ref);

        true
    }

    pub fn unsubscribe(&mut self, _ref: ID) -> bool {
        if !self.is_subscribed(&_ref) {
            return false;
        }

        self.subscribers.retain(|e| *e != _ref);

        true
    }
}
