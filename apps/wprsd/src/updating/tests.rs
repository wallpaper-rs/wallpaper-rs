use super::*;
use serde_json::json;
use std::collections::BTreeSet;
use wprs_format::Manifest;

fn manifest(extra: serde_json::Value) -> Manifest {
    let mut base = json!({"schema": wprs_format::SCHEMA, "id": "org.example.web", "version": "1.0.0",
        "title": "Web", "license": "CC0-1.0", "kind": "web", "entry": "index.html",
        "origin": {"kind": "original"}, "content": {"rating": "general"}, "photosensitive": false,
        "web": {"storage_kib": 0}, "playback": {}});
    base.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
    Manifest::from_json(base.to_string().as_bytes()).unwrap()
}

#[test]
fn asks_name_inputs_rgb_and_origins() {
    assert!(asks(&manifest(json!({}))).is_empty());
    let all = manifest(json!({"inputs": ["audio"], "permissions": {"rgb": true,
        "network": {"allow": ["https://api.example.org"]}}}));
    let want: BTreeSet<String> = ["audio", "rgb", "network https://api.example.org"].map(String::from).into();
    assert_eq!(asks(&all), want);
}

#[test]
fn only_what_is_new_is_asked() {
    let before = asks(&manifest(json!({"inputs": ["audio"]})));
    let after = asks(&manifest(json!({"inputs": ["audio", "pointer"], "permissions": {"rgb": true}})));
    let new: Vec<_> = after.difference(&before).cloned().collect();
    assert_eq!(new, ["pointer", "rgb"]);
}

#[test]
fn updates_wait_for_the_setting_and_for_the_mains() {
    assert!(postponed(false, false));
    assert!(postponed(true, true));
    assert!(!postponed(true, false));
}

#[test]
fn only_announced_asks_that_are_new_hold_a_release_before_the_download() {
    let before: BTreeSet<String> = ["audio".to_owned()].into();
    let announced = ["audio".to_owned(), "network https://a.example".to_owned()];
    assert_eq!(unseen(&announced, &before), ["network https://a.example"]);
    assert!(unseen(&announced[..1], &before).is_empty());
}
