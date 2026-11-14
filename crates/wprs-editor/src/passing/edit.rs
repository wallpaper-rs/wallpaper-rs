use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Way {
    #[default]
    In,
    Out,
}

impl Way {
    pub fn key(self) -> &'static str {
        match self {
            Way::In => "in",
            Way::Out => "out",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct TransitionEdit {
    pub way: Way,
    pub make: bool,
    pub effect: Option<String>,
    pub duration_ms: Option<u32>,
    pub curve: Option<Value>,
    pub params: Option<Map<String, Value>>,
    pub remove: bool,
}
