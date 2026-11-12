use super::*;
use wprs_scene::Quality;

const SPLIT: &str = "
@fragment fn fs_split_a(in: VertexOut) -> @location(0) vec4<f32> { return tap(in.uv); }
@fragment fn fs_split_b(in: VertexOut) -> @location(0) vec4<f32> { return tap(in.uv); }
@fragment fn fs_split_red(in: VertexOut) -> @location(0) vec4<f32> { return vec4<f32>(1.0, 0.0, 0.0, 1.0); }";

fn split(taps: u32) -> Value {
    json!({ "id": "split", "passes": [{ "entry": "fs_split_a" }, { "entry": "fs_split_b" }],
        "cost": { "taps": taps, "passes": 2 },
        "lighter": { "passes": [{ "entry": "fs_split_red" }], "cost": { "taps": 1 } } })
}

#[test]
fn blur_draws_its_lighter_variant_at_the_lowest_level_close_to_the_full_one() {
    let fx = |q| Fx::new("blur", json!({ "radius": 12 })).input(halves(RED, BLUE)).quality(q);
    let (Some(full), Some(lighter)) = (fx(Quality::Q3).shot(), fx(Quality::Q0).shot()) else { return };
    let [red, _, blue] = lighter.at(32, 16);
    assert!(red > 60 && blue > 60 && red < 250 && blue < 250, "lighter edge {:?}", lighter.at(32, 16));
    lighter.check(&[((2, 16), RED), ((61, 16), BLUE)]);
    assert!(lighter.diff(&full) < 12.0, "{}", lighter.diff(&full));
    Shot::sheet(&[full, lighter]).looks_like("blur_lighter");
}

#[test]
fn bloom_glows_with_its_lighter_variant_at_the_lowest_level() {
    let spot = RgbaImage::from_fn(W, H, |x, y| {
        Rgba(if x.abs_diff(32) < 4 && y.abs_diff(16) < 4 { [255; 4] } else { [0, 0, 0, 255] })
    });
    let params = json!({ "threshold": 0.5, "intensity": 2, "radius": 8 });
    let Some(out) = Fx::new("bloom", params).input(spot).quality(Quality::Q0).shot() else { return };
    assert!(out.at(40, 16)[0] > 10 && out.at(32, 16) == [255; 3], "{:?}", out.at(40, 16));
    out.check(&[((2, 2), [0, 0, 0])]);
}

#[test]
fn a_package_effect_draws_its_lighter_passes_where_its_level_would_leave_it_out() {
    let fx = |q| Fx::new("split", json!({})).own(split(2), SPLIT).input(solid(BLUE)).quality(q);
    let (Some(full), Some(lighter)) = (fx(Quality::Q3).shot(), fx(Quality::Q0).shot()) else { return };
    full.check(&[((10, 10), BLUE)]);
    lighter.check(&[((10, 10), RED)]);
}

#[test]
fn the_hard_budget_draws_lighter_variants_before_it_leaves_effects_out() {
    let uses: Vec<Value> = vec![json!({ "effect": "split" })];
    let layers = (0..48).map(|i| json!({ "id": format!("adj{i}"), "type": "adjustment", "effects": uses })).collect();
    let Some(mut run) = super::budget::packaged(&split(64), SPLIT, layers) else { return };
    run.shot_at((3840, 2160)).check(&[((32, 16), RED)]);
    let left = run.view.left_out();
    assert!(left.iter().any(|l| l.contains("was drawn with its lighter variant: the effects of the scene")), "{left:?}");
    assert!(left.iter().all(|l| !l.contains("was left out")), "{left:?}");
}
