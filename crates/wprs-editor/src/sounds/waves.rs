use serde_json::{Value, json};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use wprs_video::Clip;

const MOST: usize = 2048;
const KEPT: usize = 256;
const BUDGET: usize = 61 * wprs_video::RATE as usize;

fn cache() -> Option<PathBuf> {
    Some(dirs::cache_dir()?.join("wallpaper-rs").join("waves"))
}

fn trim(dir: &Path) {
    let mut kept: Vec<_> = std::fs::read_dir(dir).into_iter().flatten().flatten().collect();
    if kept.len() <= KEPT {
        return;
    }
    kept.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
    kept.iter().take(kept.len() - KEPT / 2).for_each(|e| drop(std::fs::remove_file(e.path())));
}

pub fn peaks(clip: &Clip, buckets: usize) -> Value {
    let buckets = buckets.clamp(1, MOST);
    let (channels, frames) = (clip.channels.max(1), clip.frames());
    let height = |bucket: usize| {
        let (from, to) =
            (bucket * frames / buckets, ((bucket + 1) * frames / buckets).max(bucket * frames / buckets + 1));
        let slice = clip.samples.get(from * channels..(to * channels).min(clip.samples.len()));
        let top = slice.into_iter().flatten().map(|s| i32::from(*s).unsigned_abs()).max().unwrap_or(0);
        (f64::from(top.min(32768)) / 32768.0 * 1000.0).round() / 1000.0
    };
    let seconds = frames as f64 / f64::from(wprs_video::RATE);
    json!({ "seconds": seconds, "channels": channels, "peaks": (0..buckets).map(height).collect::<Vec<_>>() })
}

pub fn wave(dir: &Path, doc: &Value, asset: &str, buckets: usize) -> Value {
    let Some(path) = doc.pointer("/scene/assets").and_then(|a| a.get(asset)).and_then(|a| a["path"].as_str())
    else {
        return json!({ "error": format!("there is no sound {asset}") });
    };
    let file = dir.join(path);
    let (Some(cache), Ok(meta)) = (cache(), std::fs::metadata(&file)) else {
        return json!({ "error": format!("cannot read {path}") });
    };
    let buckets = buckets.clamp(1, MOST);
    let mut h = DefaultHasher::new();
    (&file, meta.len(), meta.mtime(), meta.mtime_nsec(), buckets).hash(&mut h);
    let listed = cache.join(format!("{:016x}.json", h.finish()));
    let kept = std::fs::read(&listed).ok().and_then(|b| serde_json::from_slice::<Value>(&b).ok());
    if let Some(found) = kept.filter(|k| k["peaks"].is_array()) {
        return found;
    }
    let clip = match wprs_video::decode(&file, BUDGET) {
        Ok(clip) => clip,
        Err(e) => return json!({ "error": e.to_string() }),
    };
    let found = peaks(&clip, buckets);
    if std::fs::create_dir_all(&cache).is_ok() {
        trim(&cache);
        drop(std::fs::write(&listed, found.to_string()));
    }
    found
}
