use super::*;

#[test]
fn a_turned_and_zoomed_camera_and_a_tilted_layer_look_as_they_did() {
    let Some(r) = renderer() else { return };
    let ramp = picture(40, 20, |x, y| [x as u8 * 6, y as u8 * 12, 200, 255]);
    let stage = json!({
        "size": [100, 50], "background": "#101820", "perspective": 120,
        "camera": { "center": [50, 25], "zoom": 1.5, "rotation": 12 }
    });
    let layers = json!([
        { "id": "flat", "type": "image", "image": "ramp", "size": [40, 20], "transform": { "position": [25, 25] } },
        { "id": "tilted", "type": "image", "image": "ramp", "size": [40, 20],
          "transform": { "position": [75, 25], "tilt": [40, 0] } }
    ]);
    let pixels = Shot::new(r, stage, layers, &[("ramp", ramp)]).pixels(r);
    assert!((0..100).any(|x| at(&pixels, x, 25)[2] > 150), "the layers are drawn");
    looks_like("camera", &pixels);
}
