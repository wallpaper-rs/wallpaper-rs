use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Stop {
    Make {
        #[serde(default)]
        radial: bool,
    },
    Clear,
    Add {
        at: f64,
        #[serde(default)]
        color: Option<String>,
    },
    Move {
        index: usize,
        at: f64,
    },
    Color {
        index: usize,
        color: String,
    },
    Remove {
        index: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum FigureEdit {
    Shape {
        layer: String,
        handle: String,
        to: [f64; 2],
        #[serde(default)]
        even: bool,
        #[serde(default)]
        at: Option<f64>,
        #[serde(default)]
        replace: bool,
    },
    Stop {
        layer: String,
        stop: Stop,
    },
    TextWidth {
        layer: String,
        #[serde(default)]
        to: Option<[f64; 2]>,
        #[serde(default)]
        remove: bool,
        #[serde(default)]
        at: Option<f64>,
        #[serde(default)]
        replace: bool,
    },
    Fonts {
        layer: String,
        #[serde(default)]
        font: Option<String>,
        #[serde(default)]
        fallback: Option<Vec<String>>,
    },
}
