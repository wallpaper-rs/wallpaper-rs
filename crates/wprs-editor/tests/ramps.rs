use serde_json::{Value, json};
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::actions::Action;
use wprs_editor::inspect::fields_with;
use wprs_editor::starters::starter;
use wprs_editor::{Project, manual};

fn state() {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe {
            std::env::set_var("XDG_STATE_HOME", dir.path());
            std::env::set_var("XDG_DATA_HOME", dir.path().join("data"));
            std::env::set_var("XDG_CACHE_HOME", dir.path().join("cache"));
            std::env::remove_var("ORT_DYLIB_PATH");
        }
        dir
    });
}

fn act(project: &mut Project, action: Value) -> anyhow::Result<()> {
    project.act(&serde_json::from_value::<Action>(action).unwrap()).map(drop)
}

fn scene(effect: &str) -> (TempDir, Project) {
    let root = tempfile::tempdir().unwrap();
    state();
    let dir = root.path().join("glow");
    let mut project =
        Project::create(&dir, starter("empty", "org.example.glow", "Glow", "en").unwrap()).unwrap();
    let layer =
        json!([{ "id": "fill", "type": "solid", "color": "#ffffff", "effects": [{ "effect": effect }] }]);
    act(&mut project, json!({ "set": { "pointer": "/scene/layers", "value": layer } })).unwrap();
    (root, project)
}

fn picture(project: &Project, id: &str) -> image::RgbaImage {
    let path = project.doc["scene"]["assets"][id]["path"].as_str().unwrap();
    image::open(project.dir.join(path)).unwrap().into_rgba8()
}

const POINTER: &str = "/scene/layers/0/effects/0/params/gradient";

#[test]
fn stops_make_a_ramp_picture_that_an_effect_parameter_uses_and_undo_removes() {
    let (_root, mut project) = scene("fire");
    let stops = json!([{ "at": 1.0, "color": "#ffffff" }, { "at": 0.0, "color": "#000000" }]);
    act(&mut project, json!({ "ramp": { "stops": stops, "pointer": POINTER, "width": 64 } })).unwrap();
    let id = project.doc.pointer(POINTER).and_then(Value::as_str).unwrap().to_owned();
    let strip = picture(&project, &id);
    assert_eq!(strip.dimensions(), (64, 1));
    assert_eq!(strip.get_pixel(0, 0).0, [0, 0, 0, 255]);
    assert_eq!(strip.get_pixel(63, 0).0, [255, 255, 255, 255]);
    let mid = strip.get_pixel(32, 0).0[0];
    assert!((120..=135).contains(&mid), "{mid}");
    assert_eq!(project.doc["scene"]["ext"]["editor"]["ramps"][&id][0]["at"], json!(1.0));
    let asked = wprs_editor::rigs::query(&project.dir, &project.doc, "", &json!({ "ramp": id }));
    assert_eq!(asked["stops"].as_array().map(Vec::len), Some(2), "{asked}");
    let before = project.doc["scene"]["assets"][&id]["path"].clone();
    let again = json!({ "stops": [{ "at": 0.5, "color": "#ff0000" }], "asset": id });
    act(&mut project, json!({ "ramp": again })).unwrap();
    assert_ne!(project.doc["scene"]["assets"][&id]["path"], before);
    assert_eq!(project.doc.pointer(POINTER), Some(&json!(id)));
    assert_eq!(picture(&project, &id).get_pixel(5, 0).0, [255, 0, 0, 255]);
    act(&mut project, json!({ "undo": {} })).unwrap();
    assert_eq!(project.doc["scene"]["assets"][&id]["path"], before);
    act(&mut project, json!({ "undo": {} })).unwrap();
    assert!(project.doc["scene"]["assets"].get(&id).is_none() && project.doc.pointer(POINTER).is_none());
}

#[test]
fn bad_stops_and_foreign_assets_are_refused() {
    let (_root, mut project) = scene("fire");
    let ramp = |stops: Value, more: Value| {
        let mut body = json!({ "stops": stops });
        body.as_object_mut().unwrap().extend(more.as_object().unwrap().clone());
        json!({ "ramp": body })
    };
    let one = json!([{ "at": 0.0, "color": "#112233" }]);
    for (stops, more) in [
        (json!([]), json!({})),
        (json!([{ "at": 1.5, "color": "#000000" }]), json!({})),
        (json!([{ "at": 0.5, "color": "nope" }]), json!({})),
        (
            json!((0..17).map(|k| json!({ "at": k as f32 / 20.0, "color": "#000000" })).collect::<Vec<_>>()),
            json!({}),
        ),
        (one.clone(), json!({ "width": 4 })),
        (one.clone(), json!({ "pointer": "/scene/canvas/background" })),
        (one.clone(), json!({ "asset": "missing" })),
    ] {
        assert!(act(&mut project, ramp(stops, more)).is_err());
    }
    act(
        &mut project,
        json!({ "set": { "pointer": "/scene/assets/plain", "value": { "path": "assets/x.png" } } }),
    )
    .unwrap();
    assert!(act(&mut project, ramp(one, json!({ "asset": "plain" }))).is_err());
}

