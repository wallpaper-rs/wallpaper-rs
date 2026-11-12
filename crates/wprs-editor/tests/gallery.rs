use serde_json::{Value, json};
use std::path::Path;
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::inspect::fields;
use wprs_editor::seats::seats;
use wprs_editor::starters::starter;

const FILM: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/magenta-64x32.webm");

fn act(p: &mut Project, action: Value) -> anyhow::Result<()> {
    let action = serde_json::from_value::<Action>(action).unwrap();
    assert!(wprs_editor::ACTIONS.contains(&action.name().as_str()));
    p.act(&action).map(drop)
}

fn project(root: &TempDir) -> Project {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        unsafe { std::env::set_var("XDG_CACHE_HOME", dir.path().join("cache")) };
        dir
    });
    Project::create(
        &root.path().join("gallery"),
        starter("empty", "org.example.gallery", "Gallery", "en").unwrap(),
    )
    .unwrap()
}

fn picture(dir: &Path, name: &str, size: [u32; 2]) {
    std::fs::create_dir_all(dir.join("assets")).unwrap();
    image::RgbaImage::new(size[0], size[1]).save(dir.join("assets").join(name)).unwrap();
}

fn with_film(root: &TempDir) -> Project {
    let mut p = project(root);
    std::fs::create_dir_all(p.dir.join("assets")).unwrap();
    std::fs::copy(FILM, p.dir.join("assets/clip.webm")).unwrap();
    let layers = json!([{ "id": "film", "type": "video", "video": "clip", "playback": { "start": 0.0 } }]);
    act(&mut p, json!({"set": {"pointer": "/scene/assets/clip", "value": {"path": "assets/clip.webm"}}}))
        .unwrap();
    act(&mut p, json!({"set": {"pointer": "/scene/layers", "value": layers}})).unwrap();
    p
}

fn query(p: &Project, q: Value) -> Value {
    wprs_editor::rigs::query(&p.dir, &p.doc, "", &q)
}

#[test]
fn a_video_layer_has_the_real_size_of_its_video_and_can_take_it() {
    let root = tempfile::tempdir().unwrap();
    let mut p = with_film(&root);
    let seat = seats(&p.dir, &p.doc, 0.0).into_iter().find(|s| s.id == "film").unwrap();
    if seat.size == [640.0, 360.0] {
        return assert!(std::env::var_os("WPRS_REQUIRE_VIDEO").is_none(), "the video was not read");
    }
    assert_eq!(seat.size, [64.0, 32.0]);
    act(&mut p, json!({"film": {"fit": {"layer": "film"}}})).unwrap();
    assert_eq!(p.doc["scene"]["layers"][0]["size"], json!([64.0, 32.0]));
    assert!(act(&mut p, json!({"film": {"fit": {"layer": "nope"}}})).is_err());
}

#[test]
fn the_rate_of_a_video_is_offered_up_to_four_and_refused_above() {
    let root = tempfile::tempdir().unwrap();
    let mut p = with_film(&root);
    let range = |p: &Project| {
        fields(&p.doc, "/scene/layers/0")
            .into_iter()
            .find(|f| f.pointer.ends_with("/playback/rate"))
            .unwrap()
            .range
    };
    assert_eq!(range(&p), json!([0.0, 4.0]));
    act(&mut p, json!({"film": {"rate": {"layer": "film", "rate": 3.5}}})).unwrap();
    assert_eq!(p.doc["scene"]["layers"][0]["playback"]["rate"], json!(3.5));
    let refused = act(&mut p, json!({"film": {"rate": {"layer": "film", "rate": 8.0}}}));
    assert!(refused.unwrap_err().to_string().contains("0 to 4"));
}

#[test]
fn trimming_keeps_the_part_inside_the_video_and_reads_its_real_length() {
    let root = tempfile::tempdir().unwrap();
    let mut p = with_film(&root);
    let found = query(&p, json!({"film": {"layer": "film", "strip": 3}}));
    assert_eq!(found["asset"], "clip");
    let length = found["length"].as_f64().unwrap_or(0.0);
    assert!(length > 0.0, "{found}");
    act(&mut p, json!({"film": {"trim": {"layer": "film", "start": 0.0, "end": length / 2.0}}})).unwrap();
    assert_eq!(p.doc["scene"]["layers"][0]["playback"]["end"], json!(length / 2.0));
    act(&mut p, json!({"film": {"trim": {"layer": "film", "start": 0.0}}})).unwrap();
    assert!(p.doc["scene"]["layers"][0]["playback"].get("end").is_none());
    let past = act(&mut p, json!({"film": {"trim": {"layer": "film", "start": 0.0, "end": length + 5.0}}}));
    assert!(past.is_err());
    let backwards = act(&mut p, json!({"film": {"trim": {"layer": "film", "start": 0.2, "end": 0.1}}}));
    assert!(backwards.is_err());
}

