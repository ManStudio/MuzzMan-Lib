pub type UID = u64;
pub type SessionResult<T> = std::result::Result<T, SessionError>;

#[derive(Clone, Copy, Debug)]
pub enum SessionError {
    InvalidUID,
    UIDIsNotAModule,
    UIDIsNotALocation,
    UIDIsNotAElement,

    NoPermission,
}

pub enum Event {}
