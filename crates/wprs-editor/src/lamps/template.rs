use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use wprs_core::message::said;
use wprs_scene::Beam;

pub fn template(doc: &Value, id: &str, source: Option<&str>) -> Result<Value> {
    let beam: Beam = match source {
        None => Beam::Point,
        Some(s) => serde_json::from_value(json!(s)).map_err(|_| {
            anyhow!(said(
                "editor.light_type",
                "%1 is not a light type; use point, spot, tube or directional",
                &[&s]
            ))
        })?,
    };
    let size = &doc["scene"]["canvas"]["size"];
    let middle = [0, 1].map(|i| size[i].as_f64().unwrap_or([1920.0, 1080.0][i]) / 2.0);
    let mut l = json!({ "id": id, "type": "light", "light": beam, "transform": { "position": middle } });
    let extra = match beam {
        Beam::Point => json!({ "radius": 400, "z": 120 }),
        Beam::Spot => json!({ "radius": 900, "z": 150, "intensity": 2, "cone": [25, 45] }),
        Beam::Tube => json!({ "radius": 240, "z": 60, "length": 400 }),
        Beam::Directional => json!({ "intensity": 0.8 }),
    };
    l.as_object_mut().into_iter().for_each(|m| m.extend(extra.as_object().cloned().unwrap_or_default()));
    if matches!(beam, Beam::Spot | Beam::Directional) {
        l["transform"]["tilt"] = json!([if beam == Beam::Spot { 45 } else { 50 }, 0]);
    }
    Ok(l)
}
