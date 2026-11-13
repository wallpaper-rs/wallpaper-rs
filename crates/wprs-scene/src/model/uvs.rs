use crate::rigged::doc::{Gltf, Reference};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MapWrap {
    #[default]
    Repeat,
    Clamp,
    Mirror,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sampling {
    pub wrap: [MapWrap; 2],
    pub sharp: bool,
    pub shrink_sharp: bool,
    pub mips: Option<bool>,
}

impl Default for Sampling {
    fn default() -> Self {
        Sampling { wrap: [MapWrap::Repeat; 2], sharp: false, shrink_sharp: false, mips: Some(false) }
    }
}

pub const SAME_UV: [f32; 6] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapUv {
    pub set: u8,
    pub matrix: [f32; 6],
    pub sampling: Sampling,
}

impl Default for MapUv {
    fn default() -> Self {
        MapUv { set: 0, matrix: SAME_UV, sampling: Sampling::default() }
    }
}

impl MapUv {
    pub fn apply(&self, [u, v]: [f32; 2]) -> [f32; 2] {
        let m = &self.matrix;
        [m[0] * u + m[1] * v + m[2], m[3] * u + m[4] * v + m[5]]
    }
}

fn wrap(code: Option<u32>) -> MapWrap {
    match code {
        Some(33071) => MapWrap::Clamp,
        Some(33648) => MapWrap::Mirror,
        _ => MapWrap::Repeat,
    }
}

pub(crate) fn sampling(doc: &Gltf, texture: usize) -> Sampling {
    let found = doc.textures.get(texture).and_then(|t| doc.samplings.get(t.sampler?));
    let Some(s) = found else { return Sampling::default() };
    let (shrink_sharp, mips) = match s.min {
        Some(9728) => (true, None),
        Some(9729) => (false, None),
        Some(9984) => (true, Some(true)),
        Some(9985) => (false, Some(true)),
        Some(9986) => (true, Some(false)),
        _ => (false, Some(false)),
    };
    Sampling { wrap: [wrap(s.s), wrap(s.t)], sharp: s.mag == Some(9728), shrink_sharp, mips }
}

pub(super) fn map_uv(doc: &Gltf, r: &Reference, name: &str) -> Result<MapUv, String> {
    let moved = r.extensions.moved.unwrap_or_default();
    let set = moved.set.unwrap_or(r.set);
    let [ox, oy] = moved.offset.unwrap_or([0.0; 2]);
    let [sx, sy] = moved.scale.unwrap_or([1.0; 2]);
    let turn = moved.rotation.unwrap_or(0.0);
    let (sin, cos) = turn.sin_cos();
    let matrix = [cos * sx, sin * sy, ox, -sin * sx, cos * sy, oy];
    if !matrix.iter().all(|x| x.is_finite()) {
        return Err(format!("material {name} moves a texture by a value that is not a number"));
    }
    let set = if set == 1 { 1 } else { 0 };
    Ok(MapUv { set, matrix, sampling: sampling(doc, r.index) })
}
