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

#[test]
fn save_and_load_when_has_content() -> Result<(), SessionError> {
    let session = LocalSession::default().new_session();
    let default_location = session.get_default_location()?;
    let new_location = default_location.create_location("NewLocation")?;

    let saved_default_location = default_location.get_location_info()?;
    new_location.set_desc("This is the original")?;

    session.load_location_info(saved_default_location)?;

    // the last default should steel have the NewLocation with desc "This is the original" but default location now is inside the new default location
    // only in new default location wee have NewLocation with desc ""
    let locations = default_location.get_locations(0..1)?;
    assert_eq!(locations[0].get_name()?, "NewLocation".to_string());
    assert_eq!(locations[0].get_desc()?, "This is the original".to_string());
    drop(default_location);
    let default_location = session.get_default_location()?;

    // the first should be replaced with the new one because the last one is empty

    let locations = default_location.get_locations(0..1)?;
    assert_eq!(locations[0].get_name()?, "NewLocation".to_string());
    assert_eq!(locations[0].get_desc()?, "".to_string());

    Ok(())
}
