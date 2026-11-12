use super::{act, page};
use serde_json::json;

#[test]
fn a_page_asks_for_the_lights_with_the_permission_of_its_manifest() {
    let root = tempfile::tempdir().unwrap();
    let mut project = page(root.path());
    assert!(project.doc["manifest"]["permissions"]["rgb"].is_null());
    act(&mut project, json!({ "set": { "pointer": "/manifest/permissions/rgb", "value": true } })).unwrap();
    assert_eq!(project.doc["manifest"]["permissions"]["rgb"], true);
    let problems = wprs_editor::package::problems(&project.dir, &project.doc);
    assert!(problems.iter().all(|p| p.warning || !p.pointer.contains("permissions")), "{problems:?}");
    act(&mut project, json!({ "remove": { "pointer": "/manifest/permissions/rgb" } })).unwrap();
    assert!(project.doc["manifest"]["permissions"]["rgb"].is_null());
}
