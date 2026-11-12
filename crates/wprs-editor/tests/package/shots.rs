use crate::{act, ask, project};
use serde_json::json;

fn drawing() -> bool {
    wprs_render::Renderer::shared().is_ok()
}

#[test]
fn a_frame_and_a_recording_of_the_scene_become_the_previews() {
    if !drawing() {
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root, "shots");
    act(&mut p, json!({ "add_layer": { "kind": "solid", "id": "box" } })).unwrap();
    act(&mut p, json!({ "shot": { "still": { "time": 0.5 } } })).unwrap();
    let thumbnail = p.doc["manifest"]["previews"]["thumbnail"].as_str().unwrap().to_owned();
    assert!(p.dir.join(&thumbnail).is_file(), "{thumbnail}");
    act(&mut p, json!({ "shot": { "recording": { "seconds": 1.0, "fps": 5 } } })).unwrap();
    let animated = p.doc["manifest"]["previews"]["animated"].as_str().unwrap().to_owned();
    assert!(animated.ends_with(".gif") && p.dir.join(&animated).is_file(), "{animated}");
    assert!(!p.dir.join("assets/.shot.part").exists());
    act(&mut p, json!({ "shot": { "flashes": { "seconds": 1.0, "fps": 6 } } })).unwrap();
    let found = ask(&p, json!({ "flashes": {} }));
    assert_eq!(found["flashing"], json!(false), "{found}");
}

#[test]
fn bad_shots_are_refused() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root, "refused");
    for bad in [
        json!({ "shot": { "recording": { "seconds": 0.0 } } }),
        json!({ "shot": { "recording": { "seconds": 99.0 } } }),
        json!({ "shot": { "recording": { "fps": 0 } } }),
        json!({ "shot": { "flashes": { "seconds": 100.0 } } }),
    ] {
        assert!(act(&mut p, bad.clone()).is_err(), "{bad}");
    }
}
