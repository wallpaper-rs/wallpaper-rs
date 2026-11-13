use crate::updates::{Kept, newer};
use serde_json::{Value, json};

mod automatic;
mod away;
mod labelled;
mod provided;
mod repository;

fn offered(repo: &str, id: &str, versions: &[&str]) -> wprs_format::Item {
    wprs_format::Item {
        repo_id: repo.into(),
        repo_name: "Studio".into(),
        id: id.into(),
        author: None,
        release: wprs_format::Release {
            version: versions.last().copied().unwrap_or_default().into(),
            ..Default::default()
        },
        versions: versions.iter().map(|v| (*v).to_owned()).collect(),
        installed: Vec::new(),
        featured: false,
    }
}

fn kept(id: &str, version: &str) -> Kept {
    Kept {
        uid: format!("u-{id}"),
        repo: "r".into(),
        id: id.into(),
        version: version.into(),
        title: id.into(),
    }
}

#[test]
fn only_newer_releases_of_the_same_repository_are_updates() {
    let found = [
        offered("r", "dawn", &["1.0.0", "1.2.0", "1.10.0"]),
        offered("r", "dusk", &["2.0.0"]),
        offered("other", "noon", &["9.0.0"]),
        offered("r", "night", &["not a version"]),
    ];
    let have = [kept("dawn", "1.2.0"), kept("dusk", "2.0.0"), kept("noon", "1.0.0"), kept("night", "1.0.0")];
    let updates = newer(&found, &have);
    let wanted = json!([{ "uid": "u-dawn", "repo_id": "r", "id": "dawn", "title": "dawn", "installed": "1.2.0", "available": "1.10.0" }]);
    assert_eq!(Value::Array(updates), wanted);
    assert!(newer(&found, &[kept("dawn", "broken")]).is_empty());
}

pub(super) fn nothing_to_update_without_repositories() {
    let account = |request: Value| -> Value {
        serde_json::from_str(&crate::account::request(&request.to_string())).unwrap()
    };
    assert_eq!(account(json!({ "op": "updates" })), json!([]));
    assert_eq!(account(json!({ "op": "refresh_repos", "force": true })), json!([]));
    assert_eq!(account(json!({ "op": "update", "uid": "nowhere" })), json!({ "item": null }));
    assert!(account(json!({ "op": "refresh_repos" }))["error"].is_string(), "force is required");
    repository::dependencies_and_updates_follow_a_repository();
    automatic::updates_follow_the_setting();
    away::notes_follow_the_records_that_have_no_item_here();
    away::an_installed_item_leaves_the_notes_at_once();
    provided::picture_sources_download_only_when_turned_on();
    labelled::ratings_tags_and_titles_are_set_from_the_phone();
}
