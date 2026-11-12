use super::{act, manifest, state};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use wprs_editor::Project;
use wprs_editor::check::{check, check_files};
use wprs_editor::starters::starter;

fn package(root: &Path, kind: &str) -> Project {
    state();
    let doc = starter(kind, "org.example.clip", "Clip", "en").unwrap();
    Project::create(&root.join(kind), doc).unwrap()
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures").join(name)
}

fn errors(project: &Project) -> Vec<String> {
    let all = check(&project.doc).into_iter().chain(check_files(&project.dir, &project.doc));
    all.filter(|p| !p.warning).map(|p| p.message).collect()
}

#[test]
fn picture_and_video_packages_start_empty_and_take_their_file_from_an_action() {
    let root = tempfile::tempdir().unwrap();
    let mut video = package(root.path(), "video");
    assert_eq!(
        (video.doc["manifest"]["kind"].as_str(), video.doc["manifest"]["entry"].as_str()),
        (Some("video"), Some("video.mp4"))
    );
    assert!(errors(&video).iter().any(|e| e.contains("video.mp4")), "the entry is not there yet");
    let picture = root.path().join("sky.png");
    image::RgbaImage::new(64, 32).save(&picture).unwrap();
    assert!(manifest(&mut video, json!({"media": {"file": picture}})).is_err(), "a picture is no video");
    manifest(&mut video, json!({"media": {"file": fixture("quadrants-64x32.webm")}})).unwrap();
    assert_eq!(video.doc["manifest"]["entry"], "quadrants-64x32.webm");
    assert!(video.dir.join("quadrants-64x32.webm").is_file());
    assert_eq!(errors(&video), Vec::<String>::new());
    let found = wprs_editor::reels::info(&video.dir, &video.doc);
    assert_eq!((found["present"].clone(), found["aspect"].clone()), (json!(true), json!(2.0)));
    assert!(found["length"].as_f64().is_some_and(|l| l > 0.0), "{found}");
    let mut still = package(root.path(), "image");
    assert!(manifest(&mut still, json!({"media": {"file": fixture("quadrants-64x32.webm")}})).is_err());
    manifest(&mut still, json!({"media": {"file": picture}})).unwrap();
    let found = wprs_editor::reels::info(&still.dir, &still.doc);
    assert_eq!((found["size"].clone(), found["length"].clone()), (json!([64, 32]), Value::Null));
    assert_eq!(wprs_editor::reels::info(&still.dir, &json!({"manifest": {"kind": "web"}})), Value::Null);
    still.save().unwrap();
    assert_eq!(Project::open(&still.dir).unwrap().doc["manifest"]["entry"], "sky.png");
}

#[test]
fn display_and_playback_follow_the_kind_of_the_package() {
    let root = tempfile::tempdir().unwrap();
    let mut video = package(root.path(), "video");
    manifest(&mut video, json!({"media": {"file": fixture("quadrants-64x32.webm")}})).unwrap();
    let display = json!({"display": {"values": {"fit": "fit", "flip": "horizontal", "zoom": 1.5,
        "position": [0.5, -0.25], "background": "#102030"}}});
    manifest(&mut video, display).unwrap();
    let play = json!({"playback": {"values": {"speed": 0.5, "start": 0.2, "end": 1.0, "loop": false,
        "volume": 0.3, "muted": true, "max_fps": 30}}});
    manifest(&mut video, play).unwrap();
    assert_eq!(video.doc["manifest"]["display"]["position"], json!([0.5, -0.25]));
    assert_eq!(video.doc["manifest"]["playback"]["loop"], json!(false));
    for refused in [
        json!({"playback": {"values": {"start": 3.0, "end": 1.0}}}),
        json!({"playback": {"values": {"speed": 9}}}),
        json!({"display": {"values": {"position": [2, 0]}}}),
        json!({"display": {"values": {"fit": "wobble"}}}),
    ] {
        assert!(manifest(&mut video, refused.clone()).is_err(), "{refused} was accepted");
    }
    let mut still = package(root.path(), "image");
    image::RgbaImage::new(8, 8).save(still.dir.join("picture.png")).unwrap();
    manifest(&mut still, json!({"display": {"values": {"fit": "center", "flip": "vertical"}}})).unwrap();
    assert!(manifest(&mut still, json!({"playback": {"values": {"speed": 2}}})).is_err());
    assert!(manifest(&mut still, json!({"display": {"values": {"zoom": 2}}})).is_err());
    let shown = |p: &Project, pointer: &str| -> Vec<String> {
        wprs_editor::inspect::fields(&p.doc, pointer).into_iter().map(|f| f.key).collect()
    };
    assert_eq!(shown(&still, "/manifest/display"), ["accent", "background", "fit", "flip", "span"]);
    assert!(shown(&video, "/manifest/display").contains(&"zoom".to_owned()));
    assert!(shown(&video, "/manifest/playback").contains(&"loop".to_owned()));
    act(&mut video, json!({"undo": {}})).unwrap();
}

