use crate::infer::{TELEMETRY, text};
use anyhow::{Result, anyhow};
use ort::environment::Environment;
use std::path::PathBuf;
use std::sync::OnceLock;
use wprs_core::message::said;

pub(crate) fn environment(libraries: &[PathBuf]) -> Result<Environment> {
    static SHARED: OnceLock<Environment> = OnceLock::new();
    if let Some(environment) = SHARED.get() {
        return Ok(environment.clone());
    }
    let mut failure = anyhow!(
        "{}",
        said(
            "depth.onnx_runtime_is_not_installed",
            "ONNX Runtime is not installed; see wprs-ctl models list",
            &[]
        )
        .local()
    );
    unsafe { std::env::set_var(TELEMETRY, "1") };
    let loaded = libraries.iter().find_map(|lib| {
        wprs_core::loader::serialised(|| ort::init_from(lib))
            .inspect_err(|e| {
                failure = anyhow!(
                    "{}",
                    said("depth.cannot_load_onnx_runtime", "cannot load ONNX Runtime: %1", &[&e]).local()
                )
            })
            .ok()
    });
    let built =
        loaded.ok_or(failure)?.with_name("wallpaper-rs").with_telemetry(false).build().map_err(text)?;
    Ok(SHARED.get_or_init(|| built).clone())
}
