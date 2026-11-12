use serde_json::json;

#[test]
fn floats_survive_the_text_round_trip() {
    let mut x: u64 = 0x9E3779B97F4A7C15;
    let mut bad = 0;
    for _ in 0..200000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let f = (x >> 11) as f64 / (1u64 << 53) as f64 * 3000.0 - 1500.0;
        let text = serde_json::to_string_pretty(&json!({"a": f})).unwrap();
        let back: serde_json::Value = serde_json::from_str(&text).unwrap();
        if back["a"].as_f64() != Some(f) {
            bad += 1;
        }
    }
    assert_eq!(bad, 0);
}

mod project {
    fn state() {
        static STATE: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();
        STATE.get_or_init(|| {
            let dir = tempfile::tempdir().unwrap();
            unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
            dir
        });
    }

    use serde_json::json;
    use wprs_editor::Project;
    use wprs_editor::actions::Action;
    use wprs_editor::starters::starter;

    fn set(project: &mut Project, value: serde_json::Value) {
        let action = json!({"set": {"pointer": "/scene/canvas/background", "value": value}});
        project.act(&serde_json::from_value::<Action>(action).unwrap()).unwrap();
    }

    fn open(root: &std::path::Path) -> Project {
        state();
        let doc = starter("parallax", "org.example.dawn", "Dawn", "en").unwrap();
        Project::create(&root.join("dawn"), doc).unwrap()
    }

    #[test]
    fn saving_twice_after_fractional_edits_never_reports_an_outside_change() {
        let root = tempfile::tempdir().unwrap();
        let mut project = open(root.path());
        for i in 0..200 {
            let value = json!(i as f64 * 0.1234567891234 + 1.0 / 3.0);
            project
                .act(
                    &serde_json::from_value::<Action>(
                        json!({"set": {"pointer": "/scene/canvas/probe", "value": value}}),
                    )
                    .unwrap(),
                )
                .unwrap();
            project.save().unwrap();
            assert!(!project.diverged());
        }
    }

    fn touch_outside(root: &std::path::Path) {
        let file = root.join("dawn/scene.json");
        let mut doc: serde_json::Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
        doc["canvas"]["theirs"] = json!(1);
        std::fs::write(&file, doc.to_string()).unwrap();
    }

    #[test]
    fn a_real_outside_change_can_be_kept_taken_or_saved_elsewhere() {
        let root = tempfile::tempdir().unwrap();
        let mut project = open(root.path());
        set(&mut project, json!("#102030"));
        touch_outside(root.path());
        assert!(project.diverged() && project.save().is_err());
        project.save_elsewhere(&root.path().join("other")).unwrap();
        assert!(std::fs::read_to_string(root.path().join("other/scene.json")).unwrap().contains("#102030"));
        assert_eq!(project.dir, root.path().join("other"));

        let mut mine = open(&root.path().join("m"));
        set(&mut mine, json!("#111111"));
        touch_outside(&root.path().join("m"));
        mine.keep_mine().unwrap();
        let kept = std::fs::read_to_string(root.path().join("m/dawn/scene.json")).unwrap();
        assert!(kept.contains("#111111") && !kept.contains("theirs"));

        let mut theirs = open(&root.path().join("t"));
        set(&mut theirs, json!("#222222"));
        touch_outside(&root.path().join("t"));
        theirs.take_theirs().unwrap();
        assert!(theirs.doc["scene"]["canvas"]["theirs"] == 1 && !theirs.dirty() && !theirs.diverged());
    }
}
