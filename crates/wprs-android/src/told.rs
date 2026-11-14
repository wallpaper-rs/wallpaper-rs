use crate::pack::Pack;
use crate::{Error, store};
use std::collections::BTreeSet;
use std::fs;

const TOLD: &str = "told.json";

pub(crate) fn tell(pack: &Pack) -> Result<bool, Error> {
    let dir = store::dir()?;
    let read = fs::read(dir.join(TOLD)).ok();
    let mut told: BTreeSet<String> =
        read.and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default();
    if !told.insert(pack.id.clone()) {
        return Ok(false);
    }
    store::write(&dir.join(TOLD), &serde_json::to_vec_pretty(&told)?)?;
    Ok(true)
}
