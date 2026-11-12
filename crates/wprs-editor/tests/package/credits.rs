use crate::{act, ask, file, project, warned};
use serde_json::json;

#[test]
fn credits_list_the_assets_and_credit_the_chosen_ones() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root, "credits");
    file(&p, "assets/tree.png", b"png");
    file(&p, "assets/song.ogg", b"ogg");
    file(&p, "assets/sky/a.webp", b"webp");
    file(&p, "assets/sky/b.webp", b"webp");
    let found = ask(&p, json!({ "credits": {} }));
    assert_eq!(found["missing"].as_array().unwrap().len(), 4, "{found}");
    assert!(warned(&p, "/manifest/credits").is_empty(), "an original package warns of nothing");
    let credit = json!({ "manifest": { "credit": { "paths": ["assets/tree.png"], "license": "CC-BY-4.0",
        "attribution": "Ana", "url": "https://example.org/tree" } } });
    act(&mut p, credit).unwrap();
    let more = json!({ "manifest": { "credit": { "paths": ["assets/song.ogg", "assets/sky/"],
        "license": "CC-BY-4.0", "attribution": "Ana", "url": "https://example.org/tree" } } });
    act(&mut p, more).unwrap();
    let list = p.doc["manifest"]["credits"].as_array().unwrap();
    assert_eq!(list.len(), 1, "{list:?}");
    assert_eq!(list[0]["paths"], json!(["assets/tree.png", "assets/song.ogg", "assets/sky/"]));
    let found = ask(&p, json!({ "credits": {} }));
    assert_eq!(found["missing"], json!([]), "{found}");
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(p.doc["manifest"]["credits"][0]["paths"], json!(["assets/tree.png"]));
}

#[test]
fn a_package_that_credits_others_is_warned_of_files_without_credit() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root, "warn");
    file(&p, "assets/tree.png", b"png");
    file(&p, "assets/rock.png", b"png");
    let one = json!({ "manifest": { "credit": { "paths": ["assets/tree.png"], "license": "CC0-1.0",
        "attribution": "Ana" } } });
    act(&mut p, one).unwrap();
    let warnings = warned(&p, "/manifest/credits");
    assert_eq!(warnings.len(), 1, "{warnings:?}");
    assert!(warnings[0].contains("assets/rock.png"), "{warnings:?}");
    let two = json!({ "manifest": { "credit": { "paths": ["assets/rock.png"], "license": "CC0-1.0",
        "attribution": "Ana" } } });
    act(&mut p, two).unwrap();
    assert!(warned(&p, "/manifest/credits").is_empty());
    let gone = json!({ "manifest": { "uncredit": { "paths": ["assets/tree.png", "assets/rock.png"] } } });
    act(&mut p, gone).unwrap();
    assert!(p.doc["manifest"].get("credits").is_none(), "{}", p.doc["manifest"]);
}

#[test]
fn bad_credits_are_refused_and_change_nothing() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root, "bad");
    file(&p, "assets/tree.png", b"png");
    let before = p.doc.clone();
    for (paths, license, who, url) in [
        (json!([]), "CC0-1.0", "Ana", json!(null)),
        (json!(["assets/none.png"]), "CC0-1.0", "Ana", json!(null)),
        (json!(["../tree.png"]), "CC0-1.0", "Ana", json!(null)),
        (json!(["assets/tree.png"]), "", "Ana", json!(null)),
        (json!(["assets/tree.png"]), "CC0-1.0", " ", json!(null)),
        (json!(["assets/tree.png"]), "CC0-1.0", "Ana", json!("http://example.org")),
    ] {
        let bad = json!({ "manifest": { "credit": { "paths": paths, "license": license,
            "attribution": who, "url": url } } });
        assert!(act(&mut p, bad.clone()).is_err(), "{bad}");
    }
    assert_eq!(p.doc, before);
}
