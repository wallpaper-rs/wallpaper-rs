use crate::action::Takes;
use crate::layers::walk;
use crate::ops::{Op, set};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use wprs_core::message::said;

const OPEN: f64 = 0.5;

pub fn state(layer: &Value) -> (Takes, f64) {
    match &layer["solid"] {
        Value::Bool(true) => (Takes::All, OPEN),
        Value::Object(o) => (Takes::Alpha, o.get("alpha").and_then(Value::as_f64).unwrap_or(OPEN)),
        _ => (Takes::Off, OPEN),
    }
}

pub fn edit(doc: &Value, path: &str, layer: &Value, takes: Takes, alpha: Option<f64>) -> Result<Vec<Op>> {
    let at = format!("{path}/solid");
    let threshold = alpha.unwrap_or_else(|| state(layer).1);
    ensure!(
        (0.0..=1.0).contains(&threshold),
        said("editor.touch_alpha", "the opacity that takes the pointer goes from 0 to 1", &[])
    );
    Ok(match takes {
        Takes::Off => doc.pointer(&at).map(|_| Op::Remove { path: at }).into_iter().collect(),
        Takes::All => set(doc, &at, json!(true)),
        Takes::Alpha => set(doc, &at, json!({ "alpha": threshold })),
    })
}

pub fn areas(doc: &Value) -> Value {
    let found = walk(doc).into_iter().filter_map(|(pointer, layer)| {
        let (takes, alpha) = state(layer);
        let bones = layer["hit"].as_array().map_or(0, Vec::len);
        let solid = takes != Takes::Off;
        let id = layer["id"].as_str()?;
        (solid || bones > 0).then(|| {
            let kind = if solid { takes.name() } else { "off" };
            json!({ "id": id, "pointer": pointer, "takes": kind, "alpha": alpha, "bones": bones })
        })
    });
    json!({ "layers": found.collect::<Vec<_>>() })
}
