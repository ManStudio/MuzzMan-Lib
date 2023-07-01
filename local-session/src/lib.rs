use std::sync::{Arc, RwLock};

use muzzman_lib::prelude::*;

pub struct LocalSession {
    pub location: Arc<RwLock<Location>>,
}
