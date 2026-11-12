use crate::daemon::Daemon;
use crate::error::invalid;
use crate::pages::WebPack;
use crate::scenes::ScenePack;
use anyhow::{Result, ensure};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use wprs_core::QualityChoice;
use wprs_core::settings::fps;
use wprs_format::{Kind, Limits, Manifest, valid_id};

pub(crate) struct Page {
    pub(crate) folder: PathBuf,
    pub(crate) pack: Arc<WebPack>,
    pub(crate) quality: Option<QualityChoice>,
    staged: PathBuf,
}

impl Drop for Page {
    fn drop(&mut self) {
        std::fs::remove_file(&self.staged).ok();
    }
}

#[derive(Clone)]
pub(crate) enum Drawn {
    Scene(Arc<ScenePack>),
    Page(Arc<Page>),
}

impl Drawn {
    pub(crate) fn media(&self) -> crate::apply::Media {
        match self {
            Drawn::Scene(pack) => crate::apply::Media::Scene(pack.clone()),
            Drawn::Page(page) => crate::apply::Media::Web(page.pack.clone()),
        }
    }

    pub(crate) fn quality(&self) -> Option<Option<wprs_core::QualityChoice>> {
        match self {
            Drawn::Page(page) => Some(page.quality),
            Drawn::Scene(_) => None,
        }
    }

    pub(super) fn path(&self) -> &Path {
        match self {
            Drawn::Scene(pack) => &pack.path,
            Drawn::Page(page) => &page.folder,
        }
    }

    pub(super) fn same(&self, other: &Drawn) -> bool {
        match (self, other) {
            (Drawn::Scene(a), Drawn::Scene(b)) => Arc::ptr_eq(a, b),
            (Drawn::Page(a), Drawn::Page(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

pub(super) fn wanted(document: &str) -> bool {
    serde_json::from_str::<Value>(document).is_ok_and(|doc| doc["manifest"]["kind"] == "web")
}

fn named(dir: &Path, document: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(dir.as_os_str().as_encoded_bytes());
    hasher.update(document.as_bytes());
    hasher.finalize().to_hex()[..24].to_owned()
}

impl Daemon {
    pub(super) fn draft_page(&self, dir: &Path, document: &str) -> Result<Page> {
        let doc: Value = serde_json::from_str(document).map_err(|e| invalid(e.to_string()))?;
        let manifest: Manifest =
            serde_json::from_value(doc["manifest"].clone()).map_err(|e| invalid(e.to_string()))?;
        ensure!(manifest.kind == Kind::Web, invalid("the manifest is not a web page manifest"));
        ensure!(valid_id(&manifest.id), invalid("the manifest id is not a name like org.example.name"));
        let folder = self.store.data.join("preview");
        crate::store::private_dir(&folder)?;
        let staged = folder.join(format!("{}.wpak", named(dir, document)));
        let bytes = doc["manifest"].to_string();
        wprs_format::build_staged(dir, bytes.as_bytes(), &staged, &Limits::default())
            .map_err(|e| invalid(e.to_string()))?;
        let mut pack = self.web_pack(&staged)?;
        let capped = doc["preview"]["fps"].as_u64().and_then(|f| u32::try_from(f).ok()).filter(|f| *f > 0);
        if let (Some(fps), Some(fresh)) = (capped, Arc::get_mut(&mut pack)) {
            fresh.max_fps = fps::lower(fresh.max_fps, fps);
        }
        let level = doc["preview"]["quality"].as_str().and_then(|q| QualityChoice::parse(q).ok());
        let quality = level.filter(|q| q.fixed().is_some_and(|l| l < QualityChoice::TOP));
        Ok(Page { folder: dir.to_path_buf(), pack, quality, staged })
    }
}

#[cfg(test)]
mod tests {
    use super::{named, wanted};
    use std::path::Path;

    #[test]
    fn only_a_web_manifest_asks_for_a_page_and_each_document_is_staged_apart() {
        assert!(wanted(r#"{"manifest": {"kind": "web"}}"#));
        assert!(!wanted(r#"{"manifest": {"kind": "scene"}, "scene": {}}"#));
        assert!(!wanted("not json"));
        let (dir, other) = (Path::new("/a"), Path::new("/b"));
        assert_eq!(named(dir, "x"), named(dir, "x"));
        assert_ne!(named(dir, "x"), named(dir, "y"));
        assert_ne!(named(dir, "x"), named(other, "x"));
    }
}
