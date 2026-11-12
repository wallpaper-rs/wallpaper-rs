use super::tables::{Words, tables};
use super::{literals, qml_files};
use std::collections::BTreeSet;
use wprs_editor::actions::SCREENS;
use wprs_editor::wgsl::PARAM_KINDS;

fn spanish(words: &Words, table: &str, key: &str) -> bool {
    let catalog = crate::tr::catalog(&["es".to_owned()]);
    words.get(table, key).and_then(|id| catalog.get(&id).cloned()).is_some_and(|text| !text.is_empty())
}

#[test]
fn screen_size_names_and_effect_parameter_types_are_translated() {
    let words = Words(tables());
    let screens: BTreeSet<String> = SCREENS.iter().map(|(name, _)| (*name).to_owned()).collect();
    let missing: Vec<&String> = screens.iter().filter(|n| !spanish(&words, "S.names", n)).collect();
    assert!(missing.is_empty(), "screen sizes without Spanish: {missing:?}");
    let missing: Vec<&str> = PARAM_KINDS.into_iter().filter(|k| !spanish(&words, "S.types", k)).collect();
    assert!(missing.is_empty(), "parameter types without Spanish: {missing:?}");
}

#[test]
fn a_word_with_two_meanings_has_a_context_for_each() {
    let words = Words(tables());
    let catalog = crate::tr::catalog(&["es".to_owned()]);
    let mark = wprs_core::locale::CONTEXT_MARK;
    assert_ne!(catalog.get(&format!("rating{mark}Rate")), catalog.get("Rate"));
    assert_ne!(catalog.get(&format!("effect{mark}Spin")), catalog.get("Spin"));
    assert_eq!(words.get("S.effects", "spin"), Some(format!("effect{mark}Spin")));
    let files = qml_files();
    let dialogs = &files.iter().find(|(n, _)| n == "SelectionDialogs.qml").unwrap().1;
    assert_eq!(literals(dialogs, "App.trc(\"rating\", ").len(), 1);
}

fn sources(dir: &std::path::Path, out: &mut Vec<String>) {
    for entry in std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()) {
        if entry.is_dir() {
            sources(&entry, out);
        } else if entry.extension().is_some_and(|e| e == "rs") {
            out.push(std::fs::read_to_string(entry).unwrap());
        }
    }
}

#[test]
fn every_field_key_the_editor_writes_by_hand_has_a_translated_label() {
    let (words, catalog) = (Words(tables()), crate::tr::catalog(&["es".to_owned()]));
    let mut files = Vec::new();
    sources(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/wprs-editor/src"),
        &mut files,
    );
    let keys: BTreeSet<&str> = files.iter().flat_map(|f| literals(f, "key: ")).collect();
    assert!(keys.contains("wake") && keys.contains("prop"), "{keys:?}");
    let missing: Vec<&&str> = keys
        .iter()
        .filter(|k| words.label(k, "").and_then(|id| catalog.get(&id).cloned()).is_none_or(|t| t.is_empty()))
        .collect();
    assert!(missing.is_empty(), "field keys without a translated label: {missing:?}");
}
