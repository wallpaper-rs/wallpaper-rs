use super::qml_files;

#[test]
fn package_details_wrap_the_inputs_and_the_preview_text_names_its_kind() {
    let files = qml_files();
    let text = |name: &str| files.iter().find(|(n, _)| n == name).map(|(_, t)| t.clone()).unwrap();
    let about = text("PackageAbout.qml");
    let uses = about.split("App.tr(\"Uses:\")").nth(1).unwrap().split("QQC2.Label {").next().unwrap();
    assert!(uses.contains("wrapMode: Text.Wrap") && uses.contains("Layout.maximumWidth"));
    let preview = text("PackagePreview.qml");
    assert!(preview.contains("preview.web ? App.tr(\"Reload builds the page"));
    assert!(preview.contains("The preview appears here while wprsd runs."));
}
