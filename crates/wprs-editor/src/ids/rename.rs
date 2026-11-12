use super::refs::swap;
use crate::action::Named;
use crate::actions::{EDITOR, layer};
use crate::layers::taken;
use crate::ops::{Op, diff};
use anyhow::{Result, bail, ensure};
use serde_json::{Map, Value, json};
use std::collections::BTreeMap;
use wprs_core::message::said;

pub(super) const WORDS: [&str; 9] =
    ["text", "placeholder", "label", "title", "name", "description", "id", "timeline", "help"];

fn fit(to: &str) -> Result<()> {
    let ok = (1..=32).contains(&to.len())
        && to.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
    ensure!(ok, said("editor.id_shape", "an id is 1 to 32 small letters, digits or _", &[]));
    Ok(())
}

pub(super) fn strings(v: &mut Value, old: &str, new: &str, skip: &[&str]) {
    match v {
        Value::String(s) if s == old => *s = new.to_owned(),
        Value::Array(list) => list.iter_mut().for_each(|x| strings(x, old, new, skip)),
        Value::Object(map) => {
            for (_, x) in map.iter_mut().filter(|(k, _)| !skip.contains(&k.as_str())) {
                strings(x, old, new, skip);
            }
        }
        _ => {}
    }
}

fn marks(editor: &mut Value, old: &str, new: &str) {
    if let Some(list) = editor.get_mut("locked").and_then(Value::as_array_mut) {
        list.iter_mut().filter(|v| *v == old).for_each(|v| *v = json!(new));
    }
    if editor.get("solo").and_then(Value::as_str) == Some(old) {
        editor["solo"] = json!(new);
    }
    if let Some(hidden) = editor.get_mut("hidden").and_then(Value::as_object_mut)
        && let Some(v) = hidden.remove(old)
    {
        hidden.insert(new.to_owned(), v);
    }
}

pub fn rename(dir: &std::path::Path, doc: &Value, kind: Named, id: &str, to: &str) -> Result<Vec<Op>> {
    if id == to {
        return Ok(Vec::new());
    }
    fit(to)?;
    let mut scene = doc["scene"].clone();
    let mut manifest = doc["manifest"].clone();
    match kind {
        Named::Layer => {
            let (path, _) = layer(doc, id)?;
            ensure!(!taken(doc, to), said("editor.id_use", "the id %1 is in use", &[&to]));
            let inner = path.strip_prefix("/scene").unwrap_or_default();
            if let Some(l) = scene.pointer_mut(inner) {
                l["id"] = json!(to);
            }
            swap(&mut scene, &BTreeMap::from([(id.to_owned(), to.to_owned())]));
            if let Some(editor) = scene.pointer_mut(EDITOR.strip_prefix("/scene").unwrap_or_default()) {
                marks(editor, id, to);
            }
        }
        Named::Timeline => {
            let edit = crate::action::TimelineEdit {
                id: id.to_owned(),
                to: Some(to.to_owned()),
                ..Default::default()
            };
            return crate::timelines::edit(dir, doc, &edit);
        }
        Named::Asset => {
            let assets = scene.get_mut("assets").and_then(Value::as_object_mut);
            let Some(assets) = assets.filter(|a| a.contains_key(id)) else {
                bail!(said("editor.no_asset", "there is no asset %1", &[&id]))
            };
            ensure!(!taken(doc, to), said("editor.id_use", "the id %1 is in use", &[&to]));
            let moved: Map<String, Value> = std::mem::take(assets)
                .into_iter()
                .map(|(k, v)| (if k == id { to.to_owned() } else { k }, v))
                .collect();
            *assets = moved;
            strings(&mut scene, id, to, &WORDS);
            strings(&mut manifest["properties"], id, to, &WORDS);
        }
        Named::System => {
            ensure!(!taken(doc, to), said("editor.id_use", "the id %1 is in use", &[&to]));
            let at = crate::swarms::system(doc, id)?;
            return crate::swarms::rename_system(doc, &at, id, to);
        }
    }
    let mut ops = diff(&doc["scene"], &scene, "/scene");
    ops.extend(diff(&doc["manifest"], &manifest, "/manifest"));
    crate::shaping::refused(dir, doc, ops)
}
