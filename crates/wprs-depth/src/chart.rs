use crate::Map;
use crate::shifted::found;
use image::{Rgba, RgbaImage};
use serde::Serialize;
use wprs_core::edit::Depth;

const FILL: u8 = 110;
const GRID_EDGE: usize = 128;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Zone {
    pub order: usize,
    pub depth: f32,
    pub share: f32,
    pub at: [f32; 2],
    pub colour: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chart {
    pub size: (usize, usize),
    pub zones: Vec<Zone>,
    pub order: Vec<u32>,
}

pub fn colour(t: f32) -> [u8; 3] {
    const STOPS: [[f32; 3]; 5] = [
        [66.0, 135.0, 245.0],
        [34.0, 197.0, 190.0],
        [52.0, 199.0, 89.0],
        [250.0, 204.0, 21.0],
        [239.0, 68.0, 68.0],
    ];
    let at = t.clamp(0.0, 1.0) * (STOPS.len() - 1) as f32;
    let k = (at.floor() as usize).min(STOPS.len() - 2);
    let f = at - k as f32;
    let mix = |c: usize| (STOPS[k][c] + (STOPS[k + 1][c] - STOPS[k][c]) * f).round() as u8;
    [mix(0), mix(1), mix(2)]
}

pub fn chart(map: &Map, depth: &Depth) -> Chart {
    let size = (map.width() as usize, map.height() as usize);
    let levels: Vec<f32> = map.as_raw().iter().map(|v| f32::from(*v) / 65535.0).collect();
    let (_, regions) = found(&levels, size, depth);
    let count = regions.means.len();
    let mut ranked: Vec<usize> = (0..count).collect();
    ranked.sort_by(|a, b| regions.means[*b].total_cmp(&regions.means[*a]));
    let mut place = vec![0u32; count];
    ranked.iter().enumerate().for_each(|(order, label)| place[*label] = order as u32);
    let order: Vec<u32> = regions.label.iter().map(|l| place[*l as usize]).collect();
    let (mut sum, mut pixels) = (vec![[0.0f64; 2]; count], vec![0usize; count]);
    for (k, l) in order.iter().enumerate() {
        let (x, y) = ((k % size.0) as f64 + 0.5, (k / size.0) as f64 + 0.5);
        sum[*l as usize] = [sum[*l as usize][0] + x, sum[*l as usize][1] + y];
        pixels[*l as usize] += 1;
    }
    let centre = |l: usize| [sum[l][0] / pixels[l].max(1) as f64, sum[l][1] / pixels[l].max(1) as f64];
    let mut best = vec![(f64::MAX, 0usize); count];
    for (k, l) in order.iter().enumerate() {
        let c = centre(*l as usize);
        let d = (((k % size.0) as f64 + 0.5) - c[0]).powi(2) + (((k / size.0) as f64 + 0.5) - c[1]).powi(2);
        if d < best[*l as usize].0 {
            best[*l as usize] = (d, k);
        }
    }
    let total = order.len().max(1) as f32;
    let zones = ranked
        .iter()
        .enumerate()
        .map(|(o, label)| {
            let k = best[o].1;
            let at =
                [((k % size.0) as f32 + 0.5) / size.0 as f32, ((k / size.0) as f32 + 0.5) / size.1 as f32];
            Zone {
                order: o + 1,
                depth: regions.means[*label],
                share: regions.sizes[*label] as f32 / total,
                at,
            }
        })
        .collect();
    Chart { size, zones, order }
}

impl Chart {
    pub fn painted(&self) -> RgbaImage {
        let (w, h) = self.size;
        let last = (self.zones.len().max(2) - 1) as f32;
        let at = |x: usize, y: usize| self.order[y * w + x];
        RgbaImage::from_fn(w as u32, h as u32, |x, y| {
            let (x, y) = (x as usize, y as usize);
            let mine = at(x, y);
            let edge = (x + 1 < w && at(x + 1, y) != mine) || (y + 1 < h && at(x, y + 1) != mine);
            let [r, g, b] = colour(1.0 - mine as f32 / last);
            Rgba([r, g, b, if edge { 255 } else { FILL }])
        })
    }

    pub fn grid(&self) -> (usize, usize, Vec<u32>) {
        let (w, h) = self.size;
        let scale = GRID_EDGE as f32 / w.max(h) as f32;
        let (gw, gh) =
            (((w as f32 * scale).round() as usize).max(1), ((h as f32 * scale).round() as usize).max(1));
        let cells = (0..gw * gh)
            .map(|k| {
                let (x, y) = ((k % gw) * w / gw + w / gw / 2, (k / gw) * h / gh + h / gh / 2);
                self.order[y.min(h - 1) * w + x.min(w - 1)]
            })
            .collect();
        (gw, gh, cells)
    }
}
