use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

use crate::{
    common::Common,
    events::Event,
    prelude::{ERef, LRef},
    types::Ref,
};

#[derive(Clone, Debug)]
pub enum Log {
    Info(String),
    Warning(String),
    Error(String),
}

pub trait TLogger {
    fn info(&mut self, data: impl Into<String>);
    fn warn(&mut self, data: impl Into<String>);
    fn error(&mut self, data: impl Into<String>);

    fn flush(&mut self);
    fn set_instant(&mut self, instant: bool);
}

pub trait TGetLogger {
    fn get_logger(&self, dst: Option<Arc<Mutex<File>>>) -> Logger;
}

pub struct Logger {
    dst: Option<Arc<Mutex<File>>>,
    logs: Vec<Log>,
    _ref: Ref,
    instant: bool,
}

unsafe impl Sync for Logger {}
unsafe impl Send for Logger {}

impl Logger {
    pub fn for_location(dst: Option<Arc<Mutex<File>>>, _ref: LRef) -> Self {
        Self {
            dst,
            _ref: Ref::Location(_ref),
            instant: false,
            logs: Vec::new(),
        }
    }

    pub fn for_element(dst: Option<Arc<Mutex<File>>>, _ref: ERef) -> Self {
        Self {
            dst,
            _ref: Ref::Element(_ref),
            instant: false,
            logs: Vec::new(),
        }
    }
}

impl TLogger for Logger {
    fn info(&mut self, data: impl Into<String>) {
        let data: String = data.into();

        if let Some(dst) = &self.dst {
            let _ = write!(dst.lock().unwrap(), "Info: {}", data);
        }

        self.logs.push(Log::Info(data));

        if self.instant {
            self.flush()
        }
    }

    fn warn(&mut self, data: impl Into<String>) {
        let data: String = data.into();

        if let Some(dst) = &self.dst {
            let _ = write!(dst.lock().unwrap(), "Warning: {}", data);
        }

        self.logs.push(Log::Warning(data));

        if self.instant {
            self.flush()
        }
    }

    fn error(&mut self, data: impl Into<String>) {
        let data: String = data.into();

        if let Some(dst) = &self.dst {
            let _ = write!(dst.lock().unwrap(), "Error: {}", data);
        }

        self.logs.push(Log::Error(data));

        if self.instant {
            self.flush()
        }
    }

    fn flush(&mut self) {
        for log in self.logs.iter() {
            let _ = self._ref.emit(Event::Log(self._ref.clone(), log.clone()));
        }
        self.logs.clear();
    }

    fn set_instant(&mut self, instant: bool) {
        self.instant = instant;
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.flush();
    }
}
