use std::fs;
use wprs_editor::starters::starter;
use wprs_editor::{Project, applying};

#[test]
fn a_rebuilt_package_never_keeps_the_stamp_of_the_old_one() {
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", root.path()) };
    let folder = root.path().join("scene");
    let project =
        Project::create(&folder, starter("empty", "org.example.same", "Same", "en").unwrap()).unwrap();
    let out = root.path().join("a.wpak");
    applying::build(&project.dir, &project.doc, &out).unwrap();
    let first = fs::metadata(&out).unwrap();
    applying::build(&project.dir, &project.doc, &out).unwrap();
    let second = fs::metadata(&out).unwrap();
    assert_eq!(first.len(), second.len());
    let secs =
        |m: &fs::Metadata| m.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    assert_ne!(secs(&first), secs(&second), "the daemon compares whole seconds and sizes");
}
