use super::{Cost, Descriptor, PassDef};
use crate::Quality;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lighter {
    pub passes: Vec<PassDef>,
    pub cost: Cost,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub up_to: Option<Quality>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Form {
    Full,
    Lighter,
    Frozen,
    Out,
}

impl Descriptor {
    pub(crate) fn lighten(&mut self) {
        let Some(l) = &self.lighter else { return self.lite = None };
        let mut lite = Descriptor { lighter: None, lite: None, fuse: None, ..self.clone() };
        (lite.passes, lite.cost, lite.light) = (l.passes.clone(), l.cost, true);
        self.lite = Some(Arc::new(lite));
    }

    pub fn form(&self, quality: Quality, optional: bool) -> Form {
        let fits = |d: &Descriptor| quality.effect_scale(d.class(), optional).is_some();
        let below = self.lighter.as_ref().and_then(|l| l.up_to).is_some_and(|q| quality <= q);
        match self.lite.as_deref().filter(|l| fits(l)) {
            Some(_) if below || !fits(self) => Form::Lighter,
            _ if fits(self) => Form::Full,
            _ if self.frozen(quality) => Form::Frozen,
            _ => Form::Out,
        }
    }

    pub fn live(&self, quality: Quality, optional: bool) -> Option<&Descriptor> {
        match self.form(quality, optional) {
            Form::Full => Some(self),
            Form::Lighter => self.lite.as_deref(),
            Form::Frozen | Form::Out => None,
        }
    }

    pub fn drawn(&self, quality: Quality, optional: bool) -> Option<(&Descriptor, f32, bool)> {
        if self.form(quality, optional) == Form::Frozen {
            return Some((self, 0.25, true));
        }
        let d = self.live(quality, optional)?;
        Some((d, quality.effect_scale(d.class(), optional)?, false))
    }

    pub fn lighter_units(&self) -> Option<f32> {
        self.lite.as_deref().map(Descriptor::units)
    }

    pub(crate) fn lighter_problems(&self) -> Vec<String> {
        let (Some(l), id) = (&self.lighter, &self.id) else { return Vec::new() };
        let units = |c: &Cost| c.taps * c.passes as f32 * c.scale * c.scale;
        let mut found = Vec::new();
        let mut fail = |ok: bool, what: &str| {
            if !ok {
                found.push(format!("{id}: the lighter variant {what}"));
            }
        };
        fail(!l.passes.is_empty(), "needs passes");
        fail(self.fuse.is_none(), "cannot belong to a fused effect");
        fail(l.up_to.is_none_or(|q| q < Quality::Q3), "replaces the effect only up to q0, q1 or q2");
        let measured = self.lite.as_deref().is_none_or(|v| v.units() < self.units());
        fail(units(&l.cost) < units(&self.cost) && measured, "must cost less than the full effect");
        let lite = self.lite.iter().flat_map(|v| v.problems());
        found
            .extend(lite.map(|p| p.replacen(&format!("{id}: "), &format!("{id}: the lighter variant: "), 1)));
        found
    }
}

pub fn drawn_builtin(d: &Descriptor) -> Option<&'static Descriptor> {
    let full = super::descriptor(&d.id)?;
    if d.light { full.lite.as_deref() } else { Some(full) }
}