#[test]
fn a_ramp_can_be_a_color_table() {
    let (_root, mut project) = scene("color_lut");
    let stops = json!([{ "at": 0.0, "color": "#000000" }, { "at": 1.0, "color": "#ff0000" }]);
    let at = "/scene/layers/0/effects/0/params/lut";
    act(&mut project, json!({ "ramp": { "stops": stops, "pointer": at, "format": "cube" } })).unwrap();
    let id = project.doc.pointer(at).and_then(Value::as_str).unwrap();
    let path = project.doc["scene"]["assets"][id]["path"].as_str().unwrap();
    assert!(path.starts_with("luts/") && path.ends_with(".cube"), "{path}");
    let table = wprs_scene::Lut::parse(&std::fs::read(project.dir.join(path)).unwrap()).unwrap();
    assert_eq!(table.size, 17);
    assert_eq!(table.data[0], [0.0; 3]);
    assert_eq!(table.data.last().unwrap(), &[1.0, 0.0, 0.0]);
}

#[test]
fn direction_strokes_encode_where_the_brush_went() {
    let (_root, mut project) = scene("flow");
    let pointer = "/scene/layers/0/effects/0/params/map";
    let stroke = |points: Value, value: f32| {
        json!({ "comb_flow": { "pointer": pointer, "radius": 0.1, "value": value,
            "points": points } })
    };
    act(&mut project, stroke(json!([[0.2, 0.2], [0.4, 0.2]]), 1.0)).unwrap();
    act(&mut project, stroke(json!([[0.7, 0.4], [0.7, 0.7]]), 0.5)).unwrap();
    let id = project.doc.pointer(pointer).and_then(Value::as_str).unwrap().to_owned();
    let flow = picture(&project, &id);
    assert_eq!(flow.get_pixel(5, 500).0, [128, 128, 128, 255]);
    let [r, g, b, a] = flow.get_pixel(150, 102).0;
    assert!(r > 250 && (126..=130).contains(&g) && b == 128 && a == 255, "{r} {g} {b} {a}");
    let [r, g, b, a] = flow.get_pixel(358, 280).0;
    assert!((126..=130).contains(&r) && (185..=195).contains(&g) && b == 128 && a == 255, "{r} {g} {b} {a}");
}

#[test]
fn texture_parameters_name_the_tools_they_take() {
    let tools = |effect: &str, key: &str| {
        let (_root, project) = scene(effect);
        let fields = fields_with(&project.doc, "/scene/layers/0/effects/0", 0.0, &project.effects());
        fields.iter().find(|f| f.key == key).map(|f| f.sample["tools"].clone()).unwrap_or_default()
    };
    assert_eq!(tools("fire", "gradient"), json!(["ramp"]));
    assert_eq!(tools("fluid", "gradient"), json!(["ramp"]));
    assert_eq!(tools("shine", "gradient"), json!(["ramp"]));
    assert_eq!(tools("flow", "map"), json!(["gray", "flow"]));
    assert_eq!(tools("flow", "phase"), json!(["gray"]));
    assert_eq!(tools("water_ripple", "walls"), json!(["gray"]));
    assert_eq!(tools("blend_gradient", "map"), json!(["gray"]));
    assert_eq!(tools("tilt_shift", "depth"), json!(["gray", "depth"]));
    assert_eq!(tools("color_lut", "lut"), json!(["table"]));
    assert_eq!(tools("blend_texture", "texture"), Value::Null);
}

#[test]
fn a_depth_map_says_clearly_when_the_model_is_missing() {
    let (_root, mut project) = scene("tilt_shift");
    act(
        &mut project,
        json!({ "set": { "pointer": "/scene/assets/photo", "value": { "path": "assets/p.png" } } }),
    )
    .unwrap();
    std::fs::create_dir_all(project.dir.join("assets")).unwrap();
    image::RgbaImage::from_pixel(8, 8, image::Rgba([9, 9, 9, 255]))
        .save(project.dir.join("assets/p.png"))
        .unwrap();
    let missing = act(&mut project, json!({ "depth_map": { "asset": "photo" } })).unwrap_err().to_string();
    assert!(
        missing.contains("wprs-ctl models download depth-anything-v2-small")
            || missing.contains("not a picture"),
        "{missing}"
    );
    let refused = act(&mut project, json!({ "depth_map": { "asset": "nothing" } })).unwrap_err().to_string();
    assert!(refused.contains("nothing"), "{refused}");
    let input = act(&mut project, json!({ "depth_map": { "asset": "photo", "input": 100 } }));
    assert!(input.is_err());
    let state = wprs_editor::rigs::query(&project.dir, &project.doc, "", &json!({ "depth_model": {} }));
    assert_eq!((state["installed"].clone(), state["ready"].clone()), (json!(false), json!(false)));
    assert!(state["install"].as_str().unwrap().contains("models download"));
    assert!(manual::describe("ramp").unwrap().contains("stops"));
    assert!(manual::describe("depth_map").is_some());
}
