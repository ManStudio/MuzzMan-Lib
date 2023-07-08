mod session;
mod session_common;
mod session_element;
mod session_location;
mod session_module;

use std::sync::{Arc, RwLock};

use muzzman_lib::prelude::*;

pub type Path = Arc<RwLock<UIDPath>>;

#[derive(Clone, Debug)]
pub enum UIDPath {
    Element(Vec<usize>, usize),
    Location(Vec<usize>),
    Module(usize),
    None,
}

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

#[derive(Clone, Debug)]
pub struct ModuleWraper {
    pub module: Arc<RwLock<Module>>,
    pub path: Path,
}

#[derive(Clone, Debug)]
pub enum Wraper {
    Element(ElementWraper),
    Location(LocationWraper),
    Module(ModuleWraper),
}

pub use session::*;
