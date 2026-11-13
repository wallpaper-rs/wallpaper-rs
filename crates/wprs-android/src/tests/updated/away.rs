use crate::tests::ask;
use crate::{account, state};
use serde_json::{Value, json};
use wprs_core::model::Away;

pub(super) fn notes_follow_the_records_that_have_no_item_here() {
    let list = ask(json!({ "op": "create", "name": "Only here" })).as_str().unwrap().to_owned();
    let away = Away {
        key: "9f".repeat(32),
        title: "Hand made".into(),
        playlists: vec![list.clone()],
        profiles: vec![],
        repo: None,
    };
    state::update(|s| s.away = vec![away.clone()]).unwrap();
    let listed: Value = serde_json::from_str(&account::request(r#"{ "op": "away" }"#)).unwrap();
    assert_eq!(listed[0]["title"], json!("Hand made"), "{listed}");
    let lists = ask(json!({ "op": "playlists" }));
    let named = lists.as_array().unwrap().iter().find(|p| p["playlist"]["uid"] == json!(list)).unwrap();
    assert_eq!(named["away"][0]["title"], json!("Hand made"), "{lists}");
    state::update(|s| s.away.clear()).unwrap();
    let lists = ask(json!({ "op": "playlists" }));
    assert!(lists.as_array().unwrap().iter().all(|p| p["away"] == json!([])), "{lists}");
    assert_eq!(account::request(r#"{ "op": "away" }"#), "[]");
    ask(json!({ "op": "delete", "playlist": list }));
}

pub(super) fn an_installed_item_leaves_the_notes_at_once() {
    let ids = wprs_sync::SyncKey::generate().ids();
    crate::away::remember(&ids);
    let (uid, _) = crate::store::store(&mut crate::tests::open("bands-640x180.png"), "Bands.png").unwrap();
    let hash = crate::library::with(|lib| Ok(lib.hashed()?)).unwrap();
    let hash = &hash.iter().find(|(item, _)| item.uid == uid).unwrap().1;
    let note = |key: String, title: &str| Away {
        key,
        title: title.into(),
        playlists: vec![],
        profiles: vec![],
        repo: None,
    };
    let kept = note(ids.blob(&[7; 32]), "Elsewhere");
    state::update(|s| s.away = vec![note(ids.blob(hash), "Bands"), kept.clone()]).unwrap();
    let listed: Value = serde_json::from_str(&account::request(r#"{ "op": "away" }"#)).unwrap();
    assert_eq!(listed.as_array().map(Vec::len), Some(1), "{listed}");
    assert_eq!(listed[0]["title"], json!("Elsewhere"));
    assert_eq!(state::read(|s| s.away.len()), 1);
    assert!(ask(json!({ "op": "playlists" })).is_array());
    state::update(|s| s.away.clear()).unwrap();
    ask(json!({ "op": "remove", "uids": [uid] }));
}
