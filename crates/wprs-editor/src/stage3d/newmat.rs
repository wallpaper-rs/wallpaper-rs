use crate::ops::{Op, escape};
use anyhow::Result;
use serde_json::{Value, json};
use std::path::Path;

pub(crate) fn create(dir: &Path, doc: &Value, name: &str) -> Result<Vec<Op>> {
    crate::wgsl::create(dir, doc, name, "material")
}

pub(crate) fn write(dir: &Path, doc: &Value, name: &str, text: &str) -> Result<Vec<Op>> {
    let wgsl = doc.pointer(&format!("/scene/effects/{}/wgsl", escape(name))).and_then(Value::as_str);
    let wgsl =
        wgsl.ok_or_else(|| said("editor.no_package_effect", "%1 is no effect of this package", &[&name]))?;
    ensure!(
        text.len() <= wprs_scene::MAX_WGSL,
        said("editor.larger_than_kib", "%1 is larger than %2 KiB", &[&wgsl, &(wprs_scene::MAX_WGSL >> 10)])
    );
    ensure!(
        wprs_format::check_path(wgsl).is_ok(),
        said("editor.script_path", "%1 is not a path of the package", &[&wgsl])
    );
    crate::scripts::save(dir, doc, wgsl, text)
}

pub fn check(dir: &Path, doc: &Value, name: &str, draft: Option<&str>) -> Value {
    let entry = doc.pointer(&format!("/scene/effects/{}", escape(name)));
    let read =
        |k: &str| entry.and_then(|e| e[k].as_str()).and_then(|p| std::fs::read_to_string(dir.join(p)).ok());
    let (Some(json), Some(wgsl)) = (read("descriptor"), draft.map(String::from).or_else(|| read("wgsl")))
    else {
        return json!({ "name": name, "ok": false, "error": "the shader files are missing" });
    };
    let stage = serde_json::from_str::<Value>(&json).ok().and_then(|v| v["stage"].as_str().map(String::from));
    let result = wprs_scene::Descriptor::packaged(name, &json, &wgsl);
    let error = result.as_ref().err().cloned();
    let at = error.as_deref().and_then(crate::wgsl::located);
    json!({ "name": name, "ok": error.is_none(), "material": stage.as_deref() == Some("material"),
        "error": error, "line": at.map(|a| a.0), "column": at.map(|a| a.1), "source": wgsl })
}
