use super::{act, project, refused, script};
use serde_json::json;
use wprs_editor::molds::SCRIPT_TEMPLATES;
use wprs_editor::scripts::{audit, changes, listing, reference, stamp};

#[test]
fn every_script_template_imports_declared_and_in_step_with_its_code() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    for name in SCRIPT_TEMPLATES {
        act(&mut p, json!({ "import_script": { "file": "", "template": name } })).unwrap();
    }
    let listed = listing(&p.dir, &p.doc);
    assert_eq!(listed["scripts"].as_array().unwrap().len(), SCRIPT_TEMPLATES.len());
    for entry in listed["scripts"].as_array().unwrap() {
        assert_eq!(entry["synced"], true, "{entry}");
    }
    let spawn = &p.doc["scene"]["scripts"][5];
    assert_eq!(spawn["params"]["count"]["default"], 8);
    assert_eq!(spawn["uses"], json!(["layers"]));
    let problems = wprs_editor::package::problems(&p.dir, &p.doc);
    assert!(problems.iter().all(|c| c.warning), "{problems:?}");
}

#[test]
fn importing_a_module_again_keeps_its_settings_and_says_what_changed() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let file = root.path().join("bars.js");
    std::fs::write(&file, "export function update() {}").unwrap();
    act(&mut p, json!({ "import_script": { "file": file } })).unwrap();
    let param =
        json!({ "script": { "path": "scripts/bars.js", "id": "speed", "type": "number", "default": 2 } });
    act(&mut p, json!({ "script": { "param": param["script"] } })).unwrap();
    let before = p.doc["scene"]["scripts"].clone();
    std::fs::write(&file, "import { audio } from \"wprs\";\nexport function click() { audio.bass; }")
        .unwrap();
    act(&mut p, json!({ "import_script": { "file": file } })).unwrap();
    assert_eq!(p.doc["scene"]["scripts"][0]["params"]["speed"]["default"], 2);
    let said: Vec<String> =
        changes(&before, &p.doc["scene"]["scripts"]).iter().map(|m| m.to_string()).collect();
    assert!(said.contains(&"scripts/bars.js now handles click".to_owned()), "{said:?}");
    assert!(said.contains(&"scripts/bars.js now uses audio".to_owned()), "{said:?}");
    assert!(said.contains(&"scripts/bars.js kept its 1 settings".to_owned()), "{said:?}");
    let same = changes(&p.doc["scene"]["scripts"], &p.doc["scene"]["scripts"]);
    assert_eq!(same[0].to_string(), "scripts/bars.js kept its 1 settings");
}

#[test]
fn the_code_is_checked_against_its_declaration_and_declared_from_what_it_uses() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    script(&mut p, "x.js", "export function update() {}");
    act(&mut p, json!({ "script": { "add": { "path": "scripts/x.js" } } })).unwrap();
    assert_eq!(audit(&p.dir, &p.doc, "scripts/x.js").unwrap()["synced"], true);
    let code = "import { audio, setTimeout } from \"wprs\";\nexport function init({ params }) { audio.bass; params.speed; }\nexport function click() {}";
    act(&mut p, json!({ "script": { "write": { "path": "scripts/x.js", "text": code } } })).unwrap();
    let found = audit(&p.dir, &p.doc, "scripts/x.js").unwrap();
    assert_eq!(found["undeclared"]["events"], json!(["init", "click"]));
    assert_eq!(found["undeclared"]["uses"], json!(["audio", "timers"]));
    assert_eq!(found["undeclared"]["params"], json!(["speed"]));
    assert_eq!(found["synced"], false);
    act(&mut p, json!({ "script": { "declare": { "path": "scripts/x.js", "params": true } } })).unwrap();
    let entry = &p.doc["scene"]["scripts"][0];
    assert_eq!(entry["uses"], json!(["audio", "timers"]));
    assert_eq!(entry["params"]["speed"], json!({ "type": "number", "default": 0 }));
    assert_eq!(audit(&p.dir, &p.doc, "scripts/x.js").unwrap()["synced"], true);
    act(
        &mut p,
        json!({ "script": { "write": { "path": "scripts/x.js", "text": "export function init() {}" } } }),
    )
    .unwrap();
    let found = audit(&p.dir, &p.doc, "scripts/x.js").unwrap();
    assert_eq!(found["unused"]["uses"], json!(["audio", "timers"]));
    assert_eq!(found["unused"]["params"], json!(["speed"]));
    let kept = p.doc["scene"]["scripts"][0].clone();
    act(&mut p, json!({ "set": { "pointer": "/scene/scripts/0/uses", "value": ["clicks", "audio"] } }))
        .unwrap();
    act(&mut p, json!({ "script": { "declare": { "path": "scripts/x.js" } } })).unwrap();
    assert_eq!(p.doc["scene"]["scripts"][0]["uses"], json!(["clicks"]));
    assert_ne!(kept, p.doc["scene"]["scripts"][0]);
}

#[test]
fn writing_code_changes_the_stamp_the_preview_reloads_on_and_the_api_is_listed() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    script(&mut p, "x.js", "export function update() {}");
    act(&mut p, json!({ "script": { "add": { "path": "scripts/x.js" } } })).unwrap();
    let first = stamp(&p.dir, &p.doc);
    assert_eq!(first, stamp(&p.dir, &p.doc));
    act(&mut p, json!({ "script": { "write": { "path": "scripts/x.js", "text": "export function update() { 1 }" } } })).unwrap();
    assert_ne!(first, stamp(&p.dir, &p.doc));
    assert!(
        refused(&mut p, json!({ "script": { "write": { "path": "scripts/other.js", "text": "" } } }))
            .contains("no script")
    );
    let parts = reference();
    assert!(parts.as_array().unwrap().len() >= 4);
    assert!(parts[0]["items"][0]["text"].as_str().is_some_and(|t| t.starts_with("A layer by id")));
}
