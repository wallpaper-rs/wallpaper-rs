use crate::rigs::round;
use serde_json::{Value, json};
use std::path::Path;
use wprs_scene::{Kind, Model, Quality, TRIANGLE_UNIT, Tier, model_caps, model_verdict};

const LEVELS: [Quality; 4] = [Quality::Q0, Quality::Q1, Quality::Q2, Quality::Q3];

fn left_out(q: Quality, shadows: u64, fur: u64) -> Vec<&'static str> {
    let short = shadows > 0 && fur > 0 && q < Quality::Q2;
    short.then_some("fur casts the shadow of its skin only, without its length").into_iter().collect()
}

pub fn cost(dir: &Path, doc: &Value, quality: Quality) -> Value {
    let Some(scene) = super::store::scene(dir, doc) else { return Value::Null };
    let shown: Vec<(&str, &Model)> = scene
        .layers()
        .filter_map(|l| match &l.kind {
            Kind::Model(m) if l.visible != wprs_scene::Value::Fixed(false) => {
                Some((l.id.as_str(), scene.solids.get(&m.model)?.as_ref()))
            }
            _ => None,
        })
        .collect();
    let models: Vec<&Model> = shown.iter().map(|(_, m)| *m).collect();
    let bones = scene.models.values().map(|f| f.bones.len()).max().unwrap_or(0);
    let triangles: u64 = models.iter().map(|m| m.triangles).sum();
    let groups = scene
        .layers()
        .filter(|l| matches!(l.kind, Kind::Group { space: Some(wprs_scene::Space::Depth), .. }))
        .count();
    let canvas = scene.canvas.size.map(|v| v as f32);
    let levels: Vec<Value> = LEVELS
        .iter()
        .map(|q| {
            let k = q.render_scale();
            let unit = |id: &&str| scene.layer(id).map_or(0.0, |l| scene.drawn_triangles(l, *q) as f32 / TRIANGLE_UNIT + 1.0);
            let units: f32 = shown.iter().map(|(id, _)| unit(id)).sum::<f32>() * k * k;
            let (shadows, fur) = (super::spend::shadow_texels(&scene, *q), super::spend::fur(&scene, *q));
            let colliders = super::spend::colliders(&scene, *q);
            json!({ "quality": q, "triangles": triangles, "drawn": scene.model_triangles(*q), "pixels": round(canvas[0] * canvas[1] * k * k),
                "units": round(units), "scene": round(scene.cost(*q)), "specular": *q > Quality::Q0,
                "shadow_texels": shadows, "fur_shells": fur.0, "fur_triangles": fur.1,
                "fur_shadow_triangles": if shadows > 0 && *q >= Quality::Q2 { fur.2 } else { 0 },
                "left_out": left_out(*q, shadows, fur.2),
                "skinned": super::spend::skinned(&scene), "physics": *q > Quality::Q0,
                "colliders": colliders.0, "colliders_left_out": colliders.1 })
        })
        .collect();
    let tiers: Vec<Value> = [Tier::T0, Tier::T1, Tier::T2, Tier::T3]
        .into_iter()
        .map(|t| {
            let v = model_verdict(&models, bones, t);
            let cap = model_caps(t, false);
            let memory: u64 = models.iter().map(|m| m.texture_bytes(cap.texture)).sum();
            let busy = scene.cost(t.quality(false)) > t.budget();
            let over = scene.model_triangles(Quality::Q0) > cap.triangles;
            let poster = over || scene.cost(Quality::Q0) > t.budget();
            json!({ "tier": t, "fits": v.fits && !busy && !over, "poster": poster, "reasons": v.reasons, "busy": busy, "memory": memory,
                "triangles": cap.triangles, "texture": cap.texture, "quality": t.quality(false), "shadows": cap.shadows })
        })
        .collect();
    let list: Vec<Value> = shown
        .iter()
        .map(|(id, m)| json!({ "id": id, "triangles": m.triangles, "textures": m.textures.len() }))
        .collect();
    json!({ "models": list, "groups": groups, "levels": levels, "tiers": tiers, "quality": quality,
        "phone": model_caps(Tier::T1, true) })
}
