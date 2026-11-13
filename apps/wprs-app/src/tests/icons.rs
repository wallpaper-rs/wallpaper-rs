use crate::looks;
use std::collections::BTreeSet;
use std::path::Path;

const CONTEXTS: [&str; 30] = [
    "application", "arrow", "audio", "bookmark", "camera", "color", "data", "dialog", "document", "draw",
    "edit", "emblem", "fill", "folder", "font", "format", "go", "help", "image", "insert", "list", "media",
    "object", "package", "preferences", "text", "transform", "video", "view", "zoom",
];
const WEIGHTS: [&str; 3] = ["regular", "fill", "bold"];

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn literals(line: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut parts = line.split('"');
    let mut before = parts.next().unwrap_or("");
    while let (Some(inner), Some(after)) = (parts.next(), parts.next()) {
        let compared = ["===", "!==", "=="].iter().any(|op| before.trim_end().ends_with(op));
        let shaped = inner.starts_with(|c: char| c.is_ascii_lowercase())
            && inner.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
        if shaped && !compared {
            found.push(inner.to_owned());
        }
        before = after;
    }
    found
}

fn asked(text: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut inside = 0;
    let mut iconic = false;
    for line in text.lines() {
        if line.contains("Kirigami.Icon {") {
            inside = 8;
        }
        let trimmed = line.trim_start();
        let sourced = inside > 0 && (trimmed.starts_with("source:") || trimmed.starts_with("fallback:"));
        inside -= 1;
        if sourced || line.contains("icon.name:") || line.contains("iconName:") {
            iconic = true;
            names.extend(literals(line));
        }
    }
    if iconic {
        let named = |name: &String| name.split_once('-').is_some_and(|(head, _)| CONTEXTS.contains(&head));
        names.extend(text.lines().flat_map(literals).filter(named));
    }
    names
}

fn bundled() -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for weight in WEIGHTS {
        for entry in std::fs::read_dir(root().join("look/icons").join(weight)).unwrap() {
            let stem = entry.unwrap().path().file_stem().unwrap().to_str().unwrap().to_owned();
            names.insert(if weight == "regular" { stem } else { format!("{stem}-{weight}") });
        }
    }
    names
}

fn sources(dir: &Path, into: &mut Vec<std::path::PathBuf>) {
    for path in std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()) {
        if path.is_dir() {
            sources(&path, into);
        } else if path.extension().is_some_and(|e| e == "qml") {
            into.push(path);
        }
    }
}

#[test]
fn every_icon_the_app_asks_for_is_in_its_own_set() {
    let set = bundled();
    let old: BTreeSet<&str> = looks::names().map(|(old, _)| old).collect();
    let mut files = Vec::new();
    sources(&root().join("qml"), &mut files);
    sources(&root().join("look/style"), &mut files);
    let missing: Vec<String> = files
        .iter()
        .flat_map(|file| {
            let text = std::fs::read_to_string(file).unwrap();
            let shown = file.file_name().unwrap().to_string_lossy().into_owned();
            asked(&text).into_iter().map(move |name| (shown.clone(), name))
        })
        .filter(|(_, name)| !set.contains(name) && !old.contains(name.as_str()))
        .map(|(file, name)| format!("{file}: {name}"))
        .collect();
    assert!(missing.is_empty(), "icons without a drawing in the own set: {missing:?}");
}

#[test]
fn every_old_name_maps_to_one_bundled_drawing() {
    let set = bundled();
    let mut seen = BTreeSet::new();
    for (old, new) in looks::names() {
        assert!(seen.insert(old), "{old} is mapped twice");
        assert!(set.contains(new), "{old} maps to {new}, which is not bundled");
        assert!(!set.contains(old) || old == new, "{old} is a bundled name and maps to {new}");
    }
}

#[test]
fn the_scan_reads_icon_values_and_not_comparisons() {
    let text = "Kirigami.Icon {\n    source: row.level === \"error\" ? \"data-error\" : \"go-next\"\n}\n";
    let names = asked(text);
    assert!(names.contains("data-error") && names.contains("go-next"));
    assert!(!names.contains("error"));
    let table = "Button {\n    icon.name: modelData[1]\n}\nproperty var tools: [[\"a\", \"edit-cut\"], [\"b\", \"all-outputs\"]]\n";
    let names = asked(table);
    assert!(names.contains("edit-cut") && !names.contains("all-outputs"));
}
