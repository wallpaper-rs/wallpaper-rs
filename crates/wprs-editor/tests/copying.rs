use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use wprs_editor::copying::{extract, facts, lineage, new_id};

fn package(root: &Path, license: &str, kind: &str) -> PathBuf {
    let src = root.join(format!("src-{kind}-{license}"));
    fs::create_dir_all(&src).unwrap();
    let manifest = json!({
        "schema": "wallpaper-rs.wpak/2", "id": "org.example.sky", "version": "1.0.0", "title": "Sky",
        "description": "A calm sky", "license": license, "kind": kind, "entry": "image.png",
        "authors": ["Ana"], "origin": {"kind": "original"}, "content": {"rating": "general"},
        "photosensitive": false,
        "credits": [{"paths": ["image.png"], "license": license, "attribution": "Ana"}]
    });
    fs::write(src.join("manifest.json"), manifest.to_string()).unwrap();
    image::RgbaImage::from_pixel(4, 4, image::Rgba([40, 80, 120, 255])).save(src.join("image.png")).unwrap();
    let out = root.join(format!("{kind}-{license}.wpak"));
    wprs_format::build(&src, &out, &Default::default()).unwrap();
    out
}

#[test]
fn a_picture_package_is_unpacked_as_a_copy_with_a_new_id_keeping_credits_and_license() {
    let root = tempfile::tempdir().unwrap();
    let file = package(root.path(), "CC-BY-4.0", "image");
    let told = facts(&file).unwrap();
    assert_eq!(
        (told["verdict"].as_str(), told["new_id"].as_str()),
        (Some("allowed"), Some("org.example.sky.copy"))
    );
    assert_eq!(told["conditions"].as_array().map(Vec::len), Some(1));
    assert!(told["summary"].as_str().is_some_and(|s| s.contains("credit you")), "{told}");
    let dir = root.path().join("mine");
    assert_eq!(extract(&file, &dir, None, false).unwrap(), "org.example.sky.copy");
    let manifest: Value = serde_json::from_slice(&fs::read(dir.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["id"], "org.example.sky.copy");
    assert_eq!(manifest["license"], "CC-BY-4.0");
    assert_eq!(manifest["credits"][0]["attribution"], "Ana");
    assert_eq!(manifest["origin"]["kind"], "derived");
    assert_eq!(manifest["origin"]["derived_from"][0]["author"], "Ana");
    assert!(manifest.get("authors").is_none());
    assert!(dir.join("image.png").is_file() && !dir.join("SIGNATURE").exists());
    assert!(extract(&file, &dir, None, false).is_err(), "a folder in use was overwritten");
}

#[test]
fn a_package_without_derivatives_is_refused_unless_the_author_says_it_is_theirs() {
    let root = tempfile::tempdir().unwrap();
    let file = package(root.path(), "CC-BY-ND-4.0", "image");
    assert_eq!(facts(&file).unwrap()["verdict"], "refused");
    let dir = root.path().join("copy");
    let refused = extract(&file, &dir, None, false).unwrap_err().to_string();
    assert!(refused.contains("CC-BY-ND-4.0"), "{refused}");
    assert!(!dir.exists() || fs::read_dir(&dir).unwrap().next().is_none(), "a refused copy left files");
    assert_eq!(extract(&file, &dir, Some("org.example.mine"), true).unwrap(), "org.example.mine");
    assert_eq!(lineage(&file).unwrap(), json!([]), "an original has nothing to warn about");
    let again = root.path().join("again.wpak");
    wprs_format::build(&dir, &again, &Default::default()).unwrap();
    let warned = lineage(&again).unwrap();
    let text = warned[0].as_str().unwrap_or_default();
    assert!(
        text.contains("CC-BY-ND-4.0") && text.contains("Ana") && warned.as_array().unwrap().len() == 1,
        "{warned}"
    );
    assert_eq!(lineage(&dir).unwrap(), warned, "the project folder of a copy warns like its package");
    assert!(extract(&file, &root.path().join("same"), Some("org.example.sky"), true).is_err());
    assert!(extract(&file, &root.path().join("bad"), Some("Not An Id"), true).is_err());
}

#[test]
fn copies_of_copies_get_the_next_id() {
    assert_eq!(new_id("org.example.sky"), "org.example.sky.copy");
    assert_eq!(new_id("org.example.sky.copy"), "org.example.sky.copy2");
    assert_eq!(new_id("org.example.sky.copy2"), "org.example.sky.copy3");
}
