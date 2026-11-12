use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Grip {
    Move,
    Nudge,
    Turn,
    Origin,
    Scale {
        corner: [f64; 2],
        #[serde(default)]
        even: bool,
    },
    Skew {
        edge: [f64; 2],
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Fit {
    Cover,
    Contain,
    Stretch,
}
