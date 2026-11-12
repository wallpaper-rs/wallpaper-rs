use super::{Shot, UP, files, renderer, scene};
use serde_json::{Value, json};
use wprs_scene::Quality;

fn grouped(group: Value) -> Value {
    let lamp = json!({ "id": "lamp", "type": "light", "transform": { "position": [80, 45] }, "z": 10,
        "radius": 30, "intensity": 0.4, "volumetric": { "density": 0.5, "samples": 16 } });
    let dot = json!({ "id": "dot", "type": "shape", "shape": { "rect": { "size": [4, 4] } },
        "fill": "#ff0000", "transform": { "position": [80, 45] } });
    let mut g = json!({ "id": "g", "type": "group", "layers": [dot, lamp] });
    group.as_object().into_iter().flatten().for_each(|(k, v)| g[k] = v.clone());
    json!([g])
}

fn spread(image: &image::RgbaImage, pick: impl Fn([u8; 4]) -> bool) -> Option<[f32; 4]> {
    let all: Vec<[f32; 2]> =
        image.enumerate_pixels().filter(|p| pick(p.2.0)).map(|(x, y, _)| [x as f32, y as f32]).collect();
    let (lo, hi) = (
        |k: usize| all.iter().map(|p| p[k]).fold(f32::MAX, f32::min),
        |k: usize| all.iter().map(|p| p[k]).fold(f32::MIN, f32::max),
    );
    (!all.is_empty()).then(|| [(lo(0) + hi(0)) / 2.0, (lo(1) + hi(1)) / 2.0, hi(0) - lo(0), hi(1) - lo(1)])
}

fn glow(px: [u8; 4]) -> bool {
    px.iter().take(3).min().is_some_and(|c| *c > 12)
}

fn dot(px: [u8; 4]) -> bool {
    px[0].abs_diff(px[2]) > 40
}

#[test]
fn the_air_of_a_light_in_a_tilted_group_leans_with_the_group() {
    let Some(r) = renderer() else { return };
    let shot = |g| Shot::new(r, scene(grouped(g), json!({})), &files(UP)).pixels(r, Quality::Q3);
    for more in [json!({}), json!({ "opacity": 0.9 }), json!({ "effects": [] })] {
        let mut tilted = more.clone();
        tilted["transform"] = json!({ "tilt": [0, 60] });
        let (level, leaned) = (shot(more.clone()), shot(tilted));
        let (Some(flat), Some(lit)) = (spread(&level, glow), spread(&leaned, glow)) else {
            panic!("the air shows in both groups")
        };
        let Some(seen) = spread(&leaned, dot) else { panic!("the marker shows in the tilted group") };
        assert!((flat[2] - flat[3]).abs() <= 2.0, "a level group keeps a round glow: {flat:?}");
        assert!(flat[0] == 79.5 && flat[1] == 44.5 && flat[2] > 50.0, "the whole glow shows: {flat:?}");
        assert!(lit[2] < lit[3] * 0.8 && lit[3] > 25.0, "the glow narrows along the tilt: {lit:?}");
        assert!((lit[0] - seen[0]).abs() <= 3.0 && (lit[1] - seen[1]).abs() <= 2.0, "{lit:?} {seen:?}");
        assert!((seen[0] - 80.0).abs() > 10.0, "the group leans the light away: {seen:?}");
    }
}
