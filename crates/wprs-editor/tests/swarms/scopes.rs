use super::*;

fn own(p: &Project, id: &str) -> Value {
    let (path, _) = wprs_editor::actions::layer(&p.doc, id).unwrap();
    p.doc.pointer(&format!("{path}/render")).cloned().unwrap_or(Value::Null)
}

fn seen(p: &Project, id: Option<&str>) -> Value {
    wprs_editor::swarms::handles(&p.dir, &p.doc, "flakes", id, 0.0)
}

#[test]
fn a_layer_render_is_started_shown_apart_and_given_back_to_the_system() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let id = handles(&p)["layer"].as_str().unwrap().to_owned();
    edit(&mut p, "strand", json!({ "strand": "rope", "settings": { "width": 6 } })).unwrap();
    let system = seen(&p, None);
    assert_eq!(system["scope"]["layers"], json!([id]));
    assert_eq!((&system["scope"]["own"], &system["scope"]["differs"]), (&json!(false), &json!([])));
    assert_eq!(system["strands"]["rope"]["width"], json!(6));
    let before = p.doc.clone();
    edit(&mut p, "scope", json!({ "layer": id, "own": true })).unwrap();
    assert_eq!(own(&p, &id), system["render"]);
    undone(&mut p, &before);
    edit(&mut p, "scope", json!({ "layer": id, "own": true })).unwrap();
    edit(&mut p, "strand", json!({ "strand": "rope", "settings": { "width": 2 }, "layer": id })).unwrap();
    edit(&mut p, "render", json!({ "layer": id, "fps": 12 })).unwrap();
    assert_eq!(system_render(&p)["rope"]["width"], json!(6), "the system keeps its own");
    let layer = seen(&p, Some(&id));
    assert_eq!(layer["scope"]["own"], json!(true));
    assert_eq!(layer["scope"]["differs"], json!(["fps", "rope"]));
    assert_eq!(layer["strands"]["rope"]["width"], json!(2));
    assert_eq!(seen(&p, None)["strands"]["rope"]["width"], json!(6), "without a layer the system shows");
    edit(&mut p, "scope", json!({ "layer": id, "own": false })).unwrap();
    assert_eq!(own(&p, &id), Value::Null);
    assert_eq!(seen(&p, Some(&id))["scope"]["own"], json!(false));
    let other = edit(&mut p, "scope", json!({ "layer": "logo", "own": true })).unwrap_err().to_string();
    assert!(other.contains("draws no particles of this system"), "{other}");
}

fn system_render(p: &Project) -> Value {
    system(p)["render"].clone()
}
