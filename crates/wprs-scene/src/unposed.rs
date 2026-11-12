use crate::content::MaskChannel;
use crate::{Kind, Layer, Mask, Scene, Value};

fn pictured(mask: &Mask) -> Option<&str> {
    match (&mask.texture, &mask.layer, &mask.shape) {
        (Some(Value::Fixed(t)), None, None) => Some(t),
        _ => None,
    }
}

fn feeder(mesh: &Layer, id: String, texture: &str) -> Layer {
    let mut fed = mesh.clone();
    (fed.id, fed.visible, fed.opacity) = (id, Value::Fixed(false), Value::Fixed(1.0));
    (fed.mask, fed.effects, fed.lighting, fed.brightness) = (None, Vec::new(), None, None);
    (fed.blend, fed.solid, fed.cache) = Default::default();
    if let Kind::Mesh(m) = &mut fed.kind {
        (m.texture, m.color) = (Some(Value::Fixed(texture.to_owned())), None);
    }
    fed
}

fn posed(mesh: &mut Layer, taken: &mut std::collections::BTreeSet<String>) -> Vec<Layer> {
    if !matches!(mesh.kind, Kind::Mesh(_)) {
        return Vec::new();
    }
    let source = mesh.clone();
    let mut fed: Vec<(String, Layer)> = Vec::new();
    let masks = mesh.mask.iter_mut().chain(mesh.effects.iter_mut().filter_map(|e| e.mask.as_mut()));
    for mask in masks {
        let Some(texture) = pictured(mask).map(String::from) else { continue };
        let id = match fed.iter().find(|(t, _)| *t == texture) {
            Some((_, l)) => l.id.clone(),
            None => {
                let id =
                    (0..).map(|n| format!("sheet_{n}")).find(|id| !taken.contains(id)).unwrap_or_default();
                taken.insert(id.clone());
                fed.push((texture.clone(), feeder(&source, id.clone(), &texture)));
                id
            }
        };
        mask.channel = Some(mask.channel.unwrap_or(MaskChannel::Luminance));
        (mask.texture, mask.layer) = (None, Some(id));
    }
    fed.into_iter().map(|(_, l)| l).collect()
}

fn walk(layers: &mut Vec<Layer>, taken: &mut std::collections::BTreeSet<String>, made: &mut Vec<String>) {
    let mut out = Vec::with_capacity(layers.len());
    for mut layer in std::mem::take(layers) {
        if let Kind::Group { layers, .. } = &mut layer.kind {
            walk(layers, taken, made);
        }
        let fed = posed(&mut layer, taken);
        made.extend(fed.iter().map(|l| l.id.clone()));
        out.extend(fed);
        out.push(layer);
    }
    *layers = out;
}

impl Scene {
    pub(crate) fn pose_sheet_masks(&mut self) {
        let mut taken = self.layers().map(|l| l.id.clone()).collect();
        let mut made = Vec::new();
        walk(&mut self.layers, &mut taken, &mut made);
        self.sheets.extend(made);
    }
}
