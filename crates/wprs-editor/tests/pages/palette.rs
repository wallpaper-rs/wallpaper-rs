use super::page;
use serde_json::json;

#[test]
fn the_cover_colours_of_a_picture_of_a_page_folder_are_read() {
    let root = tempfile::tempdir().unwrap();
    let project = page(root.path());
    image::RgbImage::from_fn(64, 64, |x, _| image::Rgb(if x < 40 { [20, 20, 60] } else { [250, 230, 180] }))
        .save(project.dir.join("cover.png"))
        .unwrap();
    let ask = |what: serde_json::Value| {
        wprs_editor::rigs::query(&project.dir, &project.holding(""), "", &json!({ "palette": what }))
    };
    let colours = ask(json!("cover.png"));
    assert_eq!(colours.as_array().map(Vec::len), Some(5), "{colours}");
    assert_eq!((colours[0].as_str(), colours[3].as_str()), (Some("#14143c"), Some("#fae6b4")));
    assert!(ask(json!("missing.png")).is_null() && ask(json!("../cover.png")).is_null());
    assert!(ask(json!("index.html")).is_null());
}
