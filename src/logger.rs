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

unsafe impl Sync for Logger {}
unsafe impl Send for Logger {}

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

        if let Some(dst) = &self.dst {
            let _ = write!(dst.lock().unwrap(), "Info: {}", data);
        }

        let _ = self
            ._ref
            .notify(Event::Log(self._ref.clone(), Log::Info(data)));
    }

    fn warn(&mut self, data: impl Into<String>) {
        let data: String = data.into();

        if let Some(dst) = &self.dst {
            let _ = write!(dst.lock().unwrap(), "Warning: {}", data);
        }

        let _ = self
            ._ref
            .notify(Event::Log(self._ref.clone(), Log::Warning(data)));
    }

    fn error(&mut self, data: impl Into<String>) {
        let data: String = data.into();

        if let Some(dst) = &self.dst {
            let _ = write!(dst.lock().unwrap(), "Error: {}", data);
        }

        let _ = self
            ._ref
            .notify(Event::Log(self._ref.clone(), Log::Error(data)));
    }
}
