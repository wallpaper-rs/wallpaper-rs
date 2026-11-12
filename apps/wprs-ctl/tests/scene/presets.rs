use super::{ok, show};

#[test]
fn a_preset_package_and_a_shortcut_property_are_started_from_the_command_line() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "snow", "--from", "preset", "--title", "Calm dusk"]);
    assert_eq!(show(dir, "/manifest/kind"), "preset");
    assert_eq!(show(dir, "/manifest/entry"), "preset.json");
    assert!(dir.join("snow/preset.json").is_file());
    assert!(ok(dir, &["scene", "open", "snow"]).contains("Calm dusk"));
    ok(dir, &["scene", "new", "pond", "--from", "empty", "--title", "Pond"]);
    ok(dir, &["scene", "add-property", "pond", "shortcut", "--id", "open"]);
    let found = ok(dir, &["scene", "show", "pond", "/manifest/properties/open/type"]);
    assert_eq!(found.trim(), "\"shortcut\"");
}
