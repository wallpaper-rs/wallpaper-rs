use anyhow::{Result, anyhow, ensure};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::{Descriptor, GENERIC_PICTURES, IMAGES, Param, ParamKind};

fn descriptor(dir: &Path, doc: &Value, shader: &str) -> Option<Descriptor> {
    let (json, _) = crate::wgsl::files(doc, shader)?;
    let text = std::fs::read_to_string(dir.join(json)).ok()?;
    serde_json::from_str(&text).ok()
}

fn picture(doc: &Value, name: &str) -> bool {
    let path = doc["scene"]["assets"][name]["path"].as_str().unwrap_or_default();
    let ext = path.rsplit('.').next().unwrap_or_default().to_ascii_lowercase();
    GENERIC_PICTURES.contains(&name) || IMAGES.contains(&ext.as_str())
}

pub(super) fn values(
    dir: &Path,
    doc: &Value,
    shader: &str,
    values: &BTreeMap<String, Value>,
    block: &mut Value,
) -> Result<()> {
    if values.is_empty() {
        return Ok(());
    }
    let d = descriptor(dir, doc, shader).ok_or_else(|| {
        anyhow!(said("editor.material_values", "only a package material shader takes these values", &[]))
    })?;
    for (id, v) in values {
        let def = d.params.iter().find(|p| p.id == *id).ok_or_else(|| {
            anyhow!(said("editor.material_unknown", "%1 is not a parameter of %2", &[&id, &shader]))
        })?;
        if v.is_null() {
            drop(block["params"].as_object_mut().and_then(|p| p.remove(id)));
            continue;
        }
        let fits = serde_json::from_value::<Param>(v.clone()).is_ok_and(|p| def.accepts(&p));
        ensure!(fits, said("editor.material_value", "%1 does not take %2", &[&id, &v]));
        let named = v.as_str().unwrap_or_default();
        ensure!(
            def.kind != ParamKind::Texture || picture(doc, named),
            said("editor.material_picture", "%1 is no picture of the package", &[&named])
        );
        block["params"][id] = v.clone();
    }
    Ok(())
}