#[test]
fn video_presets_set_the_video_settings_with_ranges_from_the_real_length() {
    let root = tempfile::tempdir().unwrap();
    let mut video = package(root.path(), "video");
    manifest(&mut video, json!({"media": {"file": fixture("quadrants-64x32.webm")}})).unwrap();
    act(&mut video, json!({"add_preset": {"id": "calm", "label": "Calm"}})).unwrap();
    let rows = wprs_editor::inspect::fields_in(
        Some(&video.dir),
        &video.doc,
        "/manifest/presets/0",
        &Default::default(),
    );
    let keys: Vec<&str> = rows.iter().map(|f| f.key.as_str()).collect();
    assert!(
        keys.windows(8).any(|w| w == ["values", "speed", "start", "end", "volume", "zoom", "x", "y"]),
        "{keys:?}"
    );
    let speed = rows.iter().find(|f| f.key == "speed").unwrap();
    assert_eq!(
        (speed.widget.as_str(), speed.pointer.as_str()),
        ("number", "/manifest/presets/0/values/speed")
    );
    let length = wprs_editor::reels::info(&video.dir, &video.doc)["length"].as_f64().unwrap();
    let end = rows.iter().find(|f| f.key == "end").unwrap();
    assert_eq!(end.range[1].as_f64(), Some(length.clamp(1.0, 3600.0)));
    act(&mut video, json!({"set": {"pointer": "/manifest/presets/0/values/speed", "value": 0.5}})).unwrap();
    act(&mut video, json!({"set": {"pointer": "/manifest/presets/0/values/volume", "value": 20}})).unwrap();
    assert_eq!(errors(&video), Vec::<String>::new());
    let stray = json!({"set": {"pointer": "/manifest/presets/0/values/tint", "value": "#fff"}});
    act(&mut video, stray).unwrap();
    assert!(errors(&video).iter().any(|e| e.contains("tint")), "a video preset sets only video settings");
}

#[test]
fn the_preview_of_a_picture_or_video_package_is_a_scene_that_shows_the_file_as_the_manifest_says() {
    let root = tempfile::tempdir().unwrap();
    let mut video = package(root.path(), "video");
    manifest(&mut video, json!({"media": {"file": fixture("quadrants-64x32.webm")}})).unwrap();
    let show = json!({"display": {"values": {"fit": "fit", "flip": "horizontal", "zoom": 2, "background": "#102030"}}});
    manifest(&mut video, show).unwrap();
    manifest(
        &mut video,
        json!({"playback": {"values": {"speed": 2, "start": 0.1, "loop": false, "max_fps": 30}}}),
    )
    .unwrap();
    let document = wprs_editor::reels::preview_document(&video.dir, &video.doc, &json!({"quality": "q1"}));
    let (m, scene) = (&document["manifest"], &document["scene"]);
    assert_eq!((m["kind"].as_str(), m["entry"].as_str()), (Some("scene"), Some("scene.json")));
    assert_eq!(m["playback"], json!({"max_fps": 30}));
    assert!(m.get("display").is_none() && m.get("presets").is_none());
    assert_eq!(scene["canvas"], json!({"size": [2160, 1080], "fit": "fit", "background": "#102030"}));
    let layer = &scene["layers"][0];
    assert_eq!((layer["type"].as_str(), layer["video"].as_str()), (Some("video"), Some("media")));
    assert_eq!(layer["size"], json!([2160, 1080]));
    assert_eq!(layer["transform"]["scale"], json!([-2.0, 2.0]));
    assert_eq!(layer["playback"], json!({"rate": 2, "start": 0.1, "loop": false}));
    assert_eq!(scene["assets"]["media"]["path"], "quadrants-64x32.webm");
    assert_eq!(document["preview"]["quality"], "q1");
    let parsed = wprs_scene::Scene::load(&scene.to_string(), &Default::default());
    assert!(parsed.is_ok(), "{:?}", parsed.err());
    let manifest: wprs_format::Manifest = serde_json::from_value(m.clone()).unwrap();
    manifest.check_properties().unwrap();
}

#[test]
fn adding_a_picture_to_an_empty_picture_package_makes_it_the_picture_to_show() {
    let root = tempfile::tempdir().unwrap();
    let mut still = package(root.path(), "image");
    assert!(!errors(&still).is_empty());
    let picture = root.path().join("snow.png");
    image::RgbaImage::new(8, 8).save(&picture).unwrap();
    manifest(&mut still, json!({"add_file": {"file": picture}})).unwrap();
    assert_eq!(still.doc["manifest"]["entry"], "snow.png");
    assert_eq!(errors(&still), Vec::<String>::new());
    let other = root.path().join("more.png");
    image::RgbaImage::new(8, 8).save(&other).unwrap();
    manifest(&mut still, json!({"add_file": {"file": other}})).unwrap();
    assert_eq!(still.doc["manifest"]["entry"], "snow.png");
}
