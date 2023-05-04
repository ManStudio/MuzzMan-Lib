pub mod action;
pub mod common;
pub mod data;
pub mod element;
pub mod enums;
pub mod events;
pub mod local_session;
pub mod location;
pub mod logger;
pub mod module;
pub mod session;
pub mod storage;
pub mod types;

pub const VERSION: u64 = 1;

pub extern crate muzzman_lib_macros;

pub use local_session::LocalSession;

pub mod prelude {
    pub use crate::{
        action::*, common::*, data::*, element::*, enums::*, events::*, location::*, logger,
        module::*, session::*, storage::*, types::*,
    };
    pub use get_ref::*;
    pub use muzzman_lib_macros::*;
    pub use std::any::Any;
}
