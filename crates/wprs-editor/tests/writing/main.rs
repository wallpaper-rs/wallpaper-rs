mod checking;
mod form;
mod starting;

use serde_json::{Value, json};
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::starters::starter;

pub fn act(p: &mut Project, action: Value) -> anyhow::Result<()> {
    let action = serde_json::from_value::<Action>(action).unwrap();
    assert!(wprs_editor::ACTIONS.contains(&action.name().as_str()), "{}", action.name());
    p.act(&action).map(drop)
}

pub fn shader(p: &mut Project, args: Value) -> anyhow::Result<()> {
    act(p, json!({ "shader": args }))
}

pub fn ask(p: &Project, q: Value) -> Value {
    wprs_editor::rigs::query(&p.dir, &p.doc, "", &q)
}

pub fn project(root: &TempDir) -> Project {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
    let made = starter("empty", "org.example.writing", "Writing", "en").unwrap();
    Project::create(&root.path().join("writing"), made).unwrap()
}

pub fn descriptor(p: &Project, id: &str) -> Value {
    let text = std::fs::read_to_string(p.dir.join(format!("effects/{id}.json"))).unwrap();
    serde_json::from_str(&text).unwrap()
}
