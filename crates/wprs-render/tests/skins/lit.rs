use super::kit::{mesh, render, renderer, scene, sketch};
use serde_json::json;
use wprs_scene::Quality;

#[test]
fn lighting_can_make_a_mesh_layer_glow_in_the_dark() {
    let Some(r) = renderer() else { return };
    let shot =
        |lighting| render(r, scene(mesh(json!({ "lighting": lighting })), sketch(false)), 0.0, Quality::Q3);
    let dark = shot(json!({ "emissive": 0.0 }));
    let glowing = shot(json!({ "emissive": 0.5 }));
    let at = |px: &[u8]| px.as_chunks::<4>().0[32 * super::kit::SIDE as usize + 32][0];
    let (a, b) = (at(&glowing), at(&dark));
    assert!(a > b + 10, "{a} {b}");
}
