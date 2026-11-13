use super::{Reel, SceneSpec};
use std::sync::Arc;

impl SceneSpec {
    pub(crate) fn media(&self) -> Vec<(String, usize, Option<Reel>)> {
        let files = self.extra.iter().map(|(k, i)| (k.clone(), Arc::as_ptr(i) as usize, None));
        files.chain(self.clips.iter().map(|(k, reel)| (k.clone(), 0, Some(reel.clone())))).collect()
    }

    pub(crate) fn reels(&self) -> impl Iterator<Item = (&str, &Reel, u32)> {
        let replaced =
            |id: &String| self.clips.iter().any(|(k, _)| k == id) || self.extra.iter().any(|(k, _)| k == id);
        let level = (self.heard.clamp(0.0, 1.0) * 100.0).round() as u32;
        let own = self
            .pack
            .videos
            .iter()
            .filter(move |(id, _)| !replaced(id))
            .map(move |(id, r)| (id.as_str(), r, level));
        let loud = move |id: &str| if id.starts_with(wprs_scene::USER_FILES) { level } else { 0 };
        own.chain(self.clips.iter().map(move |(id, reel)| (id.as_str(), reel, loud(id))))
    }
}
