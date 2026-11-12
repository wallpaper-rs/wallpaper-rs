use serde_json::{Value, json};

pub const SCRIPT_TEMPLATES: [&str; 10] =
    ["empty", "media", "touch", "timer", "audio", "spawn", "wave", "daytime", "setting", "launches"];

pub fn script_template(name: &str) -> Option<&'static str> {
    match name {
        "empty" => Some(include_str!("molds/empty.js")),
        "media" => Some(include_str!("molds/media.js")),
        "touch" => Some(include_str!("molds/touch.js")),
        "timer" => Some(include_str!("molds/timer.js")),
        "audio" => Some(include_str!("molds/audio.js")),
        "spawn" => Some(include_str!("molds/spawn.js")),
        "wave" => Some(include_str!("molds/wave.js")),
        "daytime" => Some(include_str!("molds/daytime.js")),
        "setting" => Some(include_str!("molds/setting.js")),
        "launches" => Some(include_str!("molds/launches.js")),
        _ => None,
    }
}

pub fn script_params(name: &str) -> Option<Value> {
    Some(match name {
        "spawn" => json!({
            "count": { "type": "number", "default": 8, "min": 1, "max": 64 },
            "gap": { "type": "number", "default": 120, "min": 1, "max": 2000 },
            "template": { "type": "text", "default": "copy" }
        }),
        "wave" => json!({
            "speed": { "type": "number", "default": 1, "min": 0, "max": 20 },
            "size": { "type": "number", "default": 20, "min": 0, "max": 1000 }
        }),
        "setting" => json!({ "prop": { "type": "text", "default": "" } }),
        _ => return None,
    })
}

pub(crate) fn free_name(dir: &std::path::Path, template: &str) -> String {
    let taken = |name: &String| dir.join("scripts").join(name).exists();
    let names = std::iter::once(format!("{template}.js")).chain((2..).map(|n| format!("{template}-{n}.js")));
    names.take(1000).find(|n| !taken(n)).unwrap_or_else(|| format!("{template}.js"))
}
