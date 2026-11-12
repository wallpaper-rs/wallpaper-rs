use super::*;

const LEVELS: [Quality; 4] = [Quality::Q0, Quality::Q1, Quality::Q2, Quality::Q3];

fn forms(d: &Descriptor, optional: bool) -> Vec<Form> {
    LEVELS.iter().map(|q| d.form(*q, optional)).collect()
}

#[test]
fn a_lighter_variant_stands_in_where_the_level_would_leave_the_effect_out() {
    let blur = descriptor("blur").unwrap();
    assert_eq!((blur.units(), blur.lighter_units()), (6.0, Some(1.0)));
    assert_eq!(forms(blur, false), [Form::Lighter, Form::Full, Form::Full, Form::Full]);
    assert_eq!(forms(blur, true), [Form::Lighter, Form::Lighter, Form::Full, Form::Full]);
    let (lite, scale, frozen) = blur.drawn(Quality::Q0, false).unwrap();
    assert!(lite.light && scale == 1.0 && !frozen && lite.passes.len() == 2);
    assert_eq!(blur.drawn(Quality::Q1, false).map(|(d, s, _)| (d.light, s)), Some((false, 0.5)));
    let edges = descriptor("edges").unwrap();
    assert_eq!(forms(edges, false), [Form::Out, Form::Full, Form::Full, Form::Full]);
    assert_eq!(descriptor("fluid").unwrap().form(Quality::Q0, false), Form::Frozen);
    let bloom = descriptor("bloom").unwrap();
    assert_eq!((bloom.units(), bloom.lighter_units()), (3.0, Some(1.0)));
    assert_eq!(forms(bloom, true), [Form::Lighter, Form::Lighter, Form::Full, Form::Full]);
}

#[test]
fn up_to_takes_the_variant_at_its_level_and_below_and_a_variant_that_does_not_fit_leaves_the_effect_out() {
    let wgsl = "@fragment fn fs_lt_a(in: VertexOut) -> @location(0) vec4<f32> { return tap(in.uv); }";
    let passes = json!([{ "entry": "fs_lt_a" }, { "entry": "fs_lt_a" }]);
    let early = probe(
        json!({ "id": "lt_early", "passes": passes, "cost": { "taps": 4, "passes": 2 },
            "lighter": { "passes": [{ "entry": "fs_lt_a" }], "cost": { "taps": 1 }, "up_to": "q2" } }),
        wgsl,
    );
    assert_eq!(forms(early, false), [Form::Lighter, Form::Lighter, Form::Lighter, Form::Full]);
    let big = probe(
        json!({ "id": "lt_big", "passes": passes, "cost": { "taps": 8, "passes": 2 },
            "quality": { "q0": { "freeze": true } },
            "lighter": { "passes": [{ "entry": "fs_lt_a" }], "cost": { "taps": 3 } } }),
        wgsl,
    );
    assert_eq!(forms(big, false), [Form::Frozen, Form::Full, Form::Full, Form::Full]);
    assert_eq!(forms(big, true), [Form::Frozen, Form::Out, Form::Full, Form::Full]);
    let fused = json!({ "id": "lt_fused", "fuse": "uv", "cost": { "taps": 2 },
        "lighter": { "passes": [{ "entry": "fs_lt_a" }], "cost": { "taps": 1 } } });
    let fused =
        Descriptor::parse(&fused.to_string(), "fn uv_lt_fused(uv: vec2<f32>) -> vec2<f32> { return uv; }");
    let problems = fused.unwrap().problems();
    assert!(problems.iter().any(|p| p.contains("cannot belong to a fused effect")), "{problems:?}");
}

#[test]
fn the_cost_frames_and_hard_budget_follow_the_form_each_level_draws() {
    let s = scene(json!([{ "effect": "blur", "params": { "radius": 8 } }]));
    let plain = scene(json!([]));
    assert!(s.cost(Quality::Q0) > plain.cost(Quality::Q0));
    assert!(
        s.cost(Quality::Q3) - plain.cost(Quality::Q3) > 5.0 * (s.cost(Quality::Q0) - plain.cost(Quality::Q0))
    );
    let rt = Runtime::new(Arc::new(s.clone()), Values::new());
    let entries = |q| {
        rt.frame(VIEW, q)
            .effects
            .iter()
            .flat_map(|e| e.descriptor.passes.iter().map(|p| p.entry.clone()))
            .collect::<Vec<_>>()
    };
    assert_eq!(entries(Quality::Q0), ["fs_blur_lite", "fs_blur_lite_mix"]);
    assert_eq!(entries(Quality::Q3), ["fs_blur_h", "fs_blur_v"]);
    let spends = s.spends();
    assert_eq!((spends[0].units, spends[0].lighter), (6.0, Some(1.0)));
    let wide = [0.0, 0.0, 64.0, 32.0];
    let heavy = Spend { units: 4.0e9, lighter: Some(1.0), ..spends[0].clone() };
    let cut = over(std::slice::from_ref(&heavy), |_| false, Tier::T0, wide, 64.0 * 32.0);
    assert!(cut.len() == 1 && cut[0].lighter, "{cut:?}");
    assert!(
        cut[0].note("blur on l").starts_with("the effect blur on l was drawn with its lighter variant: ")
    );
    let none = Spend { lighter: None, ..heavy };
    let cut = over(&[none], |_| false, Tier::T0, wide, 64.0 * 32.0);
    assert!(!cut[0].lighter && cut[0].note("blur on l").starts_with("the effect blur on l was left out: "));
}
