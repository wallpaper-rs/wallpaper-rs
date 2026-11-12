use crate::store;
use crate::tests::{ask, open};
use serde_json::json;

pub(super) fn ratings_tags_and_titles_are_set_from_the_phone() {
    let (uid, _) = store::store(&mut open("bands-640x180.png"), "Bands.png").unwrap();
    assert_eq!(ask(json!({ "op": "rate", "uids": [uid], "rating": 4 })), json!(null));
    assert!(ask(json!({ "op": "rate", "uids": [uid], "rating": 9 }))["error"].is_string());
    let tagged = ask(json!({ "op": "retag", "uids": [uid], "add": ["calm", "blue"], "remove": [] }));
    assert_eq!(tagged, json!(null));
    assert_eq!(ask(json!({ "op": "rename", "uid": uid, "title": "  Evening bands " })), json!(null));
    let item = ask(json!({ "op": "item", "uid": uid }));
    assert_eq!((&item["rating"], &item["title"]), (&json!(4), &json!("Evening bands")));
    assert_eq!(item["tags"], json!(["blue", "calm"]));
    let found = ask(json!({ "op": "items", "query": "tag:calm" }));
    assert_eq!(found.as_array().map(Vec::len), Some(1));
    ask(json!({ "op": "retag", "uids": [uid], "add": [], "remove": ["calm"] }));
    let tags = ask(json!({ "op": "tags" }));
    assert!(tags.as_array().unwrap().iter().all(|t| t["name"] != "calm"), "{tags}");
    let long = "x".repeat(200);
    assert!(ask(json!({ "op": "retag", "uids": [uid], "add": [long], "remove": [] }))["error"].is_string());
    ask(json!({ "op": "remove", "uids": [uid] }));
}
