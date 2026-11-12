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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClockSet {
    #[serde(default)]
    pub poster: Option<f64>,
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default)]
    pub fresh: bool,
}

pub const SCREENS: [(&str, [u32; 2]); 10] = [
    ("HD", [1280, 720]),
    ("Full HD", [1920, 1080]),
    ("QHD", [2560, 1440]),
    ("4K UHD", [3840, 2160]),
    ("5K", [5120, 2880]),
    ("Ultrawide 21:9", [3440, 1440]),
    ("Super ultrawide 32:9", [5120, 1440]),
    ("16:10", [2560, 1600]),
    ("4:3", [2048, 1536]),
    ("Phone 9:19.5", [1080, 2340]),
];
