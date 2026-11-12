use super::*;

fn lit_by(light: Value) -> Vec<u8> {
    let Some(r) = renderer() else { return Vec::new() };
    let models = BTreeMap::from([("white", cube(ForgeMaterial::new("paint", [1.0; 4])))]);
    let mut lamp = json!({ "id": "lamp", "type": "light", "intensity": 3, "radius": 20000 });
    light.as_object().unwrap().iter().for_each(|(k, v)| lamp[k] = v.clone());
    let layers = json!([lamp, { "id": "cube", "type": "model", "model": "white", "scale": 0.7, "rotation": [0, 45, 0] }]);
    let mut shot = shoot(r, scene(json!({}), layers, models), &[]);
    shot.frame(r, 0.0, Quality::Q3)
}

fn sides(px: &[u8]) -> (u8, u8) {
    (rgb(px, 22, 32)[0], rgb(px, 42, 32)[0])
}

#[test]
fn a_light_with_a_3d_position_lights_the_side_it_stands_on() {
    let right = lit_by(json!({ "position3d": [4, 0, 2] }));
    if right.is_empty() {
        return;
    }
    let left = lit_by(json!({ "position3d": [-4, 0, 2] }));
    let behind = lit_by(json!({ "position3d": [0, 0, -4] }));
    let ((rl, rr), (ll, lr), (bl, br)) = (sides(&right), sides(&left), sides(&behind));
    assert!(rr > rl + 30 && ll > lr + 30, "right {rl} {rr}, left {ll} {lr}");
    assert!(bl + 30 < ll && br + 30 < rr, "a light behind the model leaves its front dark: {bl} {br}");
}

#[test]
fn a_spot_points_at_its_3d_aim_and_without_the_fields_the_light_stays_on_the_plane() {
    let cone = json!([20, 30]);
    let right =
        lit_by(json!({ "light": "spot", "cone": cone, "position3d": [0, 0, 6], "aim3d": [0.6, 0, 0] }));
    if right.is_empty() {
        return;
    }
    let left =
        lit_by(json!({ "light": "spot", "cone": cone, "position3d": [0, 0, 6], "aim3d": [-0.6, 0, 0] }));
    let ((rl, rr), (ll, lr)) = (sides(&right), sides(&left));
    assert!(rr > rl + 20 && ll > lr + 20, "right {rl} {rr}, left {ll} {lr}");
    let (fl, fr) = sides(&lit_by(json!({ "z": 20, "transform": { "position": [64, 32] } })));
    let (gl, gr) = sides(&lit_by(json!({ "z": 20, "transform": { "position": [0, 32] } })));
    assert!(fr > fl + 20 && gl > gr + 20, "2D lights still project: {fl} {fr}, {gl} {gr}");
}
