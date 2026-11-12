use crate::ops::{Op, escape};
use anyhow::{Result, anyhow, ensure};
use serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::{Descriptor, MAX_EFFECT_FILES, MAX_WGSL, Quality};

fn text(path: &Path) -> Result<String> {
    let size = fs::metadata(path)
        .map_err(|e| said("editor.cannot_read", "cannot read %1: %2", &[&path.display(), &e]))?
        .len();
    ensure!(
        size <= MAX_WGSL as u64,
        said("editor.larger_than_kib", "%1 is larger than %2 KiB", &[&(path.display()), &(MAX_WGSL >> 10)])
    );
    fs::read_to_string(path)
        .map_err(|e| said("editor.not_text", "%1 is not a text file: %2", &[&path.display(), &e]).into())
}

pub fn import(dir: &Path, doc: &Value, descriptor: &Path, wgsl: Option<&Path>) -> Result<Vec<Op>> {
    let shader = wgsl.map_or_else(|| descriptor.with_extension("wgsl"), Path::to_path_buf);
    let (json, code) = (text(descriptor)?, text(&shader)?);
    let parsed: Value = serde_json::from_str(&json)
        .map_err(|e| said("editor.descriptor_json", "the descriptor is not valid JSON: %1", &[&e]))?;
    let id = parsed["id"]
        .as_str()
        .ok_or_else(|| said("editor.descriptor_id", "the descriptor has no id", &[]))?
        .to_owned();
    Descriptor::packaged(&id, &json, &code)
        .map_err(|e| anyhow!(said("editor.effect", "the effect %1 %2", &[&id, &e])))?;
    let effects = doc.pointer("/scene/effects").and_then(Value::as_object);
    let known = effects.is_some_and(|e| e.contains_key(&id));
    let count = effects.map_or(0, Map::len);
    ensure!(
        known || count < MAX_EFFECT_FILES,
        said("editor.scene_most_effects", "a scene has at most %1 effects of its own", &[&MAX_EFFECT_FILES])
    );
    fs::create_dir_all(dir.join("effects"))?;
    let paths = [format!("effects/{id}.json"), format!("effects/{id}.wgsl")];
    for (path, body) in paths.iter().zip([&json, &code]) {
        fs::write(dir.join(path), body)
            .map_err(|e| said("editor.cannot_write", "cannot write %1: %2", &[&path, &e]))?;
    }
    let value = json!({ "descriptor": paths[0], "wgsl": paths[1] });
    let path = format!("/scene/effects/{}", escape(&id));
    Ok(match (effects.is_some(), known) {
        (false, _) => vec![Op::Add {
            path: "/scene/effects".into(),
            value: Value::Object([(id, value)].into_iter().collect()),
        }],
        (true, true) => vec![Op::Replace { path, value }],
        (true, false) => vec![Op::Add { path, value }],
    })
}

fn users(doc: &Value, id: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for (_, layer) in crate::layers::walk(doc) {
        let layer_id = layer["id"].as_str().unwrap_or_default();
        let effects = layer["effects"].as_array().into_iter().flatten();
        let materials = layer["materials"].as_object().into_iter().flatten();
        let used = effects.filter_map(|e| e["effect"].as_str()).any(|e| e == id)
            || materials.filter_map(|(_, m)| m["shader"].as_str()).any(|s| s == id);
        if used {
            found.push(layer_id.to_owned());
        }
    }
    found.sort();
    found.dedup();
    found
}

pub fn remove(doc: &Value, id: &str) -> Result<Vec<Op>> {
    let path = format!("/scene/effects/{}", escape(id));
    ensure!(
        doc.pointer(&path).is_some(),
        said("editor.no_package_effect", "%1 is no effect of this package", &[&id])
    );
    let users = users(doc, id);
    ensure!(
        users.is_empty(),
        said(
            "editor.effect_still_used",
            "%1 is still used by %2; remove it from there first",
            &[&id, &(users.join(", "))]
        )
    );
    Ok(vec![Op::Remove { path }])
}

