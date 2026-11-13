use serde_json::{Value, json};
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::package::problems;
use wprs_editor::starters::starter;

fn open(root: &std::path::Path) -> Project {
    static STATE: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
    let doc = starter("parallax", "org.example.sky", "Sky", "en").unwrap();
    Project::create(&root.join("sky"), doc).unwrap()
}

fn act(project: &mut Project, action: Value) -> anyhow::Result<()> {
    project.act(&serde_json::from_value::<Action>(action).unwrap()).map(drop)
}

fn errors(project: &Project) -> Vec<String> {
    problems(&project.dir, &project.doc).into_iter().filter(|p| !p.warning).map(|p| p.message).collect()
}

fn layer<'a>(project: &'a Project, id: &str) -> &'a Value {
    let layers = project.doc["scene"]["layers"].as_array().unwrap();
    layers.iter().find(|l| l["id"] == id).unwrap_or_else(|| panic!("no layer {id}"))
}

#[test]
fn a_clock_layer_is_one_step_and_shows_the_time_or_the_date() {
    let root = tempfile::tempdir().unwrap();
    let mut project = open(root.path());
    act(&mut project, json!({"layer": {"start": {"clock": {}}}})).unwrap();
    assert_eq!(layer(&project, "clock")["text"], json!({"clock": "HH:mm"}));
    act(&mut project, json!({"layer": {"start": {"clock": {"date": true, "id": "fecha"}}}})).unwrap();
    assert_eq!(layer(&project, "fecha")["text"], json!({"date": "EEEE d MMMM yyyy"}));
    act(&mut project, json!({"layer": {"start": {"clock": {"format": "hh:mm a", "size": 90.0}}}})).unwrap();
    let second = layer(&project, "clock_2");
    assert_eq!((&second["text"], &second["size"]), (&json!({"clock": "hh:mm a"}), &json!(90.0)));
    assert!(errors(&project).is_empty(), "{:?}", errors(&project));
    let taken = act(&mut project, json!({"layer": {"start": {"clock": {"id": "fecha"}}}}));
    assert!(taken.unwrap_err().to_string().contains("fecha"));
    let big = act(&mut project, json!({"layer": {"start": {"clock": {"size": 0.0}}}}));
    assert!(big.is_err());
    act(&mut project, json!({"undo": {}})).unwrap();
    assert!(project.doc["scene"]["layers"].as_array().unwrap().iter().all(|l| l["id"] != "clock_2"));
}

#[test]
fn a_spin_turns_forever_in_the_chosen_direction_and_keeps_where_it_was() {
    let root = tempfile::tempdir().unwrap();
    let mut project = open(root.path());
    act(&mut project, json!({"add_layer": {"kind": "shape", "id": "gear"}})).unwrap();
    act(&mut project, json!({"layer": {"start": {"forever": {"layer": "gear", "motion": "spin"}}}})).unwrap();
    let rotation = layer(&project, "gear")["transform"]["rotation"].clone();
    assert_eq!(rotation, json!({"keys": [[0, 0.0, "linear"], [20.0, 360.0, "linear"]], "loop": "repeat"}));
    act(&mut project, json!({"layer": {"transform": {"layer": "gear", "rotation": 30.0, "replace": true}}}))
        .unwrap();
    let spin = json!({"layer": {"start": {"forever": {"layer": "gear", "motion": "spin", "seconds": 8.0, "reverse": true}}}});
    act(&mut project, spin).unwrap();
    let keys = layer(&project, "gear")["transform"]["rotation"]["keys"].clone();
    assert_eq!(keys, json!([[0, 30.0, "linear"], [8.0, -330.0, "linear"]]));
    assert!(errors(&project).is_empty(), "{:?}", errors(&project));
    let again = json!({"layer": {"start": {"forever": {"layer": "gear", "motion": "spin"}}}});
    assert!(act(&mut project, again).is_err(), "a spin over a turning layer replaced its keys unasked");
    let wrong =
        json!({"layer": {"start": {"forever": {"layer": "gear", "motion": "spin", "by": [1.0, 0.0]}}}});
    assert!(act(&mut project, wrong).is_err());
    let long = json!({"layer": {"start": {"forever": {"layer": "gear", "motion": "spin", "seconds": 0.0}}}});
    assert!(act(&mut project, long).is_err());
}

#[test]
fn a_drift_goes_and_comes_back_forever_from_where_the_layer_is() {
    let root = tempfile::tempdir().unwrap();
    let mut project = open(root.path());
    act(&mut project, json!({"add_layer": {"kind": "shape", "id": "cloud"}})).unwrap();
    let drift = json!({"layer": {"start": {"forever": {"layer": "cloud", "motion": "drift"}}}});
    act(&mut project, drift).unwrap();
    let position = layer(&project, "cloud")["transform"]["position"].clone();
    assert_eq!(position["loop"], "pingpong");
    assert_eq!(position["keys"][1][1], json!([1160.0, 540.0]));
    assert_eq!(position["keys"][0][1], json!([960.0, 540.0]));
    act(
        &mut project,
        json!({"layer": {"transform": {"layer": "cloud", "position": [100.0, 100.0], "replace": true}}}),
    )
    .unwrap();
    let away = json!({"layer": {"start": {"forever": {"layer": "cloud", "motion": "drift", "by": [0.0, 50.0],
        "back": false, "reverse": true, "seconds": 5.0}}}});
    act(&mut project, away).unwrap();
    let position = layer(&project, "cloud")["transform"]["position"].clone();
    assert_eq!(
        position,
        json!({"keys": [[0, [100.0, 100.0], "linear"], [5.0, [100.0, 50.0], "linear"]], "loop": "repeat"})
    );
    assert!(errors(&project).is_empty(), "{:?}", errors(&project));
}

#[test]
fn the_manual_lists_the_starts_with_their_arguments() {
    let text = wprs_editor::manual::describe("layer").unwrap();
    assert!(text.contains("start:") && text.contains("clock:") && text.contains("forever:"), "{text}");
    assert!(text.contains("motion") && text.contains("seconds"), "{text}");
}
