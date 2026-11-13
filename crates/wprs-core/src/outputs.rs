use crate::filter::Filter;
use crate::fit::Fit;
use crate::quality::QualityChoice;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OutputConfig {
    pub image: Option<PathBuf>,
    pub fit: Option<Fit>,
    pub background: Option<String>,
    pub playlist: Option<String>,
    pub preset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blank: Option<crate::blank::Kept>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr: Option<crate::HdrChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub scenes: BTreeMap<String, BTreeMap<String, toml::Value>>,
}

impl OutputConfig {
    pub fn sets_content(&self) -> bool {
        self.image.is_some() || self.playlist.is_some() || self.blank.is_some()
    }
}
