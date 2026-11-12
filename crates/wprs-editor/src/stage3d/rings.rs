use super::gizmo::{Seen, px};
use serde_json::{Value, json};
use wprs_scene::Eye;
use wprs_scene::grip::ring;

const STEPS: usize = 48;

fn drawn(seen: &Seen, c: [f32; 3], axis: [f32; 3], radius: f32) -> (Vec<Value>, Option<Value>) {
    let shown: Vec<[f32; 2]> = ring(c, axis, radius, STEPS).filter_map(|p| seen.at(p)).collect();
    let middle = seen.at(c).unwrap_or_default();
    let far = |q: &&[f32; 2]| (q[0] - middle[0]).hypot(q[1] - middle[1]);
    let grip = shown.iter().max_by(|a, b| far(a).total_cmp(&far(b))).map(|q| px(*q));
    let segments = shown.windows(2).map(|w| json!([px(w[0]), px(w[1])])).collect();
    (segments, grip)
}

pub(super) fn turns(seen: &Seen, c: [f32; 3], radius: f32) -> Value {
    let (mut rings, mut grips) = (Vec::new(), Vec::new());
    for k in 0..3 {
        let axis = std::array::from_fn(|i| f32::from(u8::from(i == k)));
        let (segments, grip) = drawn(seen, c, axis, radius);
        rings.push(json!(segments));
        grips.push(grip.unwrap_or(Value::Null));
    }
    let deep = seen.at(c).map(|[x, y]| px([x - 52.0, y + 52.0]));
    json!({ "rings": rings, "grips": grips, "deep": deep })
}

pub(super) fn camera(seen: &Seen, cam: &Eye) -> Value {
    let (r, _, f) = cam.basis();
    let d = cam.distance();
    let (roll, grip) = drawn(seen, cam.at, f, d * 0.2);
    let point = |p: [f32; 3]| seen.at(p).map(px);
    let orbit: [f32; 3] = std::array::from_fn(|i| cam.at[i] + r[i] * d * 0.12);
    let dolly: [f32; 3] = std::array::from_fn(|i| cam.at[i] + f[i] * d * 0.35);
    json!({ "roll": roll, "roll_grip": grip, "orbit": point(orbit), "dolly": point(dolly) })
}
