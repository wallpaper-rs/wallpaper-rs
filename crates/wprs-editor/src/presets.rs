use crate::action::Needed;
use crate::findings::{Problem, problem};
use crate::ops::{Op, escape, set};
use anyhow::{Result, ensure};
use serde_json::{Map, Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_format::PresetFile;

pub const ENTRY: &str = "preset.json";
const BASE: &str = "/preset/base";

pub fn blank() -> Value {
    json!({"base": {"id": "", "version": "*"}})
}

pub(crate) fn attach(dir: &Path, mut doc: Value) -> Result<Value> {
    if crate::pages::kind(&doc) == "preset" {
        let name = doc["manifest"]["entry"].as_str().unwrap_or(ENTRY).to_owned();
        wprs_format::check_path(&name)?;
        let text = std::fs::read_to_string(dir.join(&name))
            .map_err(|e| said("editor.cannot_read", "cannot read %1: %2", &[&name, &e]))?;
        doc["preset"] = serde_json::from_str(&text)
            .map_err(|e| said("editor.not_json", "%1 is not valid JSON: %2", &[&name, &e]))?;
    }
    Ok(doc)
}

pub(crate) fn write(dir: &Path, doc: &Value) -> Result<()> {
    if crate::pages::kind(doc) != "preset" {
        return Ok(());
    }
    let name = doc["manifest"]["entry"].as_str().unwrap_or(ENTRY);
    wprs_format::check_path(name)?;
    let text = serde_json::to_string_pretty(&doc["preset"])? + "\n";
    crate::session::write(&dir.join(name), &text, 0o644)
}

pub(crate) fn review(doc: &Value, dir: Option<&Path>) -> Vec<Problem> {
    if crate::pages::kind(doc) != "preset" {
        return Vec::new();
    }
    let mut out = Vec::new();
    let manifest = crate::meta::read(doc, dir);
    let text = serde_json::to_vec(&doc["preset"]).unwrap_or_default();
    if let Some(e) = manifest.ok().and_then(|m| PresetFile::parse(&text, &m).err()) {
        out.push(problem("/preset", said("editor.preset", "%1", &[&e])));
    }
    let files = doc["preset"]["files"].as_object().into_iter().flatten();
    for (id, path) in files.filter_map(|(id, p)| Some((id, p.as_str()?))) {
        if dir.is_some_and(|d| !d.join(path).is_file()) {
            let at = format!("/preset/files/{}", escape(id));
            out.push(problem(at, said("editor.not_in_folder", "%1 is not in the folder", &[&path])));
        }
    }
    out
}

pub(crate) fn edit(
    dir: &Path,
    doc: &Value,
    base: &Option<Needed>,
    values: &Map<String, Value>,
    clear: &[String],
    files: &Map<String, Value>,
) -> Result<Vec<Op>> {
    ensure!(
        crate::pages::kind(doc) == "preset",
        said("editor.preset_only", "only a preset package holds the values of another package", &[])
    );
    let mut ops = Vec::new();
    if let Some(b) = base {
        let version = b.version.as_deref().map_or(Ok("*".to_owned()), crate::shaping::needs::range)?;
        ensure!(
            b.id != doc["manifest"]["id"].as_str().unwrap_or_default(),
            said("editor.needs_itself", "a package cannot require itself", &[])
        );
        let dependency = json!({"id": b.id, "version": version});
        ops.extend(set(doc, BASE, dependency.clone()));
        ops.extend(set(doc, "/manifest/requires/packages", json!([dependency])));
    }
    let map = |key: &str| doc["preset"][key].as_object().cloned().unwrap_or_default();
    let (mut now, mut copies) = (map("values"), map("files"));
    for id in clear {
        now.remove(id);
        copies.remove(id);
    }
    for (id, value) in values {
        crate::props::valid(id)?;
        copies.remove(id);
        now.insert(id.clone(), value.clone());
    }
    for (id, source) in files {
        crate::props::valid(id)?;
        let file = Path::new(source.as_str().unwrap_or_default());
        let kind = crate::layers::asset_kind(&file.to_string_lossy());
        ensure!(
            ["image", "video"].contains(&kind),
            said("editor.preset_file_media", "%1 is not a picture or a video", &[&file.display()])
        );
        now.remove(id);
        copies.insert(id.clone(), json!(crate::pages::copy_in(dir, file, Some("assets"))?));
    }
    for (key, next) in [("values", now), ("files", copies)] {
        let at = format!("/preset/{key}");
        let kept = (!next.is_empty()).then_some(Value::Object(next));
        match kept {
            Some(v) => ops.extend(set(doc, &at, v)),
            None if doc.pointer(&at).is_some() => ops.push(Op::Remove { path: at }),
            None => {}
        }
    }
    Ok(ops)
}
