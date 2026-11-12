mod credits;

use serde_json::{Value, json};
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::starters::starter;

pub fn project(root: &TempDir, name: &str) -> Project {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
    Project::create(
        &root.path().join(name),
        starter("empty", "org.example.package", "Package", "en").unwrap(),
    )
    .unwrap()
}

pub fn act(p: &mut Project, action: Value) -> anyhow::Result<()> {
    p.act(&serde_json::from_value::<Action>(action).unwrap()).map(drop)
}

pub fn ask(p: &Project, query: Value) -> Value {
    wprs_editor::rigs::query(&p.dir, &p.doc, "", &query)
}

pub fn file(p: &Project, name: &str, bytes: &[u8]) {
    let path = p.dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, bytes).unwrap();
}

pub fn warned(p: &Project, pointer: &str) -> Vec<String> {
    let all = wprs_editor::package::problems(&p.dir, &p.doc);
    all.into_iter().filter(|q| q.warning && q.pointer == pointer).map(|q| q.message).collect()
}

pub fn none() -> Value {
    json!({})
}
