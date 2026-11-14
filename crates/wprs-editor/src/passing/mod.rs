use crate::ops::{Op, set};
use anyhow::{Result, anyhow, bail, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::{MAX_PARAMS, MAX_TRANSITION_MS, MIN_TRANSITION_MS, Readied, SCHEMA, Scene};

mod ask;
mod edit;
mod starter;

pub use ask::{query, shape};
pub use edit::{TransitionEdit, Way};

pub(crate) fn path(way: Way) -> String {
    format!("/scene/transition/{}", way.key())
}

fn other(way: Way) -> Way {
    match way {
        Way::In => Way::Out,
        Way::Out => Way::In,
    }
}

pub(crate) fn act(dir: &Path, doc: &Value, e: &TransitionEdit) -> Result<Vec<Op>> {
    let at = path(e.way);
    let key = e.way.key();
    if e.remove {
        ensure!(
            doc.pointer(&at).is_some(),
            said("editor.transition_none", "there is no transition %1 to remove", &[&key])
        );
        let alone = doc.pointer(&path(other(e.way))).is_none();
        let gone = if alone { "/scene/transition".to_owned() } else { at };
        return Ok(vec![Op::Remove { path: gone }]);
    }
    let mut way = match doc.pointer(&at) {
        Some(v) => v.clone(),
        None if e.make => fresh(dir, doc, e)?,
        None => {
            bail!(said("editor.transition_make", "there is no transition %1 yet: make it first", &[&key]))
        }
    };
    if let Some(effect) = &e.effect {
        way["descriptor"] = json!(effect);
        way["wgsl"] = json!(wgsl_of(effect));
        if let Some(m) = way.as_object_mut() {
            m.remove("params");
        }
    }
    if let Some(ms) = e.duration_ms {
        way["duration_ms"] = json!(ms);
    }
    if let Some(curve) = &e.curve {
        way["curve"] = curve.clone();
    }
    for (id, value) in e.params.iter().flatten() {
        match value {
            Value::Null => {
                way.get_mut("params").and_then(Value::as_object_mut).map(|m| m.remove(id));
            }
            v => way["params"][id] = v.clone(),
        }
    }
    if way.get("params").is_some_and(|p| p.as_object().is_none_or(|m| m.is_empty())) {
        way.as_object_mut().map(|m| m.remove("params"));
    }
    check(dir, e.way, &way)?;
    Ok(set(doc, &at, way))
}

fn wgsl_of(descriptor: &str) -> String {
    let stem = descriptor.strip_suffix(".json").unwrap_or(descriptor);
    format!("{stem}.wgsl")
}

fn fresh(dir: &Path, doc: &Value, e: &TransitionEdit) -> Result<Value> {
    let twin = doc.pointer(&path(other(e.way)));
    let descriptor = match (&e.effect, twin) {
        (Some(effect), _) => effect.clone(),
        (None, Some(t)) => t["descriptor"].as_str().unwrap_or_default().to_owned(),
        (None, None) => starter::write(dir)?,
    };
    let wgsl = wgsl_of(&descriptor);
    Ok(json!({ "descriptor": descriptor, "wgsl": wgsl, "duration_ms": 1200, "curve": "out_cubic" }))
}

pub(crate) fn readied(dir: &Path, way: Way, value: &Value) -> Result<Readied> {
    let text =
        json!({ "schema": SCHEMA, "canvas": { "size": [16, 16] }, "transition": { way.key(): value } });
    let mut scene = Scene::load(&text.to_string(), &Default::default())
        .map_err(|e| anyhow!(said("editor.transition_rule", "the transition breaks a rule: %1", &[&e])))?;
    let read = crate::reach::reader(dir);
    scene.read_passages(&mut |p: &str, max: usize| {
        read(p).and_then(|b| if b.len() > max { Err(format!("{p} is too large")) } else { Ok(b) })
    });
    let found = match way {
        Way::In => scene.passages.coming,
        Way::Out => scene.passages.out,
    };
    found.ok_or_else(|| anyhow!("no transition"))
}

pub(crate) fn check(dir: &Path, way: Way, value: &Value) -> Result<()> {
    let ms = value["duration_ms"].as_u64().unwrap_or(1000);
    let (low, high) = (MIN_TRANSITION_MS as f64 / 1000.0, MAX_TRANSITION_MS as f64 / 1000.0);
    ensure!(
        (MIN_TRANSITION_MS as u64..=MAX_TRANSITION_MS as u64).contains(&ms),
        said("editor.transition_length", "a transition lasts from %1 to %2 s", &[&low, &high])
    );
    let count = value["params"].as_object().map_or(0, |m| m.len());
    ensure!(
        count <= MAX_PARAMS,
        said("editor.transition_params", "a transition takes at most %1 values", &[&MAX_PARAMS])
    );
    if let Err(why) = readied(dir, way, value)? {
        bail!(words(&why));
    }
    Ok(())
}

pub fn words(why: &str) -> wprs_core::message::Message {
    if why.contains("a transition may cost") {
        return said(
            "editor.transition_costly",
            "this effect costs too much for a transition: only light and medium effects, at most %1 units",
            &[&wprs_scene::MAX_TRANSITION_UNITS],
        );
    }
    if why.contains("\"stage\": \"transition\"") {
        return said(
            "editor.transition_stage",
            "this effect is not a transition: its descriptor needs \"stage\": \"transition\"",
            &[],
        );
    }
    said("editor.transition_broken", "the effect of the transition does not work: %1", &[&why])
}
