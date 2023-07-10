use crate::types::UID;
use bytes_kman::prelude::*;
use once_cell::sync::Lazy;
use std::{
    sync::{Arc, RwLock},
    thread::LocalKey,
};

thread_local! {
    static WHO_IAM: RwLock<Iam> = RwLock::new(Iam::MuzzManLib);
}

#[no_mangle]
pub static LOGGER_WHO_IAM: Lazy<Arc<LocalKey<RwLock<Iam>>>> = Lazy::new(|| Arc::new(WHO_IAM));

#[no_mangle]
pub static LOGGER_STATE: Lazy<Arc<RwLock<State>>> =
    Lazy::new(|| Arc::new(RwLock::new(State::new())));

#[derive(Clone)]
pub enum Iam {
    Element(UID),
    Location(UID),
    MuzzManLib,
    Session,
}

#[derive(Clone, Copy, Bytes)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Clone, Bytes)]
pub struct Record {
    pub time: f64,
    pub level: Level,
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
        let who_iam = LOGGER_WHO_IAM.with(|w| w.read().unwrap().clone());
        let record = Record {
            time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            level: match record.level() {
                log::Level::Error => Level::Error,
                log::Level::Warn => Level::Warn,
                log::Level::Info => Level::Info,
                log::Level::Debug => Level::Debug,
                log::Level::Trace => Level::Trace,
            },
            log: std::fmt::format(*record.args()),
            module_path: record.module_path().map(|m| m.to_string()),
            file: record.file().map(|f| f.to_string()),
            line: record.line(),
        };
        LOGGER_STATE.write().unwrap().log(who_iam, record);
    }

    fn flush(&self) {}
}

pub fn init() {
    let level = LOGGER_STATE.read().unwrap().log_level;
    let _ = log::set_logger(&Logger);
    log::set_max_level(level);
}
