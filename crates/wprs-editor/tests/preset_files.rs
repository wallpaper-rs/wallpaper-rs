use serde_json::json;
use std::collections::BTreeMap;
use wprs_editor::shotmedia::{chosen_files, swap};

#[test]
fn the_pictures_a_preset_chooses_replace_the_default_of_their_setting_when_it_is_drawn() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("assets")).unwrap();
    image::RgbaImage::new(2, 2).save(dir.path().join("assets/sky.png")).unwrap();
    let doc =
        json!({"preset": {"values": {}, "files": {"photo": "assets/sky.png", "gone": "assets/none.png"}}});
    let files = chosen_files(dir.path(), &doc);
    assert_eq!(files.len(), 2);
    let properties: BTreeMap<String, wprs_format::Property> = serde_json::from_value(json!({
        "photo": {"type": "file", "default_asset": "backdrop", "accept": ["image"]},
        "gone": {"type": "file", "default_asset": "other", "accept": ["image"]},
    }))
    .unwrap();
    let mut assets = wprs_render::SceneAssets::default();
    swap(&mut assets, &properties, &files);
    assert_eq!(assets.images.keys().collect::<Vec<_>>(), ["backdrop"]);
}
