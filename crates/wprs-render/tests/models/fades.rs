use super::*;

fn key(t: f32, x: f32) -> Value {
    json!({ "t": t, "eye": [x, 0, 5], "center": [x, 0, 0] })
}

fn two(fade: &str) -> Scene {
    let models = BTreeMap::from([("red", cube(unlit("paint", [1.0, 0.0, 0.0, 1.0])))]);
    let paths = json!([{ "id": "a", "keys": [key(0.0, 0.0), key(2.0, 0.0)] },
        { "id": "b", "keys": [key(0.0, 1.5), key(2.0, 1.5)] }]);
    let camera = json!({ "paths": paths, "fade": { "to": fade, "duration": 1 } });
    let layers = json!([{ "id": "floor", "type": "solid", "color": "#00ff00" },
        { "id": "cube", "type": "model", "model": "red", "scale": 0.5 }]);
    scene(json!({ "camera3d": camera }), layers, models)
}

#[test]
fn the_fade_to_black_darkens_the_whole_scene_not_only_the_models() {
    let Some(r) = renderer() else { return };
    let mut shot = shoot(r, two("black"), &[]);
    let lit = shot.frame(r, 1.0, Quality::Q3);
    assert!(rgb(&lit, 2, 2)[1] > 200, "the flat layer shows between fades");
    let half = shot.frame(r, 0.75, Quality::Q3);
    let g = rgb(&half, 2, 2)[1];
    assert!((60..200).contains(&g), "half way the flat layer is half dark: {g}");
    let dark = shot.frame(r, 0.25, Quality::Q3);
    assert!(rgb(&dark, 2, 2)[1] < 10 && rgb(&dark, 32, 32)[0] < 10, "the switch is black everywhere");
    let mut q0 = shoot(r, two("black"), &[]);
    let dark = q0.frame(r, 2.0, Quality::Q0);
    assert!(rgb(&dark, 2, 2)[1] < 10, "the lowest quality fades the same");
}

#[test]
fn a_cross_fade_renders_the_still_next_path_once() {
    let Some(r) = renderer() else { return };
    let mut shot = shoot(r, two("cross"), &[]);
    shot.frame(r, 1.2, Quality::Q3);
    let before = shot.view.model_renders();
    for _ in 0..3 {
        shot.frame(r, 0.2, Quality::Q3);
    }
    let shown = shot.frame(r, 0.1, Quality::Q3);
    assert_eq!(shot.view.cross_renders(), 1, "the start of the next path is kept");
    assert!(shot.view.model_renders() >= before + 3, "the path that plays renders each frame");
    let floor = rgb(&shown, 2, 2)[1];
    assert!(floor > 200, "a cross fade keeps the flat layers: {floor}");
}

#[test]
fn moving_paths_ask_for_half_the_frames_at_the_lowest_quality() {
    let Some(r) = renderer() else { return };
    let needed = |q: Quality| {
        let bench = wprs_render::Bench {
            frames: 40,
            size: (SIDE, SIDE),
            level: wprs_render::Level::Fixed(q),
            ..Default::default()
        };
        let runtime = Runtime::new(Arc::new(two("cross")), Values::new());
        let report = wprs_render::scene_bench(r, runtime, Default::default(), &bench).unwrap();
        report["needed"].as_u64().unwrap()
    };
    let (low, high) = (needed(Quality::Q0), needed(Quality::Q3));
    assert_eq!(high, 40);
    assert!((19..=21).contains(&low), "at most 30 a second of 60: {low}");
}

#[test]
fn timed_passes_name_the_passes_of_a_3d_stage() {
    let Some(r) = renderer() else { return };
    let bench = wprs_render::Bench { frames: 64, size: (SIDE, SIDE), passes: true, ..Default::default() };
    let runtime = Runtime::new(Arc::new(two("cross")), Values::new());
    let report = wprs_render::scene_bench(r, runtime, Default::default(), &bench).unwrap();
    let passes = report["pass_ms"]["passes"].as_array().cloned().unwrap_or_default();
    if passes.is_empty() {
        eprintln!("skipping: the device times no passes");
        return;
    }
    let named = |what: &str| passes.iter().any(|p| p[0].as_str().is_some_and(|l| l.starts_with(what)));
    assert!(named("3d models"), "{passes:?}");
    assert!(passes.iter().all(|p| p[1].as_f64().is_some_and(|ms| ms >= 0.0)), "{passes:?}");
}
