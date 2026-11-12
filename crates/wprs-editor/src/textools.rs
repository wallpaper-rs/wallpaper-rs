use wprs_scene::{ParamKind, Tool};

pub const RAMP: &str = "ramp";
pub const TABLE: &str = "table";
pub const GRAY: &str = "gray";
pub const FLOW: &str = "flow";
pub const DEPTH: &str = "depth";

pub(crate) fn of(effect: &str, id: &str, kind: ParamKind, tool: Option<Tool>) -> Vec<&'static str> {
    match (kind, id) {
        (ParamKind::Lut, _) => vec![TABLE],
        (ParamKind::Texture, "gradient" | "ramp") => vec![RAMP],
        (ParamKind::Texture, "phase" | "walls" | "reflection") => vec![GRAY],
        (ParamKind::Texture, "map") if effect == "blend_gradient" => vec![GRAY],
        (ParamKind::Texture, "map") => vec![GRAY, FLOW],
        (ParamKind::Texture, "depth") => vec![GRAY, DEPTH],
        (ParamKind::Texture, _) => match tool {
            Some(Tool::Brush) => vec![GRAY],
            Some(Tool::Direction) => vec![FLOW],
            None => Vec::new(),
        },
        _ => Vec::new(),
    }
}
