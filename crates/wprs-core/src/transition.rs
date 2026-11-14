use crate::config::{Config, OutputConfig};
use crate::settings::TransitionKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LessMotion {
    #[default]
    System,
    Always,
    Never,
}

impl LessMotion {
    pub const WORDS: [&str; 3] = ["system", "always", "never"];

    pub fn reduces(self, system: bool) -> bool {
        match self {
            LessMotion::System => system,
            LessMotion::Always => true,
            LessMotion::Never => false,
        }
    }
}

impl TransitionKind {
    pub const WORDS: [&str; 3] = ["own", "fade", "none"];
    pub const KEY: &str = "transition.kind";
    pub const OWN: &str = "transition.kind.";
    pub const GENERAL: &str = "general";

    pub fn parse(text: &str) -> Result<TransitionKind, String> {
        match text.trim().to_lowercase().as_str() {
            "own" => Ok(TransitionKind::Own),
            "fade" => Ok(TransitionKind::Fade),
            "none" => Ok(TransitionKind::None),
            _ => Err(format!(
                "expected {}, or {} to follow {}",
                Self::WORDS.join(", "),
                Self::GENERAL,
                Self::KEY
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        Self::WORDS[self as usize]
    }
}

impl Config {
    pub fn transition(&self, output: &str) -> TransitionKind {
        self.own_transition(output).unwrap_or(self.transition.kind)
    }

    pub fn own_transition(&self, output: &str) -> Option<TransitionKind> {
        self.outputs.get(output).filter(|_| !output.is_empty()).and_then(|o| o.transition)
    }

    pub fn own_transitions(&self) -> BTreeMap<String, String> {
        let own = |(name, o): (&String, &OutputConfig)| Some((name.clone(), o.transition?.as_str().into()));
        self.outputs.iter().filter_map(own).collect()
    }

    pub(crate) fn set_own_transition(&mut self, output: &str, value: &str) -> Result<(), String> {
        let (output, text) = (output.trim(), value.trim());
        if output.is_empty() || output.len() > 128 || output.chars().any(char::is_control) {
            return Err(format!("invalid output name in {}<output>", TransitionKind::OWN));
        }
        let own = match text {
            "" | TransitionKind::GENERAL => None,
            _ => Some(
                TransitionKind::parse(text)
                    .map_err(|e| format!("invalid value for {}{output}: {e}", TransitionKind::OWN))?,
            ),
        };
        match self.outputs.get_mut(output) {
            Some(o) => o.transition = own,
            None if own.is_some() => self.outputs.entry(output.to_owned()).or_default().transition = own,
            None => {}
        }
        self.outputs.retain(|_, o| *o != OutputConfig::default());
        Ok(())
    }
}
