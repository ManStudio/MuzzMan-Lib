use std::sync::RwLock;

use crate::{element::ElementId, prelude::LocationId};

thread_local! {
    pub static WHO_IAM: RwLock<Iam> = RwLock::new(Iam::MuzzManLib)
}

#[derive(Clone)]
pub enum Iam {
    Element { uid: u128, id: ElementId },
    Location { uid: u128, id: LocationId },
    MuzzManLib,
    Daemon,
}

#[derive(Clone)]
pub struct Record {
    pub time: std::time::SystemTime,
    pub level: log::Level,
    pub log: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

pub struct State {
    pub callbacks: Vec<Box<dyn Fn(&Iam, &Record) + Sync + Send>>,
}

impl State {
    const fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }
    pub fn log(&mut self, who_iam: Iam, record: Record) {
        for callback in self.callbacks.iter() {
            callback(&who_iam, &record)
        }
    }
}

pub static STATE: RwLock<State> = RwLock::new(State::new());

pub struct Logger;
impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let who_iam = WHO_IAM.with(|w| w.read().unwrap().clone());
        let record = Record {
            time: std::time::SystemTime::now(),
            level: record.level(),
            log: std::fmt::format(*record.args()),
            module_path: record.module_path().map(|m| m.to_string()),
            file: record.file().map(|f| f.to_string()),
            line: record.line(),
        };
        STATE.write().unwrap().log(who_iam, record);
    }

    fn flush(&self) {}
}

pub fn init(level: log::LevelFilter) {
    log::set_logger(&Logger);
    log::set_max_level(level);
}
