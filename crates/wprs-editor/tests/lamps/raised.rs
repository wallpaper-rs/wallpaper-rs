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
