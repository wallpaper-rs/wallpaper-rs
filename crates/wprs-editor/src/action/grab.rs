use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Grabbed {
    Model {
        layer: String,
    },
    Eye,
    Center,
    Key {
        path: String,
        key: f64,
        #[serde(default)]
        aim: bool,
    },
    Light {
        layer: String,
        #[serde(default)]
        aim: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Handle {
    X,
    Y,
    Z,
    TurnX,
    TurnY,
    TurnZ,
    Plane,
    Depth,
    Orbit,
    Dolly,
    Roll,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct LightPlace {
    pub layer: String,
    pub position: Option<[f64; 3]>,
    pub aim: Option<[f64; 3]>,
    pub place: bool,
    pub flat: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Grab {
    pub what: Grabbed,
    pub handle: Handle,
    pub from: [f64; 2],
    pub to: [f64; 2],
    #[serde(default)]
    pub time: f64,
    #[serde(default)]
    pub replace: bool,
}
