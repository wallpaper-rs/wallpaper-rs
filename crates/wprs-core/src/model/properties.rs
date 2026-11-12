use super::{SceneField, SceneOption, Time, Uid};
use serde_json::Value;
use std::collections::BTreeMap;

model! {
    pub struct SceneProperties {
        pub model: u32,
        pub serial: u64,
        pub output: String,
        pub package: String,
        pub title: String,
        pub properties: BTreeMap<String, Value>,
        pub values: BTreeMap<String, Value>,
        pub presets: Vec<Value>,
        #[serde(default)]
        pub fields: Vec<SceneField>,
        #[serde(default)]
        pub preset_labels: Vec<SceneOption>,
        #[serde(default)]
        pub warnings: Vec<String>,
        #[serde(default)]
        pub user_presets: Vec<SceneOption>,
        #[serde(default)]
        pub shared: bool,
        #[serde(default)]
        pub kind: String,
    }

    pub struct OutputPlaylist {
        pub uid: Uid,
        pub name: String,
        pub position: u32,
        pub count: u32,
        pub next_at: Option<Time>,
    }
}
