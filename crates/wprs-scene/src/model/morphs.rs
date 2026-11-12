use super::{MAX_POSED_MORPHS, Model};
use crate::rigged::Rigged;
use crate::rigged::doc::Primitive;
use crate::rigged::read::Bin;

type Delta<'a> = Option<&'a [f32]>;

fn texel(d: Delta, v: usize) -> [f32; 4] {
    d.map_or([0.0; 4], |d| [d[v * 3], d[v * 3 + 1], d[v * 3 + 2], 0.0])
}

pub(super) fn morphs(
    bin: &Bin,
    p: &Primitive,
    (names, rig): (&[String], &Rigged),
    count: usize,
    model: &mut Model,
) -> Result<Vec<(u16, u32)>, String> {
    let wide = p.targets.iter().any(|t| t.contains_key("TANGENT"));
    let mut targets = Vec::with_capacity(p.targets.len());
    for (t, target) in p.targets.iter().enumerate() {
        let read = |key: &str| target.get(key).map(|a| bin.floats(*a, 3)).transpose();
        let (Some(deltas), normals, tangents) = (read("POSITION")?, read("NORMAL")?, read("TANGENT")?) else {
            continue;
        };
        let wrong = |d: &Option<Vec<f32>>| d.as_ref().is_some_and(|d| d.len() != count * 3);
        if deltas.len() != count * 3 || wrong(&normals) || wrong(&tangents) {
            return Err("has a blend shape with a different number of vertices".into());
        }
        let shape = names.get(t).and_then(|s| rig.shape(s)).unwrap_or(0) as u16;
        targets.push((shape, model.morphs.len() as u32));
        for v in 0..count {
            let at = texel(Some(&deltas), v);
            model.morphs.extend([at, texel(normals.as_deref(), v)]);
            if wide {
                model.morphs.push(texel(tangents.as_deref(), v));
            }
        }
        if model.morphs.len() > MAX_POSED_MORPHS {
            return Err(format!("has more than {MAX_POSED_MORPHS} blend shape values"));
        }
    }
    Ok(targets)
}

pub(super) fn shaped(
    bin: &Bin,
    p: &Primitive,
    weights: &[f32],
    key: &str,
    base: &mut [f32],
) -> Result<(), String> {
    for (target, w) in p.targets.iter().zip(weights).filter(|(_, w)| **w != 0.0 && w.is_finite()) {
        let Some(&a) = target.get(key) else { continue };
        let d = bin.floats(a, 3)?;
        if d.len() != base.len() {
            return Err("has a blend shape with a different number of vertices".into());
        }
        base.iter_mut().zip(&d).for_each(|(b, x)| *b += x * w);
    }
    Ok(())
}
