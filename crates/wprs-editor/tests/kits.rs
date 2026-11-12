use serde_json::{Value, json};
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::kits::KITS;
use wprs_editor::package::problems;
use wprs_editor::starters::starter;

fn state() {
    static STATE: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
}

fn open(root: &std::path::Path) -> Project {
    state();
    let doc = starter("parallax", "org.example.sky", "Sky", "en").unwrap();
    Project::create(&root.join("sky"), doc).unwrap()
}

fn act(project: &mut Project, action: Value) -> anyhow::Result<()> {
    project.act(&serde_json::from_value::<Action>(action).unwrap()).map(drop)
}

fn errors(project: &Project) -> Vec<String> {
    problems(&project.dir, &project.doc).into_iter().filter(|p| !p.warning).map(|p| p.message).collect()
}

#[test]
fn a_new_particles_layer_fills_the_screen_at_once_and_falls() {
    let root = tempfile::tempdir().unwrap();
    let mut project = open(root.path());
    act(&mut project, json!({"add_layer": {"kind": "particles", "id": "nieve"}})).unwrap();
    let system = &project.doc["scene"]["particles"]["nieve"];
    assert!(system["prewarm"].as_f64().unwrap() >= system["init"]["life"][0].as_f64().unwrap());
    assert!(system["emitters"][0]["shape"].get("box").is_some());
    assert_eq!(system["init"]["angle"], json!([80, 100]));
    assert!(errors(&project).is_empty(), "{:?}", errors(&project));
}

#[test]
fn every_starting_kit_fits_a_particles_system_and_keeps_its_children() {
    let root = tempfile::tempdir().unwrap();
    let mut project = open(root.path());
    act(&mut project, json!({"add_layer": {"kind": "particles", "id": "fx"}})).unwrap();
    for kit in KITS {
        act(&mut project, json!({"particles": {"kit": {"system": "fx", "kit": kit}}})).unwrap();
        assert!(errors(&project).is_empty(), "{kit}: {:?}", errors(&project));
        assert!(project.doc["scene"]["particles"]["fx"]["prewarm"].is_number(), "{kit}");
    }
    assert!(act(&mut project, json!({"particles": {"kit": {"system": "fx", "kit": "fog"}}})).is_err());
    assert!(act(&mut project, json!({"particles": {"kit": {"system": "none", "kit": "snow"}}})).is_err());
}

#[test]
fn a_repeated_id_leaves_the_scene_untouched() {
    let root = tempfile::tempdir().unwrap();
    let mut project = open(root.path());
    act(&mut project, json!({"add_layer": {"kind": "particles", "id": "nieve"}})).unwrap();
    let before = project.doc.clone();
    assert!(act(&mut project, json!({"add_layer": {"kind": "particles", "id": "nieve"}})).is_err());
    assert!(
        act(&mut project, json!({"add_layer": {"kind": "particles", "id": "nieve", "source": "zz"}}))
            .is_err()
    );
    assert_eq!(project.doc, before);
    assert!(errors(&project).is_empty());
    let _ = json!(null);
}
