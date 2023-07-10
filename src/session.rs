use crate::prelude::*;

pub trait TSession: TSessionCommon + TSessionElement + TSessionLocation + TSessionModule {
    fn version(&self) -> SessionResult<u64>;
    fn version_str(&self) -> SessionResult<String>;
    fn clone_box(&self) -> Box<dyn TSession>;
}

pub struct Session {
    pub session: Box<dyn TSession>,
}

impl Clone for Session {
    fn clone(&self) -> Self {
        Self {
            session: self.session.clone_box(),
        }
    }
}

impl std::ops::Deref for Session {
    type Target = Box<dyn TSession>;

    fn deref(&self) -> &Self::Target {
        &self.session
    }
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session").finish()
    }
}

impl From<Box<dyn TSession>> for Session {
    fn from(session: Box<dyn TSession>) -> Self {
        Self { session }
    }
}

impl From<Session> for Box<dyn TSession> {
    fn from(session: Session) -> Self {
        session.session
    }
}
