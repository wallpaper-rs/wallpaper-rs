use crate::{Budget, Heat, Quality, Runtime, Why};

impl Budget {
    pub fn auto(&self, runtime: &Runtime) -> (Quality, Why) {
        let steps = |from: Quality, to: Quality, why: Why, now: Why| if to < from { why } else { now };
        let base = self.tier.quality(false);
        let why = if base < Quality::Q3 { Why::Device } else { Why::Full };
        let strained = self.tier.quality(self.on_battery || self.saver);
        let why = steps(base, strained, if self.on_battery { Why::Battery } else { Why::Saver }, why);
        let start = if self.saver { strained.lower() } else { strained };
        let why = steps(strained, start, Why::Saver, why);
        let fitted = Quality::fitting(|q| runtime.cost(q), self.tier, start);
        let why = steps(start, fitted, Why::Cost, why);
        let heated = match self.heat {
            Heat::Cool => fitted,
            Heat::Warm => fitted.lower(),
            Heat::Hot => Quality::Q0,
        };
        (heated, steps(fitted, heated, Why::Heat, why))
    }

    pub fn reach(&self) -> (Quality, Why) {
        let (start, why) = match self.saver {
            true => (self.tier.quality(true).lower(), Why::Saver),
            false => (Quality::Q3, Why::Full),
        };
        match self.heat {
            Heat::Cool => (start, why),
            Heat::Warm => (start.lower(), Why::Heat),
            Heat::Hot => (Quality::Q0, Why::Heat),
        }
    }

    pub fn steady(&self) -> Quality {
        let strained = self.tier.quality(self.on_battery || self.saver);
        let start = if self.saver { strained.lower() } else { strained };
        let auto = match self.heat {
            Heat::Cool => start,
            Heat::Warm => start.lower(),
            Heat::Hot => Quality::Q0,
        };
        self.choose((auto, Why::Full)).0
    }

    pub fn raises(&self) -> bool {
        !self.on_battery && !self.saver && self.heat == Heat::Cool
    }
}
