use futures::FutureExt;
use hyper::http::request;
use hyper::Body;
use hyper::Method;
use hyper::Request;
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
        let status = element.read().unwrap().status;
        element.write().unwrap().statuses = ["Connecting", "Downloading", "Uploading", "Completed"]
            .into_iter()
            .map(|e| e.to_string())
            .collect();
        let method = element
            .read()
            .unwrap()
            .settings
            .get("Method")
            .unwrap()
            .value
            .to_string();
        let uri = element.read().unwrap().url.clone();
        match status {
            0 => {
                let mut client = hyper::Client::new();
                let mut request = client.request(
                    Request::builder()
                        .method(Method::from_bytes(method.as_bytes()).unwrap())
                        .uri(uri)
                        .body(Body::empty())
                        .unwrap(),
                );
                match request.poll_unpin(ctx) {
                    std::task::Poll::Ready(res) => {
                        println!("Res: {res:?}");
                    }
                    std::task::Poll::Pending => {
                        println!("Pending");
                    }
                }
                // Connecting
            }
            1 => {
                // Downloading
            }
            2 => {
                // Uploading
            }
            3 => {
                // Completed
                let id = element.read().unwrap().id.clone();
                id.set_enabled(false);
                element.write().unwrap().is_completed = true;
            }
            _ => return Err(SessionError::Custom("Invalid status!".into())),
        }
        Ok(())
    }

    fn poll_location(
        &self,
        ctx: &mut std::task::Context<'_>,
        location: std::sync::Arc<std::sync::RwLock<Location>>,
        storage: &mut Storage,
    ) -> SessionResult<()> {
        Err(SessionError::Custom(
            "HTTP is not implemented for an Location".into(),
        ))
    }

    fn element_on_event(
        &self,
        element: std::sync::Arc<std::sync::RwLock<Element>>,
        event: Event,
        storage: &mut Storage,
    ) -> SessionResult<()> {
        Ok(())
    }

    fn location_on_event(
        &self,
        location: std::sync::Arc<std::sync::RwLock<Location>>,
        event: Event,
        storage: &mut Storage,
    ) -> SessionResult<()> {
        Ok(())
    }

    fn default_element_settings(&self) -> Settings {
        let mut settings = Settings::default();
        settings.add(
            "Method",
            Setting::new(
                "GET",
                vec![
                    "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
                ],
                "The HTTP Method to use!",
            ),
        );
        settings.add(
            "Port",
            Setting::new(
                80,
                Vec::<i32>::new(),
                "The port that should be used to connect to the server",
            ),
        );
        settings.add("Headers", Setting::new("User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36\r\n", Vec::<String>::new(), "The HTTP Methods that should use, should be valid every one should be separated by `\\r\\n`"));
        settings
    }

    fn default_location_settings(&self) -> Settings {
        let mut settings = Settings::default();
        settings
    }

    fn supports_protocols(&self) -> &[&'static str] {
        &["http", "https"]
    }

    fn supports_extensions(&self) -> &[&'static str] {
        &[]
    }
}
