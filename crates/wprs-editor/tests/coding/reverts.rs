use super::{act, project, refused, script};
use serde_json::json;
use wprs_editor::scripts::{audit, stamp};
use wprs_editor::templates;

fn read(p: &wprs_editor::Project) -> String {
    std::fs::read_to_string(p.dir.join("scripts/x.js")).unwrap()
}

#[test]
fn writing_code_is_an_undoable_step_that_leaves_outside_edits_alone() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    script(&mut p, "x.js", "one");
    act(&mut p, json!({ "script": { "add": { "path": "scripts/x.js" } } })).unwrap();
    act(&mut p, json!({ "script": { "write": { "path": "scripts/x.js", "text": "two" } } })).unwrap();
    act(&mut p, json!({ "script": { "write": { "path": "scripts/x.js", "text": "three" } } })).unwrap();
    assert_eq!(read(&p), "three");
    let stamped = stamp(&p.dir, &p.doc);
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(read(&p), "two");
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(read(&p), "one");
    act(&mut p, json!({ "redo": {} })).unwrap();
    act(&mut p, json!({ "redo": {} })).unwrap();
    assert_eq!(read(&p), "three");
    assert_ne!(stamped, 0);
    act(&mut p, json!({ "add_layer": { "kind": "solid", "id": "dot" } })).unwrap();
    std::fs::write(p.dir.join("scripts/x.js"), "outside").unwrap();
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(read(&p), "outside");
    let same = refused(&mut p, json!({ "script": { "write": { "path": "scripts/none.js", "text": "" } } }));
    assert!(same.contains("no script"));
}

#[test]
fn settings_read_by_destructuring_count_as_used() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let code = "export const a = (v, { params: { speed, size = 2 } }) => speed;\nexport function init(ctx) { const { gap, ...rest } = ctx.params; }";
    script(&mut p, "x.js", code);
    act(&mut p, json!({ "script": { "add": { "path": "scripts/x.js" } } })).unwrap();
    let found = audit(&p.dir, &p.doc, "scripts/x.js").unwrap();
    assert_eq!(found["undeclared"]["params"], json!(["gap", "rest", "size", "speed"]));
}

#[test]
fn renaming_a_template_rewrites_exact_literals_and_lists_the_other_mentions() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, json!({ "add_layer": { "kind": "solid", "id": "dot" } })).unwrap();
    act(&mut p, json!({ "template": { "make": { "layers": ["dot"], "name": "bar" } } })).unwrap();
    script(
        &mut p,
        "x.js",
        "scene.spawn(\"bar\");\nscene.spawn('bar', {});\nscene.spawn(`b${'ar'}`);\nconst bar = 1;\nlet fooBar = 2;",
    );
    act(&mut p, json!({ "script": { "add": { "path": "scripts/x.js" } } })).unwrap();
    let before = templates::mentions(&p.dir, &p.doc, "bar");
    assert_eq!(before.len(), 3);
    assert_eq!(before[2], json!({ "path": "scripts/x.js", "line": 4, "exact": false }));
    act(&mut p, json!({ "template": { "rename": { "name": "bar", "to": "post", "rewrite": true } } }))
        .unwrap();
    assert!(read(&p).starts_with("scene.spawn(\"post\");\nscene.spawn('post', {});"));
    assert!(read(&p).contains("const bar = 1;"));
    assert_eq!(templates::mentions(&p.dir, &p.doc, "bar").len(), 1);
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert!(read(&p).starts_with("scene.spawn(\"bar\");"));
    assert!(p.doc["scene"]["templates"]["bar"].is_object());
    act(&mut p, json!({ "template": { "rename": { "name": "bar", "to": "post" } } })).unwrap();
    assert!(read(&p).starts_with("scene.spawn(\"bar\");"));
}
