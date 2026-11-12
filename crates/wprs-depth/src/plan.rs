use crate::brush::marks;
use crate::build::MARGIN;
use crate::field::{field, field_size, sample, slope};
use crate::guided::Size;
use crate::matte::width;
use crate::regions::regions;
use crate::split::{Split, cut, distance, indices, mask, split};

pub(crate) struct Plan {
    pub size: Size,
    pub parts: Split,
    pub masks: Vec<Vec<bool>>,
    pub fields: Vec<Vec<f32>>,
    pub regions: usize,
    pub shift: f32,
    over: Vec<Vec<bool>>,
    need: Vec<Vec<f32>>,
    refill: Vec<bool>,
    protect: Vec<bool>,
}

pub(crate) struct Region {
    pub known: Vec<bool>,
    pub band: Vec<bool>,
    pub reached: Vec<f32>,
    pub limit: Vec<f32>,
    pub refill: Vec<bool>,
    pub reach: f32,
    pub safe: f32,
    pub far: bool,
}

fn either(a: &[bool], b: &[bool]) -> Vec<bool> {
    a.iter().zip(b).map(|(a, b)| *a || *b).collect()
}

impl Plan {
    pub fn new(
        levels: &[f32],
        size: Size,
        depth: &wprs_core::edit::Depth,
        (count, shift): (usize, f32),
    ) -> Plan {
        let found = regions(levels, size, depth);
        let per: Vec<f32> = found.label.iter().map(|l| found.means[*l as usize]).collect();
        let parts = match &depth.cuts {
            Some(cuts) => cut(&per, cuts),
            None => split(&per, count.clamp(1, 6)),
        };
        let owner = indices(&per, &parts);
        let masks: Vec<Vec<bool>> = (0..parts.count()).map(|i| mask(&owner, i)).collect();
        let mut over = vec![vec![false; levels.len()]; parts.count()];
        for i in (1..parts.count()).rev() {
            over[i - 1] = either(&over[i], &masks[i]);
        }
        let steep = slope(shift, size.0);
        let fields: Vec<Vec<f32>> = masks.iter().map(|m| field(levels, m, size, steep)).collect();
        let small = field_size(size).0 * field_size(size).1;
        let mut need = vec![vec![0.0f32; small]; parts.count()];
        let mut top = vec![f32::MIN; small];
        for i in (0..parts.count()).rev() {
            need[i] = top.iter().zip(&fields[i]).map(|(t, f)| (t - f).max(0.0)).collect();
            top.iter_mut().zip(&fields[i]).for_each(|(t, f)| *t = t.max(*f));
        }
        let (refill, protect) = marks(size, &depth.fills);
        let regions = found.means.len();
        Plan { size, parts, masks, fields, regions, shift, over, need, refill, protect }
    }

    pub fn region(&self, layer: usize) -> Region {
        let (own, over, far) = (&self.masks[layer], &self.over[layer], layer == 0);
        let matte = (width(self.size) + 1) as f32;
        let under: Vec<bool> = distance(over, self.size).iter().map(|d| *d <= matte.max(MARGIN)).collect();
        let mut known: Vec<bool> = own.iter().zip(&under).map(|(o, u)| *o && !*u).collect();
        if far && !known.contains(&true) {
            known.clone_from_slice(own);
        }
        let reached = distance(&known, self.size);
        let (w, h) = self.size;
        let small = field_size(self.size);
        let mut limit = vec![0.0f32; w * h];
        let (mut band, mut reach, mut safe) = (vec![false; w * h], 0.0f32, f32::MAX);
        for k in 0..w * h {
            let behind = over[k] || under[k];
            if known[k] || !behind {
                continue;
            }
            let need = sample(&self.need[layer], small, self.size, k % w, k / w) * w as f32;
            limit[k] = (self.shift * need).ceil() + MARGIN;
            band[k] = !self.protect[k] && (reached[k] <= limit[k] || self.refill[k]);
            if band[k] {
                reach = reach.max(limit[k].min(reached[k]));
            } else if need > 0.0 {
                safe = safe.min(((reached[k] - MARGIN) / need).max(0.0));
            }
        }
        Region { known, band, reached, limit, refill: self.refill.clone(), reach, safe, far }
    }
}
