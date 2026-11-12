use super::refs::{Hold, Ref, refs};
use crate::keys::value_at;
use crate::ops::{Op, apply};
use crate::seats::grips::{Moved, decompose, tidy};
use crate::seats::seats;
use anyhow::{Result, bail};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

const FIELDS: [&str; 4] = ["position", "rotation", "scale", "skew"];
const SOURCES: [&str; 7] = ["prop", "map", "audio", "cursor", "clock", "date", "script"];

fn plain(v: &Value) -> bool {
    v.is_number() || v.as_array().is_some_and(|l| l.iter().all(Value::is_number))
}

fn keyed(v: &Value) -> bool {
    v.get("keys").is_some_and(Value::is_array) && SOURCES.iter().all(|s| v.get(*s).is_none())
}

fn shifted(v: &Value, now: &Value, to: &Value, field: &str) -> Option<Value> {
    let (now, to) = (numbers(now)?, numbers(to)?);
    let by: Vec<f64> = match field {
        "scale" => {
            now.iter().zip(&to).map(|(n, t)| (n.abs() > 1e-9).then(|| t / n)).collect::<Option<_>>()?
        }
        _ => now.iter().zip(&to).map(|(n, t)| t - n).collect(),
    };
    let mut out = v.clone();
    for key in out.get_mut("keys")?.as_array_mut()? {
        let value = numbers(&key[1])?;
        let moved: Vec<f64> = match field {
            "scale" => value.iter().zip(&by).map(|(a, f)| a * f).collect(),
            _ => value.iter().zip(&by).map(|(a, d)| a + d).collect(),
        };
        key[1] = if key[1].is_array() { json!(moved) } else { json!(moved[0]) };
        key[1] = round(&key[1]);
    }
    Some(out)
}

fn round(v: &Value) -> Value {
    let cut = |x: f64| (x * 1e4).round() / 1e4;
    match v {
        Value::Array(l) => json!(l.iter().filter_map(Value::as_f64).map(cut).collect::<Vec<_>>()),
        other => json!(other.as_f64().map(cut)),
    }
}

fn numbers(v: &Value) -> Option<Vec<f64>> {
    match v {
        Value::Array(l) => l.iter().map(Value::as_f64).collect(),
        other => other.as_f64().map(|n| vec![n]),
    }
}

fn kept(doc: &Value, staged: &Value, owner: &str, id: &str) -> Option<Vec<Op>> {
    let dir = Path::new("");
    let before = seats(dir, doc, 0.0).into_iter().find(|s| s.id == id)?;
    let after = seats(dir, staged, 0.0).into_iter().find(|s| s.id == id)?;
    let t = &doc.pointer(owner).cloned().unwrap_or_default()["transform"];
    if FIELDS.iter().any(|k| t.get(*k).is_some_and(|v| !plain(v) && !keyed(v))) {
        return None;
    }
    let [a, b, c, d, e, f] = after.frame.inverse().then(before.world).0;
    let (rotation, skew, scale) = decompose([a, b, c, d]);
    let position = Some([f64::from(e), f64::from(f)]);
    let m = tidy(Moved {
        position,
        rotation: Some(rotation),
        scale: Some(scale),
        skew: Some(skew),
        origin: None,
    });
    let fields = [
        ("position", m.position.map(|p| json!(p))),
        ("rotation", m.rotation.map(|r| json!(r))),
        ("scale", m.scale.map(|s| json!(s))),
        ("skew", m.skew.map(|k| json!(k))),
    ];
    let base = format!("{owner}/transform");
    let mut out = Vec::new();
    if staged.pointer(&base).is_none() {
        out.push(Op::Add { path: base.clone(), value: json!({}) });
    }
    for (k, v) in fields.into_iter().filter_map(|(k, v)| Some((k, v?))) {
        let value = match t.get(k).filter(|c| keyed(c)) {
            Some(own) => shifted(own, &value_at(own, 0.0), &v, k)?,
            None => v,
        };
        out.push(Op::Add { path: format!("{base}/{k}"), value });
    }
    Some(out)
}

fn cleared(doc: &Value, r: &Ref) -> Vec<Op> {
    let remove = |p: &str| doc.pointer(p).map(|_| Op::Remove { path: p.to_owned() });
    match r.hold {
        Hold::Parent => [format!("{}/parent", r.owner), format!("{}/inherit", r.owner)]
            .iter()
            .filter_map(|p| remove(p))
            .collect(),
        Hold::Target => remove(&r.at).into_iter().collect(),
        _ => remove(&r.owner).into_iter().collect(),
    }
}

pub fn forget(doc: &Value, gone: &[String], inside: &str) -> Result<Vec<Op>> {
    plan(doc, gone, inside).map(|(ops, _)| ops)
}

pub fn unkept(doc: &Value, id: &str) -> Vec<String> {
    let Ok((path, value)) = crate::actions::layer(doc, id) else { return Vec::new() };
    plan(doc, &crate::arrange::ids(value), &path).map(|(_, lost)| lost).unwrap_or_default()
}

fn plan(doc: &Value, gone: &[String], inside: &str) -> Result<(Vec<Op>, Vec<String>)> {
    let prefix = format!("{inside}/");
    let found: Vec<Ref> = refs(doc)
        .into_iter()
        .filter(|r| {
            gone.contains(&r.id) && (inside.is_empty() || (r.at != inside && !r.at.starts_with(&prefix)))
        })
        .collect();
    let held: Vec<&str> = found.iter().filter(|r| r.hold == Hold::Particles).map(|r| r.at.as_str()).collect();
    if !held.is_empty() {
        bail!(said(
            "editor.layer_still_used",
            "%1 is still used by %2; change them first",
            &[&gone[0], &held.join(", ")]
        ));
    }
    let mut ops: Vec<Op> = Vec::new();
    let mut staged = doc.clone();
    for r in &found {
        let more = cleared(&staged, r);
        apply(&mut staged, &more)?;
        ops.extend(more);
    }
    let followers = found.iter().filter(|r| r.hold == Hold::Parent);
    let (mut keep, mut lost) = (Vec::new(), Vec::new());
    for r in followers {
        let id = doc.pointer(&r.owner).and_then(|l| l["id"].as_str()).unwrap_or_default();
        match kept(doc, &staged, &r.owner, id) {
            Some(more) => keep.extend(more),
            None => lost.push(id.to_owned()),
        }
    }
    apply(&mut staged, &keep)?;
    ops.extend(keep);
    Ok((ops, lost))
}
