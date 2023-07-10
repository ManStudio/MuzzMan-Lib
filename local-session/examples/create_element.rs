use muzzman_lib::prelude::*;
use muzzman_local_session::LocalSession;

fn main() {
    let local_session = LocalSession::new();
    let default_location = local_session.get_default_location().unwrap();
    let new_element = default_location
        .create_element("NewElement".into())
        .unwrap();

    assert_eq!(new_element.get_name().unwrap(), "NewElement".to_string());
    assert_eq!(new_element.get_parent().unwrap(), default_location);
}
