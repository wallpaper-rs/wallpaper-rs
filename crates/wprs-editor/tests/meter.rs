use serde_json::{Value, json};
use wprs_editor::rigs::query;

fn ask(doc: &Value, q: Value) -> Value {
    query(std::path::Path::new("."), doc, "", &q)
}

fn reading(left: &[(usize, f64)], right: &[(usize, f64)]) -> Value {
    let channel = |marks: &[(usize, f64)]| {
        let mut bands = vec![0.0; 64];
        marks.iter().for_each(|(i, v)| bands[*i] = *v);
        bands
    };
    json!({
        "granted": true,
        "listening": true,
        "raw": [channel(left), channel(right)],
        "loud": [channel(&[]), channel(&[])],
        "loudness": 0.4,
        "beat": true,
        "silent": false,
    })
}

#[test]
fn a_range_of_bands_is_marked_and_read_with_the_reduction_of_the_source() {
    let doc = json!({ "scene": {} });
    let heard = reading(&[(0, 0.8), (1, 0.2)], &[(0, 0.8), (1, 0.2)]);
    let mean = ask(
        &doc,
        json!({ "meter": { "source": { "audio": [0, 3], "of": 16 }, "reading": heard, "of": 16 } }),
    );
    assert_eq!(mean["marked"], json!([0, 3]));
    assert_eq!(mean["bands"].as_array().unwrap().len(), 16);
    assert!((mean["value"].as_f64().unwrap() - 0.0625).abs() < 1e-6, "{mean}");
    let max = ask(
        &doc,
        json!({ "meter": { "source": { "audio": [0, 3], "of": 16, "reduce": "max" }, "reading": heard, "of": 16 } }),
    );
    assert!((max["value"].as_f64().unwrap() - 0.8).abs() < 1e-6, "{max}");
}

#[test]
fn the_channel_the_shaping_and_the_names_decide_the_value() {
    let doc = json!({ "scene": {} });
    let heard = reading(&[(5, 1.0)], &[]);
    let right =
        ask(&doc, json!({ "meter": { "source": { "audio": 5, "channel": "right" }, "reading": heard } }));
    assert_eq!(right["value"], json!(0.0));
    let left = ask(
        &doc,
        json!({ "meter": { "source": { "audio": 5, "channel": "left", "scale": 2.0 }, "reading": heard } }),
    );
    assert_eq!(left["value"], json!(2.0));
    let beat = ask(&doc, json!({ "meter": { "source": { "audio": "beat" }, "reading": heard } }));
    assert_eq!((beat["value"].clone(), beat["marked"].clone()), (json!(1.0), Value::Null));
    let bass = ask(&doc, json!({ "meter": { "source": { "audio": "bass" }, "reading": heard } }));
    assert!(bass["marked"][1].as_u64().unwrap() > 0);
    assert_eq!(bass["marked"][0], json!(0));
}

#[test]
fn nothing_is_drawn_without_a_reading_or_with_a_bad_source() {
    let doc = json!({ "scene": {} });
    let silent = json!({ "granted": false, "listening": false });
    let off = ask(&doc, json!({ "meter": { "source": { "audio": "bass" }, "reading": silent } }));
    assert_eq!(
        (off["granted"].clone(), off["listening"].clone(), off["bands"].clone()),
        (json!(false), json!(false), Value::Null)
    );
    assert!(ask(&doc, json!({ "meter": { "source": { "audio": "bass" }, "reading": silent, "of": 20 } }))["error"].is_string());
    assert!(
        ask(&doc, json!({ "meter": { "source": { "nonsense": 1 }, "reading": silent } }))["error"]
            .is_string()
    );
}

#[test]
fn the_fixed_gain_of_the_scene_reads_the_loud_bands() {
    let doc = json!({ "scene": { "audio": { "gain": "fixed" } } });
    let mut heard = reading(&[(2, 1.0)], &[(2, 1.0)]);
    heard["loud"][0][2] = json!(0.3);
    heard["loud"][1][2] = json!(0.3);
    let seen = ask(&doc, json!({ "meter": { "source": { "audio": 2 }, "reading": heard } }));
    assert!((seen["value"].as_f64().unwrap() - 0.3).abs() < 1e-6, "{seen}");
}

#[test]
fn an_open_capture_without_sound_draws_no_levels() {
    let doc = json!({ "scene": {} });
    let quiet = json!({ "granted": true, "listening": true, "sound": false });
    let out = ask(&doc, json!({ "meter": { "source": { "audio": "bass" }, "reading": quiet } }));
    assert_eq!(
        (out["granted"].clone(), out["listening"].clone(), out["bands"].clone()),
        (json!(true), json!(false), Value::Null)
    );
}
