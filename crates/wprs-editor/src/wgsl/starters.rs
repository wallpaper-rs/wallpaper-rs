use serde_json::{Value, json};

pub const TEMPLATES: [&str; 5] = ["color", "distort", "passes", "material", "particle"];

const COLOR: &str = include_str!("starters/color.wgsl");

const DISTORT: &str = include_str!("starters/distort.wgsl");

const PASSES: &str = include_str!("starters/passes.wgsl");

const MATERIAL: &str = include_str!("starters/material.wgsl");

const PARTICLE: &str = include_str!("starters/particle.wgsl");

fn float(id: &str, default: f64, range: [f64; 2]) -> Value {
    json!({ "id": id, "type": "float", "default": default, "min": range[0], "max": range[1] })
}

pub fn starter(name: &str, template: &str) -> Option<(Value, String)> {
    let (descriptor, wgsl) = match template {
        "color" => (
            json!({ "category": "color", "fuse": "color", "cost": { "taps": 1, "passes": 1, "scale": 1 },
                "params": [float("amount", 1.0, [0.0, 1.0]), { "id": "color", "type": "color", "default": "#ff8800" }] }),
            COLOR,
        ),
        "distort" => (
            json!({ "category": "distort", "fuse": "uv", "cost": { "taps": 1, "passes": 1, "scale": 1 },
                "params": [{ "id": "center", "type": "position", "default": [0.5, 0.5] },
                    float("strength", 0.3, [-1.0, 1.0]), float("radius", 0.25, [0.01, 1.0])] }),
            DISTORT,
        ),
        "passes" => (
            json!({ "category": "blur", "cost": { "taps": 9, "passes": 2, "scale": 1 },
                "params": [float("length", 8.0, [0.0, 64.0]),
                    { "id": "angle", "type": "float", "default": 0, "min": -180, "max": 180, "unit": "degrees" }],
                "passes": [{ "entry": "fs_NAME_soft", "scale": 0.5 },
                    { "entry": "fs_NAME", "inputs": ["pass:0", "input"] }] }),
            PASSES,
        ),
        "material" => (
            json!({ "stage": "material", "cost": {},
                "params": [{ "id": "tint", "type": "color", "default": "#ffffff" }, float("glow", 0.0, [0.0, 4.0]),
                    { "id": "picture", "type": "texture", "default": "white" }] }),
            MATERIAL,
        ),
        "particle" => (
            json!({ "stage": "particle", "cost": {}, "params": [float("pulse", 0.3, [0.0, 1.0])] }),
            PARTICLE,
        ),
        _ => return None,
    };
    let mut descriptor = descriptor;
    descriptor["id"] = json!(name);
    let text = serde_json::to_string(&descriptor).unwrap_or_default().replace("NAME", name);
    Some((serde_json::from_str(&text).unwrap_or(descriptor), wgsl.replace("NAME", name)))
}
