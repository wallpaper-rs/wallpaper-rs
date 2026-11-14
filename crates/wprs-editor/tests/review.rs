use serde_json::{Value, json};
use std::fs;
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::review::{report, signature};
use wprs_editor::{Project, author_keys};

fn project(root: &TempDir) -> Project {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
    let made = wprs_editor::starters::starter("empty", "org.example.tune", "Tune", "en").unwrap();
    Project::create(&root.path().join("tune"), made).unwrap()
}

#[test]
fn the_report_lists_each_finding_with_its_level_file_and_pointer() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let said: Value = report(&p.dir, &p.doc);
    let findings = said["findings"].as_array().unwrap();
    assert!(findings.iter().all(|f| ["error", "warning", "note"].contains(&f["level"].as_str().unwrap())));
    assert!(
        findings
            .iter()
            .all(|f| f["pointer"].is_string() && f["file"].is_string() && f["message"].is_string())
    );
    assert!(said["platforms"]["desktop"]["refused"].is_null());
    p.act(&serde_json::from_value(json!({ "set": { "pointer": "/scene/assets/ghost", "value": {"path": "ghost.png", "kind": "image"} } })).unwrap())
        .unwrap();
    let said = report(&p.dir, &p.doc);
    let missing = said["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["pointer"] == "/scene/assets/ghost/path")
        .unwrap();
    assert_eq!((missing["level"].as_str(), missing["file"].as_str()), (Some("error"), Some("ghost.png")));
}

#[test]
fn a_folder_or_package_signature_is_checked_against_the_given_keys() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    assert_eq!(signature(&p.dir, &[])["state"], "unsigned");
    let keys = root.path().join("keys");
    let (ana, bo) = (
        author_keys::create(&keys, "ana", None, false).unwrap(),
        author_keys::create(&keys, "bo", None, false).unwrap(),
    );
    let signer = wprs_editor::package::signer(ana["secret"].as_str().unwrap().as_ref(), None).unwrap();
    let out = root.path().join("tune.wpak");
    wprs_editor::package::build(&mut p, &out, Some(&signer)).unwrap();
    let (ana_pub, bo_pub) =
        (ana["public"].as_str().unwrap().to_owned(), bo["public"].as_str().unwrap().to_owned());
    assert_eq!(signature(&p.dir, &[])["state"], "unchecked");
    for path in [p.dir.as_path(), out.as_path()] {
        let valid = signature(path, std::slice::from_ref(&ana_pub));
        assert_eq!(valid["state"], "valid", "{valid}");
        assert_eq!(valid["signers"][0], ana["fingerprint"]);
        assert_eq!(signature(path, std::slice::from_ref(&bo_pub))["state"], "invalid");
    }
    wprs_editor::package::build(&mut p, &root.path().join("plain.wpak"), None).unwrap();
    assert_eq!(signature(&root.path().join("plain.wpak"), &[ana_pub])["state"], "unsigned");
}

#[test]
fn key_pairs_are_made_listed_replaced_only_on_request_and_rotated() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path().join("keys");
    let first = author_keys::create(&dir, "studio", None, false).unwrap();
    let mode = |name: &str| {
        std::os::unix::fs::PermissionsExt::mode(&fs::metadata(dir.join(name)).unwrap().permissions()) & 0o777
    };
    assert_eq!((mode("studio.key"), mode("studio.pub")), (0o600, 0o644));
    assert_eq!(mode(""), 0o700);
    assert!(author_keys::create(&dir, "studio", None, false).is_err(), "a key was replaced unasked");
    for bad in ["", "../x", ".hidden", "a b"] {
        assert!(author_keys::create(&dir, bad, None, false).is_err(), "{bad:?}");
    }
    assert!(author_keys::create(&dir, "locked", Some(String::new()), false).is_err());
    let locked = author_keys::create(&dir, "locked", Some("pw".into()), false).unwrap();
    let again = author_keys::create(&dir, "studio", None, true).unwrap();
    assert_eq!(again["replaced"], true);
    assert_ne!(first["fingerprint"], again["fingerprint"]);
    let listed = author_keys::list(&dir);
    assert_eq!(listed.as_array().unwrap().len(), 2);
    let found = |name: &str| listed.as_array().unwrap().iter().find(|k| k["name"] == name).unwrap().clone();
    assert_eq!(
        (found("locked")["encrypted"].clone(), found("studio")["encrypted"].clone()),
        (json!(true), json!(false))
    );
    assert_eq!(found("studio")["fingerprint"], again["fingerprint"]);
    assert!(!listed.to_string().contains("RW"), "a listing carried key material");
    let secret = dir.join("studio.key");
    let signed = author_keys::rotate(&secret, None, locked["public"].as_str().unwrap(), "studio").unwrap();
    assert_eq!(
        (signed["from"].clone(), signed["to"].clone()),
        (again["fingerprint"].clone(), locked["fingerprint"].clone())
    );
    assert!(signed["signature"].as_str().unwrap().contains("untrusted comment"));
    assert!(
        author_keys::rotate(&dir.join("locked.key"), None, locked["public"].as_str().unwrap(), "studio")
            .is_err()
    );
    assert!(author_keys::rotate(&secret, None, "nonsense", "studio").is_err());
    assert!(author_keys::rotate(&secret, None, locked["public"].as_str().unwrap(), " ").is_err());
}

