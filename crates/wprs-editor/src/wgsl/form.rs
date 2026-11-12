use anyhow::{Result, anyhow, ensure};
use serde_json::{Map, Value, json};
use wprs_core::message::said;

pub const KINDS: [&str; 12] = [
    "float", "vec2", "color", "bool", "texture", "lut", "int", "enum", "blend", "position", "mask",
    "emitters",
];
pub const FIELDS: [&str; 19] = [
    "name",
    "category",
    "description",
    "uses",
    "mask_mode",
    "cost",
    "fuse",
    "reach",
    "bounded",
    "min_fps",
    "velocity",
    "state",
    "solve",
    "pace",
    "quality",
    "targets",
    "options",
    "compose",
    "lighter",
];
const PARAM_KEYS: [&str; 14] = [
    "type",
    "default",
    "min",
    "max",
    "unit",
    "label",
    "help",
    "values",
    "channels",
    "paint",
    "uses",
    "yields_to",
    "waits_for",
    "tools",
];
const PASS_KEYS: [&str; 10] =
    ["entry", "scale", "inputs", "output", "target", "copy", "blend", "vertex", "grid", "swap"];

fn default_of(kind: &str) -> Value {
    match kind {
        "vec2" => json!([0.0, 0.0]),
        "position" => json!([0.5, 0.5]),
        "color" => json!("#ffffff"),
        "bool" => json!(false),
        "texture" | "mask" => json!("white"),
        "lut" => json!(""),
        "emitters" => json!([]),
        _ => json!(0),
    }
}

fn keyed(key: &str, allowed: &[&str]) -> Result<()> {
    ensure!(allowed.contains(&key), said("editor.shader_key", "%1 is not a field this form edits", &[&key]));
    Ok(())
}

fn list<'a>(json: &'a mut Value, key: &str) -> &'a mut Vec<Value> {
    if !json[key].is_array() {
        json[key] = json!([]);
    }
    json[key].as_array_mut().expect("made above")
}

pub(crate) struct ParamChange<'a> {
    pub id: &'a str,
    pub set: &'a Map<String, Value>,
    pub clear: &'a [String],
    pub to: Option<&'a str>,
    pub at: Option<usize>,
    pub remove: bool,
}

pub(crate) fn param(json: &mut Value, c: &ParamChange) -> Result<()> {
    let params = list(json, "params");
    let found = params.iter().position(|p| p["id"] == c.id);
    if c.remove {
        let at = found
            .ok_or_else(|| anyhow!(said("editor.no_parameter", "%1 has no parameter %2", &[&"", &c.id])))?;
        params.remove(at);
        return Ok(());
    }
    let at = match found {
        Some(at) => at,
        None => {
            crate::clips::valid(c.id)?;
            let kind = c.set.get("type").and_then(Value::as_str).unwrap_or("float");
            params.push(json!({ "id": c.id, "type": kind, "default": default_of(kind) }));
            params.len() - 1
        }
    };
    let p = &mut params[at];
    let kind_before = p["type"].clone();
    for (k, v) in c.set {
        keyed(k, &PARAM_KEYS)?;
        p[k] = v.clone();
    }
    if p["type"] != kind_before && !c.set.contains_key("default") {
        p["default"] = default_of(p["type"].as_str().unwrap_or("float"));
    }
    for k in c.clear {
        keyed(k, &PARAM_KEYS[2..])?;
        drop(p.as_object_mut().and_then(|o| o.remove(k)));
    }
    if let Some(to) = c.to {
        crate::clips::valid(to)?;
        ensure!(
            !params.iter().any(|q| q["id"] == to),
            said("editor.parameter_taken", "there is already a parameter %1", &[&to])
        );
        params[at]["id"] = json!(to);
    }
    if let Some(to) = c.at {
        let moved = params.remove(at);
        params.insert(to.min(params.len()), moved);
    }
    Ok(())
}

pub(crate) fn lighter(json: &mut Value, set: &Map<String, Value>, remove: bool) -> Result<bool> {
    let id = json["id"].as_str().unwrap_or_default().to_owned();
    if remove {
        drop(json.as_object_mut().and_then(|o| o.remove("lighter")));
        return Ok(false);
    }
    let made = !json["lighter"].is_object();
    if made {
        let cost = json!({ "taps": 1, "passes": 1, "scale": 1 });
        json["lighter"] = json!({ "passes": [{ "entry": format!("fs_{id}_lite") }], "cost": cost });
    }
    for (k, v) in set {
        keyed(k, &["cost", "up_to"])?;
        match v {
            Value::Null => drop(json["lighter"].as_object_mut().and_then(|o| o.remove(k))),
            v => json["lighter"][k] = v.clone(),
        }
    }
    Ok(made)
}

pub(crate) struct PassChange<'a> {
    pub index: Option<usize>,
    pub set: &'a Map<String, Value>,
    pub clear: &'a [String],
    pub remove: bool,
    pub lighter: bool,
}

pub(crate) fn pass(json: &mut Value, c: &PassChange) -> Result<()> {
    let PassChange { index, set, clear, remove, lighter } = *c;
    let id = json["id"].as_str().unwrap_or_default().to_owned();
    if lighter && !json["lighter"].is_object() {
        self::lighter(json, &Map::new(), false)?;
    }
    let (passes, entry) = match lighter {
        true => (list(&mut json["lighter"], "passes"), format!("fs_{id}_lite_")),
        false => (list(json, "passes"), format!("fs_{id}_")),
    };
    let missing = || anyhow!(said("editor.no_pass", "there is no pass %1", &[&index.unwrap_or_default()]));
    let at = match index {
        Some(i) if i < passes.len() => i,
        Some(_) => return Err(missing()),
        None if remove => return Err(missing()),
        None => {
            passes.push(json!({ "entry": format!("{entry}{}", passes.len()) }));
            passes.len() - 1
        }
    };
    if remove {
        passes.remove(at);
        return Ok(());
    }
    for (k, v) in set {
        keyed(k, &PASS_KEYS)?;
        passes[at][k] = v.clone();
    }
    for k in clear {
        keyed(k, &PASS_KEYS[1..])?;
        drop(passes[at].as_object_mut().and_then(|o| o.remove(k)));
    }
    Ok(())
}

pub(crate) fn field(json: &mut Value, name: &str, value: &Value) -> Result<()> {
    keyed(name, &FIELDS)?;
    match value {
        Value::Null => drop(json.as_object_mut().and_then(|o| o.remove(name))),
        v => json[name] = v.clone(),
    }
    Ok(())
}
