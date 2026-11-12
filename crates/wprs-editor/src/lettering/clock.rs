use serde_json::{Value, json};
use std::time::{SystemTime, UNIX_EPOCH};

const TIME: [&str; 9] = ["HH", "H", "hh", "h", "mm", "m", "ss", "s", "a"];
const DATE: [&str; 12] = ["yyyy", "yy", "MMMM", "MMM", "MM", "M", "dd", "d", "EEEE", "EEE", "D", "w"];
const NUMBERS: [&str; 6] = ["day", "hour", "minute", "second", "weekday", "month"];

fn wall_of(q: &Value) -> f64 {
    q["wall"]
        .as_f64()
        .unwrap_or_else(|| SystemTime::now().duration_since(UNIX_EPOCH).map_or(0.0, |d| d.as_secs_f64()))
}

fn lang_of(q: &Value) -> &str {
    q["lang"].as_str().unwrap_or("en")
}

pub(super) fn sample(q: &Value) -> Value {
    let (wall, lang) = (wall_of(q), lang_of(q));
    let pattern = q["pattern"].as_str().unwrap_or_default();
    let dates = q["dates"].as_bool().unwrap_or(false);
    let (text, next) = wprs_scene::format_time(pattern, wall, lang, dates);
    json!({ "text": text, "next": next })
}

pub(super) fn help(q: &Value) -> Value {
    let (wall, lang) = (wall_of(q), lang_of(q));
    let row = |token: &str, dates: bool| {
        let (text, _) = wprs_scene::format_time(token, wall, lang, dates);
        json!({ "token": token, "dates": dates, "sample": text })
    };
    let tokens: Vec<Value> =
        TIME.iter().map(|t| row(t, false)).chain(DATE.iter().map(|t| row(t, true))).collect();
    let seconds = wall.rem_euclid(86_400.0);
    let days = wall.div_euclid(86_400.0) as i64;
    let numbers = NUMBERS.map(|name| {
        let value = match name {
            "day" => seconds / 86_400.0,
            "hour" => (seconds / 3600.0).floor(),
            "minute" => (seconds / 60.0).floor() % 60.0,
            "second" => seconds.floor() % 60.0,
            "weekday" => f64::from(((days + 3).rem_euclid(7) + 1) as i32),
            _ => wprs_scene::format_time("M", wall, lang, true).0.parse().unwrap_or(1.0),
        };
        json!({ "name": name, "sample": (value * 1000.0).round() / 1000.0 })
    });
    json!({ "tokens": tokens, "numbers": numbers, "quote": "'" })
}
