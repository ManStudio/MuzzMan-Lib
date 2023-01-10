use std::time::SystemTime;

use crate::prelude::*;

#[derive(Default, Clone, Debug)]
pub struct Events {
    pub events: [Option<(SystemTime, Event)>; 20],
    pub cursour: usize,

    pub subscribers: Vec<Ref>,
}

#[derive(Clone, Debug)]
pub enum SessionEvent {
    NewElement(ERef),
    NewLocation(LRef),
    NewModule(MRef),
    DestroyedElement(ERef),
    DestroyedLocation(LRef),
    DestroyedModule(MRef),
}

#[derive(Clone, Debug)]
pub enum Event {
    Element(ERef, ElementNotify),
    Location(LRef, LocationNotify),
    Log(Ref, Log),
    SessionEvent(SessionEvent),
}

impl Events {
    pub fn new_event(&mut self, event: Event) {
        self.events[self.cursour] = Some((SystemTime::now(), event.clone()));
        self.cursour += 1;
        if self.cursour > 19 {
            self.cursour = 0;
        }

        for subscriber in self.subscribers.iter() {
            let _ = subscriber.notify(event.clone());
        }
    }

    pub fn is_subscribed(&self, _ref: &Ref) -> bool {
        for r in self.subscribers.iter() {
            if r == _ref {
                return true;
            }
        }
        false
    }

    pub fn subscribe(&mut self, _ref: Ref) -> bool {
        if self.is_subscribed(&_ref) {
            return false;
        }

        self.subscribers.push(_ref);

        true
    }

    pub fn unsubscribe(&mut self, _ref: Ref) -> bool {
        if !self.is_subscribed(&_ref) {
            return false;
        }

        self.subscribers.retain(|e| *e != _ref);

        true
    }
}
