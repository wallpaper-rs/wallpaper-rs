use crate::ops::Op;
use crate::shelf::Pick;
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use wprs_core::message::said;

const MOST_PARTS: usize = 256;

fn implicit(scene: &Value) -> Vec<Value> {
    let ids = |list: &str, key: &str| -> Vec<String> {
        scene[list]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|v| v[key].as_str().map(str::to_owned))
            .collect()
    };
    let keys = |map: &str| -> Vec<String> {
        scene[map].as_object().into_iter().flatten().map(|(k, _)| k.clone()).collect()
    };
    let named = |kind: &'static str, list: Vec<String>| {
        list.into_iter().map(move |id| json!({ "kind": kind, "id": id }))
    };
    named("layer", ids("layers", "id"))
        .chain(named("particles", keys("particles")))
        .chain(named("script", ids("scripts", "path")))
        .chain(named("effect", keys("effects")))
        .collect()
}

fn titled(title: &str) -> Result<Option<String>> {
    let text = title.trim();
    ensure!(
        text.chars().count() <= 128 && !text.contains(['<', '>', '\n', '\r']),
        said("editor.part_title", "a part title has 1 to 128 characters on one line, without < or >", &[])
    );
    Ok((!text.is_empty()).then(|| text.to_owned()))
}

fn pictured(dir: &Path, file: &Path, n: usize) -> Result<String> {
    let data = fs::read(file)
        .map_err(|e| said("editor.cannot_read", "cannot read %1: %2", &[&file.display(), &e]))?;
    let path = format!("parts/{n}.jpg");
    fs::create_dir_all(dir.join("parts"))?;
    fs::write(dir.join(&path), crate::previews::shrink(&data)?)
        .map_err(|e| said("editor.cannot_write", "cannot write %1: %2", &[&path, &e]))?;
    Ok(path)
}

pub(super) fn part(
    dir: &Path,
    doc: &Value,
    (kind, id): (&str, &str),
    (title, thumbnail, clear): (Option<&str>, Option<&Path>, bool),
) -> Result<Vec<Op>> {
    ensure!(
        doc["manifest"]["kind"] == "assets",
        said("editor.assets_only", "only a package of resources offers parts", &[])
    );
    let scene = &doc["scene"];
    let pick = Pick::parse(&format!("{kind}:{id}"))?;
    ensure!(
        crate::shelf::picked(scene, &pick).is_some(),
        said("editor.no_such_part", "the scene has no %1", &[&pick.key()])
    );
    let mut list = match scene["ext"]["parts"].as_array() {
        Some(list) => list.clone(),
        None => implicit(scene),
    };
    let at = match list.iter().position(|p| p["kind"] == kind && p["id"] == id) {
        Some(at) => at,
        None => {
            ensure!(
                list.len() < MOST_PARTS,
                said("editor.parts_most", "a package offers at most %1 parts", &[&MOST_PARTS])
            );
            list.push(json!({ "kind": kind, "id": id }));
            list.len() - 1
        }
    };
    if let Some(title) = title {
        match titled(title)? {
            Some(text) => list[at]["title"] = json!(text),
            None => drop(list[at].as_object_mut().and_then(|o| o.remove("title"))),
        }
    }
    if clear {
        drop(list[at].as_object_mut().and_then(|o| o.remove("thumbnail")));
    }
    if let Some(file) = thumbnail {
        list[at]["thumbnail"] = json!(pictured(dir, file, at)?);
    }
    Ok(super::effects::put(doc, "/scene/ext/parts", Some(json!(list))))
}
