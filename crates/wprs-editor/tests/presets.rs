use serde_json::{Value, json};
use std::path::Path;
use wprs_editor::actions::Action;
use wprs_editor::starters::starter;
use wprs_editor::{Project, check};

fn act(project: &mut Project, action: Value) -> anyhow::Result<()> {
    project.act(&serde_json::from_value::<Action>(action).unwrap()).map(drop)
}

fn preset(root: &Path) -> Project {
    let dir = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
    std::mem::forget(dir);
    let doc = starter("preset", "org.example.dusk_calm", "Calm dusk", "en").unwrap();
    Project::create(&root.join("preset"), doc).unwrap()
}

fn errors(project: &Project) -> Vec<String> {
    let found = check::review(&project.doc, Some(&project.dir));
    found.into_iter().filter(|p| !p.warning).map(|p| p.message).collect()
}

#[test]
fn a_preset_package_names_its_base_values_and_files() {
    let root = tempfile::tempdir().unwrap();
    let mut project = preset(root.path());
    assert_eq!(project.doc["manifest"]["kind"], "preset");
    assert!(!errors(&project).is_empty(), "a preset without a base is not valid yet");
    let base = json!({"base": {"id": "org.example.dusk", "version": "1.2.3"}});
    act(&mut project, json!({"manifest": {"preset": base}})).unwrap();
    let wanted = json!({"id": "org.example.dusk", "version": "^1.2"});
    assert_eq!(project.doc["preset"]["base"], wanted);
    assert_eq!(project.doc["manifest"]["requires"]["packages"], json!([wanted]));
    assert!(project.doc["manifest"]["requires"]["features"].as_array().unwrap().contains(&json!("preset")));
    act(&mut project, json!({"manifest": {"preset": {"values": {"tint": "#3d5aff", "speed": 0.4}}}}))
        .unwrap();
    assert_eq!(project.doc["preset"]["values"], json!({"tint": "#3d5aff", "speed": 0.4}));
    let picture = root.path().join("sky.png");
    image::RgbImage::new(4, 4).save(&picture).unwrap();
    let files = json!({"preset": {"files": {"photo": picture}, "values": {"speed": 0.6}}});
    act(&mut project, json!({"manifest": files})).unwrap();
    assert_eq!(project.doc["preset"]["files"], json!({"photo": "assets/sky.png"}));
    assert!(project.dir.join("assets/sky.png").is_file());
    assert_eq!(errors(&project), Vec::<String>::new());
    act(&mut project, json!({"manifest": {"preset": {"values": {"photo": "x"}}}})).unwrap();
    assert!(project.doc["preset"]["files"].get("photo").is_none());
    let text = root.path().join("notes.txt");
    std::fs::write(&text, "x").unwrap();
    assert!(act(&mut project, json!({"manifest": {"preset": {"files": {"photo": text}}}})).is_err());
    assert!(act(&mut project, json!({"manifest": {"preset": {"values": {"2fast": 1}}}})).is_err());
    let itself = json!({"base": {"id": "org.example.dusk_calm"}});
    assert!(act(&mut project, json!({"manifest": {"preset": itself}})).is_err());
    act(&mut project, json!({"manifest": {"preset": {"clear": ["tint", "photo"]}}})).unwrap();
    assert_eq!(project.doc["preset"]["values"], json!({"speed": 0.6}));
    project.save().unwrap();
    let written: Value =
        serde_json::from_str(&std::fs::read_to_string(project.dir.join("preset.json")).unwrap()).unwrap();
    assert_eq!(written["values"], json!({"speed": 0.6}));
    assert_eq!(Project::open(&project.dir).unwrap().doc["preset"], written);
    act(&mut project, json!({"undo": {}})).unwrap();
    assert_eq!(project.doc["preset"]["values"]["tint"], "#3d5aff");
}

#[test]
fn only_a_preset_package_takes_preset_values() {
    let root = tempfile::tempdir().unwrap();
    let mut project = preset(root.path());
    project.doc["manifest"]["kind"] = json!("web");
    assert!(act(&mut project, json!({"manifest": {"preset": {"values": {"a": 1}}}})).is_err());
}
