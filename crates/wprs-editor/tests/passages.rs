use serde_json::json;
use std::fs;
use tempfile::TempDir;
use wprs_editor::assets::unused;

#[test]
fn the_files_of_a_transition_are_not_listed_as_unused() {
    let dir = TempDir::new().expect("dir");
    fs::create_dir_all(dir.path().join("t")).expect("folder");
    for name in ["t/iris.json", "t/iris.wgsl", "t/spare.wgsl"] {
        fs::write(dir.path().join(name), "").expect("file");
    }
    let way = json!({ "descriptor": "t/iris.json", "wgsl": "t/iris.wgsl" });
    let doc = json!({ "manifest": {}, "scene": { "transition": { "in": way, "out": way } } });
    assert_eq!(unused(dir.path(), &doc), ["t/spare.wgsl"]);
}
