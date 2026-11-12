use super::*;

#[test]
fn the_form_edits_parameters_passes_and_fields_and_renames_their_uses() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    shader(&mut p, json!({ "new": { "name": "bend", "template": "distort" } })).unwrap();
    act(&mut p, json!({ "add_layer": { "kind": "solid", "id": "fill" } })).unwrap();
    let use_it =
        json!({ "effects": { "add": { "layer": "fill", "effect": "bend", "params": { "strength": 0.5 } } } });
    act(&mut p, use_it).unwrap();
    let angle = json!({ "effect": "bend", "id": "spin", "set": { "type": "float", "min": -90, "max": 90, "unit": "degrees" } });
    shader(&mut p, json!({ "param": angle })).unwrap();
    let spin = &descriptor(&p, "bend")["params"][3];
    assert_eq!((spin["unit"].clone(), spin["default"].clone()), (json!("degrees"), json!(0)), "{spin}");
    let handles = ask(&p, json!({ "shader_check": { "effect": "bend" } }));
    assert!(
        handles["handles"].as_array().unwrap().iter().any(|h| h["param"] == "spin" && h["handle"] == "angle")
    );
    shader(&mut p, json!({ "param": { "effect": "bend", "id": "spin", "clear": ["unit"], "at": 0 } }))
        .unwrap();
    assert!(descriptor(&p, "bend")["params"][0].get("unit").is_none());
    shader(&mut p, json!({ "param": { "effect": "bend", "id": "spin", "remove": true } })).unwrap();
    let bad = json!({ "effect": "bend", "id": "strength", "set": { "min": 2, "max": 1 } });
    assert!(shader(&mut p, json!({ "param": bad })).is_err(), "min above max");
    let secret = json!({ "effect": "bend", "id": "strength", "set": { "secret": 1 } });
    assert!(shader(&mut p, json!({ "param": secret })).is_err());
    shader(&mut p, json!({ "param": { "effect": "bend", "id": "strength", "to": "pull" } })).unwrap();
    let pointer = wprs_editor::layers::find(&p.doc, "fill").unwrap();
    let uses = p.doc.pointer(&format!("{pointer}/effects/0/params")).unwrap();
    assert_eq!(uses, &json!({ "pull": 0.5 }), "the uses follow the rename");
    let cost = json!({ "effect": "bend", "field": "cost", "value": { "taps": 2, "passes": 1, "scale": 1 } });
    shader(&mut p, json!({ "field": cost })).unwrap();
    assert_eq!(descriptor(&p, "bend")["cost"]["taps"], 2);
    shader(&mut p, json!({ "field": { "effect": "bend", "field": "category", "value": null } })).unwrap();
    assert!(descriptor(&p, "bend").get("category").is_none());
    assert!(shader(&mut p, json!({ "field": { "effect": "bend", "field": "id", "value": "x" } })).is_err());
    shader(&mut p, json!({ "new": { "name": "soft", "template": "passes" } })).unwrap();
    shader(&mut p, json!({ "pass": { "effect": "soft", "index": 0, "set": { "scale": 0.25 } } })).unwrap();
    assert_eq!(descriptor(&p, "soft")["passes"][0]["scale"], 0.25);
    shader(&mut p, json!({ "pass": { "effect": "soft" } })).unwrap();
    assert_eq!(descriptor(&p, "soft")["passes"][2]["entry"], "fs_soft_2");
    let added = ask(&p, json!({ "shader_check": { "effect": "soft" } }));
    assert!(added["error"].as_str().unwrap().contains("fs_soft_2"), "{added}");
    shader(&mut p, json!({ "pass": { "effect": "soft", "index": 2, "remove": true } })).unwrap();
    assert!(shader(&mut p, json!({ "pass": { "effect": "soft", "index": 9, "remove": true } })).is_err());
    assert_eq!(ask(&p, json!({ "shader_check": { "effect": "soft" } }))["ok"], true);
}
