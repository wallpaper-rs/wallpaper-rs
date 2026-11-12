use super::*;

#[test]
fn sway_bends_the_top_and_swing_turns_around_the_pivot() {
    let input = Shot(ramp());
    let Some(out) = Fx::new("sway", json!({ "amplitude": 8 })).input(ramp()).shot() else { return };
    out.check(&[((10, 31), input.at(10, 31)), ((40, 31), input.at(40, 31))]);
    assert!((0..W).any(|x| out.at(x, 0)[0].abs_diff(input.at(x, 0)[0]) > 12));
    let swing = json!({ "swing": true, "angle": 20, "speed": 0.25 });
    let Some(mut run) = Fx::new("sway", swing).input(ramp()).run() else { return };
    run.runtime.advance(1.0);
    let out = run.shot();
    out.check(&[((32, 0), input.at(32, 0))]);
    assert!(out.at(32, 31)[0].abs_diff(input.at(32, 31)[0]) > 20, "{:?}", out.at(32, 31));
}

#[test]
fn foliage_trembles_every_row_along_both_directions() {
    let input = Shot(squares());
    let Some(mut run) = Fx::new("sway", json!({ "amplitude": 3, "foliage": true, "speed": 0.7 })).input(squares()).run()
    else {
        return;
    };
    let mut shots = Vec::new();
    for _ in 0..3 {
        run.runtime.advance(0.29);
        shots.push(run.shot());
    }
    let row = |s: &Shot, y: u32| (0..W).filter(|&x| s.at(x, y) != input.at(x, y)).count();
    assert!(shots.iter().any(|s| row(s, H - 2) > 4), "the bottom row stays still");
    assert!(shots.windows(2).any(|w| w[0].diff(&w[1]) > 1.0));
    let tall = json!({ "amplitude": 3, "foliage": true, "ratio": 0.1 });
    let rows = RgbaImage::from_fn(W, H, |_, y| Rgba([(y * 255 / (H - 1)) as u8, 0, 0, 255]));
    let Some(mut run) = Fx::new("sway", tall).input(rows.clone()).run() else { return };
    run.runtime.advance(0.4);
    assert!(run.shot().diff(&Shot(rows)) > 0.2);
}
