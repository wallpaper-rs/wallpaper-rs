use crate::actions::Action;
use crate::ops::Op;
use crate::paint::{Stroke, paint};
use anyhow::Result;
use serde_json::Value;
use std::path::Path;

pub(crate) fn act(dir: &Path, doc: &Value, action: &Action) -> Option<Result<Vec<Op>>> {
    if let Some(ops) = crate::placing::act(dir, doc, action) {
        return Some(ops);
    }
    Some(match action {
        Action::Import { file, id, linear, atlas } => {
            let cut = (*linear, *atlas);
            crate::assets::cuttable(&file.to_string_lossy(), cut)
                .and_then(|_| crate::assets::imported(dir, doc, file, id.as_deref()))
                .and_then(|(id, path, fresh)| {
                    let ops = crate::assets::sheet(dir, doc, &id, &path, cut);
                    crate::assets::undone(dir, (&path, fresh), ops)
                })
        }
        Action::ImportEffect { descriptor, wgsl } => {
            crate::shaders::import(dir, doc, descriptor, wgsl.as_deref())
        }
        Action::RemoveEffect { effect } => crate::shaders::remove(doc, effect),
        Action::EffectRule(r) => crate::shaders::rule(dir, doc, r),
        Action::ImportScript(s) => crate::scripts::import(dir, doc, &s.file, s.template.as_deref()),
        Action::Script(e) => crate::scripts::edit(dir, doc, e),
        Action::Template(e) => crate::templates::edit(dir, doc, e),
        Action::Shader(e) => crate::wgsl::act(dir, doc, e),
        Action::ImportAssets(wanted) => crate::bring::import(dir, doc, wanted),
        Action::Asset(wanted) => crate::packing::act(dir, doc, wanted),
        Action::Rgb(e) => crate::rgb::act(doc, e),
        Action::Thumbnail { file, animated } => crate::previews::store(dir, doc, file, *animated)
            .and_then(|path| crate::previews::adopt(doc, *animated, &path)),
        Action::Shot(e) => crate::shotting::act(dir, doc, e),
        Action::PaintMask(m) | Action::CombFlow(m) => {
            let (channel, radius, hardness, value) = (m.channel, m.radius, m.hardness, m.value);
            let (points, direction) = (&m.points, matches!(action, Action::CombFlow(_)));
            paint(
                dir,
                doc,
                &Stroke { pointer: &m.pointer, channel, radius, hardness, value, points, direction },
            )
        }
        Action::Ramp(r) => crate::ramps::make(dir, doc, r),
        Action::DepthMap(d) => crate::depthmap::make(dir, doc, d),
        action => return crate::rigging::act(dir, doc, action),
    })
}
