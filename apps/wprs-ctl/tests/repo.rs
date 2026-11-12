use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn ctl(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_wprs-ctl"))
        .env("LANG", "C")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap()
}

fn ok(dir: &Path, args: &[&str]) -> String {
    let out = ctl(dir, args);
    assert!(out.status.success(), "{args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).unwrap()
}

fn files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files(&path, out);
        } else {
            out.push(path);
        }
    }
}

fn site(work: &Path) {
    let src = work.join("src");
    fs::create_dir(&src).unwrap();
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/green-64x32.png");
    fs::copy(&fixtures, src.join("image.png")).unwrap();
    fs::write(
        src.join("manifest.json"),
        r#"{"schema": "wallpaper-rs.wpak/1", "id": "org.example.dawn", "version": "1.0.0", "title": "Dawn",
        "license": "CC0-1.0", "kind": "image", "entry": "image.png"}"#,
    )
    .unwrap();
    ok(work, &["repo", "keygen", "root", "--no-password"]);
    ok(work, &["repo", "keygen", "online", "--no-password"]);
    ok(work, &["pack", "build", "src", "-o", "dawn.wpak"]);
    ok(
        work,
        &[
            "repo",
            "init",
            "repo",
            "--name",
            "Example",
            "--root-key",
            "root.pub",
            "--online-key",
            "online.pub",
        ],
    );
    ok(work, &["repo", "sign-root", "repo", "--key", "root.key"]);
    ok(work, &["repo", "add", "repo", "dawn.wpak"]);
    ok(work, &["repo", "publish", "repo", "--key", "online.key"]);
}

#[test]
fn verify_passes_a_good_repository_and_fails_a_tampered_one() {
    let tmp = tempfile::tempdir().unwrap();
    site(tmp.path());
    let good = ctl(tmp.path(), &["repo", "verify", "repo"]);
    let said = String::from_utf8_lossy(&good.stderr).into_owned();
    assert!(good.status.success(), "{said}");
    assert!(!said.contains("panicked"), "{said}");
    let text = String::from_utf8_lossy(&good.stdout).into_owned();
    assert!(text.contains("packages = 1"), "{text}");

    let mut all = Vec::new();
    files(&tmp.path().join("repo"), &mut all);
    let index = all.iter().find(|p| p.ends_with("entry/latest")).unwrap_or_else(|| panic!("{all:?}"));
    let mut bytes = fs::read(index).unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 0xff;
    fs::write(index, bytes).unwrap();
    let bad = ctl(tmp.path(), &["repo", "verify", "repo"]);
    let said = String::from_utf8_lossy(&bad.stderr).into_owned();
    assert_eq!(bad.status.code(), Some(1), "{said}");
    assert!(!said.contains("panicked") && said.starts_with("Error"), "{said}");
}
