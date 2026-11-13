use super::wanted;
use serde_json::json;
use wprs_scene::Quality;

#[test]
fn the_panel_asks_for_a_picture_or_a_clip_and_the_limits_hold() {
    let (one, versus) = wanted(&json!({}), &json!({}));
    assert_eq!(
        (one.frames, one.quality, one.battery, one.calm, one.pointer),
        (1, Quality::Q3, false, false, None)
    );
    assert!(versus.is_none());
    let (clip, versus) = wanted(
        &json!({
            "quality": "q1", "battery": true, "calm": true, "time": 2.5, "frames": 90, "fps": 30,
            "pointer": { "from": [0.2, 0.5], "to": [0.8, 0.5], "click": true },
            "against": "q0", "without_bloom": true, "split": 0.25, "wall": 1000.0
        }),
        &json!({ "manifest": { "default_locale": "es" } }),
    );
    assert_eq!((clip.quality, clip.frames, clip.fps, clip.time), (Quality::Q1, 90, 30, 2.5));
    assert!(clip.battery && clip.calm);
    assert_eq!((clip.wall, clip.lang.as_str()), (1000.0, "es"));
    assert!(one.wall > 1.5e9 && one.lang == "en", "the clock of a picture is the real one by default");
    let trace = clip.pointer.unwrap();
    assert_eq!((trace.from, trace.to, trace.click), ([0.2, 0.5], [0.8, 0.5], true));
    let versus = versus.unwrap();
    assert_eq!((versus.quality, versus.without_bloom, versus.split), (Some(Quality::Q0), true, 0.25));
    let (wild, _) = wanted(
        &json!({ "frames": 100000, "fps": 0, "size": 1, "time": -4, "pointer": { "from": [3, -1] } }),
        &json!({}),
    );
    assert_eq!((wild.frames, wild.fps, wild.edge, wild.time), (600, 1, 16, 0.0));
    assert_eq!(wild.pointer.unwrap().from, [1.0, 0.0]);
}

#[test]
fn the_panel_sets_the_clock_by_the_time_of_day_and_an_explicit_wall_wins() {
    let (day, _) = wanted(&json!({ "time_of_day": 15.0 * 3600.0 + 1800.0 }), &json!({}));
    assert_eq!((day.wall % 86_400.0, day.wall > 1.5e9), (55_800.0, true));
    let (fixed, _) = wanted(&json!({ "time_of_day": 3600.0, "wall": 42.0 }), &json!({}));
    assert_eq!(fixed.wall, 42.0);
    let (late, _) = wanted(&json!({ "time_of_day": 1e9 }), &json!({}));
    assert_eq!(late.wall % 86_400.0, 86_399.0);
}
