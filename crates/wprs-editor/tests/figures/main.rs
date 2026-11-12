use serde_json::{Value, json};
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::seats::{Seat, seats};
use wprs_editor::starters::starter;

mod clocks;
mod fonts;
mod shapes;
mod stops;
mod text;

fn state() {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
}

pub fn act(project: &mut Project, action: Value) -> anyhow::Result<()> {
    let action = serde_json::from_value::<Action>(action).unwrap();
    assert!(wprs_editor::ACTIONS.contains(&action.name().as_str()), "{}", action.name());
    project.act(&action).map(drop)
}

pub fn scene(root: &TempDir, layers: Value) -> Project {
    state();
    static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = root.path().join(format!("figures{n}"));
    let mut p =
        Project::create(&dir, starter("empty", "org.example.figures", "Figures", "en").unwrap()).unwrap();
    act(&mut p, json!({"set": {"pointer": "/scene/canvas/size", "value": [1000, 1000]}})).unwrap();
    act(&mut p, json!({"set": {"pointer": "/scene/layers", "value": layers}})).unwrap();
    p
}

pub fn seat(p: &Project, id: &str) -> Seat {
    seats(&p.dir, &p.doc, 0.0).into_iter().find(|s| s.id == id).unwrap_or_else(|| panic!("no seat {id}"))
}

pub fn near(a: [f64; 2], b: [f64; 2], within: f64) -> bool {
    (a[0] - b[0]).abs() <= within && (a[1] - b[1]).abs() <= within
}

pub fn handles(p: &Project, id: &str) -> Value {
    let q = json!({"shape_handles": {"layer": id, "time": 0.0}});
    wprs_editor::rigs::query(&p.dir, &p.doc, id, &q)
}

pub fn handle(found: &Value, name: &str) -> [f64; 2] {
    let h = found["handles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|h| h["name"] == name)
        .unwrap_or_else(|| panic!("no {name}"));
    [h["at"][0].as_f64().unwrap(), h["at"][1].as_f64().unwrap()]
}

pub fn pull(p: &mut Project, id: &str, name: &str, to: [f64; 2]) -> anyhow::Result<()> {
    act(p, json!({"layer": {"figure": {"shape": {"layer": id, "handle": name, "to": to}}}}))
}

pub fn at(p: &Project, pointer: &str) -> Value {
    p.doc.pointer(pointer).cloned().unwrap_or(Value::Null)
}
