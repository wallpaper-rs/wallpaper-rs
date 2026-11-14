use crate::daemon::State;
use crate::streams::OutputInfo;
use wprs_core::model::StillReason;

impl State {
    pub(crate) fn still_reason(&self, info: Option<&OutputInfo>) -> Option<StillReason> {
        let p = &self.config.pause;
        let covered = p.when_covered && info.is_some_and(OutputInfo::covered);
        let reasons = [
            (self.user_paused, StillReason::Paused),
            (p.when_locked && self.locked, StillReason::Locked),
            (p.on_battery.pauses(self.on_battery, self.battery_percent), StillReason::Battery),
            (covered, StillReason::Covered),
        ];
        reasons.into_iter().find_map(|(holds, reason)| holds.then_some(reason))
    }

    pub(crate) fn active(&self, info: &OutputInfo) -> bool {
        self.still_reason(Some(info)).is_none()
    }

    pub(crate) fn awake(&self) -> bool {
        self.still_reason(None).is_none()
    }

    pub(crate) fn lowest(&self) -> bool {
        let below = self.config.playback.lowest_below_percent;
        below > 0 && self.on_battery && self.battery_percent.is_some_and(|b| b < below)
    }
}
