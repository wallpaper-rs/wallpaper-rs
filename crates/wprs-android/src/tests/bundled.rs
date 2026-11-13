use crate::repos::{self, bundled};
use std::path::Path;

pub(super) fn the_examples_come_as_a_repository_that_stays_removed(source: &Path) {
    assert!(bundled::due(7).unwrap());
    let made = bundled::offer(source, 7, false).unwrap();
    let id = made["repo_id"].as_str().unwrap().to_owned();
    assert!(repos::added().contains(&id) && bundled::is_bundled(&id));
    let listed = repos::list().unwrap();
    let ours: Vec<_> = listed["repos"].as_array().unwrap().iter().filter(|r| r["bundled"] == true).collect();
    assert_eq!(
        (ours.len(), &ours[0]["name"], &ours[0]["broken"]),
        (1, &"Example wallpapers".into(), &serde_json::Value::Null)
    );
    assert!(!repos::browse("pulse").unwrap().as_array().unwrap().is_empty());
    assert!(!bundled::due(7).unwrap() && bundled::due(8).unwrap());
    repos::remove(&id).unwrap();
    assert!(!repos::added().contains(&id));
    assert!(!bundled::due(9).unwrap());
    assert!(bundled::offer(source, 9, false).unwrap().is_null());
    let again = bundled::offer(source, 9, true).unwrap();
    assert_eq!(again["repo_id"], id.as_str());
    assert!(repos::added().contains(&id));
    repos::remove(&id).unwrap();
}
