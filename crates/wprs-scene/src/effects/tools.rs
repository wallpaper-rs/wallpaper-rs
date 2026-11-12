use super::{ParamDef, ParamKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tool {
    Ramp,
    Table,
    Gray,
    Flow,
    Depth,
}

impl ParamDef {
    pub fn tools(&self, effect: &str) -> Vec<Tool> {
        if !self.tools.is_empty() {
            return self.tools.clone();
        }
        match (self.kind, self.id.as_str()) {
            (ParamKind::Lut, _) => vec![Tool::Table],
            (ParamKind::Texture, "gradient" | "ramp") => vec![Tool::Ramp],
            (ParamKind::Texture, "phase" | "walls" | "reflection") => vec![Tool::Gray],
            (ParamKind::Texture, "map") if effect == "blend_gradient" => vec![Tool::Gray],
            (ParamKind::Texture, "map") => vec![Tool::Gray, Tool::Flow],
            (ParamKind::Texture, "depth") => vec![Tool::Gray, Tool::Depth],
            _ => Vec::new(),
        }
    }
}
