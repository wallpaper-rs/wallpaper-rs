use crate::tests::{FIXTURES, ask};
use crate::{library, repos, store, variant};
use serde_json::{Value, json};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use wprs_format::{Limits, Settings, Signer, build, init, keygen, sign_root};

pub(super) fn serve(root: PathBuf) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let root = root.clone();
            std::thread::spawn(move || answer(stream, &root));
        }
    });
    port
}

fn answer(mut stream: TcpStream, root: &Path) {
    let mut head = [0u8; 2048];
    let read = stream.read(&mut head).unwrap_or(0);
    let text = String::from_utf8_lossy(&head[..read]);
    let target = text.split_whitespace().nth(1).unwrap_or("/").split('?').next().unwrap_or("/").to_owned();
    let wanted = target.trim_start_matches('/');
    let file = root.join(wanted);
    let safe = !wanted.split('/').any(|part| part == "..");
    match std::fs::read(&file).ok().filter(|_| safe && file.is_file()) {
        Some(bytes) => {
            let head =
                format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", bytes.len());
            drop(stream.write_all(head.as_bytes()).and_then(|()| stream.write_all(&bytes)));
        }
        None => drop(
            stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"),
        ),
    }
}

fn wpak(dir: &Path, id: &str, version: &str, kind: &str, needs: &[(&str, &str)]) -> PathBuf {
    let src = dir.join(format!("src-{id}-{version}"));
    fs::create_dir_all(&src).unwrap();
    let packages: Vec<Value> =
        needs.iter().map(|(id, range)| json!({ "id": id, "version": range })).collect();
    let (entry, bytes) = if kind == "preset" {
        let base = json!({ "id": needs[0].0, "version": needs[0].1 });
        ("preset.json", json!({ "base": base, "values": { "fit": "fill" } }).to_string().into_bytes())
    } else {
        ("image.png", fs::read(Path::new(FIXTURES).join("green-64x32.png")).unwrap())
    };
    fs::write(src.join(entry), bytes).unwrap();
    let manifest = json!({ "schema": wprs_format::SCHEMA, "id": id, "version": version, "title": id,
        "license": "CC0-1.0", "kind": kind, "entry": entry, "content": { "rating": "general" },
        "photosensitive": false, "origin": { "kind": "original" }, "requires": { "packages": packages } });
    fs::write(src.join("manifest.json"), manifest.to_string()).unwrap();
    let out = dir.join(format!("{id}-{version}.wpak"));
    build(&src, &out, &Limits::default()).unwrap();
    out
}

pub(super) fn operate(repo: &Path, op: Value) -> Value {
    let now = crate::library::now();
    match op["op"].as_str() {
        Some("add") => {
            let files: Vec<PathBuf> = serde_json::from_value(op["files"].clone()).unwrap();
            wprs_format::stall::added(repo, &files, false, now).unwrap()
        }
        _ => {
            let key = zeroize::Zeroizing::new(op["key"].as_str().unwrap().to_owned());
            let signer = Signer::new(&key, op["password"].as_str().map(str::to_owned)).unwrap();
            wprs_format::stall::published(repo, &signer, op["days"].as_u64().unwrap() as u32, now).unwrap()
        }
    }
}

pub(super) fn account(request: Value) -> Value {
    serde_json::from_str(&crate::account::request(&request.to_string())).unwrap()
}

pub(super) fn uid_of(id: &str) -> Option<String> {
    library::with(|lib| Ok(lib.packages_of(id)?.into_iter().next().map(|i| i.uid))).unwrap()
}

