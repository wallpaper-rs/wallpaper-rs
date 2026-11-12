use super::{act, edit, project};
use serde_json::json;
use wprs_editor::references::script_uses;

#[test]
fn scripts_that_name_a_renamed_system_are_listed_by_line_and_rewritten_on_request() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let source = "const a = 1;\nwprs.particles(\"flakes\").rate = 4;\nconst b = 'flakes_more';\nuse('flakes'); use(`flakes`);\n";
    std::fs::create_dir_all(p.dir.join("scripts")).unwrap();
    std::fs::write(p.dir.join("scripts/a.js"), source).unwrap();
    std::fs::write(p.dir.join("scripts/b.js"), "const x = \"other\";\n").unwrap();
    act(&mut p, json!({"set": {"pointer": "/scene/scripts", "value": [{"path": "scripts/a.js"}, {"path": "scripts/b.js"}]}}))
        .unwrap();
    let found = script_uses(&p.dir, &p.doc, "flakes");
    let lines: Vec<(String, usize)> = found.into_iter().map(|u| (u.path, u.line)).collect();
    assert_eq!(lines, [("scripts/a.js".to_owned(), 2), ("scripts/a.js".to_owned(), 4)]);
    let early = act(&mut p, json!({"particles": {"scripts": {"from": "flakes", "to": "snow"}}}));
    assert!(early.is_err(), "the new name must exist and the old one must be gone");
    edit(&mut p, "rename", json!({"to": "snow"})).unwrap();
    let kept = act(&mut p, json!({"particles": {"scripts": {"from": "flakes", "to": "nowhere"}}}));
    assert!(kept.is_err(), "the target must be a system");
    act(&mut p, json!({"particles": {"scripts": {"from": "flakes", "to": "snow"}}})).unwrap();
    let text = std::fs::read_to_string(p.dir.join("scripts/a.js")).unwrap();
    assert_eq!(
        text,
        "const a = 1;\nwprs.particles(\"snow\").rate = 4;\nconst b = 'flakes_more';\nuse('snow'); use(`snow`);\n"
    );
    assert_eq!(std::fs::read_to_string(p.dir.join("scripts/b.js")).unwrap(), "const x = \"other\";\n");
    assert!(script_uses(&p.dir, &p.doc, "flakes").is_empty());
}

#[test]
fn a_scene_without_systems_gets_its_first_one_from_the_panel_actions() {
    let root = tempfile::tempdir().unwrap();
    super::state();
    let doc = wprs_editor::starters::starter("empty", "org.example.bare", "Bare", "en").unwrap();
    let mut p = wprs_editor::Project::create(&root.path().join("bare"), doc).unwrap();
    act(&mut p, json!({"particles": {"create": {"system": "first"}}})).unwrap();
    act(&mut p, json!({"particles": {"create": {"system": "second", "from": "first"}}})).unwrap();
    act(&mut p, json!({"particles": {"create": {"system": "third", "parent": "second"}}})).unwrap();
    let systems = p.doc["scene"]["particles"].as_object().unwrap();
    assert_eq!(systems.keys().collect::<Vec<_>>(), ["first", "second", "third"]);
    act(&mut p, json!({"particles": {"delete": {"system": "third"}}})).unwrap_err();
    act(&mut p, json!({"particles": {"rename": {"system": "first", "to": "base"}}})).unwrap();
    assert!(p.doc["scene"]["particles"].get("base").is_some());
}
