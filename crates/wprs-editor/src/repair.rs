use serde_json::{Value, json};
use wprs_scene::{Color, Layer};

const COLOR_KEYS: [&str; 3] = ["color", "fill", "tint"];
const FALLBACK: &str = "#808080";

fn lacks(layer: &Value) -> bool {
    serde_json::from_value::<Layer>(layer.clone()).is_err()
}

fn flat(layer: &Value) -> Value {
    let mut own = layer.clone();
    if let Some(o) = own.as_object_mut() {
        o.remove("layers");
        if layer["type"] == "group" {
            o.insert("layers".into(), json!([]));
        }
    }
    own
}

fn invalid(key: &str, text: &str) -> bool {
    (text.starts_with('#') || COLOR_KEYS.contains(&key)) && Color::parse(text).is_none()
}

fn collect(value: &Value, key: &str, at: &str, out: &mut Vec<(String, String)>) {
    match value {
        Value::String(text) if invalid(key, text) => out.push((at.to_owned(), text.clone())),
        Value::Array(items) => {
            items.iter().enumerate().for_each(|(i, v)| collect(v, key, &format!("{at}/{i}"), out))
        }
        Value::Object(map) => map
            .iter()
            .filter(|(k, _)| k.as_str() != "layers")
            .for_each(|(k, v)| collect(v, k, &format!("{at}/{k}"), out)),
        _ => {}
    }
}

pub(crate) fn colors(layer: &Value) -> Vec<(String, String)> {
    let mut out = Vec::new();
    collect(layer, "", "", &mut out);
    out
}

pub(crate) fn broken(layer: &Value) -> bool {
    lacks(&flat(layer))
}

pub(crate) fn mend(layer: &mut Value) -> bool {
    if !broken(layer) {
        return true;
    }
    let mut trial = layer.clone();
    for (at, _) in colors(layer) {
        if let Some(slot) = trial.pointer_mut(&at) {
            *slot = json!(FALLBACK);
        }
    }
    if broken(&trial) {
        return false;
    }
    *layer = trial;
    true
}

pub(crate) fn mend_all(list: &mut Value) {
    let Some(items) = list.as_array_mut() else { return };
    items.retain_mut(|layer| {
        let kids = layer.as_object_mut().and_then(|o| o.remove("layers"));
        let kept = mend(layer);
        if let (true, Some(mut kids)) = (kept, kids) {
            mend_all(&mut kids);
            layer["layers"] = kids;
        }
        kept
    });
}

pub(crate) fn problems(list: &Value, base: &str) -> Vec<crate::Problem> {
    let mut out = Vec::new();
    for (i, layer) in list.as_array().into_iter().flatten().enumerate() {
        let at = format!("{base}/{i}");
        if broken(layer) {
            let faults = colors(layer);
            out.extend(faults.into_iter().map(|(field, text)| {
                crate::findings::problem(format!("{at}{field}"), wprs_scene::Issue::Color(text).message())
            }));
        }
        out.extend(problems(&layer["layers"], &format!("{at}/layers")));
    }
    out
}
