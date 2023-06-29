use crate::prelude::SessionError;

pub type UID = u64;
pub type SessionResult<T> = std::result::Result<T, SessionError>;

pub enum Event {}
