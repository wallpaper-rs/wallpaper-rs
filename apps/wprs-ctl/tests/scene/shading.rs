use super::*;

#[test]
fn package_effects_are_imported_from_the_command_line() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "snow", "--from", "empty", "--title", "Snow"]);
    let descriptor = json!({ "id": "dim", "cost": {}, "passes": [{ "entry": "fs_dim" }] });
    std::fs::write(dir.join("dim.json"), descriptor.to_string()).unwrap();
    std::fs::write(
        dir.join("dim.wgsl"),
        "@fragment fn fs_dim(in: VertexOut) -> @location(0) vec4<f32> { return tap(in.uv) * 0.5; }",
    )
    .unwrap();
    ok(dir, &["scene", "import-effect", "snow", "dim.json"]);
    assert_eq!(show(dir, "/scene/effects/dim/wgsl"), "effects/dim.wgsl");
    std::fs::write(
        dir.join("dim.wgsl"),
        "@fragment fn fs_other(in: VertexOut) -> @location(0) vec4<f32> { return tap(in.uv); }",
    )
    .unwrap();
    assert!(
        fails(dir, &["scene", "import-effect", "snow", "dim.json"]).contains("has no entry point fs_dim")
    );
}

#[test]
fn package_effects_are_removed_unless_a_layer_still_uses_them() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    ok(dir, &["scene", "new", "snow", "--from", "empty", "--title", "Snow"]);
    let descriptor = json!({ "id": "dim", "cost": {}, "passes": [{ "entry": "fs_dim" }] });
    std::fs::write(dir.join("dim.json"), descriptor.to_string()).unwrap();
    std::fs::write(
        dir.join("dim.wgsl"),
        "@fragment fn fs_dim(in: VertexOut) -> @location(0) vec4<f32> { return tap(in.uv) * 0.5; }",
    )
    .unwrap();
    ok(dir, &["scene", "import-effect", "snow", "dim.json"]);
    assert!(fails(dir, &["scene", "remove-effect", "snow", "missing"]).contains("is no effect"));
    ok(dir, &["scene", "add-layer", "snow", "solid", "--id", "fill"]);
    ok(dir, &["scene", "effects", "snow", "fill", "add", "dim"]);
    assert!(fails(dir, &["scene", "remove-effect", "snow", "dim"]).contains("still used by fill"));
    ok(dir, &["scene", "effects", "snow", "fill", "remove", "0"]);
    ok(dir, &["scene", "remove-effect", "snow", "dim"]);
    assert!(fails(dir, &["scene", "show", "snow", "/scene/effects/dim"]).contains("nothing is at"));
    assert!(!dir.join("snow/effects/dim.json").exists(), "the file leaves the folder, kept for undo");
}
