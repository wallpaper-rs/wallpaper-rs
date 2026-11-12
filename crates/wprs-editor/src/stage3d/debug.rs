use crate::action::EDITOR;
use crate::ops::{Op, set};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::path::Path;
use wprs_core::message::said;

pub(crate) fn debug(
    dir: &Path,
    doc: &Value,
    show: Option<&str>,
    (layer, material): (&Option<String>, &Option<String>),
) -> Result<Vec<Op>> {
    let at = format!("{EDITOR}/debug3d");
    let Some(s) = show else {
        return Ok(doc.pointer(&at).map(|_| Op::Remove { path: at.clone() }).into_iter().collect());
    };
    ensure!(
        ["mask", "matte", "shadows"].contains(&s),
        said("editor.debug3d", "the 3D view shows mask, matte or shadows", &[])
    );
    let (layer, name) = match (layer, material) {
        (None, None) => return Ok(set(doc, &at, json!(s))),
        (Some(l), Some(n)) => (l, n),
        _ => anyhow::bail!(said(
            "editor.debug3d_material",
            "a debug view of one material names its layer and material",
            &[]
        )),
    };
    let (_, found) = super::edit::model_layer(doc, layer)?;
    let (model, _) = super::store::asset(dir, doc, found["model"].as_str().unwrap_or_default())?;
    ensure!(
        model.material(name).is_some(),
        said("editor.material_name", "the model has no material named %1", &[name])
    );
    Ok(set(doc, &at, json!({ "show": s, "layer": layer, "material": name })))
}
