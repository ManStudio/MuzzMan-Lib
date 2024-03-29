use std::{collections::HashMap, path::PathBuf};

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Element {
    pub name: String,
    pub desc: String,
    pub data: HashMap<String, Atom>,
    pub settings: Settings,
    pub path: PathBuf,
    pub module: Option<ModuleId>,
    pub id: ElementId,

    pub url: String,

    pub parent: LocationId,

    pub stream: Stream,

    /// From this will be readed when TSessionCommon::read
    pub buffer: Vec<u8>,

    pub status: usize,
    pub statuses: Vec<String>,

    pub progress: f32,
    /// This should only be updated per seccond from download_speed_counter
    pub download_speed: usize,
    /// This should only be updated per seccond from upload_speed_counter
    pub upload_speed: usize,

    pub download_speed_counter: usize,
    pub upload_speed_counter: usize,

    pub total_download: usize,
    pub total_upload: usize,

    pub enabled: bool,
    pub is_error: bool,
    pub is_completed: bool,
}

impl Element {
    pub fn get_session(&self) -> Option<Session> {
        self.id.session.clone()
    }
}
