use super::render;
use crate::ops::{Op, unescape};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use wprs_core::message::said;

pub(super) fn drawers<'a>(doc: &'a Value, system: &str) -> Vec<&'a str> {
    let layers = crate::layers::walk(doc).into_iter().map(|(_, l)| l);
    layers
        .filter(|l| l["type"] == "particles" && l["system"] == system)
        .filter_map(|l| l["id"].as_str())
        .collect()
}

pub(super) fn differing(system: &Value, own: &Value) -> Vec<String> {
    let (a, b) = (system.as_object(), own.as_object());
    let keys = a.into_iter().chain(b).flat_map(|m| m.keys());
    let mut keys: Vec<String> = keys.cloned().collect();
    keys.sort();
    keys.dedup();
    keys.retain(|k| system.get(k) != own.get(k));
    keys
}

pub(super) fn describe(doc: &Value, system: &str, s: &Value, chosen: Option<&Value>) -> Value {
    let own = chosen.map(|l| &l["render"]).filter(|r| r.is_object());
    let differs = own.map(|r| differing(&s["render"], r)).unwrap_or_default();
    json!({ "layers": drawers(doc, system), "layer": chosen.map(|l| &l["id"]), "own": own.is_some(),
        "differs": differs, "system": s["render"] })
}

pub(super) fn switch(doc: &Value, sys: &str, layer: &str, own: bool) -> Result<Vec<Op>> {
    if own {
        return render(doc, sys, Some(layer), |_, _| Vec::new());
    }
    let (path, found) = crate::actions::layer(doc, layer)?;
    let name = unescape(sys.rsplit('/').next().unwrap_or_default());
    let fits = found["type"] == "particles" && found["system"] == name.as_str();
    ensure!(
        fits,
        said("editor.not_particles", "the layer \"%1\" draws no particles of this system", &[&layer])
    );
    Ok(super::put(doc, &format!("{path}/render"), None))
}
