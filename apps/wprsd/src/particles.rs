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

impl Daemon {
    pub fn capture_page(&self, output: &str) -> Result<String> {
        use crate::compositor::Taken;
        let found = self.compositor.snap_page(output);
        let Some(Taken::Ready((width, height), rgba)) = found else {
            return match found {
                Some(Taken::Missing) | None => Err(not_found(format!("{output:?} shows no web page"))),
                _ => Ok(String::new()),
            };
        };
        let image = image::RgbaImage::from_raw(width, height, rgba)
            .ok_or_else(|| crate::error::failed("the page frame has the wrong size"))?;
        let dir = std::env::var_os("XDG_RUNTIME_DIR").map_or_else(std::env::temp_dir, Into::into);
        let name: String = output.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
        let path = dir.join(format!("wprs-capture-{name}.png"));
        let mut bytes = Vec::new();
        image
            .write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)
            .map_err(|e| crate::error::failed(e.to_string()))?;
        std::fs::write(&path, &bytes).map_err(|e| crate::error::failed(e.to_string()))?;
        std::fs::set_permissions(&path, std::os::unix::fs::PermissionsExt::from_mode(0o600)).ok();
        Ok(path.to_string_lossy().into_owned())
    }
}
