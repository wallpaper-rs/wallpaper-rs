use crate::action::ColliderEdit;
use crate::clips::num;
use crate::limbs::listed;
use crate::ops::Op;
use anyhow::{Result, anyhow, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::MAX_BONE_COLLIDERS;

pub const DEFAULT_RADIUS: f32 = 0.1;

fn tune(entry: &mut Value, c: &ColliderEdit, rig: &wprs_scene::Rigged) -> Result<()> {
    if let Some(bone) = &c.bone {
        let found = rig.joint(bone).is_some_and(|n| !rig.joints[n].point);
        ensure!(found, said("editor.collider_bone", "the model has no bone named %1", &[bone]));
        entry["bone"] = json!(bone);
    }
    if let Some(shape) = c.shape {
        entry["shape"] = json!(shape);
    }
    if let Some(r) = c.radius {
        ensure!(
            r.is_finite() && (0.0..=100.0).contains(&r),
            said("editor.collider_radius", "the radius of a collider goes from 0 to 100 units", &[])
        );
        entry["radius"] = num(r);
    }
    if let Some(o) = c.offset {
        ensure!(
            o.iter().all(|v| v.is_finite() && v.abs() <= 1e4),
            said("editor.collider_offset", "the offset of a collider is three numbers within 10000", &[])
        );
        match o == [0.0; 3] {
            true => drop(entry.as_object_mut().map(|e| e.remove("offset"))),
            false => entry["offset"] = json!(o.map(num)),
        }
    }
    Ok(())
}

pub(crate) fn collider(dir: &Path, doc: &Value, c: &ColliderEdit) -> Result<Vec<Op>> {
    let (path, l) = crate::motion::figure(doc, &c.layer)?;
    ensure!(l["type"] == "model", said("editor.not_model_layer", "%1 is not a model layer", &[&c.layer]));
    let (_, _, rig) = crate::skin::rigged(dir, doc, l)?;
    let mut list = l["colliders"].as_array().cloned().unwrap_or_default();
    match c.index {
        Some(i) => {
            let entry = list.get_mut(i).ok_or_else(|| {
                anyhow!(said("editor.collider_index", "the layer has no collider %1", &[&i]))
            })?;
            match c.remove {
                true => drop(list.remove(i)),
                false => tune(entry, c, &rig)?,
            }
        }
        None => {
            ensure!(
                list.len() < MAX_BONE_COLLIDERS,
                said("editor.colliders_most", "a model has at most %1 colliders", &[&MAX_BONE_COLLIDERS])
            );
            ensure!(c.bone.is_some(), said("editor.collider_new", "a new collider names its bone", &[]));
            let mut entry = json!({ "bone": "", "radius": num(DEFAULT_RADIUS) });
            tune(&mut entry, c, &rig)?;
            list.push(entry);
        }
    }
    Ok(listed(doc, &format!("{path}/colliders"), list))
}
