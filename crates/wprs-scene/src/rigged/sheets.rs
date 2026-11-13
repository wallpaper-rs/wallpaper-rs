use super::read::Bin;
use super::{Rigged, VERTEX_FLOATS};

pub const MAX_SHEETS: usize = 64;
pub const MAX_REPEATS: u32 = 4;
const SLACK: f32 = 1e-3;

fn image_of(bin: &Bin, [node, primitive]: [u16; 2]) -> Option<usize> {
    let doc = bin.doc;
    let mesh = doc.nodes.get(usize::from(node))?.mesh?;
    let material = doc.meshes.get(mesh)?.primitives.get(usize::from(primitive))?.material?;
    let texture = doc.materials.get(material)?.pbr.as_ref()?.texture.as_ref()?.index;
    doc.textures.get(texture)?.source
}

fn bytes(bin: &Bin, image: usize) -> Option<Vec<u8>> {
    Some(bin.view(bin.doc.images.get(image)?.view?).ok()?.to_vec())
}

pub(crate) fn sheets(bin: &Bin, rig: &mut Rigged) {
    let wanted: Vec<Option<usize>> = rig.pieces.iter().map(|p| image_of(bin, p.source)).collect();
    let mut chosen: Vec<usize> = Vec::new();
    for image in wanted.iter().flatten() {
        if !chosen.contains(image) && chosen.len() < MAX_SHEETS && bytes(bin, *image).is_some() {
            chosen.push(*image);
        }
    }
    if chosen.is_empty() && !bin.doc.images.is_empty() && bytes(bin, 0).is_some() {
        chosen.push(0);
    }
    let at = |w: &Option<usize>| w.and_then(|i| chosen.iter().position(|c| *c == i)).unwrap_or(0) as u16;
    rig.sheets = wanted.iter().map(at).collect();
    rig.pictures = chosen.iter().filter_map(|i| bytes(bin, *i)).collect();
    rig.repeats = repeats(rig);
}

fn repeats(rig: &Rigged) -> Vec<[f32; 4]> {
    let mut reach = vec![[f32::MAX, f32::MAX, f32::MIN, f32::MIN]; rig.pictures.len()];
    for v in rig.vertices.chunks_exact(VERTEX_FLOATS) {
        let piece = v[VERTEX_FLOATS - 1] as usize;
        let Some(r) = rig.sheets.get(piece).and_then(|s| reach.get_mut(usize::from(*s))) else { continue };
        let (u, w) = (v[3], v[4]);
        if u.is_finite() && w.is_finite() {
            *r = [r[0].min(u), r[1].min(w), r[2].max(u), r[3].max(w)];
        }
    }
    let span = |lo: f32, hi: f32| {
        let (lo, hi) = ((lo + SLACK).floor(), (hi - SLACK).ceil());
        (lo, (hi - lo).clamp(1.0, MAX_REPEATS as f32))
    };
    let all: Vec<[f32; 4]> = reach
        .iter()
        .map(|[u0, v0, u1, v1]| {
            let ((lu, nu), (lv, nv)) = (span(*u0, *u1), span(*v0, *v1));
            match u0 <= u1 && v0 <= v1 {
                true => [lu, lv, nu, nv],
                false => [0.0, 0.0, 1.0, 1.0],
            }
        })
        .collect();
    match all.iter().all(|r| *r == [0.0, 0.0, 1.0, 1.0]) {
        true => Vec::new(),
        false => all,
    }
}
