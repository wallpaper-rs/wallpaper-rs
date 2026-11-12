use super::repository::{account, operate, serve, uid_of};
use crate::tests::FIXTURES;
use crate::{library, repos, settings, store};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use wprs_format::{Limits, Settings, Signer, build, init, keygen, sign_root};

const ID: &str = "org.example.tide";

fn wpak(dir: &Path, version: &str, inputs: &[&str]) -> PathBuf {
    let src = dir.join(format!("src-{version}"));
    fs::create_dir_all(&src).unwrap();
    fs::copy(Path::new(FIXTURES).join("green-64x32.png"), src.join("image.png")).unwrap();
    let manifest = json!({ "schema": wprs_format::SCHEMA, "id": ID, "version": version, "title": ID,
        "license": "CC0-1.0", "kind": "image", "entry": "image.png", "content": { "rating": "general" },
        "photosensitive": false, "origin": { "kind": "original" }, "inputs": inputs });
    fs::write(src.join("manifest.json"), manifest.to_string()).unwrap();
    let out = dir.join(format!("{version}.wpak"));
    build(&src, &out, &Limits::default()).unwrap();
    out
}

fn installed_version() -> Option<String> {
    let uid = uid_of(ID)?;
    library::with(|lib| Ok(lib.item(&uid)?.package.map(|p| p.version))).unwrap()
}

pub(super) fn updates_follow_the_setting() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path().join("repo");
    let (root, online) = (keygen(None).unwrap(), keygen(Some("example".into())).unwrap());
    let settings_of = Settings {
        name: "Example".into(),
        root_keys: vec![root.public.clone()],
        online_keys: vec![online.public.clone()],
        ..Settings::default()
    };
    init(&repo, &settings_of, 1_800_000_000).unwrap();
    sign_root(&repo, &Signer::new(&root.secret, None).unwrap()).unwrap();
    let release = |version: &str, inputs: &[&str]| {
        operate(&repo, json!({ "op": "add", "files": [wpak(tmp.path(), version, inputs)] }));
        let key = json!({ "op": "publish", "key": online.secret, "password": "example", "days": 7 });
        operate(&repo, key);
        account(json!({ "op": "refresh_repos", "force": true }))
    };
    release("1.0.0", &[]);
    let port = serve(repo.clone());
    let url = format!("http://127.0.0.1:{port}/?fingerprint={}", root.fingerprint);
    let repo_id = repos::add(&url).unwrap()["repo_id"].as_str().unwrap().to_owned();
    repos::install(&repo_id, ID).unwrap();
    assert_eq!(settings::setting("repos.auto_update", None).unwrap(), "false", "off by default");

    let listed = release("1.1.0", &[]);
    assert_eq!(listed[0]["available"], json!("1.1.0"), "{listed}");
    assert_eq!(installed_version().as_deref(), Some("1.0.0"), "nothing is updated while the setting is off");

    settings::setting("repos.auto_update", Some("true")).unwrap();
    let listed = account(json!({ "op": "refresh_repos", "force": true }));
    assert_eq!(listed, json!([]), "{listed}");
    assert_eq!(installed_version().as_deref(), Some("1.1.0"), "the setting updates the package");

    let held = release("1.2.0", &["audio"]);
    assert_eq!(held[0]["asks"], json!(["audio"]), "{held}");
    assert_eq!(installed_version().as_deref(), Some("1.1.0"), "a new permission is not accepted silently");
    let again: Value = account(json!({ "op": "refresh_repos", "force": true }));
    assert_eq!(again[0]["asks"], json!(["audio"]), "{again}");
    assert_eq!(library::with(|lib| Ok(lib.packages_of(ID)?.len())).unwrap(), 1, "the held file was dropped");

    let uid = uid_of(ID).unwrap();
    let by_hand = account(json!({ "op": "update", "uid": uid }));
    assert_eq!(by_hand["item"], json!(uid), "{by_hand}");
    assert_eq!(installed_version().as_deref(), Some("1.2.0"), "an update asked for by hand installs it");
    assert_eq!(account(json!({ "op": "updates" })), json!([]));

    settings::setting("repos.auto_update", Some("false")).unwrap();
    assert_eq!(crate::tests::ask(json!({ "op": "remove", "uids": [uid] })), Value::Null);
    repos::remove(&repo_id).unwrap();
    store::changed();
}
