use serde_json::{Value, json};
use std::path::Path;
use std::process::{Command, Output, Stdio};

fn run(dir: &Path, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_wprs-ctl"));
    command.args(args).current_dir(dir).stdin(Stdio::null()).env("LC_ALL", "C");
    command.env("XDG_STATE_HOME", dir.join("state")).env("XDG_CACHE_HOME", dir.join("cache"));
    command.env("DBUS_SESSION_BUS_ADDRESS", format!("unix:path={}/no-bus", dir.display()));
    command.output().unwrap()
}

fn said(out: &Output) -> String {
    format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr))
}

fn scene(dir: &Path, scripts: &[(&str, Option<&str>)]) {
    let made = run(dir, &["scene", "new", "snow", "--from", "empty", "--title", "Snow"]);
    assert!(made.status.success(), "{}", said(&made));
    let file = dir.join("snow/scene.json");
    let mut doc: Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
    let listed: Vec<Value> =
        scripts.iter().map(|(p, _)| json!({ "path": p, "events": ["update"] })).collect();
    doc["scripts"] = json!(listed);
    std::fs::write(&file, doc.to_string()).unwrap();
    std::fs::create_dir_all(dir.join("snow/scripts")).unwrap();
    for (path, text) in scripts {
        text.iter().for_each(|text| std::fs::write(dir.join("snow").join(path), text).unwrap());
    }
}

#[test]
fn a_script_that_does_not_compile_or_is_missing_fails_validate_and_check() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    let bad = "export function update(frame) {\n  return 1;\n\nlet = = 3;\n}\n";
    scene(
        dir,
        &[
            ("scripts/ok.js", Some("export function update(frame) { return 1; }\n")),
            ("scripts/bad.js", Some(bad)),
            ("scripts/gone.js", None),
        ],
    );
    for args in [["scene", "validate", "snow"], ["pack", "check", "snow"]] {
        let out = run(dir, &args);
        let text = said(&out);
        assert!(!out.status.success(), "{args:?}: {text}");
        assert!(text.contains("scripts/bad.js:4:"), "{args:?}: {text}");
        assert!(text.contains("scripts/gone.js"), "{args:?}: {text}");
        assert!(!text.contains("scripts/ok.js"), "{args:?}: {text}");
    }
}

#[test]
fn working_scripts_keep_check_ok() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    scene(dir, &[("scripts/ok.js", Some("export function update(frame) { return 1; }\n"))]);
    let out = run(dir, &["pack", "check", "snow"]);
    assert!(out.status.success() && said(&out).starts_with("ok "), "{}", said(&out));
}
