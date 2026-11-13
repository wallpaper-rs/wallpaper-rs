use super::SceneState;
use crate::compositor::Level;
use crate::compositor::power::Power;
use std::time::Instant;
use wprs_scene::{Quality, Why};

#[cfg(test)]
mod tests;

pub(super) fn owned(spec: &super::SceneSpec, power: Power) -> Power {
    let held =
        if power.lowest { wprs_core::QualityChoice::Fixed(0) } else { spec.quality.unwrap_or(power.choice) };
    Power { choice: held, on_battery: power.on_battery || spec.pack.battery, ..power }
}

impl SceneState {
    pub(super) fn own(&self, power: Power) -> Power {
        owned(&self.spec, power)
    }

    fn steer(&mut self, power: Power) -> ((Quality, Why), Quality) {
        let cap = self.spec.pack.cap.unwrap_or(Quality::Q3);
        let capped = |(quality, why): (Quality, Why)| (quality.min(cap), why);
        let model = power.choose(power.auto(&self.runtime));
        let ceiling = capped(power.choose(power.reach()));
        self.pilot.start(power.floor(), model.0);
        (self.pilot.level(ceiling, model.1), ceiling.0)
    }

    fn stalled(&self, power: Power) -> bool {
        power.choice.is_auto() && self.pilot.stalled()
    }

    pub(super) fn level(&mut self, power: Power) -> Quality {
        let power = self.own(power);
        self.restop(power);
        let ((quality, why), _) = self.steer(power);
        self.why = why;
        self.over = power.over(&self.runtime, quality);
        let lowered = wprs_scene::lowers(self.over.is_some(), self.pilot.timing());
        self.automatic = match power.automatic(&self.runtime, quality) {
            _ if self.stalled(power) => "still",
            "" | "lower" if !power.choice.is_auto() && lowered => "lower",
            "lower" => "",
            automatic => automatic,
        };
        quality
    }

    fn restop(&mut self, power: Power) {
        let still = self.spec.hold.is_some() || power.still(&self.runtime) || self.stalled(power);
        if std::mem::replace(&mut self.still, still) == still {
            return;
        }
        if still && self.spec.hold.is_none() {
            tracing::debug!("the scene is too heavy for this device and stays at its poster time");
            self.runtime.seek(f64::from(self.runtime.scene.clock.poster_time));
        }
        self.uses = self.needs(self.quality);
    }

    pub(super) fn judge(&mut self, (samples, gpu_ms): (u64, f64), power: Power, refresh: u32, now: Instant) {
        if std::mem::replace(&mut self.samples, samples) == samples || self.still || self.lights {
            return;
        }
        let power = self.own(power);
        let (_, ceiling) = self.steer(power);
        let full = power.fps(&self.runtime, self.quality, refresh);
        let halves = power.pace.halves() && power.choice.fixed().is_none();
        let periods = wprs_scene::periods(halves, [full, wprs_scene::halved(full)]);
        let modes = (ceiling, power.raises(), power.choice.is_auto());
        if self.pilot.feed(now, gpu_ms as f32, periods, modes) {
            let (step, pace) = (self.pilot.step(), self.pilot.pace());
            tracing::debug!(
                ?step,
                pace,
                gpu_ms,
                "the GPU time of the frames changed the scene quality or pace"
            );
        }
    }

    pub(in super::super) fn quality(&self) -> Level {
        let (cost, budget) = self.over.unwrap_or_default();
        let (quality, reason) = (["q0", "q1", "q2", "q3"][self.quality as usize], self.why.name());
        let ((frame_ms, period_ms), automatic) = (self.pilot.timing().unzip(), self.automatic);
        let (step, pace) = (self.pilot.step().name(), self.pilot.pace());
        Level { quality, reason, cost, budget, automatic, frame_ms, period_ms, step, pace }
    }
}
