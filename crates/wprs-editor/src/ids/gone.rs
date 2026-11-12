use super::refs::{Hold, Ref, refs};
use crate::ops::{Op, apply};
use crate::seats::grips::{Moved, decompose, tidy};
use crate::seats::seats;
use anyhow::{Result, bail};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

fn plain(v: Option<&Value>) -> bool {
    v.is_none_or(|v| v.is_number() || v.as_array().is_some_and(|l| l.iter().all(Value::is_number)))
}

fn kept(doc: &Value, staged: &Value, owner: &str, id: &str) -> Vec<Op> {
    let dir = Path::new("");
    let Some(before) = seats(dir, doc, 0.0).into_iter().find(|s| s.id == id) else { return Vec::new() };
    let Some(after) = seats(dir, staged, 0.0).into_iter().find(|s| s.id == id) else { return Vec::new() };
    let t = &doc.pointer(owner).cloned().unwrap_or_default()["transform"];
    if !["position", "rotation", "scale", "skew"].iter().all(|k| plain(t.get(*k))) {
        return Vec::new();
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
        out.push(Op::Add { path: format!("{base}/{k}"), value: v });
    }
    out
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
    let mut keep = Vec::new();
    for r in followers {
        let id = doc.pointer(&r.owner).and_then(|l| l["id"].as_str()).unwrap_or_default();
        keep.extend(kept(doc, &staged, &r.owner, id));
    }
    apply(&mut staged, &keep)?;
    ops.extend(keep);
    Ok(ops)
}
