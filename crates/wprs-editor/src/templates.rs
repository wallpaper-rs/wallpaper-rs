use crate::action::TemplateEdit as E;
use crate::layers::{clean, find};
use crate::ops::{Op, escape};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::MAX_TEMPLATES;

const KINDS: [&str; 7] = ["group", "image", "text", "shape", "solid", "spectrum", "mesh"];

fn names(doc: &Value) -> Vec<String> {
    doc.pointer("/scene/templates")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .map(|(k, _)| k.clone())
        .collect()
}

fn path(name: &str) -> String {
    format!("/scene/templates/{}", escape(name))
}

fn free(doc: &Value, base: &str) -> String {
    let taken = names(doc);
    let tries = std::iter::once(base.to_owned()).chain((2..).map(|n| format!("{base}_{n}")));
    tries.take(1000).find(|n| !taken.contains(n)).unwrap_or_else(|| base.to_owned())
}

fn make(doc: &Value, ids: &[String], name: Option<&str>) -> Result<Vec<Op>> {
    ensure!(
        !ids.is_empty(),
        said("editor.name_layers_template", "name the layers to make a template of", &[])
    );
    let mut copies = Vec::new();
    for id in ids {
        let value = doc.pointer(&find(doc, id)?).cloned().unwrap_or_default();
        ensure!(
            KINDS.contains(&value["type"].as_str().unwrap_or_default()),
            said("editor.not_templated", "scripts cannot copy a layer of type %1", &[&value["type"]])
        );
        copies.push(value);
    }
    let base = match (name, ids) {
        (Some(n), _) => clean(n),
        (None, [only]) => format!("{only}_template"),
        _ => "group_template".to_owned(),
    };
    ensure!(!base.is_empty(), said("editor.template_name", "a template needs a name", &[]));
    ensure!(
        names(doc).len() < MAX_TEMPLATES,
        said("editor.template_most", "a scene has at most %1 templates", &[&MAX_TEMPLATES])
    );
    let name = match name {
        Some(_) => {
            ensure!(
                !names(doc).contains(&base),
                said("editor.template_taken", "the template %1 exists", &[&base])
            );
            base
        }
        None => free(doc, &base),
    };
    let value = match copies.len() {
        1 => copies.remove(0),
        _ => json!({"id": name, "type": "group", "layers": copies}),
    };
    Ok(match doc.pointer("/scene/templates").is_some_and(Value::is_object) {
        true => vec![Op::Add { path: path(&name), value }],
        false => vec![Op::Add { path: "/scene/templates".into(), value: json!({ name: value }) }],
    })
}

pub(crate) fn edit(dir: &Path, doc: &Value, e: &E) -> Result<Vec<Op>> {
    let exists = |name: &str| doc.pointer(&path(name)).is_some();
    match e {
        E::Make { layers, name } => make(doc, layers, name.as_deref()),
        E::Rename { name, to, rewrite } => {
            ensure!(exists(name), said("editor.no_template", "the scene has no template %1", &[&name]));
            let to = clean(to);
            ensure!(!to.is_empty(), said("editor.template_name", "a template needs a name", &[]));
            ensure!(
                to == *name || !exists(&to),
                said("editor.template_taken", "the template %1 exists", &[&to])
            );
            if to == *name {
                return Ok(Vec::new());
            }
            let mut ops = vec![Op::Move { from: path(name), path: path(&to) }];
            if *rewrite {
                ops.extend(rewritten(dir, doc, name, &to)?);
            }
            Ok(ops)
        }
        E::Remove { name } => {
            ensure!(exists(name), said("editor.no_template", "the scene has no template %1", &[&name]));
            Ok(vec![Op::Remove { path: path(name) }])
        }
    }
}

fn rows(layer: &Value, pointer: &str, depth: usize, out: &mut Vec<Value>) {
    out.push(json!({"id": layer["id"], "type": layer["type"], "pointer": pointer, "depth": depth}));
    for (i, child) in layer["layers"].as_array().into_iter().flatten().enumerate() {
        rows(child, &format!("{pointer}/layers/{i}"), depth + 1, out);
    }
}

pub fn tree(dir: &Path, doc: &Value) -> Vec<Value> {
    let all = doc.pointer("/scene/templates").and_then(Value::as_object).into_iter().flatten();
    all.map(|(name, layer)| {
        let pointer = path(name);
        let mut layers = Vec::new();
        rows(layer, &pointer, 0, &mut layers);
        let scripts = crate::references::scripts(dir, doc, name);
        json!({"name": name, "pointer": pointer, "layers": layers, "scripts": scripts, "mentions": mentions(dir, doc, name)})
    })
    .collect()
}

const QUOTES: [char; 3] = ['"', '\'', '`'];

fn literals(name: &str) -> Vec<String> {
    QUOTES.iter().map(|q| format!("{q}{name}{q}")).collect()
}

fn texts(dir: &Path, doc: &Value) -> Vec<(String, String)> {
    let list = doc.pointer("/scene/scripts").and_then(Value::as_array).into_iter().flatten();
    let paths = list.filter_map(|s| s["path"].as_str()).filter(|p| wprs_format::check_path(p).is_ok());
    paths.filter_map(|p| Some((p.to_owned(), std::fs::read_to_string(dir.join(p)).ok()?))).collect()
}

fn rewritten(dir: &Path, doc: &Value, name: &str, to: &str) -> Result<Vec<Op>> {
    let mut ops = Vec::new();
    for (path, text) in texts(dir, doc) {
        let mut next = text.clone();
        for (from, quote) in literals(name).iter().zip(QUOTES) {
            next = next.replace(from, &format!("{quote}{to}{quote}"));
        }
        if next != text {
            ops.extend(crate::scripts::save(dir, doc, &path, &next)?);
        }
    }
    Ok(ops)
}

pub fn mentions(dir: &Path, doc: &Value, name: &str) -> Vec<Value> {
    let word = |c: char| c.is_alphanumeric() || c == '_' || c == '$';
    let exact = literals(name);
    let mut out = Vec::new();
    for (path, text) in texts(dir, doc) {
        for (i, line) in text.lines().enumerate() {
            let named = line.match_indices(name).any(|(at, _)| {
                !line[..at].chars().next_back().is_some_and(word)
                    && !line[at + name.len()..].starts_with(word)
            });
            if named {
                let literal = exact.iter().any(|l| line.contains(l.as_str()));
                out.push(json!({"path": path, "line": i + 1, "exact": literal}));
            }
        }
    }
    out
}
