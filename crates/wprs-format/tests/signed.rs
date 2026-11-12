mod common;
mod fixture;

use common::{image, v2};
use fixture::{key, pair, signer};
use std::fs;
use std::path::Path;
use wprs_format::{Error, Kind, Limits, build, open, sign_source, verify_package};

fn folder(dir: &Path) {
    fs::create_dir_all(dir).unwrap();
    let mut saved = v2(Kind::Web);
    saved.requires.get_or_insert_default().features.push("web".into());
    fs::write(dir.join("manifest.json"), saved.to_json().unwrap()).unwrap();
    fs::write(dir.join("index.html"), b"<!doctype html>").unwrap();
    fs::write(dir.join("preview.png"), image(16, 16, ::image::ImageFormat::Png)).unwrap();
}

#[test]
fn a_matching_signature_is_built_in_and_verifies() {
    let tmp = tempfile::tempdir().unwrap();
    let (src, out) = (tmp.path().join("page"), tmp.path().join("page.wpak"));
    folder(&src);
    let author = key();
    sign_source(&src, &signer(&author)).unwrap();
    build(&src, &out, &Limits::default()).unwrap();
    let mut package = open(&out, &Limits::default()).unwrap();
    assert_eq!(verify_package(&mut package, &[pair(&author)]).unwrap(), author.fingerprint);
}

#[test]
fn a_stale_signature_stops_the_build() {
    let tmp = tempfile::tempdir().unwrap();
    let (src, out) = (tmp.path().join("page"), tmp.path().join("page.wpak"));
    folder(&src);
    sign_source(&src, &signer(&key())).unwrap();
    let mut edited = fs::read_to_string(src.join("manifest.json")).unwrap();
    edited.push('\n');
    fs::write(src.join("manifest.json"), edited).unwrap();
    let refused = build(&src, &out, &Limits::default());
    assert!(
        matches!(&refused, Err(Error::Invalid { field, .. }) if field == "manifest.json.minisig"),
        "{refused:?}"
    );
    assert!(!out.exists());
}
