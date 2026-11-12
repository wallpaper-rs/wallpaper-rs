mod common;

use common::{image, v2};
use std::fs;
use wprs_format::{Kind, Limits, build_staged, open};

#[test]
fn a_staged_build_packs_the_given_manifest_and_leaves_the_folder_alone() {
    let tmp = tempfile::tempdir().unwrap();
    let (src, out) = (tmp.path().join("page"), tmp.path().join("page.wpak"));
    fs::create_dir_all(&src).unwrap();
    let mut saved = v2(Kind::Web);
    saved.requires.get_or_insert_default().features.push("web".into());
    let disk = saved.to_json().unwrap();
    fs::write(src.join("manifest.json"), &disk).unwrap();
    fs::write(src.join("manifest.json.minisig"), b"stale").unwrap();
    fs::write(src.join("index.html"), b"<!doctype html>").unwrap();
    fs::write(src.join("preview.png"), image(16, 16, ::image::ImageFormat::Png)).unwrap();
    let mut edited = saved.clone();
    edited.title = "Edited".into();
    let json = edited.to_json().unwrap();
    build_staged(&src, json.as_bytes(), &out, &Limits::default()).unwrap();
    let package = open(&out, &Limits::default()).unwrap();
    assert_eq!(package.manifest.title, wprs_format::Text::Plain("Edited".to_string()));
    let names: Vec<&str> = package.entries.iter().map(|e| e.path.as_str()).collect();
    assert_eq!(names, ["manifest.json", "index.html", "preview.png"]);
    assert_eq!(fs::read_to_string(src.join("manifest.json")).unwrap(), disk);
    let invalid = json.replace("index.html", "gone.html");
    assert!(build_staged(&src, invalid.as_bytes(), tmp.path().join("bad.wpak"), &Limits::default()).is_err());
}
