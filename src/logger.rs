use std::{io::Write, sync::RwLock};

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

pub struct Record {
    pub time: std::time::Instant,
    pub level: log::Level,
    pub log: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

pub struct State {
    pub logs: Vec<(Iam, Record)>,
}

impl State {
    pub fn log(&mut self, who_iam: Iam, record: Record) {
        self.logs.push((who_iam, record));
    }
}

static STATE: RwLock<State> = RwLock::new(State { logs: Vec::new() });

pub struct Logger;
impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let who_iam = WHO_IAM.with(|w| w.read().unwrap().clone());
        let record = Record {
            time: std::time::Instant::now(),
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

fn init(level: log::LevelFilter) {
    log::set_logger(&Logger);
    log::set_max_level(level);
}
