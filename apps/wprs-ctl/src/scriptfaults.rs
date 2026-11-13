use crate::told::{fill, told};
use std::cell::RefCell;
use std::path::Path;
use wprs_editor::scriptcheck::{ScriptFault, faults};
use wprs_format::{Kind, Limits, Manifest, open};
use wprs_scene::{Scene, Way};

pub fn of(path: &Path, manifest: &Manifest) -> Vec<ScriptFault> {
    within(path, manifest, |scene, read| {
        let read = RefCell::new(read);
        faults(scene, |name| String::from_utf8((read.borrow_mut())(name)?).ok())
    })
}

pub fn passages(path: &Path, manifest: &Manifest) -> Vec<String> {
    within(path, manifest, |scene, read| {
        let mut scene = scene.clone();
        let mut files = |name: &str, max: usize| match read(name) {
            Some(bytes) if bytes.len() <= max => Ok(bytes),
            Some(_) => Err("it is too large"),
            None => Err("it is not in the package"),
        };
        scene.read_passages(&mut files);
        let said = |(way, why): (Way, &str)| match way {
            Way::In => fill(
                told("the transition for coming in does not play, the plain fade plays instead: %1"),
                &[&why],
            ),
            Way::Out => fill(
                told("the transition for leaving does not play, the plain fade plays instead: %1"),
                &[&why],
            ),
        };
        scene.passages.problems().into_iter().map(said).collect()
    })
}

fn within<T: Default>(
    path: &Path,
    manifest: &Manifest,
    work: impl FnOnce(&Scene, &mut dyn FnMut(&str) -> Option<Vec<u8>>) -> T,
) -> T {
    if manifest.kind != Kind::Scene {
        return T::default();
    }
    if path.is_dir() {
        let Some(scene) = std::fs::read(path.join(&manifest.entry)).ok().and_then(parsed) else {
            return T::default();
        };
        return work(&scene, &mut |name| std::fs::read(path.join(name)).ok());
    }
    let Ok(package) = open(path, &Limits::default()) else { return T::default() };
    let package = RefCell::new(package);
    let Some(scene) = package.borrow_mut().read_file(&manifest.entry).ok().and_then(parsed) else {
        return T::default();
    };
    work(&scene, &mut |name| package.borrow_mut().read_file(name).ok())
}

fn parsed(bytes: Vec<u8>) -> Option<Scene> {
    serde_json::from_slice(&bytes).ok()
}
