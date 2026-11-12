use crate::daemon::Daemon;
use crate::error::{Result, not_found};
use serde_json::json;

impl Daemon {
    pub fn particle_stats(&self, output: &str) -> Result<String> {
        let counted = self
            .compositor
            .count_particles(output)
            .ok_or_else(|| not_found(format!("{output:?} shows no particles")))?;
        let level = self.compositor.status().get(output).map(|s| s.level).unwrap_or_default();
        let systems: Vec<_> =
            counted.systems.iter().map(|(id, alive)| json!({ "id": id, "alive": alive })).collect();
        Ok(json!({
            "output": output, "path": counted.path, "reason": counted.reason, "alive": counted.alive,
            "slots": counted.slots, "systems": systems, "quality": level.quality,
            "quality_reason": level.reason, "tier": self.tier.0, "tier_measured": self.tier.1,
        })
        .to_string())
    }
}
