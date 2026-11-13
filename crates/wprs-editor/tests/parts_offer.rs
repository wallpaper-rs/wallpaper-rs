use image::{ImageFormat, RgbImage};
use serde_json::{Value, json};
use wprs_editor::starters::starter;
use wprs_editor::{Project, package, stock};

fn act(project: &mut Project, action: Value) -> anyhow::Result<()> {
    project.act(&serde_json::from_value(action)?).map(drop)
}

#[test]
fn the_titles_and_thumbnails_of_an_open_resource_package_are_edited_in_place() {
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", root.path()) };
    let mut doc = starter("assets", "org.example.kit", "Kit", "en").unwrap();
    doc["manifest"]["authors"] = json!(["Kit Maker"]);
    doc["manifest"]["license"] = json!("CC-BY-4.0");
    let mut kit = Project::create(&root.path().join("kit"), doc).unwrap();
    let picture = root.path().join("sky.png");
    RgbImage::from_pixel(64, 32, [40, 90, 160].into()).save_with_format(&picture, ImageFormat::Png).unwrap();
    for id in ["backdrop", "mist"] {
        act(&mut kit, json!({"add_layer": {"kind": "solid", "id": id}})).unwrap();
    }
    let part = |extra: Value| {
        let mut body = json!({"kind": "layer", "id": "mist"});
        body.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
        json!({"manifest": {"part": body}})
    };
    let edit = part(json!({"title": " Soft mist ", "thumbnail": picture}));
    act(&mut kit, edit).unwrap();
    let parts = kit.doc["scene"]["ext"]["parts"].as_array().unwrap().clone();
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], json!({"kind": "layer", "id": "backdrop"}));
    assert_eq!(
        (parts[1]["title"].as_str(), parts[1]["thumbnail"].as_str()),
        (Some("Soft mist"), Some("parts/1.jpg"))
    );
    assert!(kit.dir.join("parts/1.jpg").is_file());
    kit.save().unwrap();
    let wpak = root.path().join("kit.wpak");
    package::build_saved(&kit.dir, &kit.doc, &wpak, None).unwrap();
    let listed = stock::browse(&wpak, None).unwrap();
    assert_eq!(listed["parts"][1]["title"], "Soft mist");
    act(&mut kit, part(json!({"title": "", "clear_thumbnail": true}))).unwrap();
    assert_eq!(kit.doc["scene"]["ext"]["parts"][1], json!({"kind": "layer", "id": "mist"}));
    assert!(act(&mut kit, part(json!({"title": "<b>"}))).is_err());
    assert!(
        act(&mut kit, json!({"manifest": {"part": {"kind": "layer", "id": "ghost", "title": "x"}}})).is_err()
    );
}

#[test]
fn the_form_of_a_resource_package_has_a_parts_field_and_other_packages_do_not() {
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", root.path()) };
    let kit = starter("assets", "org.example.kit", "Kit", "en").unwrap();
    let kit = Project::create(&root.path().join("kit"), kit).unwrap();
    let scene = starter("empty", "org.example.scene", "Scene", "en").unwrap();
    let scene = Project::create(&root.path().join("scene"), scene).unwrap();
    let has = |p: &Project| {
        wprs_editor::inspect::fields(&p.doc, "/manifest").iter().any(|f| f.widget == "package:parts")
    };
    assert!(has(&kit) && !has(&scene));
}
