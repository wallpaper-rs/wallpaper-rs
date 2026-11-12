use super::*;

#[test]
fn shake_is_still_by_default_and_moves_when_enabled() {
    let Some(mut off) = Fx::new("shake", json!({})).input(ramp()).run() else { return };
    off.runtime.advance(0.3);
    assert!(off.shot().diff(&Shot(ramp())) < 0.5);
    let Some(mut on) = Fx::new("shake", json!({ "strength": 6 })).input(ramp()).run() else { return };
    let mut shots = Vec::new();
    for _ in 0..4 {
        on.runtime.advance(0.37);
        shots.push(on.shot());
    }
    assert!(shots.windows(2).any(|w| w[0].diff(&w[1]) > 1.0));
    assert!(shots.iter().all(|s| s.at(32, 16)[0].abs_diff(Shot(ramp()).at(32, 16)[0]) <= 26));
}

#[test]
fn shake_follows_a_flow_map_and_keeps_to_one_side() {
    let flow = RgbaImage::from_fn(W, H, |x, _| Rgba(if x < W / 2 { [255, 128, 0, 255] } else { [128, 128, 0, 255] }));
    let input = Shot(ramp());
    let pushed = |side: u32| {
        let params = json!({ "strength": 4, "speed": 0.5, "motion": 1, "side": side, "flow": "flow" });
        let mut run = Fx::new("shake", params).input(ramp()).image("flow", flow.clone(), true).run()?;
        Some((0..5).map(|_| (run.runtime.advance(0.3), run.shot()).1).collect::<Vec<_>>())
    };
    let Some(forward) = pushed(1) else { return };
    let Some(backward) = pushed(2) else { return };
    let start = i32::from(input.at(16, 16)[0]);
    let moved = |shots: &[Shot]| shots.iter().map(|s| i32::from(s.at(16, 16)[0]) - start).collect::<Vec<_>>();
    assert!(forward.iter().all(|s| s.at(48, 16) == input.at(48, 16)));
    let (ahead, behind) = (moved(&forward), moved(&backward));
    assert!(ahead.iter().all(|d| *d >= 0) && ahead.iter().any(|d| *d > 4), "{ahead:?}");
    assert!(behind.iter().all(|d| *d <= 0) && behind.iter().any(|d| *d < -4), "{behind:?}");
}
