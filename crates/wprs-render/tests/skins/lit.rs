use super::kit::{mesh, render, renderer, scene, sketch};
use serde_json::json;
use wprs_scene::Quality;

#[test]
fn lighting_can_make_a_mesh_layer_glow_in_the_dark() {
    let Some(r) = renderer() else { return };
    let dim = render(r, scene(mesh(json!({})), sketch(false)), 0.0, Quality::Q3);
    let glowing =
        render(r, scene(mesh(json!({ "lighting": { "emissive": 0.5 } })), sketch(false)), 0.0, Quality::Q3);
    let at = |px: &[u8]| px.as_chunks::<4>().0[32 * super::kit::SIDE as usize + 32][0];
    let (a, b) = (at(&glowing), at(&dim));
    assert!(a > b + 20, "{a} {b}");
}
