use std::{collections::HashMap, path::PathBuf};

use crate::prelude::*;

pub struct Element {
    pub name: String,
    pub desc: String,
    pub data: HashMap<String, Atom>,
    pub settings: Settings,
    pub path: PathBuf,
    pub module: Option<ModuleId>,
    pub id: ElementId,

    pub parent: LocationId,

    pub stream: Stream,

    pub progress: f32,
    /// This should only be updated per seccond from download_speed_counter
    pub download_speed: usize,
    /// This should only be updated per seccond from upload_speed_counter
    pub upload_speed: usize,

    pub download_speed_counter: usize,
    pub upload_speed_counter: usize,

    pub total_download: usize,
    pub total_upload: usize,
}
