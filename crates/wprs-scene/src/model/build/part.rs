use super::tangents;
use crate::rigged::doc::Primitive;
use crate::rigged::geometry::triangles;
use crate::rigged::math::{Mat, apply};
use crate::rigged::read::Bin;

pub(crate) struct Part {
    pub at: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<f32>,
    pub second: Vec<f32>,
    pub tangents: Vec<[f32; 4]>,
    pub list: Vec<u32>,
    pub paint: Option<Vec<f32>>,
}

fn normal_matrix(m: &Mat) -> ([f32; 9], bool) {
    let a = |r: usize, c: usize| m[r * 4 + c];
    let cof = |r: usize, c: usize| {
        let (r1, r2, c1, c2) = ((r + 1) % 3, (r + 2) % 3, (c + 1) % 3, (c + 2) % 3);
        a(r1, c1) * a(r2, c2) - a(r1, c2) * a(r2, c1)
    };
    let det: f32 = (0..3).map(|c| a(0, c) * cof(0, c)).sum();
    let s = if det < 0.0 { -1.0 } else { 1.0 };
    (std::array::from_fn(|k| cof(k / 3, k % 3) * s), det < 0.0)
}

fn turn(n: &[f32; 9], v: [f32; 3]) -> [f32; 3] {
    let o: [f32; 3] = std::array::from_fn(|r| n[r * 3] * v[0] + n[r * 3 + 1] * v[1] + n[r * 3 + 2] * v[2]);
    let l = o.iter().map(|x| x * x).sum::<f32>().sqrt();
    if l > 1e-12 { o.map(|x| x / l) } else { [0.0, 0.0, 1.0] }
}

pub(crate) fn part(
    bin: &Bin,
    p: &Primitive,
    room: usize,
    (world, weights): (&Mat, &[f32]),
) -> Result<Part, String> {
    let (normals, flipped) = normal_matrix(world);
    let at = |key: &str| p.attributes.get(key).copied();
    let position = at("POSITION").ok_or("has a primitive without positions")?;
    if bin.count(position)? > room {
        return Err(format!("has more than {} vertices", super::super::MAX_MODEL_VERTICES));
    }
    let mut positions = bin.floats(position, 3)?;
    let count = positions.len() / 3;
    let shaped = |key: &str, v: &mut [f32]| super::super::morphs::shaped(bin, p, weights, key, v);
    shaped("POSITION", &mut positions)?;
    let read = |key: &str, width: usize| at(key).map(|a| bin.floats(a, width)).transpose();
    let uvs = read("TEXCOORD_0", 2)?.unwrap_or_else(|| vec![0.0; count * 2]);
    let second = read("TEXCOORD_1", 2)?.filter(|v| v.len() == count * 2).unwrap_or_else(|| uvs.clone());
    let given = p.indices.map_or(Ok((0..count as u32).collect()), |a| bin.ints(a, 1))?;
    if given.iter().any(|i| *i as usize >= count) {
        return Err("has an index out of the range of its vertices".into());
    }
    let mut list = Vec::with_capacity(given.len());
    triangles(&given, p.mode.unwrap_or(4), 0, &mut list)?;
    if flipped {
        list.chunks_mut(3).for_each(|t| t.swap(1, 2));
    }
    let placed: Vec<[f32; 3]> = positions.chunks(3).map(|v| apply(world, [v[0], v[1], v[2]])).collect();
    let mut given = read("NORMAL", 3)?.filter(|v| v.len() == count * 3);
    given.as_mut().map(|v| shaped("NORMAL", v)).transpose()?;
    let normals: Vec<[f32; 3]> = match given {
        Some(v) => v.chunks(3).map(|n| turn(&normals, [n[0], n[1], n[2]])).collect(),
        None => tangents::smooth(&placed, &list),
    };
    let mut given = read("TANGENT", 4)?.filter(|v| v.len() == count * 4);
    given.as_mut().map(|v| bent(v, |d| shaped("TANGENT", d))).transpose()?;
    let tangents = match given {
        Some(t) => t.chunks(4).map(|t| tangents::along(world, t, flipped)).collect(),
        None => tangents::tangents(&placed, &normals, &uvs, &list),
    };
    if uvs.len() != count * 2 {
        return Err("has a primitive whose attributes count different vertices".into());
    }
    let paint = match at("COLOR_0") {
        Some(a) => {
            let width = bin.doc.accessors.get(a).map_or(4, |x| if x.kind == "VEC3" { 3 } else { 4 });
            let c = bin.floats(a, width)?;
            (c.len() == count * width).then(|| c.chunks(width).map(|c| c[0]).collect())
        }
        None => None,
    };
    Ok(Part { at: placed, normals, uvs, second, tangents, list, paint })
}

fn bent(t: &mut [f32], shape: impl FnOnce(&mut [f32]) -> Result<(), String>) -> Result<(), String> {
    let mut d: Vec<f32> = t.chunks(4).flat_map(|t| [t[0], t[1], t[2]]).collect();
    shape(&mut d)?;
    t.chunks_mut(4).zip(d.chunks(3)).for_each(|(t, d)| t[..3].copy_from_slice(d));
    Ok(())
}
