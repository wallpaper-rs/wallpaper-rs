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

#[test]
fn the_library_grid_takes_the_keyboard_and_its_tiles_do_not() {
    let (library, tile) = (read("qml/LibraryPage.qml"), read("qml/Tile.qml"));
    for needle in [
        "activeFocusOnTab: true",
        "keyNavigationEnabled: true",
        "Keys.onSpacePressed",
        "Keys.onReturnPressed",
    ] {
        assert!(library.contains(needle), "{needle}");
    }
    assert!(tile.contains("focusPolicy: Qt.NoFocus") && tile.contains("Kirigami.Theme.focusColor"));
}

#[test]
fn the_keyboard_and_wheel_rules_are_installed_for_every_window() {
    let keys = read("keys.h");
    assert!(keys.contains("wheelEnabled") && keys.contains("QQuickAbstractButton"));
    for kind in ["QQuickComboBox", "QQuickSlider", "QQuickSpinBox", "QQuickDial"] {
        assert!(keys.contains(kind), "{kind}");
    }
    assert!(read("src/backend.rs").contains("crate::keyboard::install_keys()"));
}

#[test]
fn the_self_check_walks_every_page_with_the_keyboard() {
    let tour = read("qml/Tour.qml");
    assert!(tour.contains("walking.check(host.names[at - 1])") && tour.contains("probing.begin()"));
    let walk = read("qml/KeyWalk.qml");
    assert!(walk.contains("Qt.Key_Tab") && walk.contains("does not reach"));
    let probe = read("qml/KeyProbe.qml");
    for needle in ["Qt.Key_Return", "Qt.Key_Escape", "Qt.MouseFocusReason", "App.backend.wheel"] {
        assert!(probe.contains(needle), "{needle}");
    }
}
