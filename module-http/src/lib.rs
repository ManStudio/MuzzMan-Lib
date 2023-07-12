use muzzman_lib::prelude::*;
use muzzman_lib::Storage;
pub use std::sync::{Arc, RwLock};

#[module_link]
pub struct ModuleHttp;

impl TModule for ModuleHttp {
    fn name(&self) -> &str {
        "HTTP"
    }

    fn desc(&self) -> &str {
        "Implementation of HTTP for MuzzMan"
    }

    fn id(&self) -> u64 {
        1
    }

    fn version(&self) -> u64 {
        1
    }

    fn supported_versions(&self) -> &'static [u64] {
        &[1]
    }

    fn poll_element(
        &self,
        ctx: &mut std::task::Context<'_>,
        element: std::sync::Arc<std::sync::RwLock<Element>>,
        storage: &mut Storage,
    ) -> SessionResult<()> {
        todo!()
    }

    fn poll_location(
        &self,
        ctx: &mut std::task::Context<'_>,
        location: std::sync::Arc<std::sync::RwLock<Location>>,
        storage: &mut Storage,
    ) -> SessionResult<()> {
        todo!()
    }

    fn element_on_event(
        &self,
        element: std::sync::Arc<std::sync::RwLock<Element>>,
        event: Event,
        storage: &mut Storage,
    ) -> SessionResult<()> {
        todo!()
    }

    fn location_on_event(
        &self,
        location: std::sync::Arc<std::sync::RwLock<Location>>,
        event: Event,
        storage: &mut Storage,
    ) -> SessionResult<()> {
        todo!()
    }

    fn default_element_settings(&self) -> Settings {
        let mut settings = Settings::default();
        settings
    }

    fn default_location_settings(&self) -> Settings {
        let mut settings = Settings::default();
        settings
    }

    fn supports_protocols(&self) -> &[&'static str] {
        &["http:", "https:"]
    }

    fn supports_extensions(&self) -> &[&'static str] {
        &[]
    }
}
