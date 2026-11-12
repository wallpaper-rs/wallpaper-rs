use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use wprs_editor::{ACTIONS, manual};

const FIXTURES: [(&str, &str); 7] = [
    ("foto.png", "scenes/pulse/assets/cover.png"),
    ("hoja.png", "tests/fixtures/bands-640x180.png"),
    ("sonido.ogg", "scenes/pond/assets/water.ogg"),
    ("modelo.glb", "tests/fixtures/rig.glb"),
    ("figura.glb", "tests/fixtures/toy.glb"),
    ("piezas.glb", "tests/fixtures/parts.glb"),
    ("video.webm", "tests/fixtures/magenta-64x32.webm"),
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn pages(prefix: &str) -> Vec<(String, String)> {
    let mut found: Vec<_> = std::fs::read_dir(root().join("docs/crear"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "md"))
        .filter(|p| p.file_name().unwrap().to_string_lossy().starts_with(prefix))
        .map(|p| {
            (p.file_name().unwrap().to_string_lossy().into_owned(), std::fs::read_to_string(&p).unwrap())
        })
        .collect();
    found.sort();
    found
}

fn blocks(text: &str) -> Vec<String> {
    let mut all = Vec::new();
    let mut open: Option<String> = None;
    for line in text.lines() {
        match (&mut open, line.trim_end()) {
            (None, "```sh") => open = Some(String::new()),
            (Some(_), "```") => all.extend(open.take()),
            (Some(block), _) => {
                block.push_str(line);
                block.push('\n');
            }
            _ => {}
        }
    }
    all
}

#[test]
fn every_action_and_query_has_an_entry_and_an_example_in_the_reference() {
    let text: String = pages("acciones").into_iter().map(|(_, t)| t).collect();
    let mut missing = Vec::new();
    for name in ACTIONS {
        let called = text.lines().any(|l| {
            l.strip_prefix(&format!("wprs-ctl scene act mi-escena {name} ")).is_some()
                || l == format!("wprs-ctl scene act mi-escena {name}")
        });
        if !called || !text.contains(&format!("#### `{name}`")) {
            missing.push(format!("action {name}"));
        }
    }
    for (name, ..) in manual::ASKS {
        let called = text.lines().any(|l| {
            l.strip_prefix(&format!("wprs-ctl scene ask mi-escena {name}"))
                .is_some_and(|r| r.is_empty() || r.starts_with(' '))
        });
        if !called || !text.contains(&format!("#### `{name}`")) {
            missing.push(format!("query {name}"));
        }
    }
    assert!(missing.is_empty(), "docs/crear/acciones*.md lacks: {missing:?}");
    let index = std::fs::read_to_string(root().join("docs/crear/acciones.md")).unwrap();
    let unnamed: BTreeSet<_> = ACTIONS
        .iter()
        .copied()
        .chain(manual::ASKS.iter().map(|(n, ..)| *n))
        .filter(|n| !index.contains(&format!("[{n}](")))
        .collect();
    assert!(unnamed.is_empty(), "docs/crear/acciones.md does not index {unnamed:?}");
}

fn run_block(block: &str, bin: &Path) -> Result<(), String> {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    for (name, from) in FIXTURES {
        std::fs::copy(root().join(from), dir.join(name)).unwrap();
    }
    std::fs::create_dir(dir.join("bin")).unwrap();
    std::os::unix::fs::symlink(bin, dir.join("bin/wprs-ctl")).unwrap();
    let path = format!("{}:{}", dir.join("bin").display(), std::env::var("PATH").unwrap_or_default());
    let script = if block.contains("scene new ") {
        block.to_owned()
    } else {
        format!("wprs-ctl scene new mi-escena >/dev/null\n{block}")
    };
    let out = Command::new("bash")
        .args(["-e", "-o", "pipefail", "-c", &script])
        .current_dir(dir)
        .env("PATH", path)
        .env("LC_ALL", "C")
        .env("XDG_STATE_HOME", dir.join("state"))
        .env("DBUS_SESSION_BUS_ADDRESS", format!("unix:path={}/no-bus", dir.display()))
        .output()
        .unwrap();
    match out.status.success() {
        true => Ok(()),
        false => Err(format!("{block}\n{}", String::from_utf8_lossy(&out.stderr))),
    }
}

#[test]
fn every_example_line_of_the_creator_pages_works() {
    let bin = PathBuf::from(env!("CARGO_BIN_EXE_wprs-ctl"));
    let mut failed = Vec::new();
    let mut count = 0;
    for (page, text) in pages("acciones").into_iter().chain(pages("editor")) {
        for block in blocks(&text) {
            count += 1;
            if let Err(why) = run_block(&block, &bin) {
                failed.push(format!("{page}: {why}"));
            }
        }
    }
    assert!(count > 100, "only {count} example blocks found");
    assert!(failed.is_empty(), "{} blocks fail:\n{}", failed.len(), failed.join("\n---\n"));
}
