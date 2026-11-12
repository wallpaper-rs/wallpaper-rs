use super::kit::{act, query, rigged};
use serde_json::{Value, json};
use wprs_editor::Project;

fn event(extra: Value) -> Value {
    let mut e = json!({ "layer": "net", "clip": "wave", "name": "blink" });
    e.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
    json!({ "clip_event": e })
}

fn events(p: &Project) -> Value {
    p.doc["scene"]["layers"][0]["clips"][0]["events"].clone()
}

#[test]
fn clip_events_are_added_moved_renamed_and_removed() {
    let root = tempfile::tempdir().unwrap();
    let mut p = rigged(&root);
    act(&mut p, json!({ "clip": { "layer": "net", "id": "wave", "add": true, "length": 2 } })).unwrap();
    act(&mut p, event(json!({ "time": 1.5 }))).unwrap();
    act(&mut p, event(json!({ "name": "step", "time": 0.5 }))).unwrap();
    assert_eq!(events(&p), json!([{ "t": 0.5, "name": "step" }, { "t": 1.5, "name": "blink" }]));
    act(&mut p, event(json!({ "time": 0.25 }))).unwrap();
    act(&mut p, event(json!({ "name": "step", "to": "tap" }))).unwrap();
    assert_eq!(events(&p), json!([{ "t": 0.25, "name": "blink" }, { "t": 0.5, "name": "tap" }]));
    let shown = query(&p, "net", json!({ "keys": "wave" }));
    assert_eq!(shown["events"], events(&p));
    act(&mut p, event(json!({ "time": 1.0 }))).unwrap();
    act(&mut p, event(json!({ "at": 1.0, "remove": true }))).unwrap();
    act(&mut p, event(json!({ "name": "tap", "remove": true }))).unwrap();
    assert!(p.doc["scene"]["layers"][0]["clips"][0].get("events").is_none());
    for bad in [
        event(json!({ "time": 2.5 })),
        event(json!({ "time": -1 })),
        event(json!({ "name": "", "time": 1 })),
        event(json!({ "name": "a/b", "time": 1 })),
        event(json!({ "remove": true })),
    ] {
        assert!(act(&mut p, bad).is_err());
    }
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(events(&p).as_array().map(Vec::len), Some(1));
}
