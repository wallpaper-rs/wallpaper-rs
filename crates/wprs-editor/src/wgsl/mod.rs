mod form;
mod starters;
mod trial;

use crate::action::ShaderEdit as E;
use crate::ops::{Op, escape};
use anyhow::{Result, anyhow, ensure};
use serde_json::Value;
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::{Descriptor, MAX_PARAMS, MAX_PASSES, MAX_WGSL, MAX_WGSL_TOTAL};

pub use starters::{TEMPLATES, starter};
pub(crate) use trial::files;
pub use trial::{check, located, source};

pub(crate) const QUERIES: [&str; 2] = ["shader_source", "shader_check"];

pub fn asks(q: &Value) -> bool {
    QUERIES.iter().any(|k| q.get(k).is_some())
}

pub fn query(dir: &Path, doc: &Value, q: &Value) -> Value {
    match q.get("shader_check") {
        Some(c) => trial::checked(dir, doc, c),
        None => source(dir, doc, q["shader_source"]["effect"].as_str().unwrap_or_default()),
    }
}

pub(crate) fn create(dir: &Path, doc: &Value, name: &str, template: &str) -> Result<Vec<Op>> {
    crate::clips::valid(name)?;
    ensure!(
        doc.pointer(&format!("/scene/effects/{}", escape(name))).is_none() && !wprs_scene::reserved(name),
        said("editor.material_taken", "there is already a shader named %1", &[&name])
    );
    let (descriptor, wgsl) = starter(name, template).ok_or_else(|| {
        anyhow!(said("editor.no_shader_template", "there is no shader template %1", &[&template]))
    })?;
    std::fs::create_dir_all(dir.join("effects"))?;
    let (json, code) = (dir.join(format!("effects/{name}.json")), dir.join(format!("effects/{name}.wgsl")));
    let body = serde_json::to_string_pretty(&descriptor)?;
    for (path, text) in [(&json, body), (&code, wgsl)] {
        if !path.exists() {
            std::fs::write(path, text)?;
        }
    }
    crate::shaders::import(dir, doc, &json, Some(&code))
}

fn shaders_size(dir: &Path, doc: &Value, except: &str) -> u64 {
    let effects = doc["scene"]["effects"].as_object().into_iter().flatten();
    let others = effects.filter_map(|(_, e)| e["wgsl"].as_str()).filter(|p| *p != except);
    others.filter_map(|p| std::fs::metadata(dir.join(p)).ok()).map(|m| m.len()).sum()
}

fn valid(id: &str, json: &Value) -> Result<String> {
    let text = serde_json::to_string_pretty(json)?;
    let fail = |e: &dyn std::fmt::Display| anyhow!(said("editor.effect", "the effect %1 %2", &[&id, &e]));
    ensure!(json["id"] == id, fail(&format!("declares the id {}, expected {id}", json["id"])));
    ensure!(text.len() <= MAX_WGSL, fail(&format!("has a descriptor larger than {} KiB", MAX_WGSL >> 10)));
    let d: Descriptor = serde_json::from_value(json.clone()).map_err(|e| fail(&e))?;
    ensure!(d.params.len() <= MAX_PARAMS, fail(&format!("has more than {MAX_PARAMS} parameters")));
    let lighter = d.lighter.as_ref().map_or(0, |l| l.passes.len());
    ensure!(d.passes.len().max(lighter) <= MAX_PASSES, fail(&format!("has more than {MAX_PASSES} passes")));
    if let Some(p) = d.params.iter().find(|p| !p.accepts(&p.default)) {
        return Err(fail(&format!("gives {} a default that does not fit its type", p.id)));
    }
    if let Some(p) = d.params.iter().find(|p| p.min.zip(p.max).is_some_and(|(lo, hi)| lo > hi)) {
        return Err(fail(&format!("has min above max for {}", p.id)));
    }
    Ok(text)
}

fn renamed(doc: &Value, effect: &str, from: &str, to: &str) -> Vec<Op> {
    let mut ops = Vec::new();
    for (at, layer) in crate::layers::walk(doc) {
        for (i, e) in layer["effects"].as_array().into_iter().flatten().enumerate() {
            if e["effect"] == effect && e["params"].get(from).is_some() {
                let base = format!("{at}/effects/{i}/params");
                ops.push(Op::Move {
                    from: format!("{base}/{}", escape(from)),
                    path: format!("{base}/{}", escape(to)),
                });
            }
        }
        for (name, m) in layer["materials"].as_object().into_iter().flatten() {
            if m["shader"] == effect && m["params"].get(from).is_some() {
                let base = format!("{at}/materials/{}/params", escape(name));
                ops.push(Op::Move {
                    from: format!("{base}/{}", escape(from)),
                    path: format!("{base}/{}", escape(to)),
                });
            }
        }
    }
    ops
}

pub(crate) fn act(dir: &Path, doc: &Value, e: &E) -> Result<Vec<Op>> {
    if let E::New { name, template } = e {
        return create(dir, doc, name, template.as_deref().unwrap_or("color"));
    }
    let effect = match e {
        E::Write { effect, .. }
        | E::Param { effect, .. }
        | E::Pass { effect, .. }
        | E::Lighter { effect, .. }
        | E::Field { effect, .. } => effect,
        E::New { .. } => unreachable!("handled above"),
    };
    let (descriptor, wgsl) = trial::files(doc, effect).ok_or_else(|| {
        anyhow!(said("editor.no_package_effect", "%1 is no effect of this package", &[&effect]))
    })?;
    if let E::Write { wgsl: code, descriptor: given, .. } = e {
        let mut ops = Vec::new();
        if let Some(code) = code {
            ensure!(
                code.len() <= MAX_WGSL
                    && code.len() as u64 + shaders_size(dir, doc, &wgsl) <= MAX_WGSL_TOTAL as u64,
                said(
                    "editor.shaders_take",
                    "a shader is at most 64 KiB and the shaders of a scene 256 KiB together",
                    &[]
                )
            );
            ops.extend(crate::scripts::save(dir, doc, &wgsl, code)?);
        }
        if let Some(given) = given {
            ops.extend(crate::scripts::save(dir, doc, &descriptor, &valid(effect, given)?)?);
        }
        return Ok(ops);
    }
    let text = std::fs::read_to_string(dir.join(&descriptor))
        .map_err(|e| said("editor.cannot_read", "cannot read %1: %2", &[&descriptor, &e]))?;
    let mut json: Value = serde_json::from_str(&text)
        .map_err(|e| said("editor.descriptor_json", "the descriptor is not valid JSON: %1", &[&e]))?;
    let mut more = Vec::new();
    match e {
        E::Param { id, set, clear, to, at, remove, .. } => {
            let change = form::ParamChange { id, set, clear, to: to.as_deref(), at: *at, remove: *remove };
            form::param(&mut json, &change)?;
            more = to.as_deref().map(|to| renamed(doc, effect, id, to)).unwrap_or_default();
        }
        E::Pass { index, set, clear, remove, lighter, .. } => {
            let (index, remove, lighter) = (*index, *remove, *lighter);
            form::pass(&mut json, &form::PassChange { index, set, clear, remove, lighter })?;
        }
        E::Lighter { set, remove, .. } if form::lighter(&mut json, set, *remove)? => {
            more = starters::lighter(dir, doc, effect, &wgsl)?;
        }
        E::Lighter { .. } => {}
        E::Field { field, value, .. } => form::field(&mut json, field, value)?,
        _ => {}
    }
    let mut ops = crate::scripts::save(dir, doc, &descriptor, &valid(effect, &json)?)?;
    ops.extend(more);
    Ok(ops)
}
