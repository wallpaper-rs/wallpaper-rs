use serde_json::{Value, json};
use wprs_editor::actions::Action;
use wprs_editor::starters::starter;
use wprs_editor::{Project, manual};

const ANSWERED: [&str; 25] = [
    "copy",
    "film",
    "sprites",
    "asset_uses",
    "unkept",
    "script_uses",
    "review",
    "signature",
    "tried",
    "media",
    "levels",
    "base",
    "bands",
    "touch",
    "shape_handles",
    "text_box",
    "font_list",
    "clock_sample",
    "clock_help",
    "mask_view",
    "mask_picture",
    "effect_handles",
    "capture_view",
    "level",
    "history",
];

fn act(project: &mut Project, action: Value) {
    project.act(&serde_json::from_value::<Action>(action).unwrap()).unwrap();
}

#[test]
fn the_queries_the_app_answers_are_listed_in_the_manual() {
    let listed: Vec<&str> = manual::ASKS.iter().map(|(n, _)| *n).collect();
    let missing: Vec<&str> = ANSWERED.into_iter().filter(|n| !listed.contains(n)).collect();
    assert!(missing.is_empty(), "{missing:?}");
    for name in ANSWERED {
        assert!(manual::example_lines("ask", name).is_some(), "{name} has no example");
    }
}

#[test]
fn the_history_query_lists_what_undo_and_redo_would_do() {
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", root.path().join("state")) };
    let dir = root.path().join("scene");
    let mut project = Project::create(&dir, starter("empty", "org.example.h", "H", "en").unwrap()).unwrap();
    let ask = |p: &Project| p.ask("", &json!({ "history": {} }));
    assert_eq!(ask(&project)["done"], json!([]));
    act(&mut project, json!({ "add_layer": { "kind": "solid", "id": "a" } }));
    act(&mut project, json!({ "hide": { "id": "a", "hidden": true } }));
    let found = ask(&project);
    let labels = |key: &str| -> Vec<String> {
        found[key].as_array().unwrap().iter().map(|r| r["label"].as_str().unwrap().to_owned()).collect()
    };
    assert_eq!(labels("done"), ["add_layer", "hide"]);
    assert_eq!(found["undo"]["label"], "hide");
    assert_eq!(found["redo"], Value::Null);
    assert_eq!(found["dirty"], true);
    act(&mut project, json!({ "undo": {} }));
    let found = ask(&project);
    assert_eq!(found["redo"]["label"], "hide");
    assert_eq!(found["undone"][0]["label"], "hide");
}
