use crate::UvRect;
use crate::message::{Message, said};
use serde::{Deserialize, Serialize};

pub const PHONE_GAIN: f32 = 3.0;
pub const MOST_MARGIN: f32 = 0.2;
pub const LEAST_PIXELS: f32 = 1.0;

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Reach {
    pub shift: f32,
    pub low: f32,
    pub high: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Travel {
    pub shift: f32,
    pub zoom: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Still {
    NoModel,
    NoMap,
    FillOff,
    Building,
    BuildingSimple,
    Software,
    Failed,
    Flat,
}

impl Still {
    pub fn word(self) -> &'static str {
        match self {
            Still::NoModel => "no_model",
            Still::NoMap => "no_map",
            Still::FillOff => "fill_off",
            Still::Building => "building",
            Still::BuildingSimple => "building_simple",
            Still::Software => "software",
            Still::Failed => "failed",
            Still::Flat => "flat",
        }
    }

    pub fn message(self) -> Message {
        self.sized("")
    }

    pub fn sized(self, size: &str) -> Message {
        match self {
            Still::NoModel => said(
                "still.no_model",
                "The picture stays still: the depth model is not installed. Download it (%1) with wprs-ctl models download depth-anything-v2-small or from the app; nothing is downloaded until you ask.",
                &[&size],
            ),
            Still::NoMap => said("still.no_map", "The picture stays still: it has no depth map yet.", &[]),
            Still::FillOff => {
                said("still.fill_off", "The picture stays still: the parallax fill is off.", &[])
            }
            Still::Building => {
                said("still.building", "The picture stays still until its depth layers are built.", &[])
            }
            Still::BuildingSimple => said(
                "still.building_simple",
                "The picture stays still until its depth layers are built. They are built with the simple fill because LaMa, which fills the hidden parts better, is not installed. To get it (%1) run wprs-ctl models download lama-big-fp32 or use the app; nothing is downloaded until you ask.",
                &[&size],
            ),
            Still::Software => said(
                "still.software",
                "The picture stays still: this computer draws with its processor, so depth movement is off. Turn it on with wprs-ctl config playback.software_parallax true or in the settings; it then moves in 8 steps at up to 15 frames per second.",
                &[],
            ),
            Still::Failed => {
                said("still.failed", "The picture stays still: its depth layers could not be built.", &[])
            }
            Still::Flat => said(
                "still.flat",
                "The picture stays still: its depth gives no movement that would not stretch it.",
                &[],
            ),
        }
    }
}

pub fn fill_missing(lama: bool) -> Message {
    match lama {
        true => said(
            "depth.lama_missing",
            "LaMa or ONNX Runtime is not installed. Install it from the depth tool or with wprs-ctl models download lama-big-fp32; nothing is downloaded until you ask.",
            &[],
        ),
        false => said("depth.fill_missing", "The chosen fill is not installed.", &[]),
    }
}

pub fn gain(phone: bool, visible: f32) -> f32 {
    if phone { PHONE_GAIN * visible.clamp(0.0, 2.0) } else { 1.0 }
}

impl Reach {
    pub fn lean(&self, focus: f32) -> f32 {
        (self.high - focus).abs().max((self.low - focus).abs())
    }

    pub fn moves(&self, width: u32) -> bool {
        self.shift * width as f32 * (self.high - self.low) >= LEAST_PIXELS
    }
}

fn extent(uv: &UvRect, i: usize) -> f32 {
    let (lo, hi) = (uv.offset[i], uv.offset[i] + uv.scale[i]);
    (0.5 - lo.min(hi)).max(lo.max(hi) - 0.5).max(0.0)
}

pub fn most_zoom() -> f32 {
    1.0 / (1.0 - 2.0 * MOST_MARGIN)
}

pub fn travel(
    wanted: f32,
    reach: &Reach,
    (focus, least): (f32, f32),
    (aspect, uv): (f32, &UvRect),
) -> Travel {
    let lean = reach.lean(focus).max(1e-4);
    let per = [lean, lean * aspect];
    let top = most_zoom().max(least);
    let open = (0..2).map(|i| (0.5 - extent(uv, i) / top).max(0.0) / per[i]);
    let shift = open.fold(wanted.min(reach.shift), f32::min).max(0.0);
    let zoom = (0..2).map(|i| extent(uv, i) / (0.5 - shift * per[i]).max(1e-6));
    Travel { shift, zoom: zoom.fold(least.max(1.0), f32::max).min(top) }
}

pub fn shift(pointer: [f32; 2], travel: f32, aspect: f32) -> [f32; 2] {
    let reach = pointer[0].hypot(pointer[1]).max(1.0);
    [travel * pointer[0] / reach, travel * pointer[1] * aspect / reach]
}
