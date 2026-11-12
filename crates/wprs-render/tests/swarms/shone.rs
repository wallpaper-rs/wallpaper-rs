use super::drawn::{dot, seen_among};
use super::fields::system;
use super::*;
use image::{Rgba, RgbaImage};

const AWAY: &str = "fn particle(p: Sprite) -> Sprite {\n    var o = p;\n    o.position = p.position + vec2<f32>(20.0, 0.0);\n    return o;\n}\n";

fn lamp(x: f32) -> Value {
    json!({ "id": "lamp", "type": "light", "transform": { "position": [x, 40] }, "z": 20, "radius": 60,
        "intensity": 2 })
}

fn rope(render: Value) -> Value {
    let init = json!({ "life": [4, 4], "speed": [40, 40], "angle": [0, 0], "size": [0, 0] });
    let mut s = system(json!({ "point": [20, 40] }), init, json!({ "render": render }));
    s["emitters"][0] = json!({ "shape": { "point": [20, 40] }, "rate": 30, "max_per_step": 1 });
    if s["render"].get("trail").is_some() {
        s["emitters"][0] = json!({ "shape": { "point": [20, 40] }, "burst": { "count": 1 } });
        s["init"]["size"] = json!([8, 8]);
    }
    s
}

fn strung(r: &Renderer, render: Value, mut layers: Vec<Value>, gpu: bool) -> Option<Vec<u8>> {
    layers.push(json!({ "id": "p", "type": "particles", "system": "s" }));
    let assets =
        json!({ "stripes": { "path": "stripes.png" }, "bump": { "path": "bump.png", "linear": true } });
    let stripes = RgbaImage::from_fn(160, 80, |_, y| Rgba(if y % 4 < 2 { [0, 0, 0, 255] } else { [255; 4] }));
    let bump = RgbaImage::from_pixel(8, 8, Rgba([128, 128, 255, 255]));
    let images = vec![("stripes", Pixels::new(stripes)), ("bump", Pixels::new(bump))];
    let mut run = Run::with(r, (json!({ "s": rope(render) }), json!(layers), assets), images, gpu)?;
    run.view.set_particle_trails(r, wprs_scene::Tier::T3);
    Some((0..60).map(|_| run.frame(r, 1.0 / 30.0)).last().unwrap())
}

#[test]
fn particle_shaders_take_the_scene_lights() {
    let Some(r) = renderer() else { return };
    let files = [
        ("effects/same.json", r#"{ "id": "same", "stage": "particle", "cost": {} }"#),
        ("effects/same.wgsl", AWAY),
    ];
    for gpu in [false, true] {
        let s = dot([60.0, 40.0], json!({ "render": { "shader": "same", "lit": true } }));
        let Some((near, left)) = seen_among(r, (s.clone(), vec![lamp(80.0)]), &files, gpu) else { return };
        let Some((far, _)) = seen_among(r, (s, vec![lamp(400.0)]), &files, gpu) else { return };
        assert!(left.is_empty(), "gpu {gpu}: {left:?}");
        assert!(at(&near, 80, 40)[0] > 100, "gpu {gpu}: {:?}", at(&near, 80, 40));
        assert!(at(&far, 80, 40)[0] < 10, "gpu {gpu}: {:?}", at(&far, 80, 40));
    }
}

#[test]
fn ropes_and_trails_take_the_scene_lights() {
    let Some(r) = renderer() else { return };
    let rope = json!({ "rope": { "width": 6, "subdivide": 2 }, "lit": true });
    let trail = json!({ "trail": { "length": 48, "width": 1, "segments": 8, "subdivide": 3 }, "lit": true });
    let short = json!({ "trail": { "length": 12, "width": 1 }, "lit": true });
    for (render, x) in [(rope, 60), (trail, 84), (short, 94)] {
        for gpu in [false, true] {
            let Some(near) = strung(r, render.clone(), vec![lamp(x as f32)], gpu) else { return };
            let Some(far) = strung(r, render.clone(), vec![lamp(400.0)], gpu) else { return };
            assert!(at(&near, x, 40)[0] > 100, "{render} gpu {gpu}: {:?}", at(&near, x, 40));
            assert!(at(&far, x, 40)[0] < 10, "{render} gpu {gpu}: {:?}", at(&far, x, 40));
        }
    }
}

#[test]
fn ropes_refract_what_is_behind_them() {
    let Some(r) = renderer() else { return };
    let back = json!({ "id": "back", "type": "image", "image": "stripes", "size": [160, 80],
        "transform": { "position": [80, 40] } });
    let bend =
        json!({ "rope": { "width": 12, "subdivide": 2 }, "refract": { "normal": "bump", "amount": 1 } });
    for gpu in [false, true] {
        let Some(bent) = strung(r, bend.clone(), vec![back.clone()], gpu) else { return };
        let Some(plain) = strung(r, json!({ "rope": { "width": 0.01 } }), vec![back.clone()], gpu) else {
            return;
        };
        let moved =
            (36..44).any(|y| (40..80).any(|x| at(&bent, x, y)[0].abs_diff(at(&plain, x, y)[0]) > 100));
        assert!(moved, "gpu {gpu}: the stripes bend under the rope");
        assert!(
            (40..80).all(|x| at(&bent, x, 10) == at(&plain, x, 10)),
            "gpu {gpu}: nothing bends off the rope"
        );
        let p = at(&bent, 60, 40);
        assert!(p[0].abs_diff(p[1]) < 8, "gpu {gpu}: the rope is not painted: {p:?}");
    }
}

#[test]
fn particle_shaders_refract_what_is_behind_them() {
    let Some(r) = renderer() else { return };
    let files = [
        ("effects/same.json", r#"{ "id": "same", "stage": "particle", "cost": {} }"#),
        ("effects/same.wgsl", AWAY),
    ];
    let back = json!({ "id": "back", "type": "image", "image": "pair", "size": [160, 80],
        "transform": { "position": [80, 40] } });
    let bend = |amount: f32| {
        dot(
            [80.0, 40.0],
            json!({ "render": { "shader": "same", "refract": { "normal": "pair", "amount": amount } } }),
        )
    };
    for gpu in [false, true] {
        let Some((bent, left)) = seen_among(r, (bend(1.0), vec![back.clone()]), &files, gpu) else { return };
        let Some((still, _)) = seen_among(r, (bend(0.0), vec![back.clone()]), &files, gpu) else { return };
        assert!(left.is_empty(), "gpu {gpu}: {left:?}");
        let moved = |xs: std::ops::Range<u32>| xs.into_iter().any(|x| at(&bent, x, 40) != at(&still, x, 40));
        assert!(moved(92..108), "gpu {gpu}: bent where the shader moved the sprite");
        assert!(!moved(60..76), "gpu {gpu}: not where the sprite was");
    }
}
