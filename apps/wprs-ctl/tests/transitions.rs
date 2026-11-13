use std::path::{Path, PathBuf};
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

fn example(dir: &Path) -> PathBuf {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/crear/ejemplos/transicion-propia");
    let copy = dir.join("own");
    std::fs::create_dir_all(copy.join("transiciones")).unwrap();
    for name in ["manifest.json", "scene.json", "transiciones/ojo.json", "transiciones/ojo.wgsl"] {
        std::fs::copy(source.join(name), copy.join(name)).unwrap();
    }
    copy
}

fn edit(file: &Path, from: &str, to: &str) {
    let text = std::fs::read_to_string(file).unwrap();
    assert!(text.contains(from), "{from} not in {}", file.display());
    std::fs::write(file, text.replacen(from, to, 1)).unwrap();
}

#[test]
fn the_check_of_a_package_says_why_its_transition_does_not_play() {
    let tmp = tempfile::tempdir().unwrap();
    let own = example(tmp.path());
    let fine = run(tmp.path(), &["pack", "check", "own"]);
    assert!(fine.status.success(), "{}", said(&fine));
    edit(&own.join("scene.json"), "\"borde\": 0.1", "\"radio\": 3");
    let named = run(tmp.path(), &["pack", "check", "own"]);
    let text = said(&named);
    assert!(!named.status.success(), "{text}");
    assert!(text.contains("the transition for leaving does not play") && text.contains("radio"), "{text}");
    assert!(!text.contains("for coming in"), "{text}");
    edit(&own.join("transiciones/ojo.json"), "\"taps\": 1", "\"taps\": 40");
    let costly = said(&run(tmp.path(), &["pack", "check", "own"]));
    assert!(
        costly.contains("for coming in") && costly.contains("more than the 8 a transition may cost"),
        "{costly}"
    );
}
