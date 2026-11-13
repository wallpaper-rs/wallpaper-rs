use crate::config::{Config, OutputConfig};
use crate::dbus::DEFAULT_OUTPUT;

pub const VOLUME: &str = "playback.volume.";
pub const MUTED: &str = "playback.muted.";

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct OwnSound {
    pub muted: Option<bool>,
    pub volume: Option<f64>,
}

impl Config {
    pub fn own_sound(&self, output: &str) -> OwnSound {
        OwnSound { muted: self.pick(output, |o| o.muted), volume: self.pick(output, |o| o.volume) }
    }

    pub fn heard_at(&self, output: &str) -> f64 {
        let own = self.own_sound(output);
        if own.muted == Some(true) { 0.0 } else { own.volume.unwrap_or(1.0).clamp(0.0, 1.0) }
    }

    pub(crate) fn set_own_sound(&mut self, output: &str, value: &str, volume: bool) -> Result<(), String> {
        let key = if volume { VOLUME } else { MUTED };
        let (name, text) = (output.trim(), value.trim());
        if name.is_empty() || name.len() > 128 || name.chars().any(char::is_control) {
            return Err(format!("invalid output name in {key}<output>"));
        }
        let name = if name == DEFAULT_OUTPUT { "" } else { name };
        let (mut muted, mut loud) = (None, None);
        let bad = |why: &str| format!("invalid value for {key}{output}: {why}");
        match (text, volume) {
            ("" | "none", _) => {}
            (text, true) => {
                let level: f64 = text.parse().map_err(|_| bad("expected none or a number from 0 to 1"))?;
                if !(0.0..=1.0).contains(&level) {
                    return Err(bad("expected none or a number from 0 to 1"));
                }
                loud = Some(level);
            }
            (text, false) => muted = Some(text.parse().map_err(|_| bad("expected none, true or false"))?),
        }
        let section = self.section(name, |o| if volume { o.volume = None } else { o.muted = None });
        if volume {
            section.volume = loud;
        } else {
            section.muted = muted;
        }
        self.outputs.retain(|_, o| *o != OutputConfig::default());
        Ok(())
    }
}
