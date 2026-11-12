use super::{act, project, refused, script};
use serde_json::json;
use wprs_editor::templates;

#[test]
fn templates_are_made_from_layers_renamed_removed_and_listed_as_a_tree() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    for id in ["dot", "ring"] {
        act(&mut p, json!({ "add_layer": { "kind": "solid", "id": id } })).unwrap();
    }
    act(&mut p, json!({ "template": { "make": { "layers": ["dot"] } } })).unwrap();
    act(&mut p, json!({ "template": { "make": { "layers": ["dot", "ring"], "name": "Pair!" } } })).unwrap();
    assert_eq!(p.doc["scene"]["templates"]["dot_template"]["type"], "solid");
    assert_eq!(p.doc["scene"]["templates"]["pair"]["layers"].as_array().unwrap().len(), 2);
    assert!(
        refused(&mut p, json!({ "template": { "make": { "layers": ["dot"], "name": "pair" } } }))
            .contains("exists")
    );
    assert!(refused(&mut p, json!({ "template": { "make": { "layers": [] } } })).contains("name the layers"));
    assert!(refused(&mut p, json!({ "template": { "make": { "layers": ["nope"] } } })).contains("no layer"));
    script(&mut p, "s.js", "scene.spawn(\"pair\")");
    act(&mut p, json!({ "script": { "add": { "path": "scripts/s.js" } } })).unwrap();
    let tree = templates::tree(&p.dir, &p.doc);
    assert_eq!(tree.len(), 2);
    assert_eq!(tree[1]["layers"].as_array().unwrap().len(), 3);
    assert_eq!(tree[1]["layers"][1]["pointer"], "/scene/templates/pair/layers/0");
    assert_eq!(tree[1]["scripts"], json!(["scripts/s.js:1"]));
    act(&mut p, json!({ "template": { "rename": { "name": "pair", "to": "couple" } } })).unwrap();
    assert!(
        p.doc["scene"]["templates"]["couple"].is_object()
            && p.doc["scene"]["templates"].get("pair").is_none()
    );
    assert!(
        refused(&mut p, json!({ "template": { "rename": { "name": "couple", "to": "dot_template" } } }))
            .contains("exists")
    );
    act(&mut p, json!({ "template": { "remove": { "name": "couple" } } })).unwrap();
    assert!(
        refused(&mut p, json!({ "template": { "remove": { "name": "couple" } } })).contains("no template")
    );
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert!(p.doc["scene"]["templates"]["couple"].is_object());
}
