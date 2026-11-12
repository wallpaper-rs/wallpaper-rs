use super::{act, layer, project};
use serde_json::json;

#[test]
fn text_layers_raise_their_own_letters_into_normals() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let title = json!({ "id": "title", "type": "text", "text": "Hi", "size": 64,
        "transform": { "position": [960, 540] } });
    act(&mut p, json!({ "add": { "pointer": "/scene/layers/-", "value": title } })).unwrap();
    act(&mut p, json!({ "normals": { "layer": "title", "strength": 4, "blur": 0 } })).unwrap();
    let id = layer(&p, "title")["lighting"]["normal"].as_str().unwrap().to_string();
    assert!(id.starts_with("title_normals"), "{id}");
    let path = p.doc["scene"]["assets"][&id]["path"].as_str().unwrap().to_string();
    let map = image::open(p.dir.join(&path)).unwrap().into_rgba8();
    assert!(map.height() >= 48, "the letters are drawn at the layer size");
    let tilted = map.pixels().filter(|px| px.0[0].abs_diff(128) > 20 || px.0[1].abs_diff(128) > 20).count();
    assert!(tilted > 20, "the edges of the letters tilt");
}

#[test]
fn video_layers_raise_a_frame_and_say_when_it_cannot_be_read() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    std::fs::write(p.dir.join("clip.mp4"), b"").unwrap();
    act(&mut p, json!({ "set": { "pointer": "/scene/assets/clip", "value": { "path": "clip.mp4" } } }))
        .unwrap();
    let clip = json!({ "id": "film", "type": "video", "video": "clip" });
    act(&mut p, json!({ "add": { "pointer": "/scene/layers/-", "value": clip } })).unwrap();
    let error = act(&mut p, json!({ "normals": { "layer": "film" } })).unwrap_err().to_string();
    assert!(error.contains("a frame of clip"), "{error}");
}

fn raised(p: &mut wprs_editor::Project, value: serde_json::Value) -> image::RgbaImage {
    let id = value["id"].as_str().unwrap().to_string();
    act(p, json!({ "add": { "pointer": "/scene/layers/-", "value": value } })).unwrap();
    act(p, json!({ "normals": { "layer": id, "strength": 4, "blur": 0 } })).unwrap();
    let asset = layer(p, &id)["lighting"]["normal"].as_str().unwrap().to_string();
    let path = p.doc["scene"]["assets"][&asset]["path"].as_str().unwrap().to_string();
    image::open(p.dir.join(path)).unwrap().into_rgba8()
}

#[test]
fn wrapped_text_raises_every_line_and_its_background_plate() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let one = raised(&mut p, json!({ "id": "one", "type": "text", "text": "Hello world again", "size": 40 }));
    let wrapped = json!({ "id": "two", "type": "text", "text": "Hello world again", "size": 40,
        "wrap": { "width": 140 } });
    let two = raised(&mut p, wrapped);
    assert!(
        two.height() > one.height() * 2,
        "{}x{} {}x{}",
        one.width(),
        one.height(),
        two.width(),
        two.height()
    );
    assert!(two.width() < one.width(), "the lines break at the wrap width");
    let backed = json!({ "id": "three", "type": "text", "text": "Hi", "size": 40,
        "background": { "color": "#000000", "padding": [30, 20] } });
    let plate = raised(&mut p, backed);
    let bare = raised(&mut p, json!({ "id": "four", "type": "text", "text": "Hi", "size": 40 }));
    assert!(plate.width() > bare.width() + 50 && plate.height() > bare.height() + 30);
    let edge =
        |m: &image::RgbaImage| (0..m.height()).filter(|y| m.get_pixel(1, *y).0[0].abs_diff(128) > 20).count();
    let tilted =
        (0..plate.height()).filter(|y| plate.get_pixel(plate.width() / 2, *y).0[1].abs_diff(128) > 20);
    assert!(tilted.count() >= 2, "the top and bottom edges of the plate tilt");
    assert_eq!(edge(&bare), 0, "no plate without a background");
}
