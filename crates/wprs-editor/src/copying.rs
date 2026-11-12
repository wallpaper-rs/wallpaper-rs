use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use wprs_core::message::said;
use wprs_format::{Derivatives, Limits, MANIFEST, derivatives, valid_id};

pub fn new_id(original: &str) -> String {
    let copy = original.rfind(".copy").filter(|i| original[i + 5..].bytes().all(|b| b.is_ascii_digit()));
    match copy {
        Some(i) => format!("{}.copy{}", &original[..i], original[i + 5..].parse::<u32>().unwrap_or(1) + 1),
        None => format!("{original}.copy"),
    }
}

pub fn facts(file: &Path) -> Result<Value> {
    let package = wprs_format::open(file, &Limits::default())?;
    let m = &package.manifest;
    let (verdict, conditions, reason) = match derivatives(&m.license) {
        Derivatives::Allowed(conditions) => ("allowed", conditions, String::new()),
        Derivatives::Refused(reason) => ("refused", Vec::new(), reason),
    };
    Ok(json!({
        "id": m.id, "new_id": new_id(&m.id), "title": m.title_in(&wprs_core::locale::languages()),
        "kind": m.kind.name(), "version": m.version, "license": m.license, "authors": m.authors,
        "verdict": verdict, "conditions": conditions, "reason": reason,
        "credits": m.credits.len(),
    }))
}

pub fn extract(file: &Path, dir: &Path, id: Option<&str>, mine: bool) -> Result<String> {
    let empty = !dir.exists() || fs::read_dir(dir)?.next().is_none();
    ensure!(empty, said("editor.not_empty_folder", "%1 is not an empty folder", &[&(dir.display())]));
    let mut package = wprs_format::open(file, &Limits::default())?;
    let original = package.manifest.clone();
    if let Derivatives::Refused(reason) = derivatives(&original.license) {
        ensure!(
            mine,
            said(
                "editor.no_derivatives",
                "%1: edit a copy only if you made it or its author allows it",
                &[&reason]
            )
        );
    }
    let id = id.map_or_else(|| new_id(&original.id), str::to_owned);
    ensure!(
        valid_id(&id) && id != original.id,
        said("editor.copy_id", "%1 is not a package id different from the original", &[&id])
    );
    wprs_format::extract(&mut package, dir)?;
    crate::package::unsign(dir)?;
    let path = dir.join(MANIFEST);
    let mut manifest: Value = serde_json::from_slice(&fs::read(&path)?)?;
    if let Some(fields) = manifest.as_object_mut().filter(|_| original.v2()) {
        fields.remove("authors");
        fields.insert("origin".into(), serde_json::to_value(crate::package::derived(&original))?);
    }
    manifest["id"] = json!(id);
    fs::write(&path, serde_json::to_string_pretty(&manifest)? + "\n")?;
    Ok(id)
}
