use super::{IDLE, LISTEN, RETRY, STEP, Session, Status, lock};
use std::time::Duration;
use wprs_core::rgb::Covered;
use wprs_rgb::{Device, Strip};

impl Session {
    pub(super) fn held(&self) -> bool {
        let covered = |(source, _): &(String, Strip)| super::super::covered::covered(source);
        self.rgb.when_covered == Covered::Pause && self.last.as_ref().is_some_and(covered)
    }

    pub(super) fn wait(&self) -> Duration {
        match (&self.test, &self.client) {
            (Some(_), _) => STEP,
            (None, Some(_)) if !self.held() => LISTEN,
            (None, None) if self.retries() => RETRY,
            _ => IDLE,
        }
    }

    pub(super) fn retries(&self) -> bool {
        self.rgb.enabled && self.rgb.when_covered == Covered::Follow && super::super::covered::wanted()
    }

    pub(super) fn retry(&mut self) {
        if self.client.is_none() && self.retries() {
            self.ready().ok();
        }
        self.release();
    }

    pub(super) fn frame(&mut self, source: String, strip: Strip) {
        lock(&self.status).source = Some(source.clone());
        self.painters.insert(source.clone());
        if self.test.is_none() && !self.unwanted(Some(&source)) {
            self.paint(&strip, None);
        }
        self.last = Some((source, strip));
    }

    pub(super) fn forget(&mut self, source: Option<&str>) {
        match source {
            Some(source) => drop(self.painters.remove(source)),
            None => self.painters.clear(),
        }
        if self.last.as_ref().is_some_and(|(from, _)| source.is_none_or(|s| s == from)) {
            self.last = None;
            lock(&self.status).source = None;
        }
        self.idle();
        self.release();
    }

    fn needed(&self) -> bool {
        let shown = self.painters.iter().any(|source| !super::super::covered::resting(source));
        self.test.is_some() || super::super::covered::wanted() || shown
    }

    pub(super) fn unwanted(&self, source: Option<&str>) -> bool {
        let follows = self.rgb.when_covered == Covered::Follow && !super::super::covered::wanted();
        follows && self.test.is_none() && source.is_none_or(super::super::covered::covered)
    }

    pub(super) fn release(&mut self) {
        if self.rgb.when_covered == Covered::Follow && self.client.is_some() && !self.needed() {
            self.restore();
            self.client = None;
            self.publish(None, Vec::new());
        }
    }
}

impl Status {
    pub(super) fn show(&mut self, connected: bool, devices: Vec<Device>, error: Option<String>) -> bool {
        let was = self.receiving();
        (self.connected, self.error) = (connected, error);
        self.devices = if connected { devices } else { Vec::new() };
        was != self.receiving()
    }
}
