use super::qml_files;

#[test]
fn the_rgb_choice_while_covered_applies_only_while_the_covered_wallpaper_pauses() {
    let (_, text) = qml_files().into_iter().find(|(n, _)| n == "SettingsRgb.qml").unwrap();
    assert!(
        text.contains(r#"options["pause.when_covered"] !== false"#),
        "the choice ignored pause.when_covered"
    );
    assert_eq!(text.matches("off: !lights.pauses").count(), 1, "a choice stayed open with nothing to choose");
    assert!(text.contains(r#"App.tr("While a window covers the wallpaper and it pauses")"#));
    let hint = text
        .lines()
        .map(str::trim)
        .find(|l| l.starts_with("explain:") && l.contains(r#"lights.covered === "follow" ?"#))
        .unwrap();
    assert!(
        hint.starts_with(
            r#"explain: !lights.pauses ? App.tr("Pausing while a window covers the screen is off"#
        )
    );
    assert!(
        hint.contains(r#"App.tr("While the covered wallpaper pauses, the lights keep changing"#),
        "{hint}"
    );
}

#[test]
fn the_rgb_settings_say_why_covered_lights_wait() {
    let (_, text) = qml_files().into_iter().find(|(n, _)| n == "SettingsRgb.qml").unwrap();
    assert!(text.contains("model: Object.keys(lights.status.waiting ?? {})"), "the reasons are not listed");
    for why in ["locked", "battery", "colorless", "ungranted"] {
        assert!(
            text.contains(&format!(r#"why === "{why}" ? App.tr("%1: the lights wait"#)),
            "{why} is not told"
        );
    }
}

#[test]
fn the_library_warns_before_applying_a_web_page_this_build_cannot_show() {
    let files = qml_files();
    let text = |name: &str| files.iter().find(|(n, _)| n == name).map(|(_, t)| t.clone()).unwrap();
    assert!(text("WebPrivacy.qml").contains("visible: !!privacy.pkg?.unshown"));
    assert!(
        text("LibraryMore.qml").contains("enabled: !!more.pkg && !more.pkg.unshown"),
        "a page that cannot run was tried"
    );
    let rows = include_str!("../rows.rs");
    assert!(
        rows.contains("p.unshown.as_deref()"),
        "library tiles do not mark web pages that cannot be shown"
    );
}

#[test]
fn the_lights_board_places_devices_and_zones_by_their_own_keys() {
    let files = qml_files();
    let text = |name: &str| files.iter().find(|(n, _)| n == name).map(|(_, t)| t.clone()).unwrap();
    let (board, picture) = (text("RgbBoard.qml"), text("RgbPicture.qml"));
    assert!(board.contains(r#""rgb.place." + target.key"#), "the board wrote another setting");
    assert!(board.contains(r##"(d.key ?? d.name) + "#" + i"##), "zones were not placed by their index");
    assert!(board.contains(r#"board.save("off")"#) && board.contains(r#"board.save("none")"#));
    for flow in ["right", "left", "down", "up"] {
        assert!(board.contains(&format!(r#""value": "{flow}""#)), "{flow} cannot be chosen");
    }
    assert!(picture.contains("drag.target: handle") && picture.contains("onPositionChanged"));
    assert!(text("SettingsRgb.qml").contains("RgbBoard {"));
}

#[test]
fn a_page_preview_shows_its_lights_and_gives_its_stream_back() {
    let files = qml_files();
    let text = |name: &str| files.iter().find(|(n, _)| n == name).map(|(_, t)| t.clone()).unwrap();
    let lights = text("PageLights.qml");
    assert!(
        lights.contains(r#"output = "preview:page-lights";"#),
        "the preview is not one fixed preview output"
    );
    assert!(!lights.contains("Date.now()"), "each try names a new preview output");
    let strip = text("LightsStrip.qml");
    assert!(strip.contains(r#"App.query("GetStrip", [asked], "Rgb1""#));
    assert!(
        lights.contains(r#"App.act("ReleaseStream", [output], "Daemon1")"#)
            && lights.contains("onClosed: stop()")
    );
    assert!(lights.contains("active: trial.opened") && strip.contains("running: lights.active"));
    assert!(text("PackagePreview.qml").contains("LightsStrip {"), "the package preview shows no lights");
    assert!(text("LibraryMore.qml").contains("lights.start(more.host.uid)"));
}

#[test]
fn screens_that_show_a_page_choose_their_own_quality() {
    let files = qml_files();
    let controls = files.iter().find(|(n, _)| n == "OutputControls.qml").map(|(_, t)| t.clone()).unwrap();
    assert!(controls.contains(r#"visible: (host.scene || host.info?.kind === "web") && host.name !== """#));
}

#[test]
fn screens_that_show_a_web_page_can_open_its_settings() {
    let files = qml_files();
    let row = files.iter().find(|(n, _)| n == "OutputRow.qml").map(|(_, t)| t.clone()).unwrap();
    assert!(row.contains(
        r#"readonly property bool tunable: scene || info.kind === "web" || (info.kind === "video" && pkg.kind === "video")"#
    ));
    let controls = files.iter().find(|(n, _)| n == "OutputControls.qml").map(|(_, t)| t.clone()).unwrap();
    assert!(controls.contains(r#"host.info?.kind === "web" ? host.daemon.tr("Web settings")"#));
}

#[test]
fn identical_cards_are_offered_by_their_pci_bus() {
    let files = qml_files();
    let picker = files.iter().find(|(n, _)| n == "GpuChoices.qml").map(|(_, t)| t.clone()).unwrap();
    assert!(picker.contains(r#"const value = twins ? "pci:" + card.bus : card.name;"#));
    assert!(picker.contains(r#"App.tr("%1 · on PCI bus %2", label(card.name, same), card.bus)"#));
}

#[test]
fn a_choice_that_waits_for_a_restart_offers_one() {
    let files = qml_files();
    for name in ["SettingsAdvanced.qml", "SettingsTesting.qml"] {
        let text = files.iter().find(|(n, _)| n == name).map(|(_, t)| t.clone()).unwrap();
        assert!(text.contains(r#"onOpened: App.act("Restart", [], "Daemon1")"#), "{name} offers no restart");
    }
}

#[test]
fn a_dragged_knob_commits_where_the_drag_ended_not_the_translation_after_release() {
    let files = qml_files();
    let knob = files.iter().find(|(n, _)| n == "SceneKnob.qml").map(|(_, t)| t.clone()).unwrap();
    assert!(knob.contains("knob.moved(knob.reached, true)"));
    assert_eq!(knob.matches("translation.").count(), 2, "only the change of the translation reads it");
}

#[test]
fn a_3d_model_is_added_from_the_generic_layer_from_a_file_menu() {
    let files = qml_files();
    let file = |wanted: &str| files.iter().find(|(n, _)| n == wanted).map(|(_, t)| t.clone()).unwrap();
    let menu = file("SceneLayerFiles.qml");
    assert!(menu.contains(r#"["model", App.tr("3D model from a .glb file…"), "draw-cuboid"]"#));
    assert!(menu.contains("SceneModelImport {"));
    let tools = file("SceneLayerTools.qml");
    assert!(!tools.contains("3D model from a file"), "the menu kept its own, non-generic entry");
    assert!(!tools.contains("SceneModelImport {"), "the menu kept its own import dialog");
}

#[test]
fn package_effects_list_offers_reimport_and_remove() {
    let files = qml_files();
    let file = |wanted: &str| files.iter().find(|(n, _)| n == wanted).map(|(_, t)| t.clone()).unwrap();
    let list = file("SceneEffectsList.qml");
    assert!(list.contains("model: panel.page.view.package_effects"));
    assert!(list.contains(r#"onClicked: panel.page.run("remove_effect""#));
    assert!(list.contains(r#"panel.page.run("import_effect""#));
    let assets = file("SceneAssetParts.qml");
    assert!(assets.contains("SceneEffectsList {"));
    assert!(file("SceneAssets.qml").contains("SceneAssetParts {"));
}

#[test]
fn the_3d_stage_reads_its_models_and_draws_the_bones_over_their_boxes() {
    let files = qml_files();
    let file = |wanted: &str| files.iter().find(|(n, _)| n == wanted).map(|(_, t)| t.clone()).unwrap();
    assert!(file("SceneStageBones.qml").contains("stage.selected === model.pointer"));
    let stage = file("SceneStage3d.qml");
    assert!(stage.find("delegate: SceneModelGizmo") < stage.find("delegate: SceneStageBones"));
}

#[test]
fn a_tap_on_a_canvas_handle_does_not_also_select_the_canvas() {
    let files = qml_files();
    let names = [
        "SceneGizmo",
        "SceneModelGizmo",
        "SceneStageBones",
        "ScenePoints",
        "SceneIkKnobs",
        "SceneBones",
        "SceneKnob",
    ];
    for name in names {
        let text = files.iter().find(|(n, _)| *n == format!("{name}.qml")).map(|(_, t)| t.clone()).unwrap();
        let taps = text.matches("TapHandler {").count();
        assert_eq!(taps, text.matches("gesturePolicy: TapHandler.ReleaseWithinBounds").count(), "{name}");
    }
}

#[test]
fn a_camera_path_row_reads_its_path_by_position() {
    let files = qml_files();
    let row = files.iter().find(|(n, _)| n == "ScenePathRow.qml").map(|(_, t)| t.clone()).unwrap();
    assert!(row.contains("required property int index") && !row.contains("required property var modelData"));
}

#[test]
fn the_camera_panel_is_hidden_when_the_scene_has_no_camera() {
    let files = qml_files();
    let panel = files.iter().find(|(n, _)| n == "SceneCamera3d.qml").map(|(_, t)| t.clone()).unwrap();
    assert!(panel.contains(r#"page.editor.value("/scene/camera3d") !== "null""#));
}
