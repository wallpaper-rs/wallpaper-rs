use image::{Rgba, RgbaImage};
use serde_json::json;
use wprs_editor::media::{dress, played_from};
use wprs_render::SceneAssets;

#[test]
fn a_made_up_track_brings_its_cover_to_the_pictures_that_follow_the_player() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("assets")).unwrap();
    RgbaImage::from_pixel(600, 300, Rgba([0, 0, 255, 255]))
        .save(dir.path().join("assets/cover.png"))
        .unwrap();
    let track = json!({ "title": "Night Drive", "playing": true, "cover": "assets/cover.png" });
    let made = played_from(track, dir.path()).unwrap();
    assert_eq!((made.state.title.as_str(), made.state.cover, made.state.playing), ("Night Drive", 1, true));
    assert!(made.state.colors.is_some());
    let cover = made.cover.unwrap();
    assert_eq!(cover.dimensions(), (256, 128));
    let scene = wprs_scene::Scene::load(
        &json!({ "schema": wprs_scene::SCHEMA, "canvas": { "size": [8, 8] },
            "assets": { "art": { "path": "assets/cover.png", "media": "current" } },
            "layers": [{ "id": "art", "type": "image", "image": "art" }] })
        .to_string(),
        &Default::default(),
    )
    .unwrap();
    let mut assets = SceneAssets::default();
    dress(&mut assets, &scene, &cover);
    assert_eq!(assets.images["art"].size(), (256, 128));
}

#[test]
fn a_cover_outside_the_scene_folder_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let refused = played_from(json!({ "cover": "../x.png" }), dir.path());
    assert!(refused.err().is_some_and(|e| e.to_string().contains("inside the scene folder")));
    assert!(played_from(json!({ "rate": 500 }), dir.path()).is_err());
}
