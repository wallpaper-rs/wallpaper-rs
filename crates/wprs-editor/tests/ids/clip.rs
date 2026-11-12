use super::{act, at, family, picture, scene};
use serde_json::json;

#[test]
fn layers_copied_from_one_scene_paste_into_another_with_their_files() {
    let root = tempfile::tempdir().unwrap();
    let mut from = scene(&root, "from");
    family(&mut from);
    let file = picture(&from.dir, "sky.png", 10);
    act(&mut from, json!({"add_layer": {"kind": "image", "id": "sky", "file": file, "into": "box"}}))
        .unwrap();
    let clip = wprs_editor::rigs::query(&from.dir, &from.doc, "", &json!({"copy": {"ids": ["kid", "box"]}}));
    assert!(clip["assets"].as_object().is_some_and(|a| a.len() == 1), "{clip}");
    let mut to = scene(&root, "to");
    let other = picture(&to.dir, "other.png", 200);
    act(&mut to, json!({"add_layer": {"kind": "image", "id": "lead", "file": other}})).unwrap();
    std::fs::create_dir_all(to.dir.join("assets")).unwrap();
    let path =
        clip["assets"].as_object().unwrap().values().next().unwrap()["path"].as_str().unwrap().to_owned();
    std::fs::write(to.dir.join(&path), b"not the same picture").unwrap();
    act(&mut to, json!({"paste": {"clip": clip}})).unwrap();
    let layers = at(&to, "/scene/layers");
    let ids: Vec<&str> = layers.as_array().unwrap().iter().map(|l| l["id"].as_str().unwrap()).collect();
    assert_eq!(ids, ["lead", "kid", "box"]);
    assert!(layers[1].get("parent").is_none() && layers[1].get("mask").is_none(), "{}", layers[1]);
    let inside = &layers[2]["layers"];
    assert_eq!(inside[0]["id"], "lead_2");
    assert_eq!(inside[1]["parent"], "lead_2");
    let image = inside[2]["image"].as_str().unwrap();
    let copied = at(&to, &format!("/scene/assets/{image}/path"));
    let bytes = std::fs::read(to.dir.join(copied.as_str().unwrap())).unwrap();
    assert_eq!(bytes, std::fs::read(from.dir.join(&path)).unwrap());
    assert!(wprs_editor::check::check(&to.doc).is_empty(), "{:?}", wprs_editor::check::check(&to.doc));
}

#[test]
fn a_refused_paste_leaves_no_copied_files_behind() {
    let root = tempfile::tempdir().unwrap();
    let mut from = scene(&root, "from");
    let file = picture(&from.dir, "sky.png", 10);
    act(&mut from, json!({"add_layer": {"kind": "image", "id": "sky", "file": file}})).unwrap();
    let clip = wprs_editor::rigs::query(&from.dir, &from.doc, "", &json!({"copy": {"ids": ["sky"]}}));
    let mut to = scene(&root, "to");
    let before: Vec<_> = walk(&to.dir);
    assert!(act(&mut to, json!({"paste": {"clip": clip, "into": "nowhere"}})).is_err());
    assert_eq!(walk(&to.dir), before);
    act(&mut to, json!({"paste": {"clip": clip}})).unwrap();
    assert_ne!(walk(&to.dir), before);
}

fn walk(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let mut todo = vec![dir.to_path_buf()];
    while let Some(d) = todo.pop() {
        for e in std::fs::read_dir(&d).unwrap().flatten() {
            out.push(e.path());
            if e.path().is_dir() {
                todo.push(e.path());
            }
        }
    }
    out.sort();
    out
}
