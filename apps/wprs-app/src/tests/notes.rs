use super::qml_files;

#[test]
fn a_screen_says_when_its_page_threw_and_why_a_picture_does_not_move() {
    let files = qml_files();
    let text = |name: &str| files.iter().find(|(n, _)| n == name).map(|(_, t)| t.clone()).unwrap();
    assert!(text("OutputRow.qml").contains("OutputNotes {"));
    let notes = text("OutputNotes.qml");
    assert!(notes.contains("notes.info.page_errors"));
    assert!(notes.contains("The page reported an error and may not be drawing: %1"));
    assert!(notes.contains("A script of the scene reported an error: %1"));
    for word in ["software", "building_simple"] {
        assert!(notes.contains(&format!("\"{word}\":")), "{word} has no note");
    }
    let covers = include_str!("../covers.rs");
    assert!(covers.contains(r#"tr("The file is missing")"#) && covers.contains(r#"tr("The file is damaged")"#));
}
