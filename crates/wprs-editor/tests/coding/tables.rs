use super::{act, project, refused, script};
use serde_json::{Value, json};
use wprs_editor::scripts::listing;

#[test]
fn scripts_are_attached_ordered_and_removed_unless_a_value_follows_them() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    for name in ["a.js", "b.js", "c.js"] {
        script(&mut p, name, "export const value = () => 1;");
    }
    assert_eq!(listing(&p.dir, &p.doc)["free"].as_array().unwrap().len(), 3);
    for name in ["a", "b", "c"] {
        act(&mut p, json!({ "script": { "add": { "path": format!("scripts/{name}.js") } } })).unwrap();
    }
    assert!(refused(&mut p, json!({ "script": { "add": { "path": "scripts/a.js" } } })).contains("already"));
    assert!(
        refused(&mut p, json!({ "script": { "add": { "path": "scripts/none.js" } } }))
            .contains("cannot read")
    );
    act(&mut p, json!({ "script": { "move": { "path": "scripts/c.js", "to": 0 } } })).unwrap();
    let order: Vec<&str> =
        p.doc["scene"]["scripts"].as_array().unwrap().iter().map(|s| s["path"].as_str().unwrap()).collect();
    assert_eq!(order, ["scripts/c.js", "scripts/a.js", "scripts/b.js"]);
    let bound = json!({ "script": "scripts/a.js", "fn": "value" });
    act(&mut p, json!({ "set": { "pointer": "/scene/canvas/background", "value": "#000000" } })).unwrap();
    act(&mut p, json!({ "add_layer": { "kind": "solid", "id": "box" } })).unwrap();
    act(&mut p, json!({ "set": { "pointer": "/scene/layers/0/opacity", "value": bound } })).unwrap();
    let said = refused(&mut p, json!({ "script": { "remove": { "path": "scripts/a.js" } } }));
    assert!(said.contains("/scene/layers/0/opacity"), "{said}");
    assert_eq!(listing(&p.dir, &p.doc)["scripts"][1]["bound"], json!(["/scene/layers/0/opacity"]));
    act(&mut p, json!({ "script": { "remove": { "path": "scripts/b.js" } } })).unwrap();
    assert_eq!(listing(&p.dir, &p.doc)["free"], json!(["scripts/b.js"]));
    assert_eq!(listing(&p.dir, &p.doc)["scripts"][1]["functions"], json!(["value"]));
}

#[test]
fn the_settings_of_a_module_are_a_typed_table_whose_names_follow_into_the_values() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    script(&mut p, "w.js", "export const wave = (v, { params }) => params.speed;");
    act(&mut p, json!({ "script": { "add": { "path": "scripts/w.js" } } })).unwrap();
    let set = |id: &str, extra: Value| {
        let mut body = json!({ "path": "scripts/w.js", "id": id });
        body.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
        json!({ "script": { "param": body } })
    };
    act(&mut p, set("speed", json!({ "type": "number", "default": 2, "min": 0, "max": 5 }))).unwrap();
    act(&mut p, set("label", json!({ "type": "text" }))).unwrap();
    act(&mut p, set("tint", json!({ "type": "color" }))).unwrap();
    let params = &p.doc["scene"]["scripts"][0]["params"];
    assert_eq!(params["speed"], json!({ "type": "number", "default": 2, "min": 0, "max": 5 }));
    assert_eq!((&params["label"]["default"], &params["tint"]["default"]), (&json!(""), &json!("#ffffff")));
    assert!(refused(&mut p, set("speed", json!({ "default": 9 }))).contains("outside its range"));
    assert!(refused(&mut p, set("speed", json!({ "min": 6 }))).contains("above its maximum"));
    assert!(refused(&mut p, set("Bad", json!({ "type": "number" }))).contains("1 to 32"));
    assert!(refused(&mut p, set("flag", json!({ "type": "nope" }))).contains("not valid"));
    assert!(refused(&mut p, set("flag", json!({}))).contains("type"));
    act(&mut p, set("speed", json!({ "default": 3 }))).unwrap();
    assert_eq!(p.doc["scene"]["scripts"][0]["params"]["speed"]["max"], 5);
    act(&mut p, set("speed", json!({ "type": "bool", "default": true }))).unwrap();
    assert_eq!(p.doc["scene"]["scripts"][0]["params"]["speed"], json!({ "type": "bool", "default": true }));
    act(&mut p, set("speed", json!({ "type": "number", "default": 1 }))).unwrap();
    act(&mut p, json!({ "add_layer": { "kind": "solid", "id": "box" } })).unwrap();
    let bound = json!({ "script": "scripts/w.js", "fn": "wave", "params": { "speed": 4 } });
    act(&mut p, json!({ "set": { "pointer": "/scene/layers/0/opacity", "value": bound } })).unwrap();
    act(&mut p, set("speed", json!({ "to": "pace" }))).unwrap();
    assert_eq!(p.doc["scene"]["layers"][0]["opacity"]["params"], json!({ "pace": 4 }));
    assert!(p.doc["scene"]["scripts"][0]["params"]["pace"].is_object());
    assert!(refused(&mut p, set("label", json!({ "to": "pace" }))).contains("exists"));
    act(&mut p, json!({ "script": { "param": { "path": "scripts/w.js", "id": "pace", "remove": true } } }))
        .unwrap();
    for id in ["label", "tint"] {
        act(&mut p, json!({ "script": { "param": { "path": "scripts/w.js", "id": id, "remove": true } } }))
            .unwrap();
    }
    assert!(p.doc["scene"]["scripts"][0].get("params").is_none());
}
