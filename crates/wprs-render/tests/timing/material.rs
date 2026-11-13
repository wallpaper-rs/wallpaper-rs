use super::hostile::{renderer, show};
use super::{book, draw, one};
use serde_json::json;
use std::collections::BTreeMap;
use std::time::Duration;
use wprs_render::{retry_effects, set_build_timeout, stop_effects};
use wprs_scene::{Forge, ForgeMaterial, ForgeShape, SCHEMA, Scene};

const TINT: &str = "fn material(s: Surface) -> Surface {\n    var o = s;\n    o.albedo = vec4<f32>(1.0, 0.0, 1.0, 1.0);\n    return o;\n}\n";

fn scene() -> Scene {
    let glb = Forge::default()
        .material(ForgeMaterial { unlit: true, ..ForgeMaterial::new("skin", [1.0; 4]) })
        .part("ball", ForgeShape::Sphere(8), [0.0; 3], 2.0, 0)
        .glb();
    let layers = json!([{ "id": "m", "type": "model", "model": "ball", "materials": { "skin": { "shader": "coat" } } }]);
    let doc = json!({ "schema": SCHEMA, "canvas": { "size": [64, 32] }, "assets": { "ball": { "path": "ball.glb" } },
        "effects": { "coat": { "descriptor": "fx/coat.json", "wgsl": "fx/coat.wgsl" } }, "layers": layers });
    let mut scene = Scene::from_json(&doc.to_string()).unwrap();
    let read = move |path: &str, _: usize| match path {
        "ball.glb" => Ok(glb.clone()),
        "fx/coat.json" => Ok(br#"{ "id": "coat", "stage": "material", "cost": {} }"#.to_vec()),
        "fx/coat.wgsl" => Ok(TINT.as_bytes().to_vec()),
        p => Err(format!("no {p}")),
    };
    scene.adopt(&BTreeMap::new(), read).unwrap();
    scene
}

#[test]
fn material_shaders_that_build_too_long_or_stopped_the_gpu_are_left_out_and_the_model_still_draws() {
    let _one = one();
    let Some(r) = renderer() else { return };
    book("slow-material-builds");
    let key = scene().loaded["coat"].key;
    set_build_timeout(Duration::ZERO);
    let mut shown = show(r, scene()).unwrap();
    draw(r, &mut shown, (64, 32));
    set_build_timeout(Duration::from_secs(10));
    let left = shown.view.left_out();
    assert!(left.len() == 1 && left[0].contains("coat") && left[0].contains("to build"), "{left:?}");
    draw(r, &mut shown, (64, 32));
    assert_eq!(shown.view.left_out(), left, "the stage does not try it again");
    r.finish().unwrap();
    wprs_scene::forgive_strikes().unwrap();
    stop_effects([key]);
    let mut stopped = show(r, scene()).unwrap();
    draw(r, &mut stopped, (64, 32));
    assert!(stopped.view.left_out().iter().any(|l| l.contains("stopped")), "{:?}", stopped.view.left_out());
    retry_effects();
    let mut fine = show(r, scene()).unwrap();
    draw(r, &mut fine, (64, 32));
    assert!(fine.view.left_out().is_empty(), "{:?}", fine.view.left_out());
    r.finish().unwrap();
}
