use serde_json::{Value, json};
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::history::MERGE_MS;
use wprs_editor::project::{now, skip_clock};
use wprs_editor::starters::starter;

fn drag(project: &mut Project, value: &str) {
    let set: Value = json!({"set": {"pointer": "/scene/canvas/background", "value": value}});
    project.drag(&serde_json::from_value::<Action>(set).unwrap()).unwrap();
    project.held();
}

#[test]
fn skipping_the_clock_closes_the_window_that_joins_drags_without_waiting() {
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", root.path().join("state")) };
    let dir = root.path().join("dusk");
    let mut project =
        Project::create(&dir, starter("parallax", "org.example.dusk", "Dusk", "en").unwrap()).unwrap();
    drag(&mut project, "#101010");
    let steps = project.history.done.len();
    drag(&mut project, "#202020");
    assert_eq!(project.history.done.len(), steps, "two quick drags are one step");
    let before = now();
    skip_clock(MERGE_MS + 100);
    assert!(now() >= before + MERGE_MS + 100, "the skipped time shows in the editor clock");
    drag(&mut project, "#303030");
    assert_eq!(project.history.done.len(), steps + 1, "a drag after the window closed is a step of its own");
}
