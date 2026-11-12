use serde_json::Value;
use std::path::{Path, PathBuf};

fn scenes() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenes");
    let mut dirs: Vec<_> = std::fs::read_dir(root)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.join("scene.json").is_file())
        .collect();
    dirs.sort();
    dirs
}

#[test]
fn one_example_scene_ships_its_own_small_3d_model() {
    let found: Vec<_> = scenes()
        .into_iter()
        .filter(|dir| {
            let scene: Value = serde_json::from_slice(&std::fs::read(dir.join("scene.json")).unwrap()).unwrap();
            let layers = scene["layers"].to_string();
            let models: Vec<_> = scene["assets"]
                .as_object()
                .into_iter()
                .flatten()
                .filter(|(_, a)| a["path"].as_str().is_some_and(|p| p.ends_with(".glb")))
                .collect();
            let (_, asset) = match models.first() {
                Some(first) => *first,
                None => return false,
            };
            let file = dir.join(asset["path"].as_str().unwrap());
            layers.contains("\"type\":\"model\"") && std::fs::metadata(file).is_ok_and(|m| m.len() < 64 << 10)
        })
        .collect();
    assert!(!found.is_empty(), "no example scene brings a small 3D model of its own");
}
