use crate::adopt::Files;
use crate::custom::MAX_WGSL;
use crate::effects::{Descriptor, Stage};
use crate::validate::{Check, Issue};
use crate::value::Param;
use crate::{Ease, MAX_PARAMS, Quality, Scene};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

pub(crate) mod shader;

pub const MIN_TRANSITION_MS: u32 = 100;
pub const MAX_TRANSITION_MS: u32 = 10_000;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Transition {
    #[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
    pub coming: Option<Passage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out: Option<Passage>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Passage {
    pub descriptor: String,
    pub wgsl: String,
    #[serde(default = "standard")]
    pub duration_ms: u32,
    #[serde(default)]
    pub curve: Ease,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub params: BTreeMap<String, Param>,
}

fn standard() -> u32 {
    1_000
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Way {
    In,
    Out,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ready {
    pub way: Way,
    pub effect: Arc<Descriptor>,
    pub slots: [[f32; 4]; MAX_PARAMS],
    pub duration: Duration,
    pub curve: Ease,
}

pub type Readied = Result<Arc<Ready>, String>;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Passages {
    pub coming: Option<Readied>,
    pub out: Option<Readied>,
}

impl Ready {
    pub fn at(&self, t: f32) -> f32 {
        self.curve.apply(t.clamp(0.0, 1.0)).clamp(0.0, 1.0)
    }

    pub fn plays_at(&self, quality: Quality) -> bool {
        quality.effect_scale(self.effect.class(), true).is_some()
    }
}

impl Passages {
    pub fn way(&self, way: Way) -> Option<&Readied> {
        match way {
            Way::In => self.coming.as_ref(),
            Way::Out => self.out.as_ref(),
        }
    }

    pub fn problems(&self) -> Vec<(Way, &str)> {
        let failed = |way| Some((way, self.way(way)?.as_ref().err()?.as_str()));
        [failed(Way::In), failed(Way::Out)].into_iter().flatten().collect()
    }
}

fn ready(way: Way, p: &Passage, files: &mut impl Files) -> Readied {
    let mut text = |path: &str| {
        let bytes = files.read(path, MAX_WGSL).map_err(|e| format!("{path}: {e}"))?;
        String::from_utf8(bytes).map_err(|_| format!("{path} is not UTF-8"))
    };
    let (json, wgsl) = (text(&p.descriptor)?, text(&p.wgsl)?);
    let raw: serde_json::Value = serde_json::from_str(&json).map_err(|e| format!("{}: {e}", p.descriptor))?;
    let id = raw["id"].as_str().unwrap_or_default();
    let effect = crate::adopt::compiled(id, &json, &wgsl)?;
    if effect.stage != Stage::Transition {
        return Err("its descriptor needs \"stage\": \"transition\"".into());
    }
    let units = effect.units();
    if units > shader::MAX_TRANSITION_UNITS {
        let most = shader::MAX_TRANSITION_UNITS;
        return Err(format!("it costs {units:.2} units, more than the {most} a transition may cost"));
    }
    for (id, value) in &p.params {
        let Some(i) = effect.param(id) else { return Err(format!("its descriptor has no parameter {id}")) };
        if !effect.params[i].accepts(value) {
            return Err(format!("the value of {id} does not fit its type"));
        }
    }
    let slots = effect.pack(|id| p.params.get(id).map(Cow::Borrowed));
    let duration = Duration::from_millis(p.duration_ms.clamp(MIN_TRANSITION_MS, MAX_TRANSITION_MS).into());
    Ok(Arc::new(Ready { way, effect, slots, duration, curve: p.curve }))
}

impl Scene {
    pub fn read_passages(&mut self, files: &mut impl Files) {
        let t = self.transition.clone().unwrap_or_default();
        let coming = t.coming.as_ref().map(|p| ready(Way::In, p, files));
        let out = t.out.as_ref().map(|p| ready(Way::Out, p, files));
        self.passages = Passages { coming, out };
    }

    pub fn transition_effect(json: &str, wgsl: &str) -> Result<Arc<Descriptor>, String> {
        let raw: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        crate::adopt::compiled(raw["id"].as_str().unwrap_or_default(), json, wgsl)
    }
}

impl Check<'_> {
    pub(crate) fn passages(&mut self) {
        let Some(t) = &self.scene.transition else { return };
        let ways = [("in", &t.coming), ("out", &t.out)];
        for (name, p) in ways.into_iter().filter_map(|(n, p)| Some((n, p.as_ref()?))) {
            self.uses(true, "transition");
            let at = format!("/transition/{name}");
            for (field, path, ext) in [("descriptor", &p.descriptor, ".json"), ("wgsl", &p.wgsl, ".wgsl")] {
                self.filled(&format!("{at}/{field}"), path);
                if !path.to_ascii_lowercase().ends_with(ext) {
                    self.issues.push(Issue::Type(format!("{at}/{field}")));
                }
            }
            let (lo, hi) = (MIN_TRANSITION_MS as f32, MAX_TRANSITION_MS as f32);
            self.range(&format!("{at}/duration_ms"), p.duration_ms as f32, lo, hi);
            self.count(&format!("{at}/params"), p.params.len(), MAX_PARAMS);
        }
    }
}
