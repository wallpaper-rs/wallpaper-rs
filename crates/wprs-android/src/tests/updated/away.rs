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
    };
    state::update(|s| s.away = vec![away.clone()]).unwrap();
    let listed: Value = serde_json::from_str(&account::request(r#"{ "op": "away" }"#)).unwrap();
    assert_eq!(listed[0]["title"], json!("Hand made"), "{listed}");
    let lists = ask(json!({ "op": "playlists" }));
    let named = lists.as_array().unwrap().iter().find(|p| p["playlist"]["uid"] == json!(list)).unwrap();
    assert_eq!(named["away"], json!(["Hand made"]), "{lists}");
    state::update(|s| s.away.clear()).unwrap();
    let lists = ask(json!({ "op": "playlists" }));
    assert!(lists.as_array().unwrap().iter().all(|p| p["away"] == json!([])), "{lists}");
    assert_eq!(account::request(r#"{ "op": "away" }"#), "[]");
    ask(json!({ "op": "delete", "playlist": list }));
}
