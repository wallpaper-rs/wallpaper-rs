use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct PoolEdit {
    pub id: String,
    pub add: bool,
    pub remove: bool,
    pub max: Option<u32>,
    pub max_vertices: Option<u32>,
    pub texture: Option<String>,
    pub size: Option<[u32; 2]>,
    pub clear: Vec<String>,
}
