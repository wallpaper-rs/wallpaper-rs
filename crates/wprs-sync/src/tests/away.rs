use super::doc;
use crate::*;
use serde_json::json;
use std::collections::{BTreeMap, HashSet};

const MINE: &str = "aa";
const THEIRS: &str = "bb";
const OFFERED: &str = "cc";
const FILE: &str = "dd";
const BARE: &str = "ee";

fn replica() -> Replica {
    let entry = json!({"i": THEIRS, "p": 1.0});
    let bare = json!({"i": BARE, "p": 2.0});
    let screen = json!({"wall": {"item": THEIRS}, "side": {"item": MINE}});
    let records = [
        (format!("library/{MINE}"), doc(&[("title", json!("Mine"), 1)])),
        (format!("library/{THEIRS}"), doc(&[("title", json!("Hand made"), 1)])),
        (format!("library/{OFFERED}"), doc(&[("title", json!("From a repository"), 1)])),
        (
            format!("library/{FILE}"),
            doc(&[("title", json!("File"), 1), ("blob", json!({"name": "a.png"}), 1)]),
        ),
        ("playlists/p1".into(), doc(&[("name", json!("Calm"), 1), ("e:x-0", entry, 1), ("e:x-1", bare, 1)])),
        ("settings/s1".into(), doc(&[("kind", json!("profile"), 1), ("outputs", screen, 1)])),
    ];
    Replica { bases: records.into_iter().map(|(key, doc)| (key, (1, doc))).collect(), ..Replica::default() }
}

#[test]
fn only_records_without_an_item_and_without_an_offer_are_away() {
    let mut local = Local::default();
    local.items.insert(MINE.into(), vec!["u1".into()]);
    let links =
        BTreeMap::from([("playlists/p1".to_owned(), "pl".to_owned()), ("settings/s1".into(), "pr".into())]);
    let offered = HashSet::from([OFFERED.to_owned()]);
    let found = away(&replica(), &local, &links, &offered);
    assert_eq!(found.len(), 2, "{found:?}");
    let bare = found.iter().find(|a| a.key == BARE).unwrap();
    assert_eq!(
        (bare.title.as_str(), bare.playlists.as_slice()),
        ("", ["pl".to_owned()].as_slice()),
        "a record that was never synced is still named by the playlist"
    );
    let item = found.iter().find(|a| a.key == THEIRS).unwrap();
    assert_eq!((item.key.as_str(), item.title.as_str()), (THEIRS, "Hand made"));
    assert_eq!(item.playlists, ["pl"]);
    assert_eq!(
        (item.profiles.len(), item.profiles[0].output.as_str(), item.profiles[0].profile.as_str()),
        (1, "wall", "pr")
    );
    local.items.insert(THEIRS.into(), vec!["u2".into()]);
    local.items.insert(BARE.into(), vec!["u3".into()]);
    assert!(away(&replica(), &local, &links, &offered).is_empty(), "once the items are here the notes go");
}
