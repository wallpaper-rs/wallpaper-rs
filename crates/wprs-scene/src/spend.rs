use crate::area::{Bounds, covered};
use crate::{Kind, Layer, Scene, Tier};

pub const MAX_STATEFUL: usize = 16;
const TICKS: f32 = 4.0;
const FLOOR: f32 = 0.5;

#[derive(Debug, Clone, PartialEq)]
pub struct Spend {
    pub layer: String,
    pub index: usize,
    pub effect: String,
    pub units: f32,
    pub lighter: Option<f32>,
    pub bounds: Bounds,
    pub stateful: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Over {
    pub at: usize,
    pub why: String,
    pub lighter: bool,
}

impl Over {
    pub fn note(&self, key: &str) -> String {
        let how = if self.lighter { "was drawn with its lighter variant" } else { "was left out" };
        format!("the effect {key} {how}: {}", self.why)
    }
}

impl Spend {
    pub fn key(&self) -> String {
        format!("{} on {}", self.effect, self.layer)
    }

    pub fn work(&self, view: [f32; 4], pixels: f32) -> f32 {
        Spend::done(self.units, self.bounds, view, pixels)
    }

    fn done(units: f32, bounds: Bounds, view: [f32; 4], pixels: f32) -> f32 {
        (units * covered(bounds, view) * pixels / 1e6).max(FLOOR)
    }
}

impl Scene {
    pub fn spends(&self) -> Vec<Spend> {
        let uses = self.bounds().into_iter().flat_map(|(layer, bounds)| {
            layer.effects.iter().enumerate().filter_map(move |(index, e)| {
                let d = self.effect(&e.effect)?;
                let (stateful, effect) = (d.state.is_some(), e.effect.clone());
                let ticks = if stateful { TICKS } else { 1.0 };
                let (units, lighter) = (d.units() * ticks, d.lighter_units().map(|u| u * ticks));
                Some(Spend { layer: layer.id.clone(), index, effect, units, lighter, bounds, stateful })
            })
        });
        uses.collect()
    }

    pub fn left_over(&self, tier: Tier, view: [f32; 4], pixels: f32) -> Vec<String> {
        let spends = self.spends();
        let cut = over(&spends, |_| false, tier, view, pixels);
        cut.iter().map(|c| c.note(&spends[c.at].key())).collect()
    }

    pub fn without<'a>(&self, cut: impl IntoIterator<Item = &'a Spend>) -> Scene {
        let cut: Vec<&Spend> = cut.into_iter().collect();
        let mut scene = self.clone();
        trim(&mut scene.layers, &cut);
        scene
    }
}

fn trim(layers: &mut [Layer], cut: &[&Spend]) {
    for layer in layers {
        let gone: Vec<usize> = cut.iter().filter(|s| s.layer == layer.id).map(|s| s.index).collect();
        let mut at = 0;
        layer.effects.retain(|_| {
            at += 1;
            !gone.contains(&(at - 1))
        });
        if let Kind::Group { layers, .. } = &mut layer.kind {
            trim(layers, cut);
        }
    }
}

pub fn over(
    spends: &[Spend],
    skip: impl Fn(usize) -> bool,
    tier: Tier,
    view: [f32; 4],
    pixels: f32,
) -> Vec<Over> {
    let (limit, mut total, mut states, mut out) = (tier.ceiling(), 0.0, 0, Vec::new());
    for (at, s) in spends.iter().enumerate().filter(|(i, _)| !skip(*i)) {
        let work = s.work(view, pixels);
        let lighter =
            s.lighter.map(|u| Spend::done(u, s.bounds, view, pixels)).filter(|w| total + w <= limit);
        if s.stateful && states >= MAX_STATEFUL {
            let why = format!("the scene has more than {MAX_STATEFUL} effects that keep a state");
            out.push(Over { at, why, lighter: false });
        } else if total + work > limit {
            let why = format!("the effects of the scene would do more than {limit} units of work a frame");
            out.push(Over { at, why: format!("{why} on this GPU"), lighter: lighter.is_some() });
            (total, states) =
                (total + lighter.unwrap_or(0.0), states + usize::from(s.stateful && lighter.is_some()));
        } else {
            (total, states) = (total + work, states + usize::from(s.stateful));
        }
    }
    out
}
