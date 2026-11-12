use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use wprs_scene::MaskChannel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MaskHold {
    Size,
    Feather,
    Width,
    Radius,
    Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum EffectsEdit {
    Add {
        layer: String,
        effect: String,
        #[serde(default)]
        name: Option<String>,
        #[serde(default)]
        at: Option<usize>,
        #[serde(default)]
        params: Map<String, Value>,
        #[serde(default)]
        optional: bool,
    },
    Set {
        layer: String,
        index: usize,
        #[serde(default)]
        params: Map<String, Value>,
        #[serde(default)]
        enabled: Option<bool>,
        #[serde(default)]
        name: Option<String>,
        #[serde(default)]
        optional: Option<bool>,
    },
    Remove {
        layer: String,
        index: usize,
    },
    Move {
        layer: String,
        index: usize,
        to: usize,
    },
    Grip {
        layer: String,
        index: usize,
        param: String,
        to: [f64; 2],
        #[serde(default)]
        both: bool,
        #[serde(default)]
        at: Option<f64>,
        #[serde(default)]
        replace: bool,
    },
    MaskGrip {
        layer: String,
        #[serde(default)]
        index: Option<usize>,
        hold: MaskHold,
        to: [f64; 2],
        #[serde(default)]
        at: Option<f64>,
    },
    Mask {
        layer: String,
        #[serde(default)]
        index: Option<usize>,
        #[serde(default)]
        texture: Option<String>,
        #[serde(default)]
        from: Option<String>,
        #[serde(default)]
        shape: Option<Value>,
        #[serde(default)]
        channel: Option<MaskChannel>,
        #[serde(default)]
        invert: Option<bool>,
        #[serde(default)]
        feather: Option<f64>,
        #[serde(default)]
        strength: Option<f64>,
        #[serde(default)]
        off: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum ScriptEdit {
    Add {
        path: String,
    },
    Remove {
        path: String,
    },
    Move {
        path: String,
        to: usize,
    },
    Declare {
        path: String,
        #[serde(default)]
        params: bool,
    },
    Write {
        path: String,
        text: String,
    },
    Param {
        path: String,
        id: String,
        #[serde(default, rename = "type")]
        kind: Option<String>,
        #[serde(default)]
        default: Option<Value>,
        #[serde(default)]
        min: Option<f64>,
        #[serde(default)]
        max: Option<f64>,
        #[serde(default)]
        to: Option<String>,
        #[serde(default)]
        remove: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum TemplateEdit {
    Make {
        layers: Vec<String>,
        #[serde(default)]
        name: Option<String>,
    },
    Rename {
        name: String,
        to: String,
        #[serde(default)]
        rewrite: bool,
    },
    Remove {
        name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RampStop {
    pub at: f32,
    pub color: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RampFormat {
    #[default]
    Png,
    Cube,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RampEdit {
    pub stops: Vec<RampStop>,
    #[serde(default)]
    pub pointer: Option<String>,
    #[serde(default)]
    pub asset: Option<String>,
    #[serde(default)]
    pub format: RampFormat,
    #[serde(default = "wide")]
    pub width: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DepthMapEdit {
    pub asset: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub pointer: Option<String>,
    #[serde(default)]
    pub input: Option<u32>,
}

fn wide() -> u32 {
    256
}

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
    },
    Field {
        effect: String,
        field: String,
        #[serde(default)]
        value: Value,
    },
}
