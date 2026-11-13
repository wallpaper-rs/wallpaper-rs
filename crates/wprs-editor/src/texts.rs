use crate::ops::{Op, escape, set};
use crate::schema::{at, branches, deref, root_of};
use anyhow::{Result, bail, ensure};
use serde_json::{Value, json};
use wprs_core::message::said;
use wprs_format::SCHEMA;
use wprs_scene::tag;

const TITLE: &str = "/manifest/title";

fn key<'a>(doc: &Value, value: &'a Value) -> Option<&'a str> {
    value.as_str()?.strip_prefix('@').filter(|_| doc["manifest"]["schema"] == SCHEMA)
}

fn strings(language: &str, key: &str) -> String {
    format!("/manifest/strings/{}/{}", escape(language), escape(key))
}

pub fn rows(doc: &Value, pointer: &str, value: &Value) -> Vec<Value> {
    let row =
        |language: &str, text: &Value, at: String| json!({"language": language, "text": text, "pointer": at});
    let default = doc["manifest"]["default_locale"].as_str().filter(|_| doc["manifest"]["schema"] == SCHEMA);
    let mut out: Vec<Value> = match (key(doc, value), value) {
        (Some(key), _) => {
            let tables = doc["manifest"]["strings"].as_object().into_iter().flatten();
            let found = tables.filter(|(_, t)| t.get(key).is_some());
            found.map(|(language, t)| row(language, &t[key], strings(language, key))).collect()
        }
        (None, Value::Object(map)) => map
            .iter()
            .map(|(language, text)| row(language, text, format!("{pointer}/{}", escape(language))))
            .collect(),
        _ => vec![row(default.unwrap_or(""), &json!(value.as_str().unwrap_or_default()), pointer.into())],
    };
    let first = default.unwrap_or("en");
    out.sort_by_key(|r| r["language"] != first);
    out
}

fn textual(doc: &Value, pointer: &str) -> bool {
    let (Some((root, _)), Some(node)) = (root_of(pointer), at(doc, pointer)) else { return false };
    let (schema, name) = deref(root, &node);
    name == "Text" || branches(schema).iter().any(|b| deref(root, b).1 == "Text")
}

pub(crate) fn worded(text: &Value) -> bool {
    match text {
        Value::String(_) => true,
        Value::Object(map) => map.iter().all(|(k, v)| wprs_format::valid_tag(k) && v.is_string()),
        _ => false,
    }
}

pub(crate) fn words(doc: &Value, text: &Value) -> Option<String> {
    match text {
        Value::String(s) => Some(s.clone()),
        Value::Object(map) if !map.is_empty() && worded(text) => {
            let found = map.iter().filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_owned())));
            let default: Vec<String> =
                doc["manifest"]["default_locale"].as_str().map(String::from).into_iter().collect();
            Some(wprs_scene::Text::Localized(found.collect()).pick(&default))
        }
        _ => None,
    }
}

pub fn scene_languages(doc: &Value) -> Vec<String> {
    fn walk(layers: &Value, out: &mut Vec<String>) {
        for layer in layers.as_array().into_iter().flatten() {
            if layer["type"] == "text" && worded(&layer["text"]) {
                out.extend(layer["text"].as_object().into_iter().flat_map(|m| m.keys().map(|k| tag(k))));
            }
            walk(&layer["layers"], out);
        }
    }
    let mut out = Vec::new();
    walk(&doc["scene"]["layers"], &mut out);
    doc["scene"]["templates"].as_object().into_iter().flat_map(|t| t.values()).for_each(|l| {
        walk(&json!([l]), &mut out);
    });
    out
}

pub fn preview_locale(doc: &Value, wanted: &str) -> String {
    let default = doc["manifest"]["default_locale"].as_str().unwrap_or("en");
    if wanted.is_empty() {
        return default.to_owned();
    }
    let mut found: Vec<String> = scene_languages(doc);
    found.extend(doc["manifest"]["strings"].as_object().into_iter().flat_map(|m| m.keys().map(|k| tag(k))));
    found.push(tag(default));
    let map = found.into_iter().map(|l| (l.clone(), l)).collect();
    wprs_scene::Text::Localized(map).pick(&[wanted.to_owned(), default.to_owned()])
}

