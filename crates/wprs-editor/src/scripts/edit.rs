use super::declare::BLIND;
use super::params::{self, Param};
use super::{bound, entry, place, room, used_params};
use crate::action::ScriptEdit as E;
use crate::ops::Op;
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::MAX_SCRIPT_BYTES;

fn index(doc: &Value, path: &str) -> Result<usize> {
    let list = doc.pointer("/scene/scripts").and_then(Value::as_array);
    let found = list.and_then(|l| l.iter().position(|s| s["path"] == path));
    found.ok_or_else(|| said("editor.no_script", "the scene has no script %1", &[&path]).into())
}

fn text(dir: &Path, path: &str) -> Result<String> {
    ensure!(
        wprs_format::check_path(path).is_ok(),
        said("editor.script_path", "%1 is not a path of the package", &[&path])
    );
    fs::read_to_string(dir.join(path))
        .map_err(|e| said("editor.cannot_read", "cannot read %1: %2", &[&path, &e]).into())
}

fn declare(dir: &Path, doc: &Value, path: &str, with_params: bool) -> Result<Vec<Op>> {
    let at = index(doc, path)?;
    let old = &doc["scene"]["scripts"][at];
    let source = text(dir, path)?;
    let mut fresh = entry(doc, path, &source);
    let blind: Vec<Value> = old["uses"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|u| u.as_str().is_some_and(|u| BLIND.contains(&u)))
        .cloned()
        .collect();
    if !blind.is_empty() {
        let mut uses: Vec<Value> = fresh["uses"].as_array().cloned().unwrap_or_default();
        uses.extend(blind);
        uses.sort_by_key(|u| serde_json::from_value::<wprs_scene::ScriptApi>(u.clone()).ok());
        uses.dedup();
        fresh["uses"] = json!(uses);
    }
    if with_params {
        let declared = fresh["params"].as_object().cloned().unwrap_or_default();
        let missing: Vec<String> =
            used_params(&source).into_iter().filter(|p| !declared.contains_key(p)).collect();
        let mut all = declared;
        missing.into_iter().take(wprs_scene::MAX_SCRIPT_PARAMS.saturating_sub(all.len())).for_each(|p| {
            all.insert(p, json!({"type": "number", "default": 0}));
        });
        if !all.is_empty() {
            fresh["params"] = Value::Object(all);
        }
    }
    Ok(if fresh == *old { Vec::new() } else { place(doc, fresh) })
}

pub(crate) fn edit(dir: &Path, doc: &Value, e: &E) -> Result<Vec<Op>> {
    match e {
        E::Add { path } => {
            ensure!(path.ends_with(".js"), said("editor.not_js_file", "%1 is not a .js file", &[&path]));
            ensure!(
                index(doc, path).is_err(),
                said("editor.script_declared", "%1 is already one of the scripts", &[&path])
            );
            let source = text(dir, path)?;
            room(dir, doc, path, source.len() as u64)?;
            Ok(place(doc, entry(doc, path, &source)))
        }
        E::Remove { path } => {
            let at = index(doc, path)?;
            let users = bound(doc, path);
            ensure!(
                users.is_empty(),
                said(
                    "editor.script_followed",
                    "%1 is still followed by %2; change them first",
                    &[&path, &users.join(", ")]
                )
            );
            Ok(vec![Op::Remove { path: format!("/scene/scripts/{at}") }])
        }
        E::Move { path, to } => {
            let at = index(doc, path)?;
            let last = doc["scene"]["scripts"].as_array().map_or(0, Vec::len) - 1;
            let to = (*to).min(last);
            let from = format!("/scene/scripts/{at}");
            Ok(if to == at {
                Vec::new()
            } else {
                vec![Op::Move { from, path: format!("/scene/scripts/{to}") }]
            })
        }
        E::Declare { path, params } => declare(dir, doc, path, *params),
        E::Write { path, text: code } => {
            index(doc, path)?;
            ensure!(
                code.len() <= MAX_SCRIPT_BYTES,
                said("editor.scripts_scene_take", "the scripts of a scene take at most 1 MiB together", &[])
            );
            room(dir, doc, path, code.len() as u64)?;
            ensure!(
                wprs_format::check_path(path).is_ok(),
                said("editor.script_path", "%1 is not a path of the package", &[&path])
            );
            fs::write(dir.join(path), code)
                .map_err(|e| said("editor.cannot_write", "cannot write %1: %2", &[&path, &e]))?;
            Ok(Vec::new())
        }
        E::Param { path, id, kind, default, min, max, to, remove } => {
            let given = Param {
                id,
                kind: kind.as_deref(),
                default: default.as_ref(),
                range: (*min, *max),
                to: to.as_deref(),
                remove: *remove,
            };
            params::edit(doc, path, &given)
        }
    }
}
