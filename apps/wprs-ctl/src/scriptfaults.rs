use std::cell::RefCell;
use std::path::Path;
use wprs_editor::scriptcheck::{ScriptFault, faults};
use wprs_format::{Kind, Limits, Manifest, open};
use wprs_scene::Scene;

pub fn of(path: &Path, manifest: &Manifest) -> Vec<ScriptFault> {
    if manifest.kind != Kind::Scene {
        return Vec::new();
    }
    if path.is_dir() {
        let Some(scene) = std::fs::read(path.join(&manifest.entry)).ok().and_then(parsed) else {
            return Vec::new();
        };
        return faults(&scene, |name| std::fs::read_to_string(path.join(name)).ok());
    }
    let Ok(package) = open(path, &Limits::default()) else { return Vec::new() };
    let package = RefCell::new(package);
    let Some(scene) = package.borrow_mut().read_file(&manifest.entry).ok().and_then(parsed) else {
        return Vec::new();
    };
    faults(&scene, |name| {
        let bytes = package.borrow_mut().read_file(name).ok()?;
        String::from_utf8(bytes).ok()
    })
}

fn parsed(bytes: Vec<u8>) -> Option<Scene> {
    serde_json::from_slice(&bytes).ok()
}