#[test]
fn a_video_is_offered_wherever_a_texture_is_picked() {
    let root = tempfile::tempdir().unwrap();
    let mut p = with_film(&root);
    picture(&p.dir, "mask.png", [8, 8]);
    act(&mut p, json!({"set": {"pointer": "/scene/assets/mask", "value": {"path": "assets/mask.png"}}}))
        .unwrap();
    act(&mut p, json!({"set": {"pointer": "/scene/layers/0/effects", "value": [{"effect": "blur", "mask": {"texture": "mask"}}]}}))
        .unwrap();
    let all = fields(&p.doc, "/scene/layers/0/effects/0/mask");
    let texture = all.iter().find(|f| f.key == "texture").unwrap();
    assert!(
        texture.options.contains(&json!("mask")) && texture.options.contains(&json!("clip")),
        "{:?}",
        texture.options
    );
}

#[test]
fn a_sprite_grid_cuts_in_the_chosen_order_and_free_frames_are_edited_one_by_one() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    picture(&p.dir, "walk.png", [40, 20]);
    act(&mut p, json!({"set": {"pointer": "/scene/assets/walk", "value": {"path": "assets/walk.png", "atlas": [2, 4]}}}))
        .unwrap();
    let grid = json!({"sprites": {"grid": {"asset": "walk", "rows": 2, "columns": 4, "count": 6}}});
    act(&mut p, grid).unwrap();
    let asset = &p.doc["scene"]["assets"]["walk"];
    assert!(asset.get("atlas").is_none());
    assert_eq!(asset["rects"].as_array().unwrap().len(), 6);
    assert_eq!(asset["rects"][1], json!([10, 0, 10, 10]));
    assert_eq!(asset["rects"][4], json!([0, 10, 10, 10]));
    let down = json!({"sprites": {"grid": {"asset": "walk", "rows": 2, "columns": 4, "order": "columns"}}});
    act(&mut p, down).unwrap();
    assert_eq!(p.doc["scene"]["assets"]["walk"]["rects"][1], json!([0, 10, 10, 10]));
    act(&mut p, json!({"sprites": {"change": {"asset": "walk", "index": 0, "rect": [1, 2, 3, 4]}}})).unwrap();
    act(&mut p, json!({"sprites": {"add": {"asset": "walk", "rect": [30, 10, 10, 10]}}})).unwrap();
    act(&mut p, json!({"sprites": {"reorder": {"asset": "walk", "from": 8, "to": 0}}})).unwrap();
    act(&mut p, json!({"sprites": {"remove": {"asset": "walk", "index": 1}}})).unwrap();
    let rects = &p.doc["scene"]["assets"]["walk"]["rects"];
    assert_eq!(
        (rects[0].clone(), rects[1].clone(), rects.as_array().unwrap().len()),
        (json!([30, 10, 10, 10]), json!([0, 10, 10, 10]), 8)
    );
    act(&mut p, json!({"sprites": {"play": {"asset": "walk", "fps": 12.0, "frames": 5}}})).unwrap();
    let sheet = query(&p, json!({"sprites": {"asset": "walk"}}));
    assert_eq!(
        (sheet["size"].clone(), sheet["fps"].clone(), sheet["frames"].clone()),
        (json!([40, 20]), json!(12.0), json!(5))
    );
    for bad in [
        json!({"sprites": {"change": {"asset": "walk", "index": 0, "rect": [35, 0, 10, 10]}}}),
        json!({"sprites": {"add": {"asset": "walk", "rect": [0, 0, 0, 5]}}}),
        json!({"sprites": {"remove": {"asset": "walk", "index": 99}}}),
        json!({"sprites": {"play": {"asset": "walk", "fps": 500.0}}}),
        json!({"sprites": {"play": {"asset": "walk", "frames": 99}}}),
        json!({"sprites": {"grid": {"asset": "walk", "rows": 30, "columns": 30}}}),
        json!({"sprites": {"clear": {"asset": "missing"}}}),
    ] {
        assert!(act(&mut p, bad.clone()).is_err(), "{bad}");
    }
    act(&mut p, json!({"sprites": {"clear": {"asset": "walk"}}})).unwrap();
    assert!(p.doc["scene"]["assets"]["walk"].get("rects").is_none());
}

#[test]
fn an_asset_lists_every_place_that_uses_it() {
    let root = tempfile::tempdir().unwrap();
    let mut p = with_film(&root);
    picture(&p.dir, "mask.png", [8, 8]);
    act(&mut p, json!({"set": {"pointer": "/scene/assets/mask", "value": {"path": "assets/mask.png"}}}))
        .unwrap();
    let effects = json!([{"effect": "blur", "mask": {"texture": "mask"}}]);
    act(&mut p, json!({"set": {"pointer": "/scene/layers/0/effects", "value": effects}})).unwrap();
    let found = query(&p, json!({"asset_uses": {"asset": "mask"}}));
    let rows = found["uses"].as_array().unwrap();
    assert_eq!(rows.len(), 1, "{found}");
    assert_eq!(rows[0]["pointer"], "/scene/layers/0/effects/0/mask/texture");
    assert_eq!(rows[0]["layer"], json!({"id": "film", "type": "video"}));
    assert_eq!(
        query(&p, json!({"asset_uses": {"asset": "clip"}}))["uses"][0]["pointer"],
        "/scene/layers/0/video"
    );
    assert!(query(&p, json!({"asset_uses": {"asset": "nobody"}}))["uses"].as_array().unwrap().is_empty());
}
