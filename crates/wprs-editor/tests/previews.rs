use serde_json::{Value, json};
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::starters::starter;

fn act(p: &mut Project, action: Value) -> anyhow::Result<()> {
    p.act(&serde_json::from_value::<Action>(action).unwrap()).map(drop)
}

fn project(root: &TempDir) -> Project {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
    let made = starter("empty", "org.example.previews", "Previews", "en").unwrap();
    Project::create(&root.path().join("previews"), made).unwrap()
}

fn picture(path: &std::path::Path, shade: u8) {
    image::RgbaImage::from_pixel(32, 18, image::Rgba([shade, 80, 120, 255])).save(path).unwrap();
}

fn thumbnail(p: &Project) -> String {
    let manifest = &p.doc["manifest"];
    manifest["previews"]["thumbnail"].as_str().or(manifest["preview"].as_str()).unwrap_or_default().to_owned()
}

#[test]
fn choosing_another_preview_takes_the_old_one_out_of_the_package_and_undo_brings_it_back() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    picture(&p.dir.join("preview.png"), 10);
    match p.doc["manifest"]["schema"] == wprs_format::SCHEMA {
        true => p.doc["manifest"]["previews"] = json!({ "thumbnail": "preview.png" }),
        false => p.doc["manifest"]["preview"] = json!("preview.png"),
    }
    let first = thumbnail(&p);
    assert_eq!(first, "preview.png");
    let other = root.path().join("sunset.png");
    picture(&other, 200);
    act(&mut p, json!({ "thumbnail": { "file": other, "animated": false } })).unwrap();
    let second = thumbnail(&p);
    assert_ne!(first, second);
    assert!(p.dir.join(&second).is_file());
    assert!(!p.dir.join(&first).exists(), "the old preview {first} stayed in the package");
    let unused = wprs_editor::package::problems(&p.dir, &p.doc);
    assert!(!unused.iter().any(|c| c.message.contains("preview")), "{unused:?}");
    act(&mut p, json!({ "undo": {} })).unwrap();
    assert_eq!(thumbnail(&p), first);
    assert!(p.dir.join(&first).is_file(), "undo did not bring the old preview back");
}
