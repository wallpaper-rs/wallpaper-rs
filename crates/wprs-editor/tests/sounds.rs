use serde_json::{Value, json};
use std::path::Path;
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::rigs::{preview, query};
use wprs_editor::sounds::{LISTEN, peaks};

fn wav(path: &Path, rate: u32, samples: &[i16]) {
    let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
    let mut out = b"RIFF".to_vec();
    out.extend((36 + data.len() as u32).to_le_bytes());
    out.extend(b"WAVEfmt ");
    out.extend(16u32.to_le_bytes());
    out.extend([1, 0, 1, 0]);
    out.extend(rate.to_le_bytes());
    out.extend((rate * 2).to_le_bytes());
    out.extend([2, 0, 16, 0]);
    out.extend(b"data");
    out.extend((data.len() as u32).to_le_bytes());
    out.extend(data);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, out).unwrap();
}

fn scene() -> Value {
    json!({ "scene": {
        "assets": { "wind": { "path": "sounds/wind.wav" }, "rain": { "path": "sounds/rain.wav" } },
        "layers": [
            { "id": "gusts", "type": "sound", "sound": "wind", "play_on": "event:gust", "fade_in": 2 },
            { "id": "box", "type": "group", "layers": [{ "id": "drops", "type": "sound", "sounds": ["rain", "wind"], "mode": "random" }] },
            { "id": "pic", "type": "solid" }
        ]
    } })
}

#[test]
fn the_sounds_of_the_scene_are_listed_with_their_triggers_and_fades() {
    let dir = tempfile::tempdir().unwrap();
    wav(&dir.path().join("sounds/wind.wav"), 8000, &[0; 80]);
    let found = query(dir.path(), &scene(), "", &json!({ "sounds": {} }));
    let layers = found["layers"].as_array().unwrap();
    assert_eq!(layers.len(), 2);
    assert_eq!(
        (layers[0]["id"].clone(), layers[0]["play_on"].clone(), layers[0]["fade_in"].clone()),
        (json!("gusts"), json!("event:gust"), json!(2.0))
    );
    assert_eq!(
        (layers[1]["sounds"].clone(), layers[1]["pointer"].clone()),
        (json!(["rain", "wind"]), json!("/scene/layers/1/layers/0"))
    );
    let assets = found["assets"].as_array().unwrap();
    assert_eq!(assets.len(), 2);
    assert!(assets.iter().any(|a| a["name"] == "wind" && a["bytes"].as_u64().unwrap() > 100));
}

#[test]
fn listening_plays_one_sound_once_and_mutes_the_rest() {
    let mut doc = scene()["scene"].clone();
    preview(&mut doc, &json!({ LISTEN: "drops" }));
    assert_eq!(doc["layers"][0]["play"], json!(false));
    let drops = &doc["layers"][1]["layers"][0];
    assert_eq!((drops["play"].clone(), drops["loop"].clone()), (json!(true), json!(false)));
    assert!(doc["layers"][0].get("play_on").is_some());
    let mut doc = scene()["scene"].clone();
    preview(&mut doc, &json!({ LISTEN: { "asset": "rain" } }));
    let last = doc["layers"].as_array().unwrap().last().unwrap().clone();
    assert_eq!(
        (last["type"].clone(), last["sound"].clone(), last["loop"].clone()),
        (json!("sound"), json!("rain"), json!(false))
    );
    assert_eq!(doc["layers"][0]["play"], json!(false));
    let mut untouched = scene()["scene"].clone();
    preview(&mut untouched, &json!({ "quality": "q1" }));
    assert_eq!(untouched, scene()["scene"]);
}

#[test]
fn peaks_follow_the_loudness_of_each_stretch() {
    let samples: Vec<i16> = (0..4800).map(|i| if i < 2400 { 3000 } else { -24000 }).collect();
    let clip = wprs_video::Clip { channels: 1, samples };
    let found = peaks(&clip, 4);
    let heights: Vec<f64> = found["peaks"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
    assert!(heights[0] < 0.1 && heights[1] < 0.1 && heights[2] > 0.7 && heights[3] > 0.7, "{heights:?}");
    assert!((found["seconds"].as_f64().unwrap() - 0.1).abs() < 1e-9);
    assert_eq!(peaks(&clip, 1_000_000)["peaks"].as_array().unwrap().len(), 2048);
}

fn cache() {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_CACHE_HOME", dir.path()) };
        dir
    });
}

#[test]
fn the_waveform_of_a_sound_is_read_once_and_kept() {
    cache();
    let dir = tempfile::tempdir().unwrap();
    let samples: Vec<i16> = (0..16000)
        .map(|i| ((i as f64 * 0.3).sin() * if i < 8000 { 2000.0 } else { 20000.0 }) as i16)
        .collect();
    let file = dir.path().join("sounds/wind.wav");
    wav(&file, 16000, &samples);
    let ask = || query(dir.path(), &scene(), "", &json!({ "wave": { "asset": "wind", "buckets": 8 } }));
    let first = ask();
    assert!(first["error"].is_null(), "{first}");
    let peaks: Vec<f64> = first["peaks"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
    assert!(peaks[0] < 0.2 && peaks[7] > 0.5, "{peaks:?}");
    assert!((first["seconds"].as_f64().unwrap() - 1.0).abs() < 0.1);
    std::fs::write(&file, b"gone").ok();
    wav(&file, 16000, &samples);
    assert_eq!(ask(), first);
    let missing = query(dir.path(), &scene(), "", &json!({ "wave": { "asset": "nothing" } }));
    assert!(missing["error"].is_string());
}
