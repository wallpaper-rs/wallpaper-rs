use super::{MAX_MODEL_VERTICES, MAX_POSED_MORPHS, Model};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

pub const SCULPT_MAGIC: &[u8; 4] = b"WPSH";
pub const SCULPT_ENTRY: usize = 32;
pub const MAX_SCULPTS: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Carved {
    pub node: u16,
    pub primitive: u16,
    pub vertex: u32,
    pub at: [f32; 3],
    pub normal: [f32; 3],
}

pub fn carve_encode(entries: &[Carved]) -> Vec<u8> {
    let mut out = Vec::with_capacity(12 + entries.len() * SCULPT_ENTRY);
    out.extend_from_slice(SCULPT_MAGIC);
    out.extend_from_slice(&1u32.to_le_bytes());
    out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    for e in entries {
        out.extend_from_slice(&e.node.to_le_bytes());
        out.extend_from_slice(&e.primitive.to_le_bytes());
        out.extend_from_slice(&e.vertex.to_le_bytes());
        e.at.iter().chain(&e.normal).for_each(|v| out.extend_from_slice(&v.to_le_bytes()));
    }
    out
}

pub fn carve_decode(bytes: &[u8]) -> Result<Vec<Carved>, String> {
    let word = |at: usize| u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap_or_default());
    if bytes.len() < 12 || &bytes[..4] != SCULPT_MAGIC || word(4) != 1 {
        return Err("is not a sculpted blend shape".into());
    }
    let n = word(8) as usize;
    if n > MAX_MODEL_VERTICES || bytes.len() != 12 + n * SCULPT_ENTRY {
        return Err("has a sculpted blend shape of the wrong size".into());
    }
    let float = |at: usize| f32::from_le_bytes(bytes[at..at + 4].try_into().unwrap_or_default());
    let entry = |k: usize| {
        let at = 12 + k * SCULPT_ENTRY;
        let half = |o: usize| u16::from_le_bytes([bytes[at + o], bytes[at + o + 1]]);
        let three = |o: usize| [0, 1, 2].map(|i| float(at + o + i * 4));
        Carved { node: half(0), primitive: half(2), vertex: word(at + 4), at: three(8), normal: three(20) }
    };
    let all: Vec<Carved> = (0..n).map(entry).collect();
    match all.iter().flat_map(|e| e.at.iter().chain(&e.normal)).all(|v| v.is_finite() && v.abs() < 1e6) {
        true => Ok(all),
        false => Err("has a sculpted blend shape with numbers out of range".into()),
    }
}

impl Model {
    pub fn carve(&mut self, shapes: &[(String, Vec<u8>)]) -> Result<(), String> {
        let Some(rig) = self.rig.as_mut() else { return Ok(()) };
        let rig = Arc::make_mut(rig);
        let mut h = DefaultHasher::new();
        self.digest.hash(&mut h);
        for (name, bytes) in shapes.iter().take(MAX_SCULPTS) {
            if rig.shape(name).is_some() {
                return Err(format!("has the blend shape {name:?} in the file and sculpted"));
            }
            let entries = carve_decode(bytes)?;
            bytes.hash(&mut h);
            let shape = rig.shapes.len() as u16;
            rig.shapes.push(name.clone());
            rig.defaults.push(0.0);
            for (k, piece) in rig.pieces.iter_mut().enumerate() {
                let mine: Vec<&Carved> =
                    entries.iter().filter(|e| [e.node, e.primitive] == piece.source).collect();
                let Some(prim) = self.primitives.iter().find(|p| usize::from(p.piece) == k) else { continue };
                if mine.is_empty() {
                    continue;
                }
                let (base, stride) = (self.morphs.len(), prim.stride as usize);
                self.morphs.resize(base + piece.vertices as usize * stride, [0.0; 4]);
                for e in mine.iter().filter(|e| e.vertex < piece.vertices) {
                    let at = base + e.vertex as usize * stride;
                    self.morphs[at] = [e.at[0], e.at[1], e.at[2], 0.0];
                    self.morphs[at + 1] = [e.normal[0], e.normal[1], e.normal[2], 0.0];
                }
                if self.morphs.len() > MAX_POSED_MORPHS {
                    return Err(format!("has more than {MAX_POSED_MORPHS} blend shape values"));
                }
                piece.targets.push((shape, base as u32));
            }
        }
        self.digest = h.finish();
        rig.digest = self.digest;
        Ok(())
    }
}
