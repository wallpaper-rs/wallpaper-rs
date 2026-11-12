use serde_json::{Value, json};
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::starters::starter;

fn project(root: &TempDir) -> Project {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
    let made = starter("empty", "org.example.touch", "Touch", "en").unwrap();
    let mut p = Project::create(&root.path().join("touch"), made).unwrap();
    act(&mut p, json!({ "add_layer": { "kind": "solid", "id": "button" } })).unwrap();
    act(&mut p, json!({ "add_layer": { "kind": "solid", "id": "backdrop" } })).unwrap();
    p
}

fn act(p: &mut Project, action: Value) -> anyhow::Result<()> {
    p.act(&serde_json::from_value::<Action>(action).unwrap()).map(drop)
}

fn touch(p: &mut Project, layer: &str, takes: &str, alpha: Option<f64>) -> anyhow::Result<()> {
    let edit = json!({ "layer": { "touch": { "layer": layer, "takes": takes, "alpha": alpha } } });
    act(p, edit)
}

fn areas(p: &Project) -> Vec<Value> {
    let found = wprs_editor::rigs::query(&p.dir, &p.doc, "", &json!({ "touch": true }));
    found["layers"].as_array().cloned().unwrap_or_default()
}

fn solid(p: &Project, id: &str) -> Value {
    let (path, _) = wprs_editor::layers::walk(&p.doc).into_iter().find(|(_, l)| l["id"] == id).unwrap();
    p.doc.pointer(&format!("{path}/solid")).cloned().unwrap_or(Value::Null)
}

#[test]
fn a_layer_takes_the_pointer_everywhere_by_its_opacity_or_not_at_all() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    assert!(areas(&p).is_empty());
    touch(&mut p, "button", "all", None).unwrap();
    assert_eq!(solid(&p, "button"), json!(true));
    touch(&mut p, "button", "alpha", Some(0.25)).unwrap();
    assert_eq!(solid(&p, "button"), json!({ "alpha": 0.25 }));
    touch(&mut p, "backdrop", "alpha", None).unwrap();
    assert_eq!(solid(&p, "backdrop"), json!({ "alpha": 0.5 }));
    let seen = areas(&p);
    let kinds: Vec<(&str, &str)> =
        seen.iter().map(|a| (a["id"].as_str().unwrap(), a["takes"].as_str().unwrap())).collect();
    assert_eq!(kinds, [("button", "alpha"), ("backdrop", "alpha")]);
    assert_eq!(seen[0]["alpha"], json!(0.25));
    touch(&mut p, "button", "off", None).unwrap();
    assert!(solid(&p, "button").is_null(), "a layer that lets the pointer through keeps no solid");
    assert_eq!(areas(&p).len(), 1);
    touch(&mut p, "button", "off", None).unwrap();
    assert!(touch(&mut p, "button", "alpha", Some(1.5)).is_err());
    assert!(touch(&mut p, "nobody", "all", None).is_err());
}

#[test]
fn the_change_of_a_pointer_area_is_one_undo_step() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    touch(&mut p, "button", "alpha", Some(0.8)).unwrap();
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert!(solid(&p, "button").is_null());
    act(&mut p, json!({ "redo": {} })).unwrap();
    assert_eq!(solid(&p, "button"), json!({ "alpha": 0.8 }));
}
