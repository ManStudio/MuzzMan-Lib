mod session;
mod session_common;
mod session_element;
mod session_location;
mod session_module;

use std::sync::{Arc, RwLock};

use muzzman_lib::prelude::*;

pub type Path = Arc<RwLock<Option<Vec<usize>>>>;

#[derive(Clone, Debug)]
pub struct ElementWraper {
    pub element: Arc<RwLock<Element>>,
    pub path: Path,
}

#[derive(Clone, Debug)]
pub struct LocationWraper {
    pub location: Arc<RwLock<Location>>,
    pub locations: Arc<RwLock<Vec<LocationWraper>>>,
    pub elements: Arc<RwLock<Vec<ElementWraper>>>,
    pub path: Path,
}

pub use session::*;
