use super::refs::swap;
use super::rename::{WORDS, strings};
use crate::layers::{children, find, taken, unique_among};
use crate::ops::{Op, escape};
use anyhow::{Result, ensure};
use serde_json::{Map, Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use wprs_core::message::said;

fn mentions(v: &Value, keys: &Map<String, Value>, out: &mut Vec<String>) {
    match v {
        Value::String(s) if keys.contains_key(s) && !out.contains(s) => out.push(s.clone()),
        Value::Array(list) => list.iter().for_each(|x| mentions(x, keys, out)),
        Value::Object(map) => {
            map.iter().filter(|(k, _)| !WORDS.contains(&k.as_str())).for_each(|(_, x)| mentions(x, keys, out))
        }
        _ => {}
    }
}

fn picked(scene: &Value, section: &str, from: &[Value]) -> Map<String, Value> {
    let all = scene[section].as_object().cloned().unwrap_or_default();
    let mut found = Vec::new();
    from.iter().for_each(|v| mentions(v, &all, &mut found));
    let mut i = 0;
    while i < found.len() {
        mentions(&all[&found[i]], &all, &mut found);
        i += 1;
    }
    found.into_iter().map(|k| (k.clone(), all[&k].clone())).collect()
}

pub fn copy(dir: &Path, doc: &Value, ids: &[String]) -> Result<Value> {
    let layers: Vec<Value> = ids
        .iter()
        .map(|id| Ok(doc.pointer(&find(doc, id)?).cloned().unwrap_or_default()))
        .collect::<Result<_>>()?;
    let scene = &doc["scene"];
    let particles = picked(scene, "particles", &layers);
    let mut users = layers.clone();
    users.extend(particles.values().cloned());
    let effects = picked(scene, "effects", &users);
    users.extend(effects.values().cloned());
    let assets = picked(scene, "assets", &users);
    let from = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    Ok(
        json!({ "from": from, "layers": layers, "assets": assets, "particles": particles, "effects": effects }),
    )
}

fn ids(layer: &Value, out: &mut Vec<String>) {
    out.extend(layer["id"].as_str().map(String::from));
    layer["layers"].as_array().into_iter().flatten().for_each(|l| ids(l, out));
}

fn placed(dir: &Path, from: &Path, path: &str, used: &mut Vec<String>) -> Result<String> {
    wprs_format::check_path(path)?;
    let bytes = std::fs::read(from.join(path))?;
    let (stem, ext) = path.rsplit_once('.').map_or((path, ""), |(s, e)| (s, e));
    for n in 1..10_000 {
        let name = if n == 1 { path.to_owned() } else { format!("{stem}_{n}.{ext}") };
        let target = dir.join(&name);
        let same = std::fs::read(&target).ok().map(|old| old == bytes);
        match same {
            Some(true) if !used.contains(&name) => return Ok(name),
            None if !used.contains(&name) => {
                std::fs::create_dir_all(target.parent().unwrap_or(dir))?;
                std::fs::write(&target, &bytes)?;
                used.push(name.clone());
                return Ok(name);
            }
            _ => {}
        }
    }
    Ok(path.to_owned())
}

fn moved(dir: &Path, from: &Path, v: &mut Value, keys: &[&str], used: &mut Vec<String>) -> Result<()> {
    for key in keys {
        if let Some(path) = v[*key].as_str().map(String::from) {
            v[*key] = json!(placed(dir, from, &path, used)?);
        }
    }
    Ok(())
}

pub fn paste(
    dir: &Path,
    doc: &Value,
    clip: &Value,
    into: Option<&str>,
    at: Option<usize>,
) -> Result<Vec<Op>> {
    let layers = clip["layers"].as_array().cloned().unwrap_or_default();
    ensure!(!layers.is_empty(), said("editor.paste_nothing", "there are no layers to paste", &[]));
    let from = PathBuf::from(clip["from"].as_str().unwrap_or_default());
    let (mut taken_ids, mut names, mut files) = (Vec::new(), BTreeMap::new(), Vec::new());
    let fresh = |id: &str, used: &mut Vec<String>| {
        let new = if !taken(doc, id) && !used.contains(&id.to_owned()) {
            id.to_owned()
        } else {
            unique_among(doc, id, used)
        };
        used.push(new.clone());
        new
    };
    let mut all = Vec::new();
    layers.iter().for_each(|l| ids(l, &mut all));
    for id in all {
        let new = fresh(&id, &mut taken_ids);
        names.insert(id, new);
    }
    let mut ops = Vec::new();
    let mut words: Vec<(String, String)> = Vec::new();
    for section in ["assets", "particles"] {
        for (id, value) in clip[section].as_object().into_iter().flatten() {
            let same = doc["scene"][section].get(id) == Some(value) && from == dir;
            let new = if same { id.clone() } else { fresh(id, &mut taken_ids) };
            if new != *id {
                words.push((id.clone(), new.clone()));
            }
            if !same {
                let mut value = value.clone();
                if section == "assets" {
                    moved(dir, &from, &mut value, &["path"], &mut files)?;
                }
                ops.push((section, new, value));
            }
        }
    }
    for (id, value) in clip["effects"].as_object().into_iter().flatten() {
        if doc["scene"]["effects"].get(id).is_none() {
            let mut value = value.clone();
            moved(dir, &from, &mut value, &["descriptor", "wgsl"], &mut files)?;
            ops.push(("effects", id.clone(), value));
        }
    }
    let rewrite = |v: &mut Value| {
        swap(v, &names);
        words.iter().for_each(|(old, new)| strings(v, old, new, &WORDS));
    };
    let mut out = Vec::new();
    for (section, id, mut value) in ops {
        rewrite(&mut value);
        let base = format!("/scene/{section}");
        out.push(
            match doc.pointer(&base).is_some_and(Value::is_object)
                || out.iter().any(|o: &Op| matches!(o, Op::Add { path, .. } if *path == base))
            {
                true => Op::Add { path: format!("{base}/{}", escape(&id)), value },
                false => Op::Add { path: base, value: json!({ id: value }) },
            },
        );
    }
    let list = children(doc, into)?;
    let len = doc.pointer(&list).and_then(Value::as_array).map_or(0, Vec::len);
    let start = at.unwrap_or(len).min(len);
    for (i, mut layer) in layers.into_iter().enumerate() {
        fn renamed(l: &mut Value, names: &BTreeMap<String, String>) {
            if let Some(new) = l["id"].as_str().and_then(|id| names.get(id)) {
                l["id"] = json!(new);
            }
            l.get_mut("layers")
                .and_then(Value::as_array_mut)
                .into_iter()
                .flatten()
                .for_each(|c| renamed(c, names));
        }
        renamed(&mut layer, &names);
        rewrite(&mut layer);
        out.push(Op::Add { path: format!("{list}/{}", start + i), value: layer });
    }
    let mut staged = doc.clone();
    crate::ops::apply(&mut staged, &out)?;
    let lost: Vec<String> = super::refs::refs(&staged)
        .into_iter()
        .filter(|r| find(&staged, &r.id).is_err())
        .map(|r| r.id)
        .collect();
    if !lost.is_empty() {
        out.extend(super::gone::forget(&staged, &lost, "")?);
    }
    Ok(out)
}
