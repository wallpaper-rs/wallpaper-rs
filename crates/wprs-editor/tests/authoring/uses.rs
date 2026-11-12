use super::{act, at, project, refused};
use serde_json::json;
use std::fs;
use wprs_editor::check::check_files;
use wprs_editor::rigs::query;

const SOURCE: &str = "import { scene, openShortcut } from \"wprs\";\nexport function init() {\n  const layer = scene.layer(\"card\");\n  layer.setParent(\"other\", true);\n  scene.mesh(\"bars\").create({});\n  openShortcut(\"open\");\n}\n";

fn script(p: &mut wprs_editor::Project, root: &tempfile::TempDir, declared: serde_json::Value) {
    let file = root.path().join("bars.js");
    fs::write(&file, SOURCE).unwrap();
    act(p, json!({"import_script": {"file": file}})).unwrap();
    act(p, json!({"set": {"pointer": "/scene/scripts/0", "value": {"path": "scripts/bars.js", "uses": declared, "events": ["init"]}}})).unwrap();
}

#[test]
fn importing_a_script_declares_geometry_hierarchy_and_shortcuts() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let file = root.path().join("bars.js");
    fs::write(&file, SOURCE).unwrap();
    act(&mut p, json!({"import_script": {"file": file}})).unwrap();
    assert_eq!(at(&p, "/scene/scripts/0/uses"), json!(["geometry", "shortcuts", "hierarchy"]));
}

#[test]
fn a_missing_declaration_warns_with_a_fix_and_the_fix_declares_it() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    script(&mut p, &root, json!(["layers"]));
    let dir = root.path().join("glow");
    let found = check_files(&dir, &p.doc);
    let warned = found.iter().find(|f| f.fix.is_some()).expect("a warning with a fix");
    assert!(
        warned.warning && warned.message.contains("geometry, shortcuts, hierarchy"),
        "{}",
        warned.message
    );
    let rows = query(&dir, &p.doc, "", &json!({"uses": {}}));
    assert_eq!(rows["scripts"][0]["missing"], json!(["geometry", "shortcuts", "hierarchy"]));
    let fix = warned.fix.clone().unwrap();
    act(&mut p, fix).unwrap();
    assert_eq!(at(&p, "/scene/scripts/0/uses"), json!(["layers", "geometry", "shortcuts", "hierarchy"]));
    assert!(at(&p, "/scene/pools/shapes").is_object(), "geometry needs a pool and the fix adds one");
    assert!(check_files(&dir, &p.doc).iter().all(|f| f.fix.is_none()));
}

#[test]
fn uses_can_be_added_and_removed_by_name() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    script(&mut p, &root, json!(["geometry", "hierarchy"]));
    act(&mut p, json!({"pool": {"id": "bars", "add": true}})).unwrap();
    act(&mut p, json!({"uses": {"path": "scripts/bars.js", "add": ["timers"], "remove": ["hierarchy"]}}))
        .unwrap();
    assert_eq!(at(&p, "/scene/scripts/0/uses"), json!(["timers", "geometry"]));
    assert!(refused(&mut p, json!({"uses": {"path": "scripts/none.js"}})).contains("no script"));
}