pub fn rule(dir: &Path, doc: &Value, r: &crate::actions::RuleEdit) -> Result<Vec<Op>> {
    let path = format!("/scene/effects/{}", escape(&r.effect));
    let entry = doc.pointer(&path).cloned().ok_or_else(|| {
        anyhow!(said("editor.no_package_effect", "%1 is no effect of this package", &[&r.effect]))
    })?;
    let file = |key: &str| entry[key].as_str().map(|p| dir.join(p)).ok_or_else(|| anyhow!("{path}/{key}"));
    let (descriptor, wgsl) = (file("descriptor")?, file("wgsl")?);
    let mut json: Value = serde_json::from_str(&text(&descriptor)?)?;
    let params = json["params"].as_array_mut().into_iter().flatten();
    let param = params.into_iter().find(|p| p["id"] == r.param.as_str()).ok_or_else(|| {
        anyhow!(said("editor.no_parameter", "%1 has no parameter %2", &[&r.effect, &r.param]))
    })?;
    for (key, wanted) in [("uses", &r.uses), ("waits_for", &r.waits_for), ("yields_to", &r.yields_to)] {
        match wanted.as_deref() {
            None => {}
            Some("") => drop(param.as_object_mut().and_then(|m| m.remove(key))),
            Some(v) => param[key] = json!(v),
        }
    }
    let body = serde_json::to_string_pretty(&json)?;
    Descriptor::packaged(&r.effect, &body, &text(&wgsl)?)
        .map_err(|e| anyhow!(said("editor.effect", "the effect %1 %2", &[&r.effect, &e])))?;
    let free = |n: usize| format!("effects/{}-{n}.json", r.effect);
    let written =
        (2..).map(free).take(10_000).find(|p| !dir.join(p).exists() && !crate::revisions::kept(dir, p));
    let written = written.ok_or_else(|| anyhow!("effects/{}", r.effect))?;
    fs::write(dir.join(&written), body)
        .map_err(|e| said("editor.cannot_write", "cannot write %1: %2", &[&written, &e]))?;
    Ok(vec![Op::Replace { path: format!("{path}/descriptor"), value: json!(written) }])
}

pub fn cost(dir: &Path, doc: &Value, id: &str, optional: bool) -> Value {
    let Ok(mut scene) = serde_json::from_value::<wprs_scene::Scene>(doc["scene"].clone()) else {
        return Value::Null;
    };
    crate::cache::read_files(&mut scene, dir);
    let found = scene.loaded.get(id).map(|d| (**d).clone()).or_else(|| wprs_scene::descriptor(id).cloned());
    let Some(d) = found else { return Value::Null };
    json!({ "effect": id, "units": d.units(), "lighter": d.lighter_units(), "levels": levels(&d, optional) })
}

pub fn levels(d: &wprs_scene::Descriptor, optional: bool) -> Vec<Value> {
    let level = |quality: Quality| {
        let drawn = d.live(quality, optional);
        let scale =
            drawn.and_then(|l| quality.effect_scale(l.class(), optional).map(|s| s * l.scale_at(quality)));
        let units = drawn.zip(scale).map(|(l, s)| l.units() * s * s);
        json!({ "quality": quality, "units": units, "form": d.form(quality, optional) })
    };
    [Quality::Q0, Quality::Q1, Quality::Q2, Quality::Q3].into_iter().map(level).collect()
}

pub fn rules(dir: &Path, doc: &Value, id: &str) -> Value {
    let Ok(mut scene) = serde_json::from_value::<wprs_scene::Scene>(doc["scene"].clone()) else {
        return Value::Null;
    };
    crate::cache::read_files(&mut scene, dir);
    let found = scene.loaded.get(id).map(|d| (**d).clone()).or_else(|| wprs_scene::descriptor(id).cloned());
    let Some(d) = found else { return Value::Null };
    let params = d.params.iter().map(|p| {
        json!({ "id": p.id, "kind": p.kind, "uses": p.uses, "waits_for": p.waits_for, "yields_to": p.yields_to })
    });
    let own = doc.pointer(&format!("/scene/effects/{}", escape(id))).is_some();
    json!({ "effect": id, "own": own, "uses": d.uses, "settle": d.state.as_ref().map(|s| s.settle), "params": params.collect::<Vec<_>>() })
}
