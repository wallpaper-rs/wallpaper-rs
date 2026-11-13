use crate::pages::{Privacy, WebPack};
use std::path::PathBuf;
use std::sync::Arc;
use wprs_scene::Values;

#[derive(Clone)]
pub struct PageSpec {
    pub pack: Arc<WebPack>,
    pub values: Values,
    pub privacy: Privacy,
    pub files: Vec<(String, PathBuf)>,
    pub folders: Vec<(String, PathBuf, bool)>,
    pub storage: Option<PathBuf>,
    pub quality: Option<wprs_core::QualityChoice>,
    pub drafted: bool,
    pub heard: f64,
}

pub type PageKey = (PathBuf, wprs_core::QualityChoice);

impl PageSpec {
    pub fn key(&self, general: wprs_core::QualityChoice) -> PageKey {
        (self.pack.path.clone(), self.quality.unwrap_or(general))
    }
}

impl PartialEq for PageSpec {
    fn eq(&self, other: &PageSpec) -> bool {
        let same = (
            &self.values,
            self.privacy,
            &self.files,
            &self.folders,
            (&self.storage, self.quality, self.heard),
        );
        let theirs = (
            &other.values,
            other.privacy,
            &other.files,
            &other.folders,
            (&other.storage, other.quality, other.heard),
        );
        self.pack.path == other.pack.path && same == theirs
    }
}
