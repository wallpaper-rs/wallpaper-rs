use super::grips::ClockSet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum CanvasEdit {
    Size {
        size: [u32; 2],
    },
    Clock(ClockSet),
    Bloom {
        #[serde(default)]
        threshold: Option<f64>,
        #[serde(default)]
        strength: Option<f64>,
        #[serde(default)]
        radius: Option<f64>,
        #[serde(default)]
        tint: Option<String>,
        #[serde(default)]
        hdr: Option<Value>,
        #[serde(default)]
        off: bool,
    },
    Parallax {
        #[serde(default)]
        amount: Option<f64>,
        #[serde(default)]
        delay: Option<f64>,
        #[serde(default)]
        pointer: Option<f64>,
        #[serde(default)]
        offset: Option<[f64; 2]>,
        #[serde(default)]
        gyro: Option<bool>,
        #[serde(default)]
        off: bool,
    },
    Camera {
        #[serde(default)]
        center: Option<[f64; 2]>,
        #[serde(default)]
        zoom: Option<f64>,
        #[serde(default)]
        rotation: Option<f64>,
        #[serde(default)]
        at: Option<f64>,
        #[serde(default)]
        replace: bool,
        #[serde(default)]
        off: bool,
    },
    CameraGrip {
        hold: CameraHold,
        to: [f64; 2],
        #[serde(default)]
        at: Option<f64>,
        #[serde(default)]
        replace: bool,
    },
    CameraView {
        whole: bool,
    },
    Probe {
        #[serde(default)]
        pointer: Option<[f64; 2]>,
        #[serde(default)]
        tilt: Option<[f64; 2]>,
        #[serde(default)]
        off: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum CameraHold {
    Move,
    Zoom { corner: [f64; 2] },
    Turn,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ShotFormat {
    #[default]
    Gif,
    Webp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum ShotEdit {
    Still {
        #[serde(default)]
        time: Option<f64>,
        #[serde(default)]
        base: Option<std::path::PathBuf>,
    },
    Recording {
        #[serde(default)]
        from: Option<f64>,
        #[serde(default)]
        seconds: Option<f64>,
        #[serde(default)]
        fps: Option<u32>,
        #[serde(default)]
        format: ShotFormat,
        #[serde(default)]
        base: Option<std::path::PathBuf>,
    },
    Flashes {
        #[serde(default)]
        from: Option<f64>,
        #[serde(default)]
        seconds: Option<f64>,
        #[serde(default)]
        fps: Option<u32>,
    },
}
