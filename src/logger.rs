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
}

pub struct Logger {
    dst: Option<Arc<Mutex<File>>>,
    _ref: Ref,
}

impl Logger {
    pub fn for_location(dst: Option<Arc<Mutex<File>>>, _ref: LRef) -> Self {
        Self {
            dst,
            _ref: Ref::Location(_ref),
        }
    }

    pub fn for_element(dst: Option<Arc<Mutex<File>>>, _ref: ERef) -> Self {
        Self {
            dst,
            _ref: Ref::Element(_ref),
        }
    }
}

impl TLogger for Logger {
    fn info(&mut self, data: impl Into<String>) {
        let data: String = data.into();

        if let Some(dst) = self.dst {
            write!(dst.lock().unwrap(), "Info: {}", data);
        }

        self._ref.notify(Event::Log(Log::Info(data)));
    }

    fn warn(&mut self, data: impl Into<String>) {
        let data: String = data.into();

        if let Some(dst) = self.dst {
            write!(dst.lock().unwrap(), "Warning: {}", data);
        }

        self._ref.notify(Event::Log(Log::Warning(data)));
    }

    fn error(&mut self, data: impl Into<String>) {
        let data: String = data.into();

        if let Some(dst) = self.dst {
            write!(dst.lock().unwrap(), "Error: {}", data);
        }

        self._ref.notify(Event::Log(Log::Error(data)));
    }
}
