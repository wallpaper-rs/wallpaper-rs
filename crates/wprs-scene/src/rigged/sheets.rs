use super::Rigged;
use super::read::Bin;

pub const MAX_SHEETS: usize = 8;

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
}
