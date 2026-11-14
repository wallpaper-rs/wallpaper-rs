use anyhow::Result;
use std::fs;
use std::path::Path;

pub const DESCRIPTOR: &str = "transitions/opening.json";
const WGSL: &str = "transitions/opening.wgsl";

const JSON_TEXT: &str = r##"{
  "id": "opening",
  "name": { "en": "Soft reveal", "es": "Apertura suave" },
  "stage": "transition",
  "cost": { "taps": 1, "passes": 1, "scale": 1 },
  "params": [
    { "id": "center", "type": "vec2", "default": [0.5, 0.5], "label": { "en": "Center", "es": "Centro" } },
    { "id": "edge", "type": "float", "default": 0.2, "min": 0.01, "max": 1, "label": { "en": "Edge", "es": "Borde" } }
  ]
}
"##;

const WGSL_TEXT: &str = "fn transition(c: Crossing) -> Crossing {
    var o = c;
    let center = param(0u).xy;
    let edge = max(param(1u).x, 0.01);
    let aspect = vec2<f32>(c.resolution.x / max(c.resolution.y, 1.0), 1.0);
    let far = length(max(center, 1.0 - center) * aspect);
    let d = length((c.uv - center) * aspect) / max(far, 0.001);
    let r = c.progress * (1.0 + edge);
    let k = smoothstep(r - edge, r, d);
    o.color = vec4<f32>(mix(after(c.uv).rgb, before(c.uv).rgb, k), 1.0);
    return o;
}
";

pub fn write(dir: &Path) -> Result<String> {
    for (name, text) in [(DESCRIPTOR, JSON_TEXT), (WGSL, WGSL_TEXT)] {
        let file = dir.join(name);
        if !file.exists() {
            fs::create_dir_all(file.parent().unwrap_or(dir))?;
            fs::write(&file, text)?;
        }
    }
    Ok(DESCRIPTOR.to_owned())
}
