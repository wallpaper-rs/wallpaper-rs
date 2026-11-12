use super::{Shot, UP, at, files, renderer, scene};
use serde_json::{Value, json};
use wprs_scene::Quality;

fn lamp() -> Value {
    json!({ "id": "lamp", "type": "light", "transform": { "position": [80, 45] }, "z": 10, "radius": 50,
        "intensity": 0.4, "volumetric": { "density": 0.8, "samples": 16 } })
}

fn wall() -> Value {
    json!({ "id": "wall", "type": "image", "image": "wall", "size": [160, 90], "transform": { "position": [80, 45] } })
}

fn shot(layers: Value) -> Option<image::RgbaImage> {
    let r = renderer()?;
    Some(Shot::new(r, scene(layers, json!({})), &files(UP)).pixels(r, Quality::Q3))
}

#[test]
fn the_air_of_a_light_inside_an_isolated_group_stays_under_the_layers_above_it_in_the_group() {
    let group = |inner: Value| json!({ "id": "g", "type": "group", "opacity": 0.999, "layers": inner });
    let (Some(covered), Some(open), Some(plain)) = (
        shot(json!([group(json!([lamp(), wall()]))])),
        shot(json!([group(json!([wall(), lamp()]))])),
        shot(json!([group(json!([wall()]))])),
    ) else {
        return;
    };
    let near = |a: [u8; 4], b: [u8; 4]| a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 2);
    assert!(near(at(&covered, 80, 45), at(&plain, 80, 45)), "the wall above hides the air inside the group");
    assert!(at(&open, 80, 45)[0] > at(&plain, 80, 45)[0] + 20, "{:?}", at(&open, 80, 45));
}

#[test]
fn the_air_of_a_light_last_in_a_group_is_drawn_before_the_layers_after_the_group() {
    let group = json!({ "id": "g", "type": "group", "opacity": 0.999, "layers": [lamp()] });
    let (Some(covered), Some(plain)) = (shot(json!([group, wall()])), shot(json!([wall()]))) else { return };
    let near = |a: [u8; 4], b: [u8; 4]| a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 2);
    assert!(near(at(&covered, 80, 45), at(&plain, 80, 45)), "{:?}", at(&covered, 80, 45));
}