fn gone(doc: &Value, path: &str) -> Vec<Op> {
    match doc.pointer(path) {
        Some(_) => vec![Op::Remove { path: path.into() }],
        None => Vec::new(),
    }
}

pub fn translate(
    doc: &Value,
    pointer: &str,
    language: &str,
    text: Option<&str>,
    add: bool,
) -> Result<Vec<Op>> {
    let layer =
        pointer.strip_suffix("/text").and_then(|owner| doc.pointer(owner)).filter(|l| l["type"] == "text");
    ensure!(
        layer.is_none_or(|l| worded(&l["text"])),
        said(
            "editor.layer_text_bound",
            "the text of this layer follows a value; make it a fixed text first",
            &[]
        )
    );
    let named = language.len() <= 35 && language.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-');
    ensure!(
        named
            && (pointer.starts_with("/manifest/") || pointer.starts_with("/scene/"))
            && textual(doc, pointer),
        said(
            "editor.name_language_such",
            "name a language such as en or pt-BR and a text of the manifest or the scene",
            &[]
        )
    );
    let current = doc.pointer(pointer).cloned().unwrap_or(Value::Null);
    let mut text = text.filter(|t| !t.trim().is_empty()).map(String::from);
    if add {
        let shown = rows(doc, pointer, &current);
        if shown.iter().any(|r| tag(r["language"].as_str().unwrap_or_default()) == tag(language)) {
            return Ok(Vec::new());
        }
        let base = shown.first().and_then(|r| r["text"].as_str()).filter(|t| !t.is_empty());
        text = text.or(base.map(String::from));
        ensure!(
            text.is_some(),
            said("editor.write_text_default", "write the text in the default language first", &[])
        );
    }
    let default = doc["manifest"]["default_locale"].as_str();
    let mapped = current.is_object() || key(doc, &current).is_some();
    let language = if language.is_empty() && mapped { default.unwrap_or("en") } else { language };
    let plain = language.is_empty() || default.is_some_and(|d| tag(d) == tag(language));
    let v2 = doc["manifest"]["schema"] == SCHEMA;
    ensure!(
        v2 || plain || mapped,
        said(
            "editor.texts_several_languages",
            "texts in several languages need %1; upgrade the manifest first",
            &[&SCHEMA]
        )
    );
    let (entry, value) = (format!("{pointer}/{}", escape(language)), |t: &str| json!(t));
    Ok(match (key(doc, &current), &current, text.as_deref()) {
        (Some(key), _, Some(t)) => set(doc, &strings(language, key), value(t)),
        (Some(key), _, None) => gone(doc, &strings(language, key)),
        (None, Value::Object(map), Some(t)) if !map.is_empty() => set(doc, &entry, value(t)),
        (None, Value::Object(map), None) if map.len() > 1 => gone(doc, &entry),
        (None, Value::Object(map), None) if map.contains_key(language) => {
            ensure!(pointer != TITLE, said("editor.title_needs_text", "the title needs a text", &[]));
            ensure!(layer.is_none(), said("editor.layer_needs_text", "a text layer needs a text", &[]));
            gone(doc, pointer)
        }
        (None, Value::Object(_), None) => Vec::new(),
        (None, _, Some(t)) if plain => set(doc, pointer, value(t)),
        (None, Value::String(old), Some(t)) => {
            let base = default.unwrap_or("en");
            let mut ops = set(doc, pointer, json!({base: old, language: t}));
            if default.is_none() {
                ops.extend(set(doc, "/manifest/default_locale", value(base)));
            }
            ops
        }
        (None, _, Some(_)) => {
            bail!(said("editor.write_text_default", "write the text in the default language first", &[]))
        }
        (None, _, None) if plain => {
            ensure!(pointer != TITLE, said("editor.title_needs_text", "the title needs a text", &[]));
            ensure!(layer.is_none(), said("editor.layer_needs_text", "a text layer needs a text", &[]));
            gone(doc, pointer)
        }
        (None, _, None) => Vec::new(),
    })
}
