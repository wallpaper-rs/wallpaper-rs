use image::{Rgba, RgbaImage};
use serde_json::{Value as Json, json};
use std::collections::BTreeMap;
use std::sync::Arc;
use wprs_core::Fit;
use wprs_render::{GpuImage, Layer, Renderer};
use wprs_scene::{Quality, Ready, SCHEMA, Scene};

const W: u32 = 64;
const H: u32 = 32;
const WIPE: &str = "fn transition(c: Crossing) -> Crossing {
    var o = c;
    o.color = mix(before(c.uv), after(c.uv), step(c.uv.x, c.progress));
    return o;
}";

fn renderer() -> Option<&'static Renderer> {
    match Renderer::shared() {
        Ok(r) => Some(r),
        Err(e) if std::env::var_os("WPRS_REQUIRE_GPU").is_none() => {
            eprintln!("skipping GPU test: {e}");
            None
        }
        Err(e) => panic!("GPU required but unavailable: {e}"),
    }
}

fn ready(cost: Json) -> Arc<Ready> {
    let descriptor = json!({ "id": "wipe", "stage": "transition", "cost": cost });
    let transition =
        json!({ "in": { "descriptor": "t/wipe.json", "wgsl": "t/wipe.wgsl", "duration_ms": 500 } });
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [W, H] }, "transition": transition });
    let mut scene = Scene::load(&text.to_string(), &BTreeMap::new()).unwrap_or_else(|e| panic!("{e}"));
    let files = |path: &str, _: usize| match path {
        "t/wipe.json" => Ok(descriptor.to_string().into_bytes()),
        "t/wipe.wgsl" => Ok(WIPE.as_bytes().to_vec()),
        _ => Err("missing"),
    };
    scene.adopt(&BTreeMap::new(), files).unwrap_or_else(|e| panic!("{e}"));
    scene.passages.coming.clone().expect("declared").unwrap_or_else(|e| panic!("{e}"))
}

fn solid(r: &Renderer, color: [u8; 3]) -> GpuImage {
    let [red, green, blue] = color;
    let picture = RgbaImage::from_pixel(W, H, Rgba([red, green, blue, 255]));
    r.upload(&picture, Fit::Stretch, (W, H)).expect("upload")
}

fn layer(image: &GpuImage) -> Layer<'_> {
    Layer {
        image: Some(image),
        fit: Fit::Stretch,
        background: [0; 3],
        opacity: 1.0,
        uv: None,
        relief: None,
        flip: [false; 2],
        grade: None,
    }
}

#[test]
fn a_transition_is_drawn_from_the_previous_picture_to_the_new_one() {
    let Some(r) = renderer() else { return };
    let (red, blue) = (solid(r, [255, 0, 0]), solid(r, [0, 0, 255]));
    let mut crossing = r.crossing(ready(json!({ "taps": 1 })), (W, H), Quality::Q0).expect("built");
    assert_eq!(crossing.size(), (W, H));
    for (progress, split) in [(0.0, 0), (0.5, W / 2), (1.0, W)] {
        r.cross(&mut crossing, &[layer(&red)], &[layer(&blue)], progress).expect("drawn");
        let picture = r.read_image(crossing.image()).expect("read");
        for x in [0, W / 2 - 2, W / 2 + 2, W - 1] {
            let want = if x < split { [0, 0, 255, 255] } else { [255, 0, 0, 255] };
            let got = picture.get_pixel(x, H / 2).0;
            assert!(got.iter().zip(want).all(|(a, b)| a.abs_diff(b) <= 2), "{progress} at {x}: {got:?}");
        }
    }
}

#[test]
fn a_transition_too_costly_for_the_quality_is_not_built() {
    let Some(r) = renderer() else { return };
    let medium = ready(json!({ "taps": 4 }));
    let refused = r.crossing(medium.clone(), (W, H), Quality::Q1).err().expect("refused");
    assert!(refused.contains("quality 1"), "{refused}");
    assert!(r.crossing(medium, (W, H), Quality::Q2).is_ok());
}
