use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use wprs_format::OriginKind;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum ManifestEdit {
    Origin {
        kind: OriginKind,
        #[serde(default)]
        derived_from: Vec<Value>,
        #[serde(default)]
        provider: Option<Value>,
        #[serde(default)]
        source_url: Option<String>,
    },
    License {
        license: String,
    },
    Tags {
        tags: Vec<String>,
        #[serde(default)]
        add: bool,
        #[serde(default)]
        remove: bool,
    },
    Authors {
        authors: Vec<String>,
    },
    Display {
        #[serde(default)]
        values: Map<String, Value>,
        #[serde(default)]
        clear: Vec<String>,
    },
    Playback {
        #[serde(default)]
        values: Map<String, Value>,
        #[serde(default)]
        clear: Vec<String>,
    },
    Web {
        #[serde(default)]
        values: Map<String, Value>,
        #[serde(default)]
        clear: Vec<String>,
    },
    Network {
        origins: Vec<String>,
        #[serde(default)]
        add: bool,
        #[serde(default)]
        remove: bool,
    },
    Inputs {
        inputs: Vec<String>,
        #[serde(default)]
        add: bool,
        #[serde(default)]
        remove: bool,
    },
    Requires {
        packages: Vec<Needed>,
        #[serde(default)]
        add: bool,
        #[serde(default)]
        remove: bool,
    },
    Preset {
        #[serde(default)]
        base: Option<Needed>,
        #[serde(default)]
        values: Map<String, Value>,
        #[serde(default)]
        clear: Vec<String>,
        #[serde(default)]
        files: Map<String, Value>,
    },
    Credit {
        paths: Vec<String>,
        license: String,
        attribution: String,
        #[serde(default)]
        url: Option<String>,
    },
    Uncredit {
        paths: Vec<String>,
    },
    Text {
        key: String,
        language: String,
        #[serde(default)]
        text: Option<String>,
    },
    Locale {
        language: String,
        file: bool,
    },
    AddFile {
        file: std::path::PathBuf,
        #[serde(default)]
        to: Option<String>,
    },
    Media {
        file: std::path::PathBuf,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Needed {
    pub id: String,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PropertyEdit {
    pub id: String,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
    #[serde(default)]
    pub step: Option<f64>,
    #[serde(default)]
    pub default: Option<Value>,
    #[serde(default)]
    pub order: Option<usize>,
}
