use crate::repos::{self, bundled};
use crate::store;
use std::fs;
use std::path::Path;

pub(super) fn the_examples_come_as_a_repository_that_stays_removed(source: &Path) {
    assert!(bundled::due(7).unwrap());
    let made = bundled::offer(source, 7, false).unwrap();
    let id = made["repo_id"].as_str().unwrap().to_owned();
    assert!(repos::added().contains(&id) && bundled::is_bundled(&id));
    let listed = repos::list().unwrap();
    let ours: Vec<_> = listed["repos"].as_array().unwrap().iter().filter(|r| r["bundled"] == true).collect();
    assert_eq!(
        (ours.len(), &ours[0]["name"], &ours[0]["broken"]),
        (1, &"Example wallpapers".into(), &serde_json::Value::Null)
    );
    assert!(!repos::browse("pulse").unwrap().as_array().unwrap().is_empty());
    assert!(!bundled::due(7).unwrap() && bundled::due(8).unwrap());
    repos::remove(&id).unwrap();
    assert!(!repos::added().contains(&id));
    assert!(!bundled::due(9).unwrap());
    assert!(bundled::offer(source, 9, false).unwrap().is_null());
    let again = bundled::offer(source, 9, true).unwrap();
    assert_eq!(again["repo_id"], id.as_str());
    assert!(repos::added().contains(&id));
    repos::remove(&id).unwrap();
}

pub(super) fn drawn_examples() -> std::path::PathBuf {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenes");
    let copy = store::dir().unwrap().join("examples");
    let scenes = fs::read_dir(&source).unwrap().flatten().map(|e| e.path());
    scenes
        .filter(|d| d.join("scene.json").is_file())
        .for_each(|d| tree(&d, &copy.join(d.file_name().unwrap())));
    copy
}

fn tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap().flatten() {
        let target = to.join(entry.file_name());
        match entry.path().is_dir() {
            true => tree(&entry.path(), &target),
            false => drop(fs::copy(entry.path(), target).unwrap()),
        }
    }
}
