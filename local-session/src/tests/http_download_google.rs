use crate::LocalSession;
use muzzman_lib::prelude::*;

#[test]
fn main() {
    let mut local_session = LocalSession::new();
    let http = local_session
        .add_module(ModuleSource::Dynamic(
            "../target/debug/libmuzzman_module_http.so".into(),
        ))
        .unwrap();
    let default_location = local_session.get_default_location().unwrap();
    let element = default_location.create_element("HTTP".into()).unwrap();
    element.set_module(Some(http)).unwrap();
    element.set_url("http://google.com".into()).unwrap();
    element.set_enabled(true).unwrap();
    let path = element.get_path().unwrap();
    println!("Path: {path:?}");
    while !element.is_completed().unwrap() && !element.is_error().unwrap() {
        println!("Waiting");
    }
}
