use serde_json::json;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::starters::starter;

fn actions(list: serde_json::Value) -> Vec<Action> {
    serde_json::from_value(list).unwrap()
}

fn made() -> (tempfile::TempDir, Project) {
    let root = tempfile::tempdir().unwrap();
    let doc = starter("empty", "org.example.batch", "Batch", "en").unwrap();
    let project = Project::create(&root.path().join("batch"), doc).unwrap();
    (root, project)
}

#[test]
fn several_actions_are_one_undo_step() {
    let (_root, mut p) = made();
    let before = p.doc.clone();
    let steps = p.history.done.len();
    p.act_all(&actions(json!([
        {"add_layer": {"kind": "solid", "id": "a"}},
        {"add_layer": {"kind": "solid", "id": "b"}},
        {"hide": {"id": "a", "hidden": true}}
    ])))
    .unwrap();
    assert_eq!(p.history.done.len(), steps + 1);
    assert_eq!(p.doc["scene"]["layers"].as_array().unwrap().len(), 2);
    p.act(&Action::Undo {}).unwrap();
    assert_eq!(p.doc, before);
    p.act(&Action::Redo {}).unwrap();
    assert_eq!(p.doc["scene"]["layers"].as_array().unwrap().len(), 2);
}

#[test]
fn a_failing_action_leaves_the_project_untouched() {
    let (_root, mut p) = made();
    let before = (p.doc.clone(), p.history.clone());
    let refused = p.act_all(&actions(json!([
        {"add_layer": {"kind": "solid", "id": "a"}},
        {"remove_layer": {"id": "missing"}}
    ])));
    assert!(refused.is_err());
    assert_eq!((p.doc.clone(), p.history.clone()), before);
    assert!(p.act_all(&actions(json!([{"undo": {}}]))).is_err());
}
