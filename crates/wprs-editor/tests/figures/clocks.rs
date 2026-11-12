use serde_json::json;
use std::path::Path;

fn ask(q: serde_json::Value) -> serde_json::Value {
    wprs_editor::rigs::query(Path::new("."), &json!({}), "", &q)
}

const WALL: f64 = 20_000.0 * 86_400.0 + 13.0 * 3600.0 + 5.0 * 60.0 + 9.0;

#[test]
fn a_format_shows_a_live_sample_of_the_same_formatter_the_scene_uses() {
    let found = ask(json!({"clock_sample": {"pattern": "HH:mm:ss", "wall": WALL}}));
    assert_eq!(found["text"], "13:05:09");
    let found = ask(json!({"clock_sample": {"pattern": "hh:mm a", "wall": WALL}}));
    assert_eq!(found["text"], "01:05 PM");
    let found = ask(
        json!({"clock_sample": {"pattern": "EEEE d 'de' MMMM yyyy", "dates": true, "wall": WALL, "lang": "es"}}),
    );
    assert_eq!(found["text"], "viernes 4 de octubre 2024");
    let plain = ask(json!({"clock_sample": {"pattern": "EEEE", "wall": WALL}}));
    assert_eq!(plain["text"], "EEEE");
}

#[test]
fn the_help_lists_every_token_with_its_sample() {
    let found = ask(json!({"clock_help": {"wall": WALL}}));
    let tokens = found["tokens"].as_array().unwrap();
    let sample = |t: &str| tokens.iter().find(|r| r["token"] == t).unwrap()["sample"].clone();
    assert_eq!(tokens.len(), 21);
    assert_eq!(sample("HH"), "13");
    assert_eq!(sample("h"), "1");
    assert_eq!(sample("a"), "PM");
    assert_eq!(sample("yyyy"), "2024");
    assert_eq!(sample("MMM"), "Oct");
    assert_eq!(tokens.iter().filter(|r| r["dates"] == true).count(), 12);
    let numbers = found["numbers"].as_array().unwrap();
    let number = |n: &str| numbers.iter().find(|r| r["name"] == n).unwrap()["sample"].clone();
    assert_eq!(number("hour"), 13.0);
    assert_eq!(number("minute"), 5.0);
    assert_eq!(number("second"), 9.0);
    assert_eq!(number("weekday"), 5.0);
    assert_eq!(number("month"), 10.0);
}
