use crate::prelude::*;
use crate::LocalSession;

#[test]
fn add_location() -> Result<(), SessionError> {
    let session = LocalSession::default().new_session();
    let default_location = session.get_default_location()?;
    let new_location = default_location.create_location("NewLocation")?;
    let _location_in_new_location = new_location.create_location("OtherLocation")?;
    let info = default_location.get_location_info()?;

    assert_eq!(info.locations.len(), 1);
    assert_eq!(info.locations[0].name, "NewLocation".to_string());
    assert_eq!(
        info.locations[0].locations[0].name,
        "OtherLocation".to_string()
    );

    Ok(())
}

#[test]
fn save_and_load() -> Result<(), SessionError> {
    let session = LocalSession::default().new_session();
    let default_location = session.get_default_location()?;
    let new_location = default_location.create_location("NewLocation")?;

    let saved_default_location = default_location.get_location_info()?;
    let _ = new_location.destroy();

    session.load_location_info(saved_default_location)?;

    let new_location = default_location.get_locations(0..1)?[0].clone();
    assert_eq!(new_location.get_name()?, "NewLocation".to_string());

    Ok(())
}