#[test]
fn scripts_that_do_not_compile_or_are_missing_are_problems_everywhere() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    fs::create_dir_all(p.dir.join("scripts")).unwrap();
    fs::write(p.dir.join("scripts/ok.js"), "export function update(frame) {\n  return 1;\n}\n").unwrap();
    fs::write(
        p.dir.join("scripts/bad.js"),
        "export function update(frame) {\n  return 1;\n\nlet = = 3;\n}\n",
    )
    .unwrap();
    let listed = ["scripts/ok.js", "scripts/bad.js", "scripts/gone.js"];
    p.doc["scene"]["scripts"] = json!(listed.map(|path| json!({ "path": path, "events": ["update"] })));
    let shown = |problems: Vec<wprs_editor::Problem>| {
        problems
            .into_iter()
            .filter(|f| f.pointer.starts_with("/scene/scripts/"))
            .map(|f| (f.pointer, f.warning, f.said.local()))
            .collect::<Vec<_>>()
    };
    let list = shown(wprs_editor::check::review(&p.doc, Some(&p.dir)));
    assert_eq!(list.len(), 2, "{list:?}");
    assert!(
        list[0].0 == "/scene/scripts/1/path" && !list[0].1 && list[0].2.starts_with("scripts/bad.js:4:"),
        "{list:?}"
    );
    assert!(
        list[1].0 == "/scene/scripts/2/path" && !list[1].1 && list[1].2.contains("scripts/gone.js"),
        "{list:?}"
    );
    assert_eq!(shown(wprs_editor::package::problems(&p.dir, &p.doc)), list);
    let said = report(&p.dir, &p.doc);
    let findings = said["findings"].as_array().unwrap();
    let bad = findings.iter().find(|f| f["pointer"] == "/scene/scripts/1/path").unwrap();
    assert_eq!((bad["level"].as_str(), bad["file"].as_str()), (Some("error"), Some("scripts/bad.js")));
}

#[test]
fn a_repository_key_is_not_offered_as_an_author_signature() {
    let root = tempfile::tempdir().unwrap();
    let dir = root.path().join("keys");
    author_keys::create(&dir, "ana", None, false).unwrap();
    let shelf = author_keys::create(&author_keys::repositories(&dir), "shelf-repo", None, false).unwrap();
    assert!(shelf["secret"].as_str().unwrap().starts_with(author_keys::repositories(&dir).to_str().unwrap()));
    let names: Vec<_> =
        author_keys::list(&dir).as_array().unwrap().iter().map(|k| k["name"].clone()).collect();
    assert_eq!(names, ["ana"]);
}

#[test]
fn a_thumbnail_older_than_the_last_change_is_outdated() {
    let dir = tempfile::tempdir().unwrap();
    let at = |secs| std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(secs);
    let touch = |name: &str, secs| {
        std::fs::write(dir.path().join(name), b"x").unwrap();
        std::fs::File::options()
            .write(true)
            .open(dir.path().join(name))
            .unwrap()
            .set_modified(at(secs))
            .unwrap();
    };
    assert!(wprs_editor::previews::outdated(dir.path(), ""));
    assert!(wprs_editor::previews::outdated(dir.path(), "preview.jpg"));
    touch("scene.json", 1000);
    touch("preview.jpg", 1001);
    touch("manifest.json.minisig", 5000);
    assert!(!wprs_editor::previews::outdated(dir.path(), "preview.jpg"));
    touch("scene.json", 2000);
    assert!(wprs_editor::previews::outdated(dir.path(), "preview.jpg"));
}
