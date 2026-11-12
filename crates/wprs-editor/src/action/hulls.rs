use serde::{Deserialize, Serialize};
use wprs_scene::Hull;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ColliderEdit {
    pub layer: String,
    #[serde(default)]
    pub index: Option<usize>,
    #[serde(default)]
    pub bone: Option<String>,
    #[serde(default)]
    pub shape: Option<Hull>,
    #[serde(default)]
    pub radius: Option<f32>,
    #[serde(default)]
    pub offset: Option<[f32; 3]>,
    #[serde(default)]
    pub remove: bool,
}
