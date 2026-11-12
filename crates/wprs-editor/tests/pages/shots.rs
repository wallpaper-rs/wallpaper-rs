use super::{act, manifest, state};
use serde_json::json;
use std::path::{Path, PathBuf};
use wprs_editor::Project;
use wprs_editor::starters::starter;

fn package(root: &Path, kind: &str) -> Project {
    state();
    let doc = starter(kind, "org.example.shots", "Shots", "en").unwrap();
    Project::create(&root.join(kind), doc).unwrap()
}

fn video(root: &Path) -> Project {
    let mut p = package(root, "video");
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/quadrants-64x32.webm");
    manifest(&mut p, json!({"media": {"file": file}})).unwrap();
    p
}

fn stored(p: &Project, field: &str) -> PathBuf {
    p.dir.join(p.doc["manifest"]["previews"][field].as_str().unwrap())
}

#[test]
fn a_picture_package_keeps_its_own_picture_as_the_thumbnail_and_refuses_a_recording() {
    let root = tempfile::tempdir().unwrap();
    let mut p = package(root.path(), "image");
    image::RgbaImage::from_pixel(1600, 800, image::Rgba([10, 200, 30, 255]))
        .save(p.dir.join("picture.png"))
        .unwrap();
    act(&mut p, json!({"shot": {"still": {}}})).unwrap();
    let saved = stored(&p, "thumbnail");
    let size = image::image_dimensions(&saved).unwrap();
    assert_eq!(size, (1024, 512), "{saved:?}");
    assert!(act(&mut p, json!({"shot": {"recording": {"seconds": 1.0, "fps": 5}}})).is_err());
}

#[test]
fn a_video_package_gives_a_frame_and_a_recording_in_either_file_type() {
    let root = tempfile::tempdir().unwrap();
    let mut p = video(root.path());
    if wprs_video::init().is_err() {
        return;
    }
    act(&mut p, json!({"shot": {"still": {"time": 0.0}}})).unwrap();
    assert_eq!(image::image_dimensions(stored(&p, "thumbnail")).unwrap(), (64, 32));
    act(&mut p, json!({"shot": {"recording": {"from": 0.0, "seconds": 0.4, "fps": 5, "format": "webp"}}}))
        .unwrap();
    let found = wprs_format::media::probe(&std::fs::read(stored(&p, "animated")).unwrap()).unwrap();
    assert_eq!((found.format, found.width, found.height), ("webp", 64, 32));
    assert_eq!(found.duration_ms, 400);
    act(&mut p, json!({"shot": {"recording": {"seconds": 0.4, "fps": 5}}})).unwrap();
    assert!(stored(&p, "animated").extension().is_some_and(|e| e == "gif"));
}

#[test]
fn an_animated_webp_has_every_frame_and_reads_back() {
    let frames: Vec<_> =
        (0..3u8).map(|n| image::RgbaImage::from_pixel(5, 3, image::Rgba([n * 80, 9, 9, 255]))).collect();
    let bytes = wprs_editor::webp::animated(&frames, 10).unwrap();
    let found = wprs_format::media::probe(&bytes).unwrap();
    assert_eq!((found.format, found.width, found.height, found.duration_ms), ("webp", 5, 3, 300));
    assert!(wprs_format::media::preview(true, "preview-animated.webp", &bytes).is_ok());
    let first = image::load_from_memory(&bytes).unwrap().to_rgba8();
    assert_eq!(first.get_pixel(0, 0).0, [0, 9, 9, 255]);
    assert!(wprs_editor::webp::animated(&[], 10).is_err());
}

#[test]
fn a_web_page_cannot_be_drawn_in_the_editor_and_says_so() {
    let root = tempfile::tempdir().unwrap();
    let mut p = super::page(root.path());
    let said = act(&mut p, json!({"shot": {"still": {}}})).unwrap_err().to_string();
    assert!(said.contains("web engine"), "{said}");
    assert!(act(&mut p, json!({"shot": {"recording": {"format": "webp"}}})).is_err());
}
