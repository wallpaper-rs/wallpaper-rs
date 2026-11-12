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
    NoMap,
    FillOff,
    Edited,
    Building,
    Failed,
    Flat,
}

impl Still {
    pub fn word(self) -> &'static str {
        match self {
            Still::NoMap => "no_map",
            Still::FillOff => "fill_off",
            Still::Edited => "edited",
            Still::Building => "building",
            Still::Failed => "failed",
            Still::Flat => "flat",
        }
    }

    pub fn message(self) -> Message {
        match self {
            Still::NoMap => said("still.no_map", "The picture stays still: it has no depth map yet.", &[]),
            Still::FillOff => {
                said("still.fill_off", "The picture stays still: the parallax fill is off.", &[])
            }
            Still::Edited => said(
                "still.edited",
                "The picture stays still: its colour operations are not drawn on depth layers.",
                &[],
            ),
            Still::Building => {
                said("still.building", "The picture stays still until its depth layers are built.", &[])
            }
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
