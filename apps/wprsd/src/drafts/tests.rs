#[test]
fn a_preview_on_a_real_screen_reports_its_console_and_a_plain_screen_does_not() {
    let said = crate::compositor::OutputStatus {
        console: vec!["log main.js: hello".into()],
        error: Some("main.js: boom (main.js line 3)".into()),
        ..Default::default()
    };
    let status: std::collections::BTreeMap<_, _> =
        [("DP-1".to_owned(), said.clone()), ("DP-2".to_owned(), said)].into();
    let shown = super::previews(&status, |name| name == "DP-1");
    assert_eq!(shown.keys().collect::<Vec<_>>(), ["DP-1"]);
    assert_eq!(shown["DP-1"].console, ["log main.js: hello"]);
    assert!(super::previews(&status, |_| false).is_empty());
}

#[test]
fn previews_left_in_the_settings_are_forgotten_and_screens_kept() {
    let mut config = wprs_core::Config::default();
    for name in ["preview:page-lights", "preview:1", "DP-1"] {
        config.outputs.insert(name.into(), Default::default());
    }
    assert!(super::forget_previews(&mut config));
    assert_eq!(config.outputs.keys().collect::<Vec<_>>(), ["DP-1"]);
    assert!(!super::forget_previews(&mut config), "nothing left to forget was reported");
}

#[test]
fn previews_stay_in_memory_but_never_reach_the_saved_settings() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    let mut config = wprs_core::Config::default();
    for name in ["preview:page-lights", "DP-1"] {
        config.outputs.insert(name.into(), Default::default());
    }
    super::save_config(&config, &path).unwrap();
    assert_eq!(config.outputs.len(), 2, "the settings in memory lost the preview");
    let saved = wprs_core::Config::load(&path).unwrap();
    assert_eq!(saved.outputs.keys().collect::<Vec<_>>(), ["DP-1"]);
}
