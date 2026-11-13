use serde_json::{Value, json};
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::starters::starter;

fn act(project: &mut Project, action: Value) -> anyhow::Result<()> {
    project.act(&serde_json::from_value::<Action>(action).unwrap()).map(drop)
}

#[test]
fn painting_a_picture_another_effect_uses_makes_a_copy_for_the_painted_one() {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
    let root = tempfile::tempdir().unwrap();
    let mut p =
        Project::create(&root.path().join("veil"), starter("empty", "org.example.veil", "Veil", "en").unwrap())
            .unwrap();
    let effect = json!({ "effect": "blend_gradient" });
    let layers = json!([
        { "id": "a", "type": "solid", "color": "#ffffff", "effects": [effect] },
        { "id": "b", "type": "solid", "color": "#ffffff", "effects": [effect] }
    ]);
    act(&mut p, json!({ "set": { "pointer": "/scene/layers", "value": layers } })).unwrap();
    let (first, second) = ("/scene/layers/0/effects/0/params/map", "/scene/layers/1/effects/0/params/map");
    let stroke = |pointer: &str| {
        json!({ "paint_mask": { "pointer": pointer, "radius": 0.1, "value": 1.0, "points": [[0.5, 0.5]] } })
    };
    act(&mut p, stroke(first)).unwrap();
    let shared = p.doc.pointer(first).and_then(Value::as_str).unwrap().to_owned();
    act(&mut p, json!({ "set": { "pointer": second, "value": shared } })).unwrap();
    let path = |p: &Project, id: &str| p.doc["scene"]["assets"][id]["path"].clone();
    let before = path(&p, &shared);
    act(&mut p, stroke(second)).unwrap();
    let own = p.doc.pointer(second).and_then(Value::as_str).unwrap().to_owned();
    assert_ne!(own, shared);
    assert_eq!(p.doc.pointer(first).and_then(Value::as_str), Some(shared.as_str()));
    assert_eq!(path(&p, &shared), before);
    act(&mut p, stroke(second)).unwrap();
    assert_eq!(p.doc.pointer(second).and_then(Value::as_str), Some(own.as_str()));
}
