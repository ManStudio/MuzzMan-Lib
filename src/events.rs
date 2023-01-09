use std::time::SystemTime;

use crate::prelude::*;

#[derive(Clone)]
pub struct Events {
    pub events: [Option<(SystemTime, Event)>; 20],
    pub cursour: usize,

    pub subscribers: Vec<Ref>,
}

#[derive(Clone)]
pub enum Event {
    Element(ERef, ElementNotify),
    Location(LRef, LocationNotify),
    Log(Log),
}

impl Events {
    pub fn new_event(&mut self, event: Event) {
        self.events[self.cursour] = Some((SystemTime::now(), event.clone()));
        self.cursour += 1;
        if self.cursour > 19 {
            self.cursour = 0;
        }

        for subscriber in self.subscribers.iter() {
            subscriber.notify(event.clone());
        }
    }
}
