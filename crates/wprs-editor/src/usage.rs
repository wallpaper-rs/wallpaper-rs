use crate::action::UsesEdit;
use crate::findings::{Problem, warning};
use crate::ops::{Op, set};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::{ScriptApi, imported};

const CALLS: [(&str, ScriptApi); 5] = [
    (".bloom", ScriptApi::Bloom),
    (".spawn(", ScriptApi::Layers),
    (".order(", ScriptApi::Layers),
    (".effect(", ScriptApi::Effects),
    ("console.", ScriptApi::Console),
];

const MEMBERS: [(&str, ScriptApi); 4] = [
    (".setParent(", ScriptApi::Hierarchy),
    (".children", ScriptApi::Hierarchy),
    (".world", ScriptApi::Hierarchy),
    (".parent", ScriptApi::Hierarchy),
];

const STAGE: [&str; 7] = ["camera", "camera3d", "parallax", "bloom", "background", "ambient", "sky"];
const FREE: [ScriptApi; 4] = [ScriptApi::Audio, ScriptApi::Pointer, ScriptApi::Clicks, ScriptApi::Particles];

fn word(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

fn standing(source: &str, call: &str) -> bool {
    let left = |before: &str| match before.chars().next_back() {
        Some('.') => before.ends_with("wprs."),
        Some(c) => !word(c),
        None => true,
    };
    let mut found = source.match_indices(call);
    found.any(|(i, _)| left(&source[..i]) && !source[i + call.len()..].starts_with(word))
}

fn member(source: &str, call: &str) -> bool {
    let open = call.ends_with('(');
    source.match_indices(call).any(|(i, _)| open || !source[i + call.len()..].starts_with(word))
}

fn staged(source: &str) -> bool {
    STAGE.iter().any(|key| standing(source, &format!("scene.{key}")))
}

fn unpacked(source: &str) -> bool {
    source.match_indices('}').any(|(i, _)| {
        let rest = source[i + 1..].trim_start();
        let Some(rest) = rest.strip_prefix('=').map(str::trim_start) else { return false };
        let rest = rest.strip_prefix("wprs.").unwrap_or(rest);
        let named = rest.strip_prefix("scene").is_some_and(|after| !after.starts_with(word));
        let open = source[..i].rfind('{').unwrap_or(i);
        named && source[open + 1..i].split(|c: char| !word(c)).any(|w| STAGE.contains(&w))
    })
}

pub fn detect(source: &str) -> Vec<ScriptApi> {
    let mut uses = imported(source);
    uses.extend(CALLS.iter().filter(|(call, _)| source.contains(call)).map(|(_, api)| *api));
    uses.extend(MEMBERS.iter().filter(|(call, _)| member(source, call)).map(|(_, api)| *api));
    uses.extend((staged(source) || unpacked(source)).then_some(ScriptApi::Camera));
    uses.extend(standing(source, "scene.mesh").then_some(ScriptApi::Geometry));
    uses.extend(standing(source, "openShortcut").then_some(ScriptApi::Shortcuts));
    uses.sort();
    uses.dedup();
    uses
}

pub fn missing(declared: &[ScriptApi], source: &str) -> Vec<ScriptApi> {
    detect(source).into_iter().filter(|u| !declared.contains(u) && !FREE.contains(u)).collect()
}

fn declared(script: &Value) -> Vec<ScriptApi> {
    serde_json::from_value(script["uses"].clone()).unwrap_or_default()
}

fn names(list: &[ScriptApi]) -> Vec<Value> {
    list.iter().map(|a| json!(a)).collect()
}

pub fn report(dir: &Path, doc: &Value) -> Value {
    let scripts = doc["scene"]["scripts"].as_array().into_iter().flatten().enumerate();
    let rows = scripts.filter_map(|(i, s)| {
        let path = s["path"].as_str()?;
        let source = std::fs::read_to_string(dir.join(path)).ok()?;
        let (have, lack) = (declared(s), missing(&declared(s), &source));
        Some(json!({ "path": path, "pointer": format!("/scene/scripts/{i}"), "declared": names(&have),
            "detected": names(&detect(&source)), "missing": names(&lack) }))
    });
    json!({ "scripts": rows.collect::<Vec<_>>() })
}

pub(crate) fn declare(dir: &Path, doc: &Value, e: &UsesEdit) -> Result<Vec<Op>> {
    let list = doc["scene"]["scripts"].as_array().cloned().unwrap_or_default();
    let wanted = |s: &Value| e.path.as_deref().is_none_or(|p| s["path"] == p);
    ensure!(
        list.iter().any(wanted),
        said("editor.there_no_script", "the scene has no script %1", &[&e.path.clone().unwrap_or_default()])
    );
    let (mut ops, mut geometry) = (Vec::new(), false);
    for (i, script) in list.iter().enumerate().filter(|(_, s)| wanted(s)) {
        let path = script["path"].as_str().unwrap_or_default();
        let source = std::fs::read_to_string(dir.join(path)).unwrap_or_default();
        let mut uses = declared(script);
        let explicit = !e.add.is_empty() || !e.remove.is_empty();
        let more = if explicit { e.add.clone() } else { missing(&uses, &source) };
        uses.extend(more);
        uses.retain(|u| !e.remove.contains(u));
        uses.sort();
        uses.dedup();
        geometry |= uses.contains(&ScriptApi::Geometry);
        if uses != declared(script) {
            ops.extend(set(doc, &format!("/scene/scripts/{i}/uses"), json!(uses)));
        }
    }
    let pools = doc["scene"]["pools"].as_object().is_some_and(|p| !p.is_empty());
    if geometry && !pools {
        let pool = json!({ "type": "mesh", "max": 16, "max_vertices": 1024 });
        ops.extend(set(doc, "/scene/pools/shapes", pool));
    }
    crate::shaping::refused(dir, doc, ops)
}

pub(crate) fn problems(dir: &Path, doc: &Value, skip: Option<usize>) -> Vec<Problem> {
    let scripts = doc["scene"]["scripts"].as_array().into_iter().flatten().enumerate();
    let mut out = Vec::new();
    for (i, script) in scripts.filter(|(i, _)| Some(*i) != skip) {
        let Some(path) = script["path"].as_str() else { continue };
        let Ok(source) = std::fs::read_to_string(dir.join(path)) else { continue };
        let lack = missing(&declared(script), &source);
        if lack.is_empty() {
            continue;
        }
        let list = lack.iter().map(|a| json!(a).as_str().unwrap_or_default().to_owned()).collect::<Vec<_>>();
        let said = said(
            "editor.script_uses",
            "%1 uses %2 but does not declare it; declare it in its uses",
            &[&path, &list.join(", ")],
        );
        let mut found = warning(format!("/scene/scripts/{i}"), said);
        found.fix = Some(json!({ "uses": { "path": path } }));
        out.push(found);
    }
    out
}
