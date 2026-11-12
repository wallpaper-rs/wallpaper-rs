use serde_json::{Value, json};
use std::path::Path;
use wprs_editor::actions::Action;
use wprs_editor::starters::starter;
use wprs_editor::{Project, package};

fn state() {
    let dir = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
    std::mem::forget(dir);
}

fn act(project: &mut Project, action: Value) -> anyhow::Result<()> {
    project.act(&serde_json::from_value::<Action>(action).unwrap()).map(drop)
}

fn preset(root: &Path) -> Project {
    state();
    let doc = starter("preset", "org.example.dusk_calm", "Calm dusk", "en").unwrap();
    Project::create(&root.join("preset"), doc).unwrap()
}

fn done(project: &Project) -> Vec<String> {
    let steps = project.ask("", &json!({"preset_steps": {}}));
    let rows = steps["steps"].as_array().unwrap().iter();
    rows.filter(|s| s["done"] == true).map(|s| s["id"].as_str().unwrap().to_owned()).collect()
}

#[test]
fn the_steps_of_a_preset_fill_in_one_by_one_until_it_can_be_published() {
    let root = tempfile::tempdir().unwrap();
    let mut p = preset(root.path());
    assert_eq!(p.ask("", &json!({"preset_steps": {}}))["ready"], false);
    assert_eq!(done(&p), ["title", "license"]);
    let base = json!({"base": {"id": "org.example.dusk", "version": "1.2.3"}});
    act(&mut p, json!({"manifest": {"preset": base}})).unwrap();
    act(&mut p, json!({"manifest": {"preset": {"values": {"speed": 0.4}}}})).unwrap();
    assert_eq!(done(&p), ["base", "values", "title", "license", "check"]);
    let picture = root.path().join("sky.png");
    image::RgbImage::new(4, 4).save(&picture).unwrap();
    act(&mut p, json!({"thumbnail": {"file": picture}})).unwrap();
    let steps = p.ask("", &json!({"preset_steps": {}}));
    assert_eq!((steps["ready"].clone(), steps["errors"].clone()), (json!(true), json!(0)), "{steps}");
}

#[test]
fn a_package_that_is_not_a_preset_has_no_steps() {
    state();
    let root = tempfile::tempdir().unwrap();
    let doc = starter("empty", "org.example.sea", "Sea", "en").unwrap();
    let p = Project::create(&root.path().join("sea"), doc).unwrap();
    assert_eq!(p.ask("", &json!({"preset_steps": {}}))["preset"], false);
}

#[test]
fn a_preset_is_drawn_over_its_base_with_its_values() {
    if wprs_render::Renderer::shared().is_err() {
        return;
    }
    state();
    let root = tempfile::tempdir().unwrap();
    let doc = starter("empty", "org.example.base", "Base", "en").unwrap();
    let mut scene = Project::create(&root.path().join("base"), doc).unwrap();
    act(&mut scene, json!({"add_layer": {"kind": "solid", "id": "box"}})).unwrap();
    scene.save().unwrap();
    let file = root.path().join("base.wpak");
    package::build(&mut scene, &file, None).unwrap();
    let mut p = preset(root.path());
    let base = json!({"base": {"id": "org.example.base", "version": "1.0.0"}});
    act(&mut p, json!({"manifest": {"preset": base}})).unwrap();
    assert!(act(&mut p, json!({"shot": {"still": {}}})).is_err(), "a preset needs its base file to be drawn");
    act(&mut p, json!({"shot": {"still": {"base": file}}})).unwrap();
    let saved = p.dir.join(p.doc["manifest"]["previews"]["thumbnail"].as_str().unwrap());
    assert!(image::image_dimensions(saved).unwrap().0 > 0);
    act(&mut p, json!({"shot": {"recording": {"seconds": 0.4, "fps": 5, "base": file}}})).unwrap();
    assert!(p.doc["manifest"]["previews"]["animated"].is_string());
}
