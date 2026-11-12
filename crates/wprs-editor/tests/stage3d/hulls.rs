use super::motion::rigged;
use super::{act, ask, layer, undone};
use serde_json::json;

fn collider(p: &mut wprs_editor::Project, args: serde_json::Value) -> anyhow::Result<()> {
    act(p, json!({ "collider": args }))
}

#[test]
fn colliders_of_a_model_are_added_tuned_shown_and_removed() {
    let root = tempfile::tempdir().unwrap();
    let mut p = rigged(&root);
    let before = p.doc.clone();
    collider(&mut p, json!({ "layer": "doll", "bone": "root", "radius": 0.3 })).unwrap();
    undone(&mut p, &before);
    collider(&mut p, json!({ "layer": "doll", "index": 0, "shape": "sphere", "offset": [0, 1.5, 0] }))
        .unwrap();
    assert_eq!(
        layer(&p, "doll")["colliders"],
        json!([{ "bone": "root", "radius": 0.3, "shape": "sphere", "offset": [0.0, 1.5, 0.0] }])
    );
    for bad in [
        json!({ "layer": "doll", "bone": "nobody" }),
        json!({ "layer": "doll", "index": 0, "radius": 200 }),
        json!({ "layer": "doll", "index": 4, "radius": 1 }),
        json!({ "layer": "doll" }),
    ] {
        assert!(collider(&mut p, bad.clone()).is_err(), "{bad}");
    }
    let body = json!({ "layer": "doll", "bones": ["tip"], "translation": [-20, 30] });
    act(&mut p, json!({ "physics": body })).unwrap();
    let at = wprs_editor::actions::layer(&p.doc, "doll").unwrap().0;
    let chain = json!([{ "chain": ["root", "tip"], "target": [40, 30] }]);
    act(&mut p, json!({ "set": { "pointer": format!("{at}/ik"), "value": chain } })).unwrap();
    let shown = ask(&p, json!({ "stage3d": { "time": 0.0 } }));
    let doll = &shown["models"][0];
    let hull = &doll["colliders"][0];
    assert!(hull["at"] == hull["to"] && hull["per"].as_f64().unwrap() > 1.0, "{doll}");
    let slide = &doll["slides"][0];
    assert_eq!(
        (slide["lines"].as_array().unwrap().len(), &slide["translation"]),
        (12, &json!([-20.0, 30.0]))
    );
    assert_eq!(doll["reach"][0]["point"], json!([40.0, 30.0]));
    collider(&mut p, json!({ "layer": "doll", "index": 0, "remove": true })).unwrap();
    assert!(layer(&p, "doll").get("colliders").is_none());
}
