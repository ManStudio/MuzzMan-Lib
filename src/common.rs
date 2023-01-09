use crate::{events::Event, prelude::SessionError};

pub trait Common {
    fn get_name(&self) -> Result<String, SessionError>;
    fn set_name(&self, name: impl Into<String>) -> Result<(), SessionError>;

    fn get_desc(&self) -> Result<String, SessionError>;
    fn set_desc(&self, desc: impl Into<String>) -> Result<(), SessionError>;

    fn notify(&self, event: Event) -> Result<(), SessionError>;
}
