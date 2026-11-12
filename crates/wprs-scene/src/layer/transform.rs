use crate::value::{Value, Vec2};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct Transform {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-unit" = "px"))]
    pub position: Option<Value<Vec2>>,
    pub scale: Value<Vec2>,
    #[schemars(extend("x-unit" = "°", "x-range" = [-360, 360]))]
    pub rotation: Value<f32>,
    pub skew: Value<Vec2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<[f32; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-unit" = "°", "x-range" = [-89, 89]))]
    pub tilt: Option<Value<Vec2>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(extend("x-unit" = "px"))]
    pub z: Option<Value<f32>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Anchor {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Inherit {
    Transform,
    Visible,
    Opacity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct Parallax(pub Value<Vec2>);

impl Default for Parallax {
    fn default() -> Self {
        Parallax(Value::Fixed(Vec2([0.0; 2])))
    }
}

impl Parallax {
    pub fn is_zero(&self) -> bool {
        matches!(&self.0, Value::Fixed(Vec2(v)) if *v == [0.0; 2])
    }

    pub fn apart(&self) -> bool {
        matches!(&self.0, Value::Fixed(Vec2([x, y])) if x != y)
    }

    pub fn bound(&self) -> bool {
        matches!(self.0, Value::Bound(_))
    }
}

impl Default for Transform {
    fn default() -> Self {
        let (unit, zero) = (Value::Fixed(Vec2([1.0; 2])), Value::Fixed(Vec2([0.0; 2])));
        let (position, origin, tilt, z) = Default::default();
        Transform { position, scale: unit, rotation: Value::Fixed(0.0), skew: zero, origin, tilt, z }
    }
}

impl Transform {
    pub(super) fn is_default(&self) -> bool {
        *self == Transform::default()
    }
}

impl Anchor {
    pub fn fraction(self) -> [f32; 2] {
        let i = self as usize;
        [(i % 3) as f32 / 2.0, (i / 3) as f32 / 2.0]
    }
}
