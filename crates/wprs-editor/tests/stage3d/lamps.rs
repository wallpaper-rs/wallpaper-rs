use super::{act, ask, edit, layer, project, undone};
use serde_json::{Value, json};

fn lights(p: &wprs_editor::Project) -> Value {
    ask(p, json!({ "stage3d": { "time": 0 } }))["lights"].clone()
}

fn xy(v: &Value) -> [f64; 2] {
    [v[0].as_f64().unwrap(), v[1].as_f64().unwrap()]
}

#[test]
fn a_light_is_placed_in_3d_dragged_there_and_sent_back_to_the_plane() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    edit(&mut p, json!({ "model": { "add": { "asset": "toy", "id": "doll" } } })).unwrap();
    act(&mut p, json!({ "add_layer": { "kind": "light", "id": "lamp" } })).unwrap();
    let kind = wprs_editor::actions::layer(&p.doc, "lamp").unwrap().0 + "/light";
    act(&mut p, json!({ "set": { "pointer": kind, "value": "spot" } })).unwrap();
    edit(&mut p, json!({ "view": { "orbit": [40, 15] } })).unwrap();
    let shown = lights(&p)[0].clone();
    assert_eq!((shown["id"].clone(), shown["space"].clone()), (json!("lamp"), json!(false)));
    let before = p.doc.clone();
    edit(&mut p, json!({ "light": { "layer": "lamp", "place": true } })).unwrap();
    undone(&mut p, &before);
    assert!(layer(&p, "lamp")["position3d"].is_array() && layer(&p, "lamp")["aim3d"].is_array());
    let placed = lights(&p)[0].clone();
    assert!((xy(&placed["at"])[0] - xy(&shown["at"])[0]).abs() < 0.5, "placing keeps it where it shows");
    let at = xy(&placed["at"]);
    let to = [at[0] + 35.0, at[1] - 20.0];
    let what = json!({ "light": { "layer": "lamp" } });
    edit(&mut p, json!({ "grab": { "what": what, "handle": "plane", "from": at, "to": to } })).unwrap();
    let now = xy(&lights(&p)[0]["at"]);
    assert!((now[0] - to[0]).hypot(now[1] - to[1]) < 1.0, "{now:?} {to:?}");
    let aim = xy(&lights(&p)[0]["aim"]);
    let what = json!({ "light": { "layer": "lamp", "aim": true } });
    edit(
        &mut p,
        json!({ "grab": { "what": what, "handle": "depth", "from": aim, "to": [aim[0], aim[1] - 50.0] } }),
    )
    .unwrap();
    edit(&mut p, json!({ "light": { "layer": "lamp", "position": [1, 2, 3] } })).unwrap();
    assert_eq!(layer(&p, "lamp")["position3d"], json!([1.0, 2.0, 3.0]));
    assert!(edit(&mut p, json!({ "light": { "layer": "lamp", "position": [1e9, 0, 0] } })).is_err());
    assert!(edit(&mut p, json!({ "light": { "layer": "doll", "flat": true } })).is_err());
    edit(&mut p, json!({ "light": { "layer": "lamp", "flat": true } })).unwrap();
    assert!(layer(&p, "lamp").get("position3d").is_none() && layer(&p, "lamp").get("aim3d").is_none());
    edit(&mut p, json!({ "light": { "layer": "lamp", "position": [0, 1, 2] } })).unwrap();
    let doc: wprs_scene::Scene = serde_json::from_value(p.doc["scene"].clone()).unwrap();
    let issues: Vec<String> = doc.validate(&Default::default()).iter().map(|i| i.to_string()).collect();
    assert!(!issues.iter().any(|i| i.contains("lamp")), "{issues:?}");
}
