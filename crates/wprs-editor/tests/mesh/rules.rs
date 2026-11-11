use super::kit::{act, rigged};
use serde_json::{Value, json};
use wprs_editor::Project;

fn rule(extra: Value) -> Value {
    let mut r = json!({ "layer": "net" });
    r.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
    json!({ "bone_rule": r })
}

fn rules(p: &Project) -> Value {
    p.doc["scene"]["layers"][0]["rules"].clone()
}

#[test]
fn a_rule_hands_a_bone_to_another_parent() {
    let root = tempfile::tempdir().unwrap();
    let mut p = rigged(&root);
    let hand = json!({ "bone": { "layer": "net", "name": "hand", "parent": "arm", "at": [0.9, 0.5] } });
    act(&mut p, hand).unwrap();
    act(&mut p, rule(json!({ "bone": "hand", "parent": "root" }))).unwrap();
    assert_eq!(rules(&p), json!([{ "bone": "hand", "parent": "root" }]));
    act(&mut p, rule(json!({ "index": 0, "weight": 0.5 }))).unwrap();
    assert_eq!(rules(&p), json!([{ "bone": "hand", "parent": "root", "weight": 0.5 }]));
    act(&mut p, rule(json!({ "index": 0, "parent": "arm" }))).unwrap();
    assert_eq!(rules(&p), json!([{ "bone": "hand", "parent": "arm", "weight": 0.5 }]));
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(rules(&p)[0]["parent"], json!("root"));
    act(&mut p, json!({ "redo": {} })).unwrap();
    for bad in [
        json!({}),
        json!({ "bone": "hand", "parent": "hand" }),
        json!({ "bone": "hand", "parent": "nothing" }),
        json!({ "bone": "nothing", "parent": "root" }),
        json!({ "index": 5, "weight": 0.5 }),
        json!({ "index": 0, "weight": 2 }),
        json!({ "remove": true }),
    ] {
        assert!(act(&mut p, rule(bad.clone())).is_err(), "{bad}");
    }
    act(&mut p, rule(json!({ "index": 0, "remove": true }))).unwrap();
    assert!(p.doc["scene"]["layers"][0].get("rules").is_none());
}
