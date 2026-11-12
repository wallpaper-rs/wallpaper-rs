use super::{act, card, near, scene, seat};
use serde_json::{Value, json};
use wprs_editor::seats::parents;

fn line() -> Value {
    json!([
        card("a", json!({"position": [300, 300], "rotation": 40, "scale": [2, 1]})),
        {"id": "b", "type": "solid", "color": "#ffffff", "size": [50, 50], "parent": "a", "transform": {"position": [50, 0]}},
        {"id": "c", "type": "solid", "color": "#ffffff", "size": [50, 50], "parent": "b", "transform": {"position": [0, 40]}},
        card("d", json!({"position": [700, 600], "rotation": -15, "skew": [10, 0]})),
        {"id": "g", "type": "group", "layers": [card("inside", json!({}))]},
        {"id": "hum", "type": "sound", "sound": "none"}
    ])
}

#[test]
fn the_parent_picker_leaves_out_cycles_and_what_cannot_lead() {
    let root = tempfile::tempdir().unwrap();
    let p = scene(&root, line());
    assert_eq!(parents(&p.doc, "a"), ["d", "g", "inside"]);
    assert_eq!(parents(&p.doc, "g"), ["a", "b", "c", "d"]);
    let fields = wprs_editor::inspect::fields(&p.doc, "/scene/layers/0");
    let parent = fields.iter().find(|f| f.pointer == "/scene/layers/0/parent").expect("a parent field");
    assert_eq!(parent.options, [json!("d"), json!("g"), json!("inside")]);
}

#[test]
fn a_new_parent_keeps_the_layer_where_it_is() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, line());
    let before = seat(&p, "c").outline();
    act(&mut p, json!({"layer": {"parent": {"layer": "c", "parent": "d", "keep": true}}})).unwrap();
    assert_eq!(p.doc.pointer("/scene/layers/2/parent"), Some(&json!("d")));
    let after = seat(&p, "c").outline();
    assert!(after.iter().zip(before).all(|(a, b)| near(*a, b, 0.05)), "{after:?} {before:?}");
    act(&mut p, json!({"layer": {"parent": {"layer": "c", "keep": true}}})).unwrap();
    assert_eq!(p.doc.pointer("/scene/layers/2/parent"), None);
    let freed = seat(&p, "c").outline();
    assert!(freed.iter().zip(before).all(|(a, b)| near(*a, b, 0.05)), "{freed:?} {before:?}");
    let refused = act(&mut p, json!({"layer": {"parent": {"layer": "a", "parent": "b"}}})).unwrap_err();
    assert!(refused.to_string().contains("cannot follow"), "{refused}");
}
