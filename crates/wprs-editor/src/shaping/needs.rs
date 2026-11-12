use crate::action::Needed;
use crate::ops::Op;
use anyhow::{Result, ensure};
use semver::{Version, VersionReq};
use serde_json::{Value, json};
use wprs_core::message::said;

const MOST: usize = 16;

fn range_of(version: &str) -> Option<String> {
    let v = Version::parse(version.trim()).ok()?;
    Some(match (v.major, v.minor) {
        (0, 0) => format!("^0.0.{}", v.patch),
        (0, minor) => format!("^0.{minor}"),
        (major, minor) => format!("^{major}.{minor}"),
    })
}

pub(crate) fn range(given: &str) -> Result<String> {
    let text = given.trim();
    if let Some(suggested) = range_of(text) {
        return Ok(suggested);
    }
    ensure!(
        VersionReq::parse(text).is_ok(),
        said("editor.not_version_range", "%1 is not a version or a version range such as ^1.2", &[&text])
    );
    Ok(text.to_owned())
}

fn listed(doc: &Value) -> Vec<(String, String)> {
    let all = doc["manifest"]["requires"]["packages"].as_array().into_iter().flatten();
    let pair = |p: &Value| Some((p["id"].as_str()?.to_owned(), p["version"].as_str()?.to_owned()));
    all.filter_map(pair).collect()
}

pub(super) fn packages(doc: &Value, given: &[Needed], add: bool, remove: bool) -> Result<Vec<Op>> {
    let own = doc["manifest"]["id"].as_str().unwrap_or_default();
    let mut next = listed(doc);
    if remove {
        next.retain(|(id, _)| !given.iter().any(|g| g.id == *id));
    } else {
        let mut wanted = Vec::new();
        for g in given {
            ensure!(g.id != own, said("editor.needs_itself", "a package cannot require itself", &[]));
            let version = g.version.as_deref().map(range).transpose()?;
            wanted.push((g.id.clone(), version.unwrap_or_else(|| "*".into())));
        }
        if !add {
            next.clear();
        }
        for (id, version) in wanted {
            match next.iter_mut().find(|(n, _)| *n == id) {
                Some(found) => found.1 = version,
                None => next.push((id, version)),
            }
        }
    }
    ensure!(
        next.len() <= MOST,
        said("editor.needs_most", "a package requires at most %1 other packages", &[&MOST])
    );
    let rows: Vec<Value> = next.iter().map(|(id, version)| json!({"id": id, "version": version})).collect();
    Ok(super::effects::put(doc, "/manifest/requires/packages", (!rows.is_empty()).then(|| json!(rows))))
}
