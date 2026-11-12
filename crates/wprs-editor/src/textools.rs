use wprs_scene::{ParamDef, Tool};

pub const RAMP: &str = "ramp";
pub const TABLE: &str = "table";
pub const GRAY: &str = "gray";
pub const FLOW: &str = "flow";
pub const DEPTH: &str = "depth";

pub(crate) fn of(effect: &str, param: &ParamDef) -> Vec<&'static str> {
    param
        .tools(effect)
        .into_iter()
        .map(|tool| match tool {
            Tool::Ramp => RAMP,
            Tool::Table => TABLE,
            Tool::Gray => GRAY,
            Tool::Flow => FLOW,
            Tool::Depth => DEPTH,
        })
        .collect()
}
