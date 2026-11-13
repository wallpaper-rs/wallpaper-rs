use serde_json::{Value, json};
use std::path::Path;
use wprs_scene::Quality;

const LEVELS: [Quality; 4] = [Quality::Q0, Quality::Q1, Quality::Q2, Quality::Q3];

pub fn asks(q: &Value) -> bool {
    q.get("level").is_some()
}

pub fn effective(asked: Quality, battery: bool) -> Quality {
    match battery {
        true => asked.on_battery(),
        false => asked,
    }
}

fn quality(q: &Value) -> Quality {
    serde_json::from_value(q["quality"].clone()).unwrap_or_default()
}

pub fn query(dir: &Path, doc: &Value, q: &Value) -> Value {
    let q = &q["level"];
    let (asked, battery, calm) = (quality(q), q["battery"] == true, q["calm"] == true);
    let level = effective(asked, battery);
    let Ok(mut scene) = serde_json::from_value::<wprs_scene::Scene>(doc["scene"].clone()) else {
        return Value::Null;
    };
    crate::cache::read_files(&mut scene, dir);
    let mut effects = Vec::new();
    for (_, layer) in crate::layers::walk(doc) {
        for use_ in layer["effects"].as_array().into_iter().flatten() {
            let id = use_["effect"].as_str().unwrap_or_default();
            let found =
                scene.loaded.get(id).map(|d| (**d).clone()).or_else(|| wprs_scene::descriptor(id).cloned());
            let Some(d) = found else { continue };
            let optional = use_["optional"] == true;
            let follows = serde_json::from_value::<wprs_scene::EffectUse>(use_.clone())
                .is_ok_and(|e| d.needs(wprs_scene::Use::Cursor, &e, None));
            let at = |l: Quality| l.effect_scale(d.class(), optional).map(|s| s * d.scale_at(l));
            let scale = at(level);
            let units = |s: Option<f32>| s.map(|s| d.units() * s * s);
            effects.push(json!({ "layer": layer["id"], "effect": id, "optional": optional,
                "kept": scale.is_some(), "follows": follows, "scale": scale, "units": units(scale), "full_units": units(at(Quality::Q3)),
                "levels": LEVELS.map(|l| units(at(l))) }));
        }
    }
    let swarms: Vec<_> = scene.particles.iter().filter(|(_, s)| s.listens().1).map(|(id, _)| id).collect();
    let bloom = &doc["scene"]["canvas"]["bloom"];
    let glare = bloom["hdr"].is_array() && (level == Quality::Q3 || (level == Quality::Q2 && !battery));
    let left = effects.iter().filter(|e| e["kept"] == false).count();
    let spent: f64 = effects.iter().filter_map(|e| e["units"].as_f64()).sum();
    json!({ "asked": asked, "quality": level, "battery": battery, "calm": calm,
        "render_scale": level.render_scale(), "density": level.density(), "particles": level.particles(),
        "bloom": { "on": bloom.is_object(), "hdr": bloom["hdr"].is_array(), "glare": glare },
        "pointer": { "emitters": swarms },
        "effects": effects, "left_out": left, "units": spent })
}

pub fn shown(doc: &Value, bloom: bool) -> Value {
    let mut scene = crate::actions::visible(doc);
    crate::swarms::pin_seed(&mut scene);
    crate::rigs::preview(&mut scene, &json!({ "bloom": bloom }));
    scene
}

pub fn open(doc: &Value, bloom: bool) -> anyhow::Result<(wprs_scene::Scene, wprs_scene::Values)> {
    let scene = wprs_scene::Scene::from_json(&shown(doc, bloom).to_string())?;
    let properties: std::collections::BTreeMap<String, wprs_format::Property> =
        serde_json::from_value(doc["manifest"]["properties"].clone()).unwrap_or_default();
    Ok((scene, wprs_scene::effective(&properties, &[])))
}

pub fn reader(dir: &Path) -> impl Fn(&str) -> Result<Vec<u8>, String> + '_ {
    move |path: &str| match std::path::Path::new(path)
        .components()
        .all(|c| matches!(c, std::path::Component::Normal(_)))
    {
        true => std::fs::read(dir.join(path)).map_err(|e| format!("{path}: {e}")),
        false => Err(format!("{path} is outside the scene folder")),
    }
}
