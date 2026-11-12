use anyhow::{Result, bail};
use serde_json::{Value, json};
use wprs_core::message::said;

pub const KITS: [&str; 4] = ["snow", "sparks", "rain", "dust"];

pub fn size(doc: &Value) -> [f64; 2] {
    let side = |i: usize| doc.pointer(&format!("/scene/canvas/size/{i}")).and_then(Value::as_f64);
    [side(0).unwrap_or(1920.0), side(1).unwrap_or(1080.0)]
}

fn fall(h: f64) -> Value {
    json!([{"bounds": {"plane": {"point": [0, h + 60.0], "normal": [0, -1]}}, "kill": true}])
}

pub fn kit(name: &str, [w, h]: [f64; 2]) -> Result<Value> {
    Ok(match name {
        "snow" => json!({"capacity": 3000, "prewarm": 14,
            "emitters": [{"shape": {"box": [[-100, -50], [w + 100.0, -20]]}, "rate": 90}],
            "init": {"life": [12, 18], "speed": [40, 80], "angle": [80, 100], "size": [3, 8],
                "color": ["#ffffff", "#dce6f4"]},
            "forces": [{"gravity": [8, 14]}, {"turbulence": {"strength": 16, "scale": 0.004}}, {"drag": 0.2}],
            "collide": fall(h),
            "render": {"color_over_life": [[0, "#ffffff00"], [0.08, "#ffffffff"], [0.9, "#ffffffff"], [1, "#ffffff00"]]}}),
        "sparks" => json!({"capacity": 1500, "prewarm": 3,
            "emitters": [{"shape": {"box": [[w * 0.35, h - 20.0], [w * 0.65, h]]}, "rate": 60}],
            "init": {"life": [1.5, 3], "speed": [180, 420], "angle": [240, 300], "size": [2, 5],
                "color": ["#ffd27a", "#ff7a1a"]},
            "forces": [{"gravity": [0, 160]}, {"turbulence": {"strength": 60, "scale": 0.006}}, {"drag": 0.3}],
            "render": {"color_over_life": [[0, "#ffffffff"], [0.5, "#ff9a30ff"], [1, "#ff300000"]]}}),
        "rain" => json!({"capacity": 4000, "prewarm": 3,
            "emitters": [{"shape": {"box": [[-200, -60], [w + 200.0, -20]]}, "rate": 400}],
            "init": {"life": [1.2, 1.8], "speed": [900, 1200], "angle": [78, 84], "size": [2, 3],
                "color": ["#a8c4e8", "#7fa3d1"]},
            "collide": fall(h),
            "render": {"stretch": 4}}),
        "dust" => json!({"capacity": 800, "prewarm": 12,
            "emitters": [{"shape": {"box": [[0, 0], [w, h]]}, "rate": 20}],
            "init": {"life": [8, 14], "speed": [3, 12], "angle": [0, 360], "size": [2, 5], "alpha": [0.3, 0.7],
                "color": ["#ffffff", "#ffe8c8"]},
            "forces": [{"turbulence": {"strength": 10, "scale": 0.002}}],
            "render": {"color_over_life": [[0, "#ffffff00"], [0.2, "#ffffffff"], [0.8, "#ffffffff"], [1, "#ffffff00"]]}}),
        other => bail!(said(
            "editor.no_kit",
            "%1 is not a starting kit; use one of %2",
            &[&other, &(KITS.join(", "))]
        )),
    })
}
