use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use wprs_editor::{ACTIONS, manual};

const FIXTURES: [(&str, &str); 6] = [
    ("picture.png", "scenes/pulse/assets/cover.png"),
    ("sheet.png", "tests/fixtures/bands-640x180.png"),
    ("sound.ogg", "scenes/pond/assets/water.ogg"),
    ("model.glb", "tests/fixtures/rig.glb"),
    ("figure.glb", "tests/fixtures/toy.glb"),
    ("video.webm", "tests/fixtures/magenta-64x32.webm"),
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn shell(dir: &Path, script: &str) -> Output {
    let bin = PathBuf::from(env!("CARGO_BIN_EXE_wprs-ctl"));
    let path = format!("{}:{}", dir.join("bin").display(), std::env::var("PATH").unwrap_or_default());
    let _ = std::os::unix::fs::symlink(&bin, dir.join("bin/wprs-ctl"));
    Command::new("bash")
        .args(["-e", "-o", "pipefail", "-c", script])
        .current_dir(dir)
        .env("PATH", path)
        .env("LC_ALL", "C")
        .env("XDG_STATE_HOME", dir.join("state"))
        .env("DBUS_SESSION_BUS_ADDRESS", format!("unix:path={}/no-bus", dir.display()))
        .output()
        .unwrap()
}

fn prepared() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    for (name, from) in FIXTURES {
        std::fs::copy(root().join(from), tmp.path().join(name)).unwrap();
    }
    std::fs::create_dir(tmp.path().join("bin")).unwrap();
    tmp
}

fn failure(kind: &str, name: &str, lines: &[&str], out: &Output, query: bool) -> Option<String> {
    let said = String::from_utf8_lossy(&out.stdout);
    let broken = !out.status.success()
        || (query
            && (said.trim() == "null"
                || serde_json::from_str::<Value>(&said).is_ok_and(|v| v["error"].is_string())));
    broken.then(|| {
        format!("{kind} {name}:\n{}\n{said}{}", lines.join("\n"), String::from_utf8_lossy(&out.stderr))
    })
}

fn tried(kind: &str, name: &str, tmp: &Path, ready: bool) -> Option<String> {
    let Some(lines) = manual::example_lines(kind, name) else {
        return Some(format!("{kind} {name} has no example"));
    };
    let (last, before) = lines.split_last().unwrap();
    let scene = |l: &str| l.replace("DIR", "mi-escena");
    let work = tmp.join(format!("{kind}-{name}"));
    std::fs::create_dir_all(work.join("bin")).unwrap();
    for (file, _) in FIXTURES {
        std::fs::copy(tmp.join(file), work.join(file)).unwrap();
    }
    let setup: Vec<String> = before.iter().map(|l| scene(l)).collect();
    let script = format!("wprs-ctl scene new mi-escena >/dev/null\n{}\n", setup.join("\n"));
    let first = shell(&work, &script);
    if let Some(why) = failure(kind, name, before, &first, false) {
        return Some(why);
    }
    if !ready && name == "depth_map" {
        return None;
    }
    failure(kind, name, &lines, &shell(&work, &scene(last)), kind == "ask")
}

#[test]
fn every_example_the_program_prints_runs_on_a_scene_prepared_for_it() {
    let tmp = prepared();
    let bin =
        shell(tmp.path(), "wprs-ctl scene new probe >/dev/null && wprs-ctl scene ask probe depth_model");
    let ready = serde_json::from_slice::<Value>(&bin.stdout).is_ok_and(|v| v["ready"] == true);
    let mut failed = Vec::new();
    for name in ACTIONS {
        failed.extend(tried("act", name, tmp.path(), ready));
    }
    for (name, _) in manual::ASKS {
        failed.extend(tried("ask", name, tmp.path(), ready));
    }
    assert!(failed.is_empty(), "{} examples fail:\n{}", failed.len(), failed.join("\n---\n"));
}
