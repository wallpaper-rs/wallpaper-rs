use crate::fetch::is_gone;
use wprs_client::Error;
use wprs_core::dbus::error::NOT_FOUND;

fn failure(name: &str) -> Error {
    Error { name: name.into(), message: "playlist \"01M7853NKY81WMYFBXFZMH8R25\" not found".into() }
}

#[test]
fn a_playlist_that_was_deleted_is_gone_and_not_an_error_for_the_status_bar() {
    let playlist = ("Playlists1", "Get", "01M7853NKY81WMYFBXFZMH8R25".to_owned());
    assert!(is_gone(&playlist, &failure(NOT_FOUND)));
    assert!(!is_gone(&playlist, &failure("org.freedesktop.DBus.Error.Failed")));
    let search = ("Library1", "Search", String::new());
    assert!(!is_gone(&search, &failure(NOT_FOUND)));
}
