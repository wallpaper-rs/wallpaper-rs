mod audit;
mod changes;
mod declare;
mod edit;
mod listing;
mod params;
mod reference;
mod revert;
mod rigcalls;

use crate::ops::Op;
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::{MAX_SCRIPT_BYTES, MAX_SCRIPTS2};

pub use audit::audit;
pub use changes::changes;
pub use declare::{declaration, functions, used_params, valid_param};
pub(crate) use edit::edit;
pub use listing::{bound, listing, stamp, stamped};
pub use reference::reference;
pub(crate) use revert::{restore, save};
pub use rigcalls::calls;

pub(crate) fn room(dir: &Path, doc: &Value, path: &str, size: u64) -> Result<()> {
    let list = doc.pointer("/scene/scripts").and_then(Value::as_array).cloned().unwrap_or_default();
    let others = list.iter().filter_map(|s| s["path"].as_str()).filter(|p| *p != path);
    let total: u64 = others.clone().filter_map(|p| fs::metadata(dir.join(p)).ok()).map(|m| m.len()).sum();
    ensure!(
        size + total <= MAX_SCRIPT_BYTES as u64,
        said("editor.scripts_scene_take", "the scripts of a scene take at most 1 MiB together", &[])
    );
    ensure!(
        others.count() < MAX_SCRIPTS2,
        said("editor.scene_most_scripts", "a scene has at most %1 scripts", &[&MAX_SCRIPTS2])
    );
    Ok(())
}

pub(crate) fn entry(doc: &Value, path: &str, source: &str) -> Value {
    let mut entry = declaration(source);
    entry["path"] = json!(path);
    let old = doc.pointer("/scene/scripts").and_then(Value::as_array);
    if let Some(params) = old.and_then(|l| l.iter().find(|s| s["path"] == path)).and_then(|s| s.get("params"))
    {
        entry["params"] = params.clone();
    }
    entry
}

pub(crate) fn place(doc: &Value, entry: Value) -> Vec<Op> {
    let list = doc.pointer("/scene/scripts").and_then(Value::as_array).cloned().unwrap_or_default();
    match list.iter().position(|s| s["path"] == entry["path"]) {
        Some(i) => vec![Op::Replace { path: format!("/scene/scripts/{i}"), value: entry }],
        None if list.is_empty() && doc.pointer("/scene/scripts").is_none() => {
            vec![Op::Add { path: "/scene/scripts".into(), value: json!([entry]) }]
        }
        None => vec![Op::Add { path: "/scene/scripts/-".into(), value: entry }],
    }
}

pub fn import(dir: &Path, doc: &Value, file: &Path, template: Option<&str>) -> Result<Vec<Op>> {
    let free = template.filter(|_| file.as_os_str().is_empty()).map(|t| crate::molds::free_name(dir, t));
    let file = free.as_deref().map_or(file, Path::new);
    let name = file
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| said("editor.script_name", "the script has no file name", &[]))?;
    ensure!(name.ends_with(".js"), said("editor.not_js_file", "%1 is not a .js file", &[&name]));
    let made = template.map(|t| crate::molds::script_template(t).ok_or(t)).transpose();
    let made = made.map_err(|t| said("editor.no_template", "there is no script template %1", &[&t]))?;
    ensure!(
        made.is_none() || !dir.join("scripts").join(name).exists(),
        said("editor.script_taken", "scripts/%1 already exists", &[&name])
    );
    let size = match made {
        Some(source) => source.len() as u64,
        None => fs::metadata(file)
            .map_err(|e| said("editor.cannot_read", "cannot read %1: %2", &[&file.display(), &e]))?
            .len(),
    };
    let path = format!("scripts/{name}");
    room(dir, doc, &path, size)?;
    let source = match made {
        Some(source) => source.to_owned(),
        None => fs::read_to_string(file)
            .map_err(|e| said("editor.not_text", "%1 is not a text file: %2", &[&file.display(), &e]))?,
    };
    fs::create_dir_all(dir.join("scripts"))?;
    fs::write(dir.join(&path), &source)
        .map_err(|e| said("editor.cannot_write", "cannot write %1: %2", &[&path, &e]))?;
    let mut entry = entry(doc, &path, &source);
    if let Some(params) =
        template.and_then(crate::molds::script_params).filter(|_| entry.get("params").is_none())
    {
        entry["params"] = params;
    }
    Ok(place(doc, entry))
}
