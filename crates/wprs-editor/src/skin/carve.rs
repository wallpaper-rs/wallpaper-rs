use super::{Piece, Skin};
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use wprs_core::message::said;

pub const MAX_SHAPES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Carve {
    Push,
    Pull,
    Smooth,
    Erase,
}

pub struct Chisel {
    pub mode: Carve,
    pub by: f32,
    pub radius: f32,
    pub falloff: f32,
}

fn reach(d: f32, falloff: f32) -> f32 {
    let hard = 1.0 - falloff;
    match d {
        d if d >= 1.0 => 0.0,
        d if d <= hard => 1.0,
        d => {
            let s = (d - hard) / falloff.max(1e-6);
            1.0 - s * s * (3.0 - 2.0 * s)
        }
    }
}

impl Skin {
    fn target(&mut self, mesh: usize, name: &str) -> Result<usize> {
        let json = &mut self.glb.json["meshes"][mesh];
        let used = json["primitives"]
            .as_array()
            .map_or(0, |p| p.iter().map(|p| p["targets"].as_array().map_or(0, Vec::len)).max().unwrap_or(0));
        let mut names: Vec<Value> = json["extras"]["targetNames"].as_array().cloned().unwrap_or_default();
        let known =
            (0..used.max(names.len())).find(|t| names.get(*t).and_then(Value::as_str).unwrap_or("") == name);
        if let Some(t) = known {
            return Ok(t);
        }
        ensure!(
            used < MAX_SHAPES,
            said("editor.shapes_most", "a mesh has at most %1 blend shapes", &[&MAX_SHAPES])
        );
        while names.len() < used {
            names.push(json!(format!("shape_{}", names.len())));
        }
        names.push(json!(name));
        json["extras"]["targetNames"] = json!(names);
        for p in json["primitives"].as_array_mut().into_iter().flatten() {
            let list = p["targets"].as_array().cloned().unwrap_or_default();
            let mut list = list;
            list.resize(used, json!({}));
            list.push(json!({}));
            p["targets"] = json!(list);
        }
        Ok(used)
    }

    fn slot(piece: &mut Piece, target: usize) -> usize {
        match piece.targets.iter().position(|t| t.0 == target) {
            Some(i) => i,
            None => {
                piece.targets.push((target, vec![[0.0; 3]; piece.count]));
                piece.targets.len() - 1
            }
        }
    }

    pub fn carve(&mut self, name: &str, stroke: &[[f32; 2]], c: &Chisel) -> Result<usize> {
        let mut pieces = std::mem::take(&mut self.pieces);
        let outcome = self.carved(&mut pieces, name, stroke, c);
        self.pieces = pieces;
        outcome
    }

    fn carved(&mut self, pieces: &mut [Piece], name: &str, stroke: &[[f32; 2]], c: &Chisel) -> Result<usize> {
        ensure!(!pieces.is_empty(), said("editor.no_parts", "the mesh has no parts to shape", &[]));
        let mut slots = Vec::with_capacity(pieces.len());
        for p in pieces.iter_mut() {
            let t = self.target(p.mesh, name)?;
            slots.push(Skin::slot(p, t));
        }
        let mut deltas = vec![[0.0f32; 3]; self.points.len()];
        for (p, s) in pieces.iter().zip(&slots) {
            deltas[p.first..p.first + p.count].copy_from_slice(&p.targets[*s].1);
        }
        let around = {
            let mut out = vec![Vec::new(); self.points.len()];
            for t in &self.triangles {
                for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
                    out[a as usize].push(b as usize);
                    out[b as usize].push(a as usize);
                }
            }
            out
        };
        let before = deltas.clone();
        let r = c.radius.max(1e-3);
        let limit = self.size[0].max(self.size[1]);
        let mut touched = 0;
        for (v, p) in self.points.iter().enumerate() {
            let nearest = stroke.iter().map(|s| ((s[0] - p[0]).hypot(s[1] - p[1]) / r, *s));
            let Some(near) = nearest.min_by(|a, b| a.0.total_cmp(&b.0)) else { continue };
            let a = reach(near.0, c.falloff);
            if a <= 0.0 {
                continue;
            }
            touched += 1;
            let d = &mut deltas[v];
            match c.mode {
                Carve::Push | Carve::Pull => {
                    let (dx, dy) = (p[0] - near.1[0], p[1] - near.1[1]);
                    let len = dx.hypot(dy);
                    if len > 1e-4 {
                        let sign = if c.mode == Carve::Push { 1.0 } else { -1.0 };
                        d[0] += sign * c.by * a * dx / len;
                        d[1] += sign * c.by * a * dy / len;
                    }
                }
                Carve::Smooth if !around[v].is_empty() => {
                    for k in 0..3 {
                        let mean =
                            around[v].iter().map(|n| before[*n][k]).sum::<f32>() / around[v].len() as f32;
                        d[k] += (mean - d[k]) * c.by * a;
                    }
                }
                Carve::Smooth => {}
                Carve::Erase => d.iter_mut().for_each(|x| *x *= 1.0 - c.by * a),
            }
            d.iter_mut().for_each(|x| *x = x.clamp(-limit, limit));
        }
        for (p, s) in pieces.iter_mut().zip(&slots) {
            p.targets[*s].1.copy_from_slice(&deltas[p.first..p.first + p.count]);
        }
        Ok(touched)
    }
}
