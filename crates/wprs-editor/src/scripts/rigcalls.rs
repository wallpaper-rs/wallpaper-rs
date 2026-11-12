use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::Path;
use wprs_core::message::said;

const KINDS: [(&str, &str); 3] = [(".bone(", "bone"), (".impulse(", "bone"), (".shape(", "shape")];

pub(super) fn named(text: &str) -> Vec<(&'static str, String, usize)> {
    let mut out = Vec::new();
    for (call, kind) in KINDS {
        for (at, _) in text.match_indices(call) {
            let rest = text[at + call.len()..].trim_start();
            let Some(quote) = rest.chars().next().filter(|c| matches!(c, '"' | '\'' | '`')) else { continue };
            let Some(end) = rest[1..].find(quote) else { continue };
            let name = &rest[1..1 + end];
            if !name.is_empty() && !name.contains(['\n', '$']) {
                out.push((kind, name.to_owned(), text[..at].matches('\n').count() + 1));
            }
        }
    }
    out.sort_by_key(|n| n.2);
    out
}

fn figures<'a>(list: &'a Value, out: &mut Vec<&'a Value>) {
    for l in list.as_array().into_iter().flatten() {
        if l["type"] == "mesh" || l["type"] == "model" {
            out.push(l);
        }
        figures(&l["layers"], out);
    }
}

pub(super) fn missing(dir: &Path, doc: &Value, text: &str) -> Value {
    let calls = named(text);
    let mut all = Vec::new();
    figures(&doc["scene"]["layers"], &mut all);
    let literal = |id: &str| ["\"", "'", "`"].iter().any(|q| text.contains(&format!("{q}{id}{q}")));
    let named_layers: Vec<&Value> =
        all.iter().copied().filter(|l| l["id"].as_str().is_some_and(literal)).collect();
    let targets = if named_layers.is_empty() { all } else { named_layers };
    if targets.is_empty() || calls.is_empty() {
        return json!([]);
    }
    let mut bones = BTreeSet::new();
    let mut shapes = BTreeSet::new();
    let mut ids = Vec::new();
    for l in &targets {
        let id = l["id"].as_str().unwrap_or_default();
        ids.push(id.to_owned());
        let rig = crate::rigs::rig(dir, doc, id);
        for key in ["bones", "points"] {
            bones.extend(
                rig[key]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|b| b["name"].as_str())
                    .map(String::from),
            );
        }
        bones.extend(
            rig["parts"].as_array().into_iter().flatten().filter_map(Value::as_str).map(String::from),
        );
        shapes.extend(
            rig["shapes"].as_array().into_iter().flatten().filter_map(Value::as_str).map(String::from),
        );
        shapes.extend(rig["expressions"].as_object().into_iter().flatten().map(|(k, _)| k.clone()));
    }
    let lacking = calls.into_iter().filter(|(kind, name, _)| match *kind {
        "bone" => !bones.contains(name),
        _ => !shapes.contains(name),
    });
    json!(
        lacking
            .map(|(kind, name, line)| json!({"kind": kind, "name": name, "line": line, "layers": ids}))
            .collect::<Vec<_>>()
    )
}

fn call(layer: &str, kind: &str, name: &str, what: &str) -> Option<String> {
    let l = format!("scene.layer({})", json!(layer));
    let n = json!(name);
    Some(match (kind, what) {
        ("bone", "turn") => format!("{l}.bone({n}).rotation = 0;"),
        ("bone", "move") => format!("{l}.bone({n}).position = [0, 0];"),
        ("bone", "scale") => format!("{l}.bone({n}).scale = [1, 1];"),
        ("bone", "read") => format!("const {} = {l}.bone({n}).rotation;", ident(name)),
        ("bone", "reset") => format!("{l}.bone({n}).reset();"),
        ("shape", "set") => format!("{l}.shape({n}, 0.5);"),
        ("shape", "read") => format!("const {} = {l}.shape({n});", ident(name)),
        ("impulse", "push") => format!("{l}.impulse({n}, [0, -300]);"),
        _ => return None,
    })
}

fn ident(name: &str) -> String {
    let mut s: String = name.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    if s.is_empty() || s.starts_with(|c: char| c.is_ascii_digit()) {
        s.insert(0, '_');
    }
    s
}

pub fn calls(dir: &Path, doc: &Value, layer: &str, ask: &Value) -> Result<Value> {
    let (_, l) = crate::actions::layer(doc, layer)?;
    ensure!(
        l["type"] == "mesh" || l["type"] == "model",
        said("editor.not_figure", "the layer is not a mesh or a model", &[])
    );
    let rig = crate::rigs::rig(dir, doc, layer);
    let names = |key: &str| -> Vec<String> {
        rig[key]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|b| b["name"].as_str().or(b.as_str()))
            .map(String::from)
            .collect()
    };
    let mut shapes = names("shapes");
    shapes.extend(rig["expressions"].as_object().into_iter().flatten().map(|(k, _)| k.clone()));
    let bones: Vec<String> = names("bones").into_iter().chain(names("points")).collect();
    let bodies: Vec<String> = l["physics"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|b| b["bone"].as_str())
        .map(String::from)
        .collect();
    let listing = json!({
        "layer": layer, "bones": bones, "shapes": shapes, "bodies": bodies,
        "uses": ["animations"],
        "calls": {"bone": ["turn", "move", "scale", "read", "reset"], "shape": ["set", "read"], "impulse": ["push"]},
    });
    let (Some(kind), Some(name), Some(what)) =
        (ask["kind"].as_str(), ask["name"].as_str(), ask["what"].as_str())
    else {
        return Ok(listing);
    };
    let pool = match kind {
        "shape" => &listing["shapes"],
        "impulse" => &listing["bodies"],
        _ => &listing["bones"],
    };
    ensure!(
        pool.as_array().is_some_and(|p| p.iter().any(|n| n == name)),
        said("editor.no_rig_name", "the layer has no %1 named \"%2\"", &[&kind, &name])
    );
    let code = call(layer, kind, name, what)
        .ok_or_else(|| said("editor.no_rig_call", "there is no call \"%1\" for a %2", &[&what, &kind]))?;
    Ok(json!({"layer": layer, "kind": kind, "name": name, "code": code, "uses": ["animations"]}))
}
