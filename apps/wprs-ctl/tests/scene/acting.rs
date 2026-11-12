use super::{ok, run};
use serde_json::{Value, json};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use wprs_editor::ACTIONS;

fn show(dir: &Path, pointer: &str) -> Value {
    serde_json::from_str(&ok(dir, &["scene", "show", "s", pointer])).unwrap()
}

#[test]
fn every_action_is_listed_and_has_a_schema_and_an_example() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    let listed = ok(dir, &["scene", "actions"]);
    let names: Vec<&str> = listed.lines().filter_map(|l| l.split_whitespace().next()).collect();
    assert_eq!(names, ACTIONS);
    let all: Vec<Value> = serde_json::from_str(&ok(dir, &["scene", "actions", "--json"])).unwrap();
    assert!(all.iter().all(|a| !a["about"].as_str().unwrap().is_empty()));
    for name in ACTIONS {
        let text = ok(dir, &["scene", "actions", name]);
        assert!(
            text.starts_with(&format!("{name}: ")) && text.contains(&format!("scene act DIR {name} ")),
            "{name}"
        );
        let schema: Value = serde_json::from_str(&ok(dir, &["scene", "actions", name, "--json"])).unwrap();
        assert!(schema["allOf"][0].is_object(), "{name}");
    }
    let queries = ok(dir, &["scene", "actions", "--queries"]);
    assert!(queries.lines().any(|l| l.starts_with("wind ")));
    assert!(ok(dir, &["scene", "actions", "--queries", "wind"]).contains("scene ask DIR wind"));
}

#[test]
fn any_action_is_applied_from_an_argument_a_file_or_the_standard_input() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "s", "--from", "empty"]);
    assert_eq!(ok(dir, &["scene", "act", "s", "add_layer", r#"{"kind": "solid", "id": "a"}"#]), "a\n");
    std::fs::write(dir.join("hide.json"), r#"{"id": "a", "hidden": true}"#).unwrap();
    ok(dir, &["scene", "act", "s", "hide", "--file", "hide.json"]);
    assert_eq!(show(dir, "/scene/layers/0/visible"), json!(false));
    let steps =
        r#"[{"add_layer": {"kind": "solid", "id": "b"}}, {"add_layer": {"kind": "solid", "id": "c"}}]"#;
    let mut child = Command::new(env!("CARGO_BIN_EXE_wprs-ctl"))
        .args(["scene", "act", "s", "batch", "-"])
        .current_dir(dir)
        .env("XDG_STATE_HOME", dir.join("state"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(steps.as_bytes()).unwrap();
    let done = child.wait_with_output().unwrap();
    assert_eq!(String::from_utf8(done.stdout).unwrap(), "b\nc\n");
    assert_eq!(show(dir, "/scene/layers").as_array().unwrap().len(), 3);
    ok(dir, &["scene", "act", "s", "undo"]);
    assert_eq!(show(dir, "/scene/layers").as_array().unwrap().len(), 1);
    let bad = [r#"[{"add_layer": {"kind": "solid", "id": "d"}}, {"remove_layer": {"id": "none"}}]"#];
    assert!(!run(dir, &["scene", "act", "s", "batch", bad[0]]).status.success());
    assert_eq!(show(dir, "/scene/layers").as_array().unwrap().len(), 1);
    let report: Value = serde_json::from_str(&ok(
        dir,
        &["scene", "act", "s", "lock", r#"{"id": "a", "locked": true}"#, "--json"],
    ))
    .unwrap();
    assert!(report["ops"].as_array().is_some_and(|o| !o.is_empty()));
}

#[test]
fn a_question_is_answered_as_json() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "s", "--from", "empty"]);
    let wind: Value = serde_json::from_str(&ok(dir, &["scene", "ask", "s", "wind"])).unwrap();
    assert!(wind["strength"].is_number());
    assert!(!run(dir, &["scene", "ask", "s", "nope"]).status.success());
}

#[test]
fn a_wrong_name_or_json_is_a_usage_error() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "s", "--from", "empty"]);
    for args in [
        &["scene", "act", "s", "nope"][..],
        &["scene", "act", "s", "hide", "{"],
        &["scene", "actions", "nope"],
    ] {
        assert_eq!(run(dir, args).status.code(), Some(2), "{args:?}");
    }
}

#[test]
fn a_change_that_breaks_a_rule_is_kept_and_its_problems_are_printed() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "s", "--from", "empty"]);
    ok(dir, &["scene", "act", "s", "add_layer", r#"{"kind": "solid", "id": "a"}"#]);
    let out =
        run(dir, &["scene", "act", "s", "set", r#"{"pointer": "/scene/layers/0/opacity", "value": 7}"#]);
    assert!(out.status.success());
    let said = String::from_utf8_lossy(&out.stderr);
    assert!(said.starts_with("problem: /scene/layers/0/opacity: "), "{said}");
    let out = run(
        dir,
        &["scene", "act", "s", "--json", "set", r#"{"pointer": "/scene/layers/0/opacity", "value": 7}"#],
    );
    let all: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(all["problems"], serde_json::json!([]));
    let out = run(
        dir,
        &["scene", "act", "s", "--json", "set", r#"{"pointer": "/scene/layers/0/opacity", "value": 9}"#],
    );
    let all: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(all["problems"][0]["pointer"], "/scene/layers/0/opacity", "{all}");
    let fine =
        run(dir, &["scene", "act", "s", "set", r#"{"pointer": "/scene/layers/0/opacity", "value": 0.5}"#]);
    assert!(fine.stderr.is_empty(), "{}", String::from_utf8_lossy(&fine.stderr));
}
