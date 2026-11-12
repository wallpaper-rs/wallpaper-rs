use anyhow::{Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};
use wprs_scene::{Model, ModelFile};

type Read = (Arc<Model>, Arc<ModelFile>);
type Key = (PathBuf, Vec<(String, PathBuf)>);

static KEPT: Mutex<Option<HashMap<Key, Read>>> = Mutex::new(None);

pub(super) fn shapes(dir: &Path, doc: &Value, asset: &str) -> Vec<(String, PathBuf)> {
    let at = format!("/scene/assets/{}/shapes", crate::ops::escape(asset));
    let listed = doc.pointer(&at).and_then(Value::as_object).into_iter().flatten();
    listed.filter_map(|(k, v)| Some((k.clone(), dir.join(v.as_str()?)))).collect()
}

pub(super) fn read(dir: &Path, doc: &Value, asset: &str, posed: bool) -> Result<Read> {
    let path = super::store::path(dir, doc, asset)?;
    let read = super::store::file_as(&path, posed)?;
    let carved = shapes(dir, doc, asset);
    if carved.is_empty() || !posed {
        return Ok(read);
    }
    let mut kept = KEPT.lock().unwrap_or_else(PoisonError::into_inner);
    let kept = kept.get_or_insert_with(HashMap::new);
    let key = (path, carved);
    if let Some(found) = kept.get(&key) {
        return Ok(found.clone());
    }
    let mut files = Vec::with_capacity(key.1.len());
    for (name, file) in &key.1 {
        files.push((name.clone(), std::fs::read(file)?));
    }
    let mut model = (*read.0).clone();
    model.carve(&files).map_err(|e| anyhow!("{} {e}", key.0.display()))?;
    let mut head = (*read.1).clone();
    head.shapes.extend(key.1.iter().map(|(n, _)| n.clone()));
    let out = (Arc::new(model), Arc::new(head));
    kept.retain(|_, r| Arc::strong_count(&r.0) > 1);
    kept.insert(key, out.clone());
    Ok(out)
}
