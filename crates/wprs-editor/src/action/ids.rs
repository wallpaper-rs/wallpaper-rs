use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Named {
    Layer,
    Timeline,
    Asset,
    System,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenameEdit {
    pub kind: Named,
    pub id: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PasteEdit {
    pub clip: Value,
    #[serde(default)]
    pub into: Option<String>,
    #[serde(default)]
    pub at: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranslateEdit {
    pub pointer: String,
    pub language: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub add: bool,
}
