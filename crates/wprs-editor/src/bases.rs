use crate::listing::{named, ordered};
use crate::ops::escape;
use crate::settings::properties;
use serde_json::{Value, json};
use std::path::Path;
use wprs_scene::Control;

fn manifest(file: &Path) -> Option<Value> {
    let text = match file.is_dir() {
        true => std::fs::read(file.join(wprs_format::MANIFEST)).ok()?,
        false => {
            let package = wprs_format::open(file, &wprs_format::Limits::default()).ok()?;
            package.manifest.to_json().ok()?.into_bytes()
        }
    };
    serde_json::from_slice(&text).ok()
}

pub fn settings(file: &Path, preset: &Value) -> Value {
    let Some(manifest) = manifest(file) else { return Value::Null };
    let base = json!({ "manifest": manifest });
    let props = properties(&base);
    let given = &preset["values"];
    let files = &preset["files"];
    let mut fields = Vec::new();
    let mut media = Vec::new();
    for id in ordered(&base) {
        let Some(p) = props.get(&id) else { continue };
        let at = format!("/preset/values/{}", escape(&id));
        match &p.control {
            Control::File { accept, .. } => media.push(json!({
                "id": id, "title": named(&base, &id), "accept": accept, "file": files.get(&id),
            })),
            _ if p.holds_value() && !p.personal() => {
                fields.push(crate::forms::value_field(&base, p, &id, at, given.get(&id), 0));
            }
            _ => {}
        }
    }
    json!({ "id": manifest["id"], "version": manifest["version"], "fields": fields, "files": media })
}
