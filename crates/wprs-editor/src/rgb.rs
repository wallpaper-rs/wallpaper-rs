use crate::action::RgbEdit;
use crate::ops::{Op, diff};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use wprs_core::message::said;

fn lit(layers: &Value) -> bool {
    layers.as_array().into_iter().flatten().any(|l| l["rgb"] == true || lit(&l["layers"]))
}

pub(crate) fn kept(doc: &Value, pointer: &str) -> bool {
    pointer == "/manifest/permissions/rgb" && crate::pages::scened(doc)
}

fn permitted(doc: &Value) -> bool {
    doc["manifest"]["permissions"]["rgb"] == true
}

fn grant(m: &mut serde_json::Map<String, Value>, wanted: bool) {
    let permissions = m.entry("permissions").or_insert_with(|| json!({}));
    match (wanted, permissions.as_object_mut()) {
        (true, Some(p)) => drop(p.insert("rgb".into(), json!(true))),
        (false, Some(p)) => drop(p.remove("rgb")),
        _ => {}
    }
    if permissions.as_object().is_some_and(serde_json::Map::is_empty) {
        m.remove("permissions");
    }
}

pub(crate) fn permission(doc: &Value) -> Vec<Op> {
    let wanted = lit(&doc["scene"]["layers"]);
    if !crate::pages::scened(doc) || wanted == permitted(doc) {
        return Vec::new();
    }
    let mut manifest = doc["manifest"].clone();
    if let Some(m) = manifest.as_object_mut() {
        grant(m, wanted);
    }
    diff(&doc["manifest"], &manifest, "/manifest")
}

pub(crate) fn act(doc: &Value, e: &RgbEdit) -> Result<Vec<Op>> {
    ensure!(
        e.on.is_some() || e.size.is_some() || e.whole,
        said("editor.rgb_nothing", "say what to change in %1: on, size or whole", &[&e.layer])
    );
    let (pointer, layer) = crate::actions::layer(doc, &e.layer)?;
    ensure!(
        layer["type"] == "adjustment",
        said("editor.rgb_layer", "%1 is not an adjustment layer", &[&e.layer])
    );
    let canvas = doc.pointer("/scene/canvas/size").and_then(Value::as_array).cloned().unwrap_or_default();
    let most = |i: usize| canvas.get(i).and_then(Value::as_f64).unwrap_or(1920.0) * 2.0;
    let mut new = doc.clone();
    let target = new.pointer_mut(&pointer).and_then(Value::as_object_mut);
    let Some(target) = target else { return Ok(Vec::new()) };
    if let Some(on) = e.on {
        target.insert("rgb".into(), json!(on));
    }
    if let Some([w, h]) = e.size {
        let fits = |v: f64, i: usize| v.is_finite() && (1.0..=most(i)).contains(&v);
        ensure!(
            fits(w, 0) && fits(h, 1),
            said("editor.rgb_size", "a region is from 1 pixel to twice the canvas, not %1 × %2", &[&w, &h])
        );
        target.insert("size".into(), json!([w, h]));
        let transform = target.entry("transform").or_insert_with(|| json!({}));
        if let Some(t) = transform.as_object_mut().filter(|t| !t.contains_key("position")) {
            t.insert("position".into(), json!([most(0) / 4.0, most(1) / 4.0]));
        }
    }
    if e.whole {
        target.remove("size");
    }
    let wanted = lit(&new["scene"]["layers"]);
    if let Some(m) = new["manifest"].as_object_mut().filter(|_| wanted || permitted(doc)) {
        grant(m, wanted);
    }
    Ok(diff(doc, &new, ""))
}
