use super::system;
use crate::ops::Op;
use anyhow::{Result, ensure};
use serde_json::Value;
use std::path::Path;
use wprs_core::message::said;

fn swapped(text: &str, from: &str, to: &str) -> String {
    ['"', '\'', '`']
        .iter()
        .fold(text.to_owned(), |text, q| text.replace(&format!("{q}{from}{q}"), &format!("{q}{to}{q}")))
}

pub(super) fn rewrite(dir: &Path, doc: &Value, from: &str, to: &str) -> Result<Vec<Op>> {
    system(doc, to)?;
    ensure!(
        doc.pointer(&format!("/scene/particles/{}", crate::ops::escape(from))).is_none(),
        said("editor.id_use", "the id %1 is in use", &[&from])
    );
    let list = doc["scene"]["scripts"].as_array().into_iter().flatten().filter_map(|s| s["path"].as_str());
    let mut ops = Vec::new();
    for path in list.filter(|p| wprs_format::check_path(p).is_ok()) {
        let Ok(text) = std::fs::read_to_string(dir.join(path)) else { continue };
        let next = swapped(&text, from, to);
        if next != text {
            ops.extend(crate::scripts::save(dir, doc, path, &next)?);
        }
    }
    Ok(ops)
}
