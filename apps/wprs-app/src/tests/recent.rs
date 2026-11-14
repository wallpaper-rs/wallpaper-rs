use crate::scenerecent::{order, put, short_path};
use serde_json::json;
use std::path::{Path, PathBuf};

fn entry(name: &str) -> (PathBuf, String) {
    (PathBuf::from(name), name.to_owned())
}

#[test]
fn the_last_project_opened_goes_first_and_is_not_repeated() {
    let list = put(vec![entry("/p/a"), entry("/p/b")], Path::new("/p/b"), "B");
    let names: Vec<_> = list.iter().map(|(f, _)| f.to_string_lossy().into_owned()).collect();
    assert_eq!(names, ["/p/b", "/p/a"]);
    assert_eq!(list[0].1, "B");
}

#[test]
fn the_list_keeps_at_most_forty_projects() {
    let mut list = Vec::new();
    for n in 0..50 {
        list = put(list, Path::new(&format!("/p/{n}")), "x");
    }
    assert_eq!(list.len(), 40);
    assert_eq!(list[0].0, Path::new("/p/49"));
}

#[test]
fn present_projects_come_first_and_the_newest_edit_leads() {
    let mut rows = vec![
        json!({"folder": "gone", "missing": true, "edited": 0}),
        json!({"folder": "old", "missing": false, "edited": 10}),
        json!({"folder": "new", "missing": false, "edited": 20}),
    ];
    order(&mut rows);
    let folders: Vec<_> = rows.iter().map(|r| r["folder"].as_str().unwrap()).collect();
    assert_eq!(folders, ["new", "old", "gone"]);
}

#[test]
fn a_path_shows_the_home_folder_as_a_tilde_and_shortens_in_the_middle() {
    assert_eq!(short_path("/h/me/Documents/wallpaper-rs/a", "/h/me", 56), "~/Documents/wallpaper-rs/a");
    assert_eq!(short_path("/h/meow/a", "/h/me", 56), "/h/meow/a");
    assert_eq!(short_path("/srv/a", "", 56), "/srv/a");
    let long = short_path("/h/me/one/two/three/four/five/six/seven/my-scene", "/h/me/", 20);
    assert_eq!(long.chars().count(), 20);
    assert!(long.starts_with("~/one/") && long.ends_with("my-scene") && long.contains('…'), "{long}");
}
