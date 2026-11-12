use super::kinds::{group, red, sparks};
use super::*;

fn streak(depth: f32, render: Value) -> Value {
    let mut s = sparks(depth, render);
    s["particles"]["s"]["emitters"][0]["shape"] = json!({ "point": [4, 32] });
    s["particles"]["s"]["init"]["speed"] = json!([300, 300]);
    s["particles"]["s"]["init"]["angle"] = json!([0, 0]);
    s["particles"]["s"]["init"]["size"] = json!([16, 16]);
    s
}

pub fn covered(r: &Renderer, extra: Value, quality: Quality) -> usize {
    shaded(r, extra, quality, |c| !near(c, [255, 0, 0], 8))
}

fn shaded(r: &Renderer, extra: Value, quality: Quality, pick: impl Fn([u8; 3]) -> bool) -> usize {
    let layer = json!({ "id": "p", "type": "particles", "system": "s", "depth_test": true });
    let mut shot = shoot(r, scene(extra, group(layer), red()), &[]);
    shot.view.set_particle_trails(r, wprs_scene::Tier::T3);
    let px = (0..4).map(|_| shot.frame(r, 0.05, quality)).last().unwrap();
    let pixels = (22..42).flat_map(|x| (29..36).map(move |y| (x, y)));
    pixels.filter(|(x, y)| pick(rgb(&px, *x, *y))).count()
}

#[test]
fn trails_in_a_3d_group_hide_one_by_one_behind_the_models_in_front_of_them() {
    let Some(r) = renderer() else { return };
    for render in [
        json!({ "trail": { "length": 16, "width": 1 } }),
        json!({ "trail": { "length": 16, "width": 1, "segments": 4 } }),
        json!({ "trail": { "length": 32, "width": 1 } }),
    ] {
        let front = covered(r, streak(-50.0, render.clone()), Quality::Q3);
        let behind = covered(r, streak(50.0, render.clone()), Quality::Q3);
        let low = covered(r, streak(-50.0, render.clone()), Quality::Q0);
        assert!(front > 20 && behind == 0 && low == 0, "{render}: {front} {behind} {low}");
    }
}

fn strung(depth: f32) -> Value {
    let mut s = streak(depth, json!({ "rope": { "width": 12 } }));
    s["particles"]["s"]["emitters"][0] =
        json!({ "shape": { "point": [4, 32] }, "rate": 60, "max_per_step": 1 });
    s["particles"]["s"]["init"]["speed"] = json!([200, 200]);
    s["particles"]["s"]["init"]["size"] = json!([1, 1]);
    s["particles"]["s"]["capacity"] = json!(64);
    s
}

#[test]
fn ropes_in_a_3d_group_hide_one_by_one_behind_the_models_in_front_of_them() {
    let Some(r) = renderer() else { return };
    let (front, behind) = (covered(r, strung(-50.0), Quality::Q3), covered(r, strung(50.0), Quality::Q3));
    let low = covered(r, strung(-50.0), Quality::Q0);
    assert!(front > 20 && behind == 0 && low == 0, "{front} {behind} {low}");
}

fn family(depth: f32) -> Value {
    let mut s = sparks(depth, json!({}));
    let base = s["particles"]["s"].clone();
    let mut kid = |name: &str, next: Option<&str>, life: f32, size: f32| {
        let mut k = base.clone();
        k["emitters"] = json!([{ "shape": { "point": [0, 0] }, "bursts": [{ "count": 1 }] }]);
        k["init"]["life"] = json!([life, life]);
        k["init"]["size"] = json!([size, size]);
        k["init"]["color"] = json!(["#0000ff", "#0000ff"]);
        k["init"]["depth"] = json!([0, 0]);
        if let Some(n) = next {
            k["children"] = json!([{ "system": n, "on": "death" }]);
        }
        s["particles"][name] = k;
    };
    kid("g", None, 50.0, 20.0);
    kid("c", Some("g"), 0.05, 2.0);
    s["particles"]["s"]["children"] = json!([{ "system": "c", "on": "death" }]);
    s["particles"]["s"]["init"]["life"] = json!([0.05, 0.05]);
    s
}

#[test]
fn grandchildren_of_a_system_in_a_3d_group_hide_one_by_one_too() {
    let Some(r) = renderer() else { return };
    let blue = |c: [u8; 3]| c[2] > 128 && c[0] < 64;
    let (front, behind) =
        (shaded(r, family(-50.0), Quality::Q3, blue), shaded(r, family(50.0), Quality::Q3, blue));
    assert!(front > 5 && behind == 0, "{front} {behind}");
}
