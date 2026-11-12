use super::{act, at, family, scene};
use serde_json::json;
use wprs_editor::seats::seats;

fn pivot(p: &wprs_editor::Project, id: &str) -> [f64; 2] {
    seats(&p.dir, &p.doc, 0.0).into_iter().find(|s| s.id == id).unwrap().pivot()
}

#[test]
fn removing_a_layer_drops_what_named_it_and_keeps_followers_in_place() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, "gone");
    family(&mut p);
    let before = pivot(&p, "kid");
    act(&mut p, json!({"remove_layer": {"id": "base"}})).unwrap();
    let kid = at(&p, "/scene/layers/0");
    assert_eq!(kid["id"], "kid");
    assert!(kid.get("parent").is_none() && kid.get("mask").is_none(), "{kid}");
    let after = pivot(&p, "kid");
    assert!(
        (after[0] - before[0]).abs() < 0.05 && (after[1] - before[1]).abs() < 0.05,
        "{before:?} {after:?}"
    );
    assert!(at(&p, "/scene/timelines/0").get("trigger").is_none());
    assert_eq!(wprs_editor::actions::locked(&p.doc), Vec::<String>::new());
    assert!(wprs_editor::check::check(&p.doc).is_empty(), "{:?}", wprs_editor::check::check(&p.doc));
}

#[test]
fn a_duplicated_group_points_inside_at_its_own_copies() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, "twins");
    family(&mut p);
    act(&mut p, json!({"duplicate_layer": {"id": "box"}})).unwrap();
    let copy = at(&p, "/scene/layers/3");
    assert_eq!(copy["layers"][1]["parent"], "lead_copy", "{copy}");
    assert_eq!(copy["layers"][1]["mask"]["layer"], "lead_copy");
    assert_eq!(at(&p, "/scene/layers/2/layers/1/parent"), "lead");
}

#[test]
fn renaming_a_layer_rewrites_every_reference_and_warns_about_scripts() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, "renamed");
    family(&mut p);
    std::fs::create_dir_all(p.dir.join("scripts")).unwrap();
    std::fs::write(p.dir.join("scripts/a.js"), "const a = 1;\nscene.layer('base');\n").unwrap();
    act(&mut p, json!({"set": {"pointer": "/scene/scripts", "value": [{"path": "scripts/a.js"}]}})).unwrap();
    assert_eq!(wprs_editor::references::scripts(&p.dir, &p.doc, "base"), ["scripts/a.js:2"]);
    act(&mut p, json!({"rename": {"kind": "layer", "id": "base", "to": "stage"}})).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/id"), "stage");
    assert_eq!(at(&p, "/scene/layers/1/parent"), "stage");
    assert_eq!(at(&p, "/scene/layers/1/mask/layer"), "stage");
    assert_eq!(at(&p, "/scene/timelines/0/trigger"), "click:stage");
    assert_eq!(wprs_editor::actions::locked(&p.doc), ["stage"]);
    let taken = act(&mut p, json!({"rename": {"kind": "layer", "id": "kid", "to": "lead"}})).unwrap_err();
    assert!(taken.to_string().contains("in use"), "{taken}");
    let bad = act(&mut p, json!({"rename": {"kind": "layer", "id": "kid", "to": "Big Kid"}})).unwrap_err();
    assert!(bad.to_string().contains("small letters"), "{bad}");
}

#[test]
fn timelines_assets_and_systems_are_renamed_with_their_users() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, "things");
    family(&mut p);
    let keyed = json!({"timeline": "pop", "keys": [[0, 0], [1, 1]]});
    act(&mut p, json!({"set": {"pointer": "/scene/layers/0/opacity", "value": keyed}})).unwrap();
    act(&mut p, json!({"rename": {"kind": "timeline", "id": "pop", "to": "bounce"}})).unwrap();
    assert_eq!(at(&p, "/scene/timelines/0/id"), "bounce");
    assert_eq!(at(&p, "/scene/layers/0/opacity/timeline"), "bounce");
    let file = super::picture(&p.dir, "sky.png", 10);
    act(&mut p, json!({"add_layer": {"kind": "image", "id": "sky", "file": file}})).unwrap();
    let asset = at(&p, "/scene/layers/3/image").as_str().unwrap().to_owned();
    act(&mut p, json!({"rename": {"kind": "asset", "id": asset, "to": "clouds"}})).unwrap();
    assert_eq!(at(&p, "/scene/layers/3/image"), "clouds");
    assert!(at(&p, "/scene/assets/clouds").is_object());
    act(&mut p, json!({"add_layer": {"kind": "particles", "id": "dust"}})).unwrap();
    act(&mut p, json!({"rename": {"kind": "layer", "id": "dust", "to": "dust_layer"}})).unwrap();
    act(&mut p, json!({"rename": {"kind": "system", "id": "dust", "to": "motes"}})).unwrap();
    assert_eq!(at(&p, "/scene/layers/4/system"), "motes");
}

#[test]
fn renaming_an_asset_rewrites_only_the_fields_that_refer_to_assets() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, "refs");
    let file = super::picture(&p.dir, "sky.png", 10);
    act(&mut p, json!({"add_layer": {"kind": "image", "id": "sky", "file": file}})).unwrap();
    let asset = at(&p, "/scene/layers/0/image").as_str().unwrap().to_owned();
    act(&mut p, json!({"add_layer": {"kind": "text", "id": "words"}})).unwrap();
    act(&mut p, json!({"set": {"pointer": "/scene/layers/1/text", "value": asset}})).unwrap();
    act(&mut p, json!({"set": {"pointer": "/scene/ext/note", "value": asset}})).unwrap();
    act(&mut p, json!({"rename": {"kind": "asset", "id": asset, "to": "clouds"}})).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/image"), "clouds");
    assert_eq!(at(&p, "/scene/layers/1/text"), json!(asset));
    assert_eq!(at(&p, "/scene/ext/note"), json!(asset));
}
