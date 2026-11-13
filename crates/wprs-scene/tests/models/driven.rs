use super::posing::rigged;
use super::*;

const VIEW: [f32; 4] = [0.0, 0.0, 200.0, 100.0];

fn driven(rigs: Value) -> Runtime {
    let mut rt = runtime(rigged(
        json!({}),
        json!([model("m", json!({ "clips": [{ "id": "bend", "animation": "bend" }] }))]),
    ));
    rt.frame(VIEW, Quality::Q3);
    rt.set_script(serde_json::from_value(json!({ "rigs": rigs })).unwrap());
    rt.advance(1.0 / 60.0);
    rt.frame(VIEW, Quality::Q3);
    rt
}

fn tip(rt: &Runtime) -> [f32; 2] {
    let p = rt.puppet("m").unwrap();
    let f = p.bone_frames_in([200.0, 100.0], MODEL_UNIT)[1];
    [f[4], f[5]]
}

#[test]
fn scripts_move_and_turn_the_bones_of_a_model_in_pixels_of_its_box() {
    let rest = tip(&driven(json!({})));
    let moved = tip(&driven(json!({ "m": { "bones": { "tip": { "position": [10, 5] } } } })));
    assert!(((moved[0] - rest[0]) - 10.0).abs() < 1e-2, "{rest:?} {moved:?}");
    assert!(((moved[1] - rest[1]) - 5.0).abs() < 1e-2, "{rest:?} {moved:?}");
    let placed =
        tip(&driven(json!({ "m": { "bones": { "tip": { "position": [120, 20], "world": true } } } })));
    assert!(placed[0] > 100.0 && (placed[1] - 20.0).abs() < 2.0, "{placed:?}");
}
