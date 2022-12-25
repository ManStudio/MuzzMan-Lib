#![feature(io_error_more)]

pub mod action;
pub mod data;
pub mod element;
pub mod enums;
pub mod local_session;
pub mod location;
pub mod module;
pub mod session;
pub mod storage;
pub mod types;

pub extern crate muzzman_lib_macros;

pub mod prelude {
    pub use crate::{
        action::*, data::*, element::*, enums::*, location::*, module::*, session::*, storage::*,
        types::*,
    };
    pub use get_ref::*;
    pub use muzzman_lib_macros::*;
    pub use std::any::Any;
    pub use url::Url;
}
