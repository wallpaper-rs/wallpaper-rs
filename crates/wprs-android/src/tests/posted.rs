use crate::pack;
use image::{Rgba, RgbaImage};
use serde_json::json;
use std::fs;

#[test]
fn the_poster_of_a_scene_with_scripts_is_the_preview_its_author_made_never_a_drawing_with_scripts() {
    let source = tempfile::tempdir().unwrap();
    fs::create_dir_all(source.path().join("assets")).unwrap();
    fs::create_dir_all(source.path().join("scripts")).unwrap();
    let layers = json!([{ "id": "sky", "type": "solid", "color": "#ff0000" }]);
    let script = json!({ "path": "scripts/paint.js", "events": ["update"], "uses": ["layers"] });
    let text = json!({ "schema": wprs_scene::SCHEMA, "canvas": { "size": [64, 32] }, "layers": layers,
        "scripts": [script] });
    fs::write(source.path().join("scene.json"), text.to_string()).unwrap();
    fs::write(
        source.path().join("scripts/paint.js"),
        "import { scene } from 'wprs';\nexport function update() { scene.layer('sky').visible = false; }\n",
    )
    .unwrap();
    RgbaImage::from_pixel(8, 8, Rgba([0, 0, 255, 255]))
        .save(source.path().join("assets/preview.png"))
        .unwrap();
    let manifest = json!({ "schema": wprs_format::SCHEMA, "id": "org.example.posted", "version": "1.0.0",
        "title": "Posted", "license": "CC0-1.0", "kind": "scene", "entry": "scene.json",
        "content": { "rating": "general" }, "photosensitive": false, "origin": { "kind": "original" },
        "previews": { "thumbnail": "assets/preview.png" } });
    fs::write(source.path().join("manifest.json"), manifest.to_string()).unwrap();
    let built = source.path().join("posted.wpak");
    wprs_format::build(source.path(), &built, &wprs_format::Limits::default()).unwrap();
    let (poster, size) = pack::check(&built).unwrap();
    assert_eq!(size, Some((64, 32)));
    let poster = poster.expect("the preview of the package is the poster");
    assert_eq!(poster.get_pixel(0, 0).0, [0, 0, 255, 255]);
}
