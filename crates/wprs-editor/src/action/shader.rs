use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum ShaderEdit {
    New {
        name: String,
        #[serde(default)]
        template: Option<String>,
    },
    Write {
        effect: String,
        #[serde(default)]
        wgsl: Option<String>,
        #[serde(default)]
        descriptor: Option<Value>,
    },
    Param {
        effect: String,
        id: String,
        #[serde(default)]
        set: Map<String, Value>,
        #[serde(default)]
        clear: Vec<String>,
        #[serde(default)]
        to: Option<String>,
        #[serde(default)]
        at: Option<usize>,
        #[serde(default)]
        remove: bool,
    },
    Pass {
        effect: String,
        #[serde(default)]
        index: Option<usize>,
        #[serde(default)]
        set: Map<String, Value>,
        #[serde(default)]
        clear: Vec<String>,
        #[serde(default)]
        remove: bool,
        #[serde(default)]
        lighter: bool,
    },
    Lighter {
        effect: String,
        #[serde(default)]
        set: Map<String, Value>,
        #[serde(default)]
        remove: bool,
    },
    Field {
        effect: String,
        field: String,
        #[serde(default)]
        value: Value,
    },
}
