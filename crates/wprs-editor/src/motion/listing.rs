use crate::rigs::round;
use serde_json::{Value, json};
use std::path::Path;
use wprs_scene::Rigged;

pub fn root_path(rig: &Rigged, clip: &Value, laps: f32) -> Vec<[f32; 3]> {
    let Ok(clip) = serde_json::from_value::<wprs_scene::AnimClip>(clip.clone()) else { return Vec::new() };
    wprs_scene::clip_course(rig, &clip, laps)
}

pub fn motion(dir: &Path, doc: &Value, id: &str) -> Value {
    let Ok((pointer, l)) = super::figure(doc, id) else { return Value::Null };
    let Ok((_, _, rig)) = crate::skin::rigged(dir, doc, l) else { return Value::Null };
    let clips: Vec<Value> = l["clips"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|c| {
            let file = rig.motion(c["animation"].as_str()).map_or(0.0, |m| m.length);
            let from = c["from"].as_f64().unwrap_or(0.0);
            let to = c["to"].as_f64().unwrap_or(f64::from(file));
            let path: Vec<Value> = root_path(&rig, c, 2.0).iter().map(|p| json!(p.map(round))).collect();
            json!({ "id": c["id"], "animation": c["animation"], "from": from, "to": to, "length": round((to - from).max(0.0) as f32),
                "loop": c["loop"].as_str().unwrap_or("repeat"), "events": c["events"].as_array().cloned().unwrap_or_default(),
                "bones": c["bones"].as_object().map_or(0, |b| b.len()), "shapes": c["shapes"].as_object().map_or(0, |s| s.len()),
                "path": path })
        })
        .collect();
    let files: Vec<Value> =
        rig.motions.iter().map(|m| json!({ "name": m.name, "length": round(m.length) })).collect();
    let plays: Vec<Value> = l["animations"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(i, a)| {
            json!({ "index": i, "clip": a["clip"], "weight": a.get("weight").cloned().unwrap_or(json!(1)),
                "rate": a.get("rate").cloned().unwrap_or(json!(1)), "additive": a["additive"] == true,
                "blend_in": a["blend_in"].as_f64().unwrap_or(0.0), "blend_out": a["blend_out"].as_f64().unwrap_or(0.0),
                "enabled": a["enabled"] != false, "bones": a["bones"].as_array().cloned().unwrap_or_default() })
        })
        .collect();
    let bones: Vec<Value> = rig
        .joints
        .iter()
        .map(|j| json!({ "name": j.name, "parent": j.parent.map(|p| rig.joints[p as usize].name.clone()), "point": j.point }))
        .collect();
    let presets: Vec<&str> = super::PRESETS.iter().map(|p| p.0).collect();
    json!({ "layer": id, "pointer": pointer, "model": l["type"] == "model", "clips": clips, "files": files,
        "layers": plays, "bones": bones, "shapes": rig.shapes, "defaults": rig.defaults,
        "weights": l["shapes"].clone(), "root_motion": l["root_motion"] == true, "physics": l["physics"].clone(),
        "presets": presets })
}
