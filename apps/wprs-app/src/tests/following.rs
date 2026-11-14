use super::qml_files;

#[test]
fn the_battery_pause_row_follows_the_service_after_a_change_in_the_app() {
    let (_, text) = qml_files().into_iter().find(|(n, _)| n == "SettingsPause.qml").unwrap();
    let held = text.split("Binding {").nth(1).expect("the row's value is only a plain binding");
    assert!(held.contains("target: onBattery") && held.contains(r#"property: "current""#), "{held}");
    assert!(held.contains("part.values.indexOf(part.chosen)"), "the row follows something else");
}
