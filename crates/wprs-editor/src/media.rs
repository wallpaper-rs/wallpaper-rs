use anyhow::{Result, ensure};
use serde_json::Value;
use std::path::Path;

pub fn simulated(path: &Path) -> Result<Value> {
    let bytes = std::fs::read(path).map_err(|e| anyhow::anyhow!("cannot read {}: {e}", path.display()))?;
    ensure!(bytes.len() <= 64 << 10, "{} is larger than 64 KiB", path.display());
    let value: Value = serde_json::from_slice(&bytes)?;
    ensure!(value.is_object(), "{} holds a JSON object of the made-up track", path.display());
    Ok(value)
}
