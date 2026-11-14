use super::icons::{bundled, sources};
use std::collections::BTreeSet;
use std::path::Path;

const NOT_DRAWN: [&str; 5] = ["Action", "ActionGroup", "ButtonGroup", "Overlay", "SplitHandle"];

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn controls(text: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for piece in text.split("QQC2.").skip(1) {
        let name: String = piece.chars().take_while(|c| c.is_ascii_alphanumeric()).collect();
        if name.starts_with(|c: char| c.is_ascii_uppercase()) {
            found.insert(name);
        }
    }
    found
}

fn listed() -> BTreeSet<String> {
    let qmldir = std::fs::read_to_string(root().join("look/style/qmldir")).unwrap();
    qmldir
        .lines()
        .filter_map(|line| {
            let mut words = line.split_whitespace();
            let name = words.next()?;
            let file = words.nth(1)?;
            (name != "internal" && file.ends_with(".qml")).then(|| name.to_owned())
        })
        .collect()
}

fn used() -> BTreeSet<String> {
    let mut files = Vec::new();
    sources(&root().join("qml"), &mut files);
    files.iter().flat_map(|path| controls(&std::fs::read_to_string(path).unwrap())).collect()
}

#[test]
fn every_control_the_app_uses_is_drawn_by_its_own_style() {
    let listed = listed();
    let missing: Vec<String> = used()
        .into_iter()
        .filter(|name| !NOT_DRAWN.contains(&name.as_str()) && !listed.contains(name))
        .collect();
    assert!(missing.is_empty(), "controls that fall back to the built-in look: {missing:?}");
}

#[test]
fn every_style_entry_has_its_file_and_every_file_its_entry() {
    let qmldir = std::fs::read_to_string(root().join("look/style/qmldir")).unwrap();
    let named: BTreeSet<String> = qmldir
        .lines()
        .filter_map(|line| line.split_whitespace().nth(2))
        .filter(|f| f.ends_with(".qml"))
        .map(str::to_owned)
        .collect();
    let mut files = Vec::new();
    sources(&root().join("look/style"), &mut files);
    let present: BTreeSet<String> =
        files.iter().map(|path| path.file_name().unwrap().to_str().unwrap().to_owned()).collect();
    assert_eq!(named, present);
}

#[test]
fn every_icon_the_style_draws_is_bundled() {
    let set = bundled();
    let mut files = Vec::new();
    sources(&root().join("look/style"), &mut files);
    for path in files {
        let text = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        let drawn = lines.windows(2).filter(|pair| !pair[0].ends_with("State {")).map(|pair| pair[1]);
        for line in drawn.filter(|line| line.trim_start().starts_with("name:")) {
            for name in line.split('"').skip(1).step_by(2) {
                if name.chars().all(|c| c.is_ascii_lowercase() || c == '-') && !name.is_empty() {
                    assert!(set.contains(name), "{} draws {name}, which is not bundled", path.display());
                }
            }
        }
    }
}

#[test]
fn the_scan_reads_types_and_attached_uses_alike() {
    let found = controls("QQC2.ToolTip.text: x\nQQC2.Slider {\nQQC2.Overlay.overlay\nQQC2.dummy");
    assert_eq!(found, ["Overlay", "Slider", "ToolTip"].map(String::from).into());
}

#[test]
fn the_scripts_of_the_app_call_only_what_its_script_engine_has() {
    let missing = ["Object.fromEntries"];
    let mut files = Vec::new();
    sources(&root().join("qml"), &mut files);
    for path in files {
        let text = std::fs::read_to_string(&path).unwrap();
        if let Some(call) = missing.iter().find(|call| text.contains(*call)) {
            panic!("{} calls {call}, which the script engine of Qt does not have", path.display());
        }
    }
}
