mod removing;

use super::{fails, ok, show};

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

#[test]
fn a_preset_is_started_with_its_base_values_license_and_picture_in_one_command() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    std::fs::write(dir.join("values.json"), r#"{"speed": 0.4}"#).unwrap();
    image::RgbImage::new(8, 8).save(dir.join("sky.png")).unwrap();
    ok(dir, &["scene", "new", "calm", "--from", "preset", "--base", "org.example.dusk", "--range", "^1.2"]);
    ok(
        dir,
        &[
            "scene",
            "new",
            "calm2",
            "--from",
            "preset",
            "--base",
            "org.example.dusk",
            "--values",
            "values.json",
            "--license",
            "CC0-1.0",
            "--preview",
            "sky.png",
        ],
    );
    let at = |name: &str, pointer: &str| -> serde_json::Value {
        serde_json::from_str(&ok(dir, &["scene", "show", name, pointer])).unwrap()
    };
    assert_eq!(at("calm", "/preset/base/version"), "^1.2");
    assert_eq!(at("calm2", "/preset/base/id"), "org.example.dusk");
    assert_eq!(at("calm2", "/preset/values/speed"), 0.4);
    assert_eq!(at("calm2", "/manifest/license"), "CC0-1.0");
    assert!(at("calm2", "/manifest/previews/thumbnail").as_str().unwrap().contains("preview"));
    let steps: serde_json::Value =
        serde_json::from_str(&ok(dir, &["scene", "ask", "calm2", "preset_steps", "{}"])).unwrap();
    assert_eq!(steps["ready"], true, "{steps}");
    fails(dir, &["scene", "new", "plain", "--base", "org.example.dusk"]);
}
