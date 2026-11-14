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
    assert!(read("qml/SceneTop.qml").contains("sequence: \"Ctrl+S\""));
}

#[test]
fn every_menu_word_reads_in_spanish() {
    let menu = read("qml/WordMenu.qml");
    let words = literals(&menu, "App.tr(");
    assert_eq!(words.len(), 8, "{words:?}");
    for text in words {
        assert!(!spanish(text).is_empty() && spanish(text) != text, "{text}");
    }
    assert!(!read("qml/Main.qml").contains("globalDrawer"));
}

#[test]
fn the_library_covers_take_the_keyboard() {
    let covers = read("qml/LibraryCovers.qml");
    for needle in ["FocusScope {", "Keys.onDownPressed", "Keys.onEscapePressed", "Qt.Key_A"] {
        assert!(covers.contains(needle), "{needle}");
    }
    assert!(
        read("qml/Estante.qml").contains("focus: true") && read("qml/Rejilla.qml").contains("focus: true")
    );
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

#[test]
fn every_list_and_click_target_of_the_main_pages_takes_the_keyboard() {
    let control = read("qml/Mando.qml");
    assert!(control.contains("activeFocusOnTab: !off") && control.contains("Keys.onReturnPressed"));
    for page in ["Browse", "RepoList", "RepoStatic", "Installed"] {
        let text = read(&format!("qml/{page}.qml"));
        assert!(text.contains("FocusList {") && !text.contains("ListView {"), "{page}");
    }
    for page in ["ListaEditor", "ListaHistorial"] {
        assert!(read(&format!("qml/{page}.qml")).contains("Sublista {"), "{page}");
    }
    let sublist = read("qml/Sublista.qml");
    assert!(sublist.contains("activeFocusOnTab: true") && sublist.contains("Qt.ControlModifier ? sublista.move("));
    let list = read("qml/FocusList.qml");
    for needle in ["activeFocusOnTab: true", "keyNavigationEnabled: true", "Kirigami.Theme.focusColor"] {
        assert!(list.contains(needle), "{needle}");
    }
    let row = read("qml/BrowseRow.qml");
    assert!(
        row.contains("activeFocusOnTab: row.veiled")
            && row.contains("Qt.Key_Return")
            && row.contains("row.reveal()")
    );
    assert!(
        read("qml/AlignPad.qml").contains("Keys.onPressed")
            && read("qml/RgbPicture.qml").contains("Keys.onPressed")
    );
    let walk = read("qml/KeyWalk.qml");
    assert!(walk.contains("keyTarget") && walk.contains("focusList"));
    let probe = read("qml/KeyProbe.qml");
    assert!(probe.contains("alignment pad") && probe.contains("through a list"));
}
