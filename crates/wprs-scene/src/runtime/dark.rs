use crate::lens::FadeTo;
use crate::resolve::Context;
use crate::{Color, Kind, Layer, Quality, Value, Wake};

pub(crate) type Veil = Option<Box<Layer>>;

pub(crate) fn veil(scene: &crate::Scene) -> Veil {
    let cam = scene.camera3d.as_ref()?;
    let black = cam.fade.is_some_and(|f| f.to == FadeTo::Black && f.duration > 0.0);
    (black && cam.paths.len() > 1).then(|| {
        let color = Value::Fixed(Color([0.0, 0.0, 0.0, 1.0]));
        Box::new(Layer::new("", Kind::Solid { color, size: None }))
    })
}

pub(super) fn dark(cx: &Context, quality: Quality) -> f32 {
    let Some(cam) = cx.scene.camera3d.as_ref() else { return 0.0 };
    let Some(f) = crate::eval::staged::paths::flight(cx, cam) else { return 0.0 };
    if f.moving {
        cx.paced(Wake::Frame, (quality == Quality::Q0).then_some(30));
    }
    f.fade.clamp(0.0, 1.0)
}
