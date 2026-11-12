use crate::ops::{Op, escape, set, unescape};
use anyhow::Result;
use serde_json::{Value, json};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

const AT: &str = "/scene/ext/editor/code/";

fn hash(text: &str) -> String {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn origin(path: &str) -> String {
    format!("{}.origin", path.replace('/', "%"))
}

pub(crate) fn save(dir: &Path, doc: &Value, path: &str, text: &str) -> Result<Vec<Op>> {
    let old = fs::read_to_string(dir.join(path)).unwrap_or_default();
    let (before, after) = (hash(&old), hash(text));
    if before == after {
        return Ok(Vec::new());
    }
    let kept = crate::session::codes(dir)?;
    fs::create_dir_all(&kept)?;
    for (name, body) in [(&before, &old), (&after, &text.to_owned())] {
        fs::write(kept.join(format!("{name}.js")), body)?;
    }
    if !kept.join(origin(path)).exists() {
        fs::write(kept.join(origin(path)), &before)?;
    }
    fs::write(dir.join(path), text)?;
    Ok(set(doc, &format!("{AT}{}", escape(path)), json!(after)))
}

pub(crate) fn restore(dir: &Path, doc: &Value, ops: &[Op]) {
    let Ok(kept) = crate::session::codes(dir) else { return };
    let mut paths: Vec<String> = ops
        .iter()
        .flat_map(Op::paths)
        .filter_map(|p| p.strip_prefix(AT))
        .map(|p| unescape(p.split('/').next().unwrap_or_default()))
        .collect();
    paths.sort();
    paths.dedup();
    for path in paths.iter().filter(|p| wprs_format::check_path(p).is_ok()) {
        let recorded =
            doc.pointer(&format!("{AT}{}", escape(path))).and_then(Value::as_str).map(String::from);
        let wanted = recorded.or_else(|| fs::read_to_string(kept.join(origin(path))).ok());
        let Some(wanted) = wanted else { continue };
        let now = fs::read_to_string(dir.join(path)).map(|t| hash(&t)).unwrap_or_default();
        if now != wanted
            && let Ok(text) = fs::read_to_string(kept.join(format!("{wanted}.js")))
        {
            fs::write(dir.join(path), text).ok();
        }
    }
}
