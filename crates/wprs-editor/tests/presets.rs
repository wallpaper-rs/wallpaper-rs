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

#[test]
fn the_settings_of_the_base_come_with_their_widgets_and_what_the_preset_sets() {
    let root = tempfile::tempdir().unwrap();
    let mut project = preset(root.path());
    let mut doc = starter("particles", "org.example.dusk", "Dusk", "en").unwrap();
    doc["manifest"]["properties"] = json!({
        "tint": {"type": "color", "default": "#ffffff", "order": 1},
        "speed": {"type": "slider", "min": 0.5, "max": 4, "default": 1, "unit": "x", "order": 2},
        "mood": {"type": "combo", "options": [{"value": "calm", "label": "Calm"}, {"value": "storm", "label": "Storm"}],
            "default": "calm", "order": 3},
        "snow": {"type": "bool", "default": true, "order": 4},
        "name": {"type": "text", "default": "", "order": 5},
        "photo": {"type": "file", "accept": ["image"], "order": 6},
        "go": {"type": "button", "order": 7}
    });
    let base = Project::create(&root.path().join("base"), doc).unwrap();
    act(&mut project, json!({"manifest": {"preset": {"values": {"speed": 2.5}}}})).unwrap();
    let found = wprs_editor::bases::settings(&base.dir, &project.doc["preset"]);
    assert_eq!((found["id"].as_str(), found["version"].as_str()), (Some("org.example.dusk"), Some("0.1.0")));
    let rows = found["fields"].as_array().unwrap();
    let widgets: Vec<(&str, &str)> =
        rows.iter().map(|f| (f["key"].as_str().unwrap(), f["widget"].as_str().unwrap())).collect();
    let wanted =
        [("tint", "rgb"), ("speed", "number"), ("mood", "ref:option"), ("snow", "bool"), ("name", "text")];
    assert_eq!(widgets, wanted, "files, buttons and groups are not values");
    let speed = &rows[1];
    assert_eq!(
        (&speed["range"], &speed["unit"], &speed["set"]),
        (&json!([0.5, 4.0]), &json!("x"), &json!(true))
    );
    assert_eq!((&speed["value"], &speed["pointer"]), (&json!(2.5), &json!("/preset/values/speed")));
    assert_eq!((&rows[0]["set"], &rows[0]["default"]), (&json!(false), &json!("#ffffff")));
    assert_eq!(rows[2]["names"], json!(["Calm", "Storm"]));
    let photos = found["files"].as_array().unwrap();
    assert_eq!(
        (photos.len(), photos[0]["id"].as_str(), photos[0]["accept"].clone()),
        (1, Some("photo"), json!(["image"]))
    );
    assert_eq!(wprs_editor::bases::settings(&root.path().join("missing"), &Value::Null), Value::Null);
    let asked =
        wprs_editor::rigs::query(&project.dir, &project.doc, "", &json!({"base": {"file": base.dir}}));
    assert_eq!(asked, found, "the editor query answers the same");
}
