use serde_json::{Value, json};
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::seats::{Seat, seats};
use wprs_editor::starters::starter;

mod capture;
mod effect_points;
mod grips;
mod kin;
mod masks;
mod placed;

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
    let dir = root.path().join("seats");
    let mut p = Project::create(&dir, starter("empty", "org.example.seats", "Seats", "en").unwrap()).unwrap();
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

pub fn card(id: &str, transform: Value) -> Value {
    json!({"id": id, "type": "solid", "color": "#ffffff", "size": [200, 100], "transform": transform})
}
