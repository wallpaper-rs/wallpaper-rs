use serde_json::Value;

pub(super) fn resolved(v: &Value, defs: &Value) -> Value {
    let name = v["$ref"].as_str().and_then(|r| r.rsplit('/').next());
    let mut out = match name.and_then(|n| defs.get(n)) {
        Some(found) => found.clone(),
        None => v.clone(),
    };
    if let (Some(map), Some(own)) = (out.as_object_mut(), v.as_object()) {
        own.iter().filter(|(k, _)| *k != "$ref").for_each(|(k, x)| {
            map.insert(k.clone(), x.clone());
        });
    }
    if let Some(only) = out["allOf"].as_array().filter(|a| a.len() == 1).map(|a| a[0].clone()) {
        return resolved(&only, defs);
    }
    out
}

fn kinds(v: &Value) -> Vec<&str> {
    match &v["type"] {
        Value::String(s) => vec![s.as_str()],
        Value::Array(a) => a.iter().filter_map(Value::as_str).filter(|k| *k != "null").collect(),
        _ => Vec::new(),
    }
}

fn alternatives(v: &Value) -> Vec<&Value> {
    ["oneOf", "anyOf"].into_iter().filter_map(|k| v[k].as_array()).flatten().collect()
}

fn choices(v: &Value) -> Option<Vec<String>> {
    let listed = v["enum"].as_array().cloned().or_else(|| v.get("const").map(|c| vec![c.clone()]));
    let all = alternatives(v);
    let names = |l: Vec<Value>| l.iter().map(|c| c.to_string()).collect();
    match listed {
        Some(l) => Some(names(l)),
        None if !all.is_empty()
            && all.iter().all(|a| a.get("const").is_some() || a.get("enum").is_some()) =>
        {
            Some(names(
                all.iter()
                    .flat_map(|a| a["enum"].as_array().cloned().unwrap_or_else(|| vec![a["const"].clone()]))
                    .collect(),
            ))
        }
        None => None,
    }
}

fn nullable(v: &Value) -> bool {
    alternatives(v).iter().any(|a| a["type"] == "null")
}

fn inner(v: &Value, defs: &Value) -> Value {
    match alternatives(v).into_iter().find(|a| a["type"] != "null") {
        Some(a) if alternatives(v).iter().filter(|a| a["type"] != "null").count() == 1 => resolved(a, defs),
        _ => v.clone(),
    }
}

fn flat(v: &Value, defs: &Value) -> Vec<Value> {
    let mut out = Vec::new();
    for alt in alternatives(v) {
        let alt = resolved(alt, defs);
        let nested = alternatives(&alt).len();
        match () {
            _ if alt["properties"].is_object() || alt.get("const").is_some() => out.push(alt),
            _ if nested > 0 && choices(&alt).is_none() => out.extend(flat(&alt, defs)),
            _ if alt["enum"].is_array() => out.extend(
                alt["enum"].as_array().into_iter().flatten().map(|c| serde_json::json!({ "const": c })),
            ),
            _ => {}
        }
    }
    out
}

fn named(alt: &Value) -> Option<String> {
    match alt.get("const").and_then(Value::as_str) {
        Some(c) => Some(c.to_owned()),
        None => alt["properties"].as_object().and_then(|o| o.keys().next().cloned()),
    }
}

pub(super) fn brief(v: &Value, defs: &Value) -> String {
    let v = &inner(&resolved(v, defs), defs);
    if let Some(list) = choices(v) {
        return list.join(" | ");
    }
    let alts = alternatives(v);
    if !alts.is_empty() {
        let keys: Vec<String> = flat(v, defs).iter().filter_map(named).collect();
        return format!("one of {}", keys.join(" | "));
    }
    match kinds(v).first().copied() {
        Some("array") => match v["items"].is_object() {
            true => format!("list of {}", brief(&v["items"], defs)),
            false => "list".into(),
        },
        Some("object") if v["properties"].is_object() => "object".into(),
        Some("object") => "map".into(),
        Some(k) => k.into(),
        None => "any JSON value".into(),
    }
}

fn note(v: &Value, required: bool) -> String {
    let default = v.get("default").filter(|d| !d.is_null() && !required).map(|d| format!(", default {d}"));
    let level = if required { "required" } else { "optional" };
    format!(" ({level}{})", default.unwrap_or_default())
}

pub(super) fn lines(v: &Value, defs: &Value, at: &str, depth: usize, out: &mut Vec<String>) {
    fields(v, defs, (at, 1), depth, false, out)
}

const NEEDED: [(&str, &str); 9] = [
    ("scene3d.model.set", "layer"),
    ("scene3d.light", "layer"),
    ("scene3d.shadows", "layer"),
    ("scene3d.material", "layer"),
    ("scene3d.material", "name"),
    ("scene3d.path.key", "path"),
    ("animate.layer", "layer"),
    ("pool", "id"),
    ("timeline", "id"),
];

fn fields(v: &Value, defs: &Value, place: (&str, usize), depth: usize, variant: bool, out: &mut Vec<String>) {
    let (at, indent) = place;
    let pad = "  ".repeat(indent);
    if let Some(props) = v["properties"].as_object() {
        let need: Vec<&str> =
            v["required"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
        for (key, field) in props {
            let shown = resolved(field, defs);
            let forced = NEEDED.contains(&(at, key.as_str()));
            let required = (forced || need.contains(&key.as_str())) && !nullable(&shown);
            let tag = if variant { String::new() } else { note(&shown, required) };
            out.push(format!("{pad}{key}: {}{tag}", brief(field, defs)));
            if depth > 0 {
                let here = format!("{at}.{key}");
                fields(&inner(&shown, defs), defs, (&here, indent + 1), depth - 1, false, out);
            }
        }
    }
    if choices(v).is_none() {
        let bodies = flat(v, defs);
        if !bodies.is_empty() && depth > 0 {
            out.push(format!("{pad}one of:"));
        }
        for alt in bodies.iter().filter(|_| depth > 0) {
            match alt["properties"].is_object() {
                true => fields(alt, defs, place, depth, true, out),
                false => out.push(format!("{pad}{}: no arguments", named(alt).unwrap_or_default())),
            }
        }
    }
    if let Some(items) = v.get("items").filter(|_| depth > 0) {
        fields(&inner(&resolved(items, defs), defs), defs, place, depth - 1, false, out);
    }
}
