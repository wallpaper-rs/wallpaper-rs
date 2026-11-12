use super::{listing::bound, valid_param};
use crate::ops::{Op, diff, escape};
use anyhow::{Result, ensure};
use serde_json::{Map, Value, json};
use wprs_core::message::said;
use wprs_scene::{MAX_SCRIPT_PARAMS, ScriptParam};

pub(super) struct Param<'a> {
    pub id: &'a str,
    pub kind: Option<&'a str>,
    pub default: Option<&'a Value>,
    pub range: (Option<f64>, Option<f64>),
    pub to: Option<&'a str>,
    pub remove: bool,
}

fn blank(kind: &str) -> Value {
    match kind {
        "number" => json!(0),
        "bool" => json!(false),
        "color" => json!("#ffffff"),
        "vec2" => json!([0, 0]),
        _ => json!(""),
    }
}

fn whole(v: f64) -> Value {
    if v.fract() == 0.0 && v.abs() < 1e15 { json!(v as i64) } else { json!(v) }
}

fn spec(old: Option<&Value>, p: &Param) -> Result<Value> {
    let kind = p.kind.or_else(|| old.and_then(|o| o["type"].as_str()));
    let kind = kind.ok_or_else(|| said("editor.param_type", "name the type of the setting %1", &[&p.id]))?;
    let default = match (p.default, old.filter(|o| o["type"] == kind)) {
        (Some(d), _) => d.clone(),
        (None, Some(o)) => o["default"].clone(),
        (None, None) => blank(kind),
    };
    let mut out = Map::from_iter([("type".to_owned(), json!(kind)), ("default".to_owned(), default)]);
    let number = kind == "number";
    let keep = |key: &str, given: Option<f64>| match (p.kind, given) {
        (_, Some(v)) => Some(whole(v)),
        (None, None) => old.and_then(|o| o.get(key).cloned()),
        _ => None,
    };
    for (key, given) in [("min", p.range.0), ("max", p.range.1)] {
        out.extend(keep(key, given).filter(|_| number).map(|v| (key.to_owned(), v)));
    }
    let out = Value::Object(out);
    let parsed = serde_json::from_value::<ScriptParam>(out.clone())
        .map_err(|e| said("editor.param_invalid", "the setting %1 is not valid: %2", &[&p.id, &e]))?;
    let (min, max) = (parsed.min, parsed.max);
    ensure!(
        min.zip(max).is_none_or(|(a, b)| a <= b),
        said("editor.param_range", "the minimum of %1 is above its maximum", &[&p.id])
    );
    let below = |v: f64| min.is_some_and(|m| v < f64::from(m)) || max.is_some_and(|m| v > f64::from(m));
    ensure!(
        !number || !out["default"].as_f64().is_some_and(below),
        said("editor.param_default", "the default of %1 is outside its range", &[&p.id])
    );
    Ok(out)
}

pub(super) fn edit(doc: &Value, path: &str, p: &Param) -> Result<Vec<Op>> {
    let list = doc.pointer("/scene/scripts").and_then(Value::as_array);
    let at = list.and_then(|l| l.iter().position(|s| s["path"] == path));
    let at = at.ok_or_else(|| said("editor.no_script", "the scene has no script %1", &[&path]))?;
    let base = format!("/scene/scripts/{at}/params");
    let old = doc.pointer(&format!("{base}/{}", escape(p.id)));
    let all = doc.pointer(&base).and_then(Value::as_object).cloned().unwrap_or_default();
    ensure!(
        valid_param(p.id),
        said("editor.param_id", "a setting id has 1 to 32 characters of a-z, 0-9 and _", &[])
    );
    let mut next = all.clone();
    if p.remove {
        ensure!(old.is_some(), said("editor.no_param", "the script has no setting %1", &[&p.id]));
        next.remove(p.id);
    } else {
        let made = spec(old, p)?;
        match p.to.filter(|to| *to != p.id) {
            Some(to) => {
                ensure!(old.is_some(), said("editor.no_param", "the script has no setting %1", &[&p.id]));
                ensure!(
                    valid_param(to),
                    said("editor.param_id", "a setting id has 1 to 32 characters of a-z, 0-9 and _", &[])
                );
                ensure!(!all.contains_key(to), said("editor.param_taken", "the setting %1 exists", &[&to]));
                next.remove(p.id);
                next.insert(to.to_owned(), made);
            }
            None => drop(next.insert(p.id.to_owned(), made)),
        }
        ensure!(
            next.len() <= MAX_SCRIPT_PARAMS,
            said("editor.param_most", "a script has at most %1 settings", &[&MAX_SCRIPT_PARAMS])
        );
    }
    let mut scene = doc["scene"].clone();
    let params = &mut scene["scripts"][at];
    match next.is_empty() {
        true => drop(params.as_object_mut().and_then(|o| o.remove("params"))),
        false => params["params"] = Value::Object(next),
    }
    if let Some(to) = p.to.filter(|_| !p.remove) {
        let renamed = bound(doc, path)
            .into_iter()
            .filter_map(|pointer| pointer.strip_prefix("/scene").map(String::from));
        for pointer in renamed {
            let Some(values) = scene.pointer_mut(&format!("{pointer}/params")).and_then(Value::as_object_mut)
            else {
                continue;
            };
            if let Some(v) = values.remove(p.id) {
                values.insert(to.to_owned(), v);
            }
        }
    }
    Ok(diff(&doc["scene"], &scene, "/scene"))
}
