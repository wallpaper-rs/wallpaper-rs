use crate::action::Stop;
use crate::ops::{Op, set};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use wprs_core::message::said;

const MOST: usize = 4;

fn stops(layer: &Value) -> Vec<(f64, String)> {
    let list = layer["gradient"]["stops"].as_array().cloned().unwrap_or_default();
    list.iter().filter_map(|s| Some((s[0].as_f64()?, s[1].as_str()?.to_owned()))).collect()
}

fn round(v: f64) -> f64 {
    (v.clamp(0.0, 1.0) * 1000.0).round() / 1000.0
}

fn blend(list: &[(f64, String)], at: f64) -> String {
    let after = list.iter().position(|s| s.0 >= at);
    match (after, list.last()) {
        (Some(i), _) => list[i].1.clone(),
        (None, Some(last)) => last.1.clone(),
        _ => "#ffffff".into(),
    }
}

pub(crate) fn edit(doc: &Value, path: &str, layer: &Value, stop: &Stop) -> Result<Vec<Op>> {
    let gradient = format!("{path}/gradient");
    if let Stop::Clear = stop {
        return Ok(layer.get("gradient").map(|_| Op::Remove { path: gradient }).into_iter().collect());
    }
    if let Stop::Make { radial } = stop {
        let made = json!({ "radial": radial, "stops": [[0.0, "#ffffff"], [1.0, "#000000"]] });
        return Ok(set(doc, &gradient, made));
    }
    ensure!(
        layer["gradient"].is_object(),
        said(
            "editor.no_gradient",
            "the layer %1 has no gradient",
            &[&layer["id"].as_str().unwrap_or_default()]
        )
    );
    let mut list = stops(layer);
    let short = |i: usize| said("editor.no_stop", "the gradient has no stop %1", &[&i]);
    match stop {
        Stop::Add { at, color } => {
            ensure!(
                list.len() < MOST,
                said("editor.stops_most", "a gradient has at most %1 stops", &[&MOST])
            );
            let at = round(*at);
            let color = color.clone().unwrap_or_else(|| blend(&list, at));
            list.push((at, color));
        }
        Stop::Move { index, at } => {
            ensure!(*index < list.len(), short(*index));
            list[*index].0 = round(*at);
        }
        Stop::Color { index, color } => {
            ensure!(*index < list.len(), short(*index));
            list[*index].1 = color.clone();
        }
        Stop::Remove { index } => {
            ensure!(*index < list.len(), short(*index));
            ensure!(list.len() > 2, said("editor.stops_least", "a gradient keeps at least 2 stops", &[]));
            list.remove(*index);
        }
        Stop::Make { .. } | Stop::Clear => {}
    }
    list.sort_by(|a, b| a.0.total_cmp(&b.0));
    let value: Vec<Value> = list.into_iter().map(|(t, c)| json!([t, c])).collect();
    Ok(set(doc, &format!("{gradient}/stops"), Value::Array(value)))
}
