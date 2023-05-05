use std::sync::{Arc, RwLock};

use once_cell::sync::Lazy;

use crate::{element::ElementId, prelude::LocationId};

thread_local! {
    #[no_mangle]
    pub static WHO_IAM: RwLock<Arc<RwLock<Iam>>> = RwLock::new(Arc::new(RwLock::new(Iam::MuzzManLib)));
}

#[no_mangle]
pub static LOGGER_STATE: Lazy<Arc<RwLock<State>>> =
    Lazy::new(|| Arc::new(RwLock::new(State::new())));

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

pub type Callback = Box<dyn Fn(&Iam, &Record) + Sync + Send>;

pub struct State {
    pub callbacks: Vec<Callback>,
    pub log_level: log::LevelFilter,
}

impl State {
    const fn new() -> Self {
        Self {
            callbacks: Vec::new(),
            log_level: log::LevelFilter::Off,
        }
    }
    pub fn log(&mut self, who_iam: Iam, record: Record) {
        for callback in self.callbacks.iter() {
            callback(&who_iam, &record)
        }
    }
    pub fn register_callback(&mut self, callback: Callback) {
        self.callbacks.push(callback)
    }
}

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
        LOGGER_STATE
            .write()
            .unwrap()
            .log(who_iam.read().unwrap().clone(), record);
    }

    fn flush(&self) {}
}

pub fn init() {
    let level = LOGGER_STATE.read().unwrap().log_level;
    log::set_logger(&Logger);
    log::set_max_level(level);
}
