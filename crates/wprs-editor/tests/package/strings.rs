use crate::{act, ask, file, project};
use serde_json::json;

fn text(key: &str, language: &str, text: Option<&str>) -> serde_json::Value {
    json!({ "manifest": { "text": { "key": key, "language": language, "text": text } } })
}

#[test]
fn the_table_of_texts_lists_the_keys_the_languages_lack_and_the_engine_texts() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root, "table");
    act(&mut p, json!({ "upgrade": {} })).ok();
    act(&mut p, json!({ "translate": { "pointer": "/manifest/title", "language": "es", "add": true } }))
        .unwrap();
    act(&mut p, json!({ "set": { "pointer": "/manifest/description", "value": "@about" } })).unwrap();
    act(&mut p, text("about", "en", Some("A calm scene"))).unwrap();
    act(&mut p, text("hello", "en", Some("Hello"))).unwrap();
    act(&mut p, text("hello", "es", Some("Hola"))).unwrap();
    let found = ask(&p, json!({ "strings": {} }));
    let row = |key: &str| found["keys"].as_array().unwrap().iter().find(|k| k["key"] == key).unwrap().clone();
    assert_eq!(row("hello")["missing"], json!([]), "{found}");
    assert_eq!(row("about")["missing"], json!(["es"]), "{found}");
    assert_eq!(row("about")["used"], json!(true));
    assert_eq!(row("hello")["used"], json!(false));
    assert!(
        found["engine"]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| e["key"] == "wprs.speed" && e["es"] == "Velocidad")
    );
    act(&mut p, text("hello", "es", None)).unwrap();
    assert_eq!(row_missing(&p, "hello"), json!(["es"]));
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(row_missing(&p, "hello"), json!([]));
}

fn row_missing(p: &wprs_editor::Project, key: &str) -> serde_json::Value {
    let found = ask(p, json!({ "strings": {} }));
    found["keys"].as_array().unwrap().iter().find(|k| k["key"] == key).unwrap()["missing"].clone()
}

#[test]
fn a_language_moves_to_its_locale_file_and_back() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root, "files");
    act(&mut p, json!({ "translate": { "pointer": "/manifest/title", "language": "es", "add": true } }))
        .unwrap();
    act(&mut p, text("hello", "es", Some("Hola"))).unwrap();
    act(&mut p, text("hello", "en", Some("Hello"))).unwrap();
    act(&mut p, json!({ "manifest": { "locale": { "language": "es", "file": true } } })).unwrap();
    let written = std::fs::read_to_string(p.dir.join("locale/es.json")).unwrap();
    assert!(written.contains("\"hello\": \"Hola\""), "{written}");
    assert!(p.doc["manifest"]["strings"].get("es").is_none(), "{}", p.doc["manifest"]);
    assert!(
        p.doc["manifest"]["requires"]["features"].as_array().unwrap().contains(&json!("locale_files")),
        "{}",
        p.doc["manifest"]
    );
    let found = ask(&p, json!({ "strings": {} }));
    assert_eq!(
        (found["places"]["es"].clone(), found["places"]["en"].clone()),
        (json!("file"), json!("manifest"))
    );
    act(&mut p, text("bye", "es", Some("Adiós"))).unwrap();
    assert!(std::fs::read_to_string(p.dir.join("locale/es.json")).unwrap().contains("Adiós"));
    act(&mut p, json!({ "manifest": { "locale": { "language": "es", "file": false } } })).unwrap();
    assert!(!p.dir.join("locale/es.json").exists());
    assert_eq!(p.doc["manifest"]["strings"]["es"]["bye"], json!("Adiós"));
    let features = p.doc["manifest"]["requires"]["features"].as_array().cloned().unwrap_or_default();
    assert!(!features.contains(&json!("locale_files")), "{features:?}");
}

#[test]
fn bad_texts_by_key_are_refused() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root, "bad");
    file(&p, "locale/xx.json", b"{}");
    for bad in [
        text("wprs.speed", "en", Some("x")),
        text("has space", "en", Some("x")),
        text("ok", "not a language", Some("x")),
    ] {
        assert!(act(&mut p, bad.clone()).is_err(), "{bad}");
    }
}
