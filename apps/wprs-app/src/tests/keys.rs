use super::{literals, qml_files};
use std::path::Path;

fn read(path: &str) -> String {
    std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap()
}

fn spanish(text: &str) -> String {
    crate::tr::catalog(&["es".to_owned()]).get(text).cloned().unwrap_or_default()
}

#[test]
fn an_action_shortcut_is_written_as_keys_so_its_tooltip_never_shows_a_number() {
    let loose: Vec<String> = qml_files()
        .into_iter()
        .filter(|(_, text)| text.contains("shortcut: StandardKey"))
        .map(|(name, _)| name)
        .collect();
    assert!(loose.is_empty(), "shortcuts given as a standard key number: {loose:?}");
    assert!(read("qml/PackagePage.qml").contains("shortcut: \"Ctrl+S\""));
}

#[test]
fn the_side_menu_closes_with_its_own_translated_button() {
    let menu = read("qml/Navigation.qml");
    assert!(menu.contains("collapseButtonVisible: false"));
    for text in literals(&menu, "App.tr(").into_iter().filter(|t| t.ends_with("sidebar")) {
        assert!(!spanish(text).is_empty() && spanish(text) != text, "{text}");
    }
    assert_eq!(literals(&menu, "App.tr(").iter().filter(|t| t.ends_with("sidebar")).count(), 2);
    assert!(menu.contains("sequence: \"Escape\""));
}
