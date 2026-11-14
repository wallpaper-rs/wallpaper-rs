use std::fs;
use tempfile::TempDir;
use wprs_editor::review::{report_in, signature};
use wprs_editor::{Project, author_keys, package};
use wprs_format::SIGNATURE;

fn project(root: &TempDir) -> Project {
    let made = wprs_editor::starters::starter("empty", "org.example.tune", "Tune", "en").unwrap();
    Project::create(&root.path().join("tune"), made).unwrap()
}

fn names(dir: &std::path::Path) -> Vec<String> {
    let mut all: Vec<String> =
        fs::read_dir(dir).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
    all.sort();
    all
}

#[test]
fn a_signed_publish_signs_the_package_and_leaves_the_folder_as_it_was() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    p.settle().unwrap();
    let before = names(&p.dir);
    let ana = author_keys::create(&root.path().join("keys"), "ana", None, false).unwrap();
    let signer = package::signer(ana["secret"].as_str().unwrap().as_ref(), None).unwrap();
    let out = root.path().join("tune.wpak");
    package::publish(&p.dir, &p.doc, &out, Some(&signer)).unwrap();
    assert_eq!(names(&p.dir), before);
    let public = ana["public"].as_str().unwrap().to_owned();
    assert_eq!(signature(&out, &[public])["state"], "valid");
    let plain = |t: &str| t.to_owned();
    let said = report_in(&p.dir, &p.doc, &plain);
    let findings = said["findings"].as_array().unwrap();
    assert!(!findings.iter().any(|f| f["message"].as_str().unwrap().contains(SIGNATURE)), "{said}");
}

#[test]
fn the_findings_of_the_review_read_in_the_language_asked() {
    let root = tempfile::tempdir().unwrap();
    let p = project(&root);
    fs::write(p.dir.join(SIGNATURE), "stale").unwrap();
    fs::create_dir_all(p.dir.join("assets")).unwrap();
    fs::write(p.dir.join("assets").join("spare.png"), b"x").unwrap();
    let es = wprs_core::message::tr_in(&["es".into()]);
    let said = report_in(&p.dir, &p.doc, &es);
    let messages: Vec<&str> =
        said["findings"].as_array().unwrap().iter().filter_map(|f| f["message"].as_str()).collect();
    assert!(messages.iter().any(|m| m.contains("firma una versión anterior")), "{messages:?}");
    assert!(messages.iter().any(|m| m.contains("la escena no usa")), "{messages:?}");
}
