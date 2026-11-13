use super::{CRITICAL, Power};
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

static BELOW: [AtomicU32; 2] = [AtomicU32::new(0), AtomicU32::new(0)];

pub(crate) fn below(pause: u8, lowest: u8) {
    BELOW[0].store(pause.into(), Relaxed);
    BELOW[1].store(lowest.into(), Relaxed);
}

pub(super) fn limits() -> (u32, u32) {
    (BELOW[0].load(Relaxed), BELOW[1].load(Relaxed))
}

impl Power {
    pub(super) fn drained(self, below: u32) -> bool {
        below > 0 && !self.charging && self.percent >= 0 && (self.percent as u32) < below
    }

    pub(super) fn paused_below(self, below: u32) -> bool {
        self.saver || (self.low && !self.charging) || self.drained(below) || self.thermal >= CRITICAL
    }

    pub(crate) fn lowest(self) -> bool {
        self.drained(limits().1)
    }
}
