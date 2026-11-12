use crate::resolve::Context;
use crate::{Affine, Layer, Lean, Quality, Runtime};

#[derive(Debug, Clone, PartialEq)]
pub struct Seated {
    pub id: String,
    pub frame: Affine,
    pub world: Affine,
    pub lean: Option<Lean>,
    pub origin: [f32; 2],
    pub visible: bool,
}

fn visit<'a>(cx: &Context<'a>, layers: &'a [Layer], out: &mut Vec<Seated>) {
    for layer in layers {
        let set = cx.script.layers.get(&layer.id).unwrap_or(&crate::input::UNSET);
        let place = cx.placed(layer, None, 0);
        out.push(Seated {
            id: layer.id.clone(),
            frame: place.frame,
            world: place.world,
            lean: place.lean,
            origin: crate::kin::origin_of(layer, set),
            visible: place.visible,
        });
        visit(cx, layer.children(), out);
    }
}

impl Runtime {
    pub fn seats(&self, view: [f32; 4], quality: Quality) -> Vec<Seated> {
        let cx = Context { view, ..self.context() };
        let cx = Context { camera: cx.lens(quality), ..cx };
        let mut out = Vec::new();
        visit(&cx, &self.scene.layers, &mut out);
        out
    }
}
