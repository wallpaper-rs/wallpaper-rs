use crate::assets::files;
use crate::findings::{Problem, warning};
use crate::layers::asset_kind;
use crate::ops::{Op, set};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_format::check_path;

const MOST: usize = 64;
const PATHS: usize = 256;

fn list(doc: &Value) -> Vec<Value> {
    doc["manifest"]["credits"].as_array().cloned().unwrap_or_default()
}

fn paths(credit: &Value) -> Vec<&str> {
    credit["paths"].as_array().into_iter().flatten().filter_map(Value::as_str).collect()
}

fn covers(held: &str, path: &str) -> bool {
    held == path || (held.ends_with('/') && path.starts_with(held))
}

fn previews(doc: &Value) -> Vec<&str> {
    let m = &doc["manifest"];
    let named = m["previews"].as_object().into_iter().flatten().filter_map(|(_, p)| p.as_str());
    named.chain(m["preview"].as_str()).collect()
}

pub fn assets(dir: &Path, doc: &Value) -> Vec<Value> {
    let held = list(doc);
    let own = previews(doc);
    let credited = |path: &str| held.iter().any(|c| paths(c).iter().any(|h| covers(h, path)));
    let found = files(dir).into_iter().map(|(path, size)| (asset_kind(&path), path, size));
    found
        .filter(|(kind, path, _)| *kind != "other" && !own.contains(&path.as_str()))
        .map(|(kind, path, size)| json!({ "credited": credited(&path), "path": path, "kind": kind, "size": size }))
        .collect()
}

pub fn query(dir: &Path, doc: &Value) -> Value {
    let assets = assets(dir, doc);
    let missing: Vec<&Value> = assets.iter().filter(|a| a["credited"] == false).collect();
    json!({ "assets": assets, "credits": list(doc), "missing": missing.iter().map(|a| a["path"].clone()).collect::<Vec<_>>() })
}

pub fn gaps(dir: &Path, doc: &Value) -> Vec<Problem> {
    let others = !list(doc).is_empty()
        || matches!(doc["manifest"]["origin"]["kind"].as_str(), Some("derived" | "curated"));
    let missing: Vec<String> = assets(dir, doc)
        .iter()
        .filter(|a| a["credited"] == false)
        .filter_map(|a| a["path"].as_str().map(String::from))
        .collect();
    if !others || missing.is_empty() {
        return Vec::new();
    }
    let shown: Vec<&str> = missing.iter().take(3).map(String::as_str).collect();
    let more = missing.len().saturating_sub(shown.len());
    let said = said(
        "editor.assets_uncredited",
        "%1 files have no credit: %2 and %3 more; credit them, or their folder, if someone else made them",
        &[&missing.len(), &shown.join(", "), &more],
    );
    vec![warning("/manifest/credits", said)]
}

fn exists(dir: &Path, path: &str) -> bool {
    let plain = path.trim_end_matches('/');
    let at = dir.join(plain);
    if path.ends_with('/') { at.is_dir() } else { at.is_file() }
}

pub fn credit(
    dir: &Path,
    doc: &Value,
    chosen: &[String],
    license: &str,
    attribution: &str,
    url: Option<&str>,
) -> Result<Vec<Op>> {
    ensure!(
        !chosen.is_empty() && !license.trim().is_empty() && !attribution.trim().is_empty(),
        said("editor.credit_needs", "a credit needs files, a license and who it is attributed to", &[])
    );
    for path in chosen {
        check_path(path.trim_end_matches('/'))?;
        ensure!(
            exists(dir, path),
            said("editor.credit_missing", "%1 is not a file or folder of the package", &[&path])
        );
    }
    let mut next = list(doc);
    let same = |c: &Value| {
        c["license"] == license
            && c["attribution"] == attribution
            && c["url"].as_str() == url
            && c.get("source").is_none()
    };
    let at = next.iter().position(|c| same(c) && paths(c).len() < PATHS);
    let mut held: Vec<String> =
        at.map(|i| paths(&next[i]).iter().map(|p| p.to_string()).collect()).unwrap_or_default();
    let fresh: Vec<String> = chosen.iter().filter(|p| !held.contains(p)).cloned().collect();
    held.extend(fresh);
    for (n, chunk) in held.chunks(PATHS).enumerate() {
        let mut credit = json!({ "paths": chunk, "license": license, "attribution": attribution });
        if let Some(url) = url.filter(|u| !u.is_empty()) {
            credit["url"] = json!(url);
        }
        match at {
            Some(i) if n == 0 => next[i] = credit,
            _ => next.push(credit),
        }
    }
    ensure!(
        next.len() <= MOST,
        said("editor.credits_most", "a package has at most 64 credits; credit folders instead of files", &[])
    );
    Ok(set(doc, "/manifest/credits", json!(next)))
}

pub fn uncredit(doc: &Value, gone: &[String]) -> Vec<Op> {
    let mut next = Vec::new();
    for mut credit in list(doc) {
        let kept: Vec<&str> = paths(&credit).into_iter().filter(|p| !gone.iter().any(|g| g == p)).collect();
        if kept.is_empty() {
            continue;
        }
        credit["paths"] = json!(kept);
        next.push(credit);
    }
    if next == list(doc) {
        return Vec::new();
    }
    if next.is_empty() {
        return vec![Op::Remove { path: "/manifest/credits".into() }];
    }
    set(doc, "/manifest/credits", json!(next))
}
