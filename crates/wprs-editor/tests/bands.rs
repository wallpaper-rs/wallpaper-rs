use serde_json::{Value, json};
use wprs_editor::bands::query;

fn pick(q: Value) -> Value {
    query(&q)
}

#[test]
fn the_layout_tells_the_pitch_of_every_band_at_each_resolution() {
    let full = query(&json!({}));
    assert_eq!(full["of"], 64);
    let bands = full["bands"].as_array().unwrap();
    assert_eq!(bands.len(), 64);
    assert_eq!(bands[0]["from"], json!(40.0));
    assert!((bands[63]["to"].as_f64().unwrap() - 16000.0).abs() < 1.0);
    let coarse = query(&json!({ "of": 16 }));
    assert_eq!(coarse["bands"].as_array().unwrap().len(), 16);
    assert_eq!(coarse["bands"][0]["to"], bands[3]["to"]);
    assert!(query(&json!({ "of": 20 }))["error"].is_string());
    let named = &coarse["named"];
    assert_eq!(named["bass"][0], 0);
    assert_eq!(named["treble"][1], 15);
    assert_eq!(named["mid"][0].as_u64(), named["bass"][1].as_u64().map(|b| b + 1));
    assert_eq!(named["treble"][0].as_u64(), named["mid"][1].as_u64().map(|m| m + 1));
}

#[test]
fn a_band_or_a_range_becomes_a_source_that_keeps_the_shaping() {
    let base = json!({ "audio": "bass", "scale": 2, "offset": 1, "clamp": [0, 1] });
    let named = pick(json!({ "pick": "treble", "base": base }));
    assert_eq!(named["source"], json!({ "audio": "treble", "scale": 2, "offset": 1, "clamp": [0, 1] }));
    let one = pick(json!({ "pick": 3, "of": 16, "channel": "left" }));
    assert_eq!(one["source"], json!({ "audio": 3, "of": 16, "channel": "left" }));
    assert!(one["hertz"][0].as_f64().unwrap() < one["hertz"][1].as_f64().unwrap());
    let range = pick(json!({ "pick": [0, 3], "of": 16, "reduce": "max" }));
    assert_eq!(range["source"], json!({ "audio": [0, 3], "of": 16, "reduce": "max" }));
    assert_eq!(range["label"], "0-3");
    let plain =
        pick(json!({ "pick": [2, 5], "reduce": "mean", "base": { "audio": "bass", "reduce": "max" } }));
    assert_eq!(plain["source"], json!({ "audio": [2, 5] }));
}

#[test]
fn a_pointer_direction_replaces_the_source_and_bad_picks_are_refused() {
    let moved = pick(json!({ "direction": "speed", "base": { "audio": 3, "of": 16, "invert": true } }));
    assert_eq!(moved["source"], json!({ "cursor": "speed", "invert": true }));
    for bad in [
        json!({ "direction": "z" }),
        json!({ "pick": "volume" }),
        json!({ "pick": 16, "of": 16 }),
        json!({ "pick": [5, 2] }),
        json!({ "pick": [0, 1], "reduce": "median" }),
        json!({ "pick": 1, "channel": "rear" }),
        json!({ "pick": true }),
    ] {
        assert!(pick(bad.clone())["error"].is_string(), "{bad}");
    }
}
