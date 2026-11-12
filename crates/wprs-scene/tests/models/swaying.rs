use super::posing::rigged;
use super::*;

const VIEW: [f32; 4] = [0.0, 0.0, 200.0, 100.0];

fn run(extra: Value, layer: Value, frames: usize, quality: Quality) -> Runtime {
    let mut rt = runtime(rigged(extra, json!([model("m", layer)])));
    rt.set_quality(quality);
    rt.frame(VIEW, quality);
    for _ in 0..frames {
        rt.advance(1.0 / 30.0);
        rt.frame(VIEW, quality);
    }
    rt
}

fn tip(rt: &Runtime) -> [f32; 12] {
    rt.puppet("m").unwrap().joint_worlds()[1]
}

#[test]
fn a_swinging_bone_of_a_model_lags_in_depth_as_well() {
    let clips = json!([{ "id": "stroll", "animation": "stroll" }]);
    let moving = json!({ "clips": clips, "animations": [{ "clip": "stroll" }] });
    let mut swung = moving.clone();
    swung["physics"] = json!([{ "bone": "tip", "stiffness": 0.2, "damping": 0.1, "inertia": 1.0 }]);
    let (stiff, loose) =
        (tip(&run(json!({}), moving, 12, Quality::Q3)), tip(&run(json!({}), swung, 12, Quality::Q3)));
    let lean = |w: [f32; 12]| w[9];
    assert!(lean(stiff).abs() < 1e-3, "{stiff:?}");
    assert!(lean(loose).abs() > 0.05, "the tip trails the walk along z: {loose:?}");
}

#[test]
fn bone_colliders_keep_a_swinging_tip_out_and_fewer_count_at_low_quality() {
    let windy = json!({ "canvas": { "size": [200, 100], "wind": [600, 0] } });
    let body = json!([{ "bone": "tip", "stiffness": 0.1, "wind": 1 }]);
    let free = json!({ "physics": body });
    let mut blocked = free.clone();
    let ball = json!({ "bone": "root", "shape": "sphere", "radius": 0.4, "offset": [0.6, 1.7, 0] });
    blocked["colliders"] = json!([{ "bone": "root", "radius": 0.1, "offset": [-5, 0, 0] }, { "bone": "root", "radius": 0.1, "offset": [-5, 0, 0] }, ball]);
    let lean = |rt: &Runtime| tip(rt)[1].abs();
    let open = lean(&run(windy.clone(), free, 40, Quality::Q3));
    let kept = lean(&run(windy.clone(), blocked.clone(), 40, Quality::Q3));
    let low = lean(&run(windy, blocked, 40, Quality::Q1));
    assert!(open > 0.5 && kept < open - 0.15, "the ball stops the tip: {open} {kept}");
    assert!((low - open).abs() < 0.05, "at Q1 only the first two colliders count: {low} {open}");
}

#[test]
fn a_model_chain_reaches_for_a_point_of_its_box_through_the_camera() {
    let square = json!({ "canvas": { "size": [100, 100] } });
    let ik = json!({ "ik": [{ "chain": ["root", "tip"], "target": [60, 50] }] });
    let mut rt = runtime(rigged(square, json!([model("m", ik)])));
    rt.frame([0.0, 0.0, 100.0, 100.0], Quality::Q3);
    rt.advance(0.1);
    rt.frame([0.0, 0.0, 100.0, 100.0], Quality::Q3);
    let rig = rt.scene.solids["toy"].rig.clone().unwrap();
    let w = tip(&rt);
    let end = mesh_math::apply(&w, rig.tip(1));
    let want = 5.0 * 0.2 * (30.0f32).to_radians().tan();
    assert!((end[0] - want).abs() < 0.05 && end[1].abs() < 0.05, "the end lands under the point: {end:?}");
}