pub(super) fn dependencies_and_updates_follow_a_repository() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path().join("repo");
    let (root, online) = (keygen(None).unwrap(), keygen(Some("example".into())).unwrap());
    let settings = Settings {
        name: "Example".into(),
        root_keys: vec![root.public.clone()],
        online_keys: vec![online.public.clone()],
        ..Settings::default()
    };
    init(&repo, &settings, 1_800_000_000).unwrap();
    sign_root(&repo, &Signer::new(&root.secret, None).unwrap()).unwrap();
    let publish = || {
        let key = json!({ "op": "publish", "key": online.secret, "password": "example", "days": 7 });
        operate(&repo, key)
    };
    let (base, dep, orphan, calm) = (
        wpak(tmp.path(), "org.example.base", "1.0.0", "image", &[]),
        wpak(tmp.path(), "org.example.dep", "1.0.0", "image", &[("org.example.base", "^1")]),
        wpak(tmp.path(), "org.example.orphan", "1.0.0", "image", &[("org.example.ghost", "^1")]),
        wpak(tmp.path(), "org.example.calm", "1.0.0", "preset", &[("org.example.base", "^1")]),
    );
    operate(&repo, json!({ "op": "add", "files": [base, dep, orphan, calm] }));
    publish();
    let port = serve(repo.clone());
    let url = format!("http://127.0.0.1:{port}/?fingerprint={}", root.fingerprint);
    let added = repos::add(&url).unwrap();
    let repo_id = added["repo_id"].as_str().unwrap().to_owned();
    assert_eq!(added["packages"], json!(4), "{added}");

    let installed = repos::install(&repo_id, "org.example.dep").unwrap();
    assert_eq!(installed["failed"], json!([]), "{installed}");
    assert!(uid_of("org.example.base").is_some(), "the base a package needs came with it");
    let stranded = repos::install(&repo_id, "org.example.orphan").unwrap();
    assert_eq!(stranded["failed"], json!(["org.example.ghost"]), "a missing dependency is told");

    let preset = repos::install(&repo_id, "org.example.calm").unwrap()["item"].as_str().unwrap().to_owned();
    let base_uid = uid_of("org.example.base").unwrap();
    assert_eq!(ask(json!({ "op": "remove", "uids": [base_uid] })), Value::Null);
    assert!(uid_of("org.example.base").is_none());
    let bring = account(json!({ "op": "install_base", "uid": preset }));
    assert_eq!(bring["failed"], json!([]), "{bring}");
    let base_uid = uid_of("org.example.base").unwrap();
    assert_eq!(bring["item"], json!(base_uid), "{bring}");

    operate(
        &repo,
        json!({ "op": "add", "files": [wpak(tmp.path(), "org.example.base", "1.1.0", "image", &[])] }),
    );
    publish();
    let found = account(json!({ "op": "refresh_repos", "force": true }));
    let wanted = json!([{ "uid": base_uid, "repo_id": repo_id, "id": "org.example.base", "title": "org.example.base",
        "installed": "1.0.0", "available": "1.1.0" }]);
    assert_eq!(found, wanted, "{found}");
    let before = library::with(|lib| Ok(lib.item(&base_uid)?.path)).unwrap();
    let updated = account(json!({ "op": "update", "uid": base_uid }));
    assert_eq!(updated, json!({ "item": base_uid, "failed": [] }), "{updated}");
    let item = library::with(|lib| Ok(lib.item(&base_uid)?)).unwrap();
    assert_eq!(item.package.map(|p| p.version), Some("1.1.0".to_owned()));
    assert_ne!(item.path, before, "the item points at the new file");
    assert!(!before.exists(), "the old file went away");
    assert_eq!(account(json!({ "op": "updates" })), json!([]));
    assert_eq!(
        library::with(|lib| variant::preset(lib, &preset)).unwrap(),
        Some(base_uid.clone()),
        "a preset still finds its base after the update"
    );

    let all: Vec<String> = ["org.example.base", "org.example.dep", "org.example.orphan", "org.example.calm"]
        .iter()
        .filter_map(|id| uid_of(id))
        .collect();
    assert_eq!(ask(json!({ "op": "remove", "uids": all })), Value::Null);
    repos::remove(&repo_id).unwrap();
    store::changed();
}
