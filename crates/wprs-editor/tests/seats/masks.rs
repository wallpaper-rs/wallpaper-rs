use super::{act, card, near, scene, seat};
use serde_json::{Value, json};
use wprs_editor::Project;

fn nested(mask: Value) -> Value {
    json!([
        card("base", json!({"position": [400, 500], "rotation": 30, "scale": [1.5, 0.75], "skew": [12, 0]})),
        {"id": "kid", "type": "solid", "color": "#ff0000", "size": [120, 80], "parent": "base",
            "transform": {"position": [60, 20], "origin": [0.2, 0.3], "rotation": -20, "scale": [1.2, 0.9]},
            "mask": mask,
            "effects": [{"effect": "vignette", "mask": mask}]}
    ])
}

fn view(p: &Project, index: Option<usize>) -> Value {
    let q = json!({"mask_view": {"layer": "kid", "index": index, "time": 0.0}});
    wprs_editor::rigs::query(&p.dir, &p.doc, "kid", &q)
}

fn hold(p: &mut Project, index: Option<usize>, hold: &str, local: [f64; 2]) -> anyhow::Result<()> {
    let to = centered(p, local);
    act(p, json!({"effects": {"mask_grip": {"layer": "kid", "index": index, "hold": hold, "to": to}}}))
}

fn spot(v: &Value, grip: &str) -> [f64; 2] {
    let found = v["holds"].as_array().unwrap().iter().find(|h| h["grip"] == grip).unwrap();
    [found["at"][0].as_f64().unwrap(), found["at"][1].as_f64().unwrap()]
}

fn mask(p: &Project, index: Option<usize>) -> Value {
    let at = index.map_or("/scene/layers/1/mask".to_owned(), |i| format!("/scene/layers/1/effects/{i}/mask"));
    p.doc.pointer(&at).cloned().unwrap()
}

fn centered(p: &Project, local: [f64; 2]) -> [f64; 2] {
    seat(p, "kid").at([0.5 + local[0] / 120.0, 0.5 + local[1] / 80.0])
}

#[test]
fn the_outline_and_its_handles_sit_where_the_layer_is_drawn() {
    let root = tempfile::tempdir().unwrap();
    let p = scene(&root, nested(json!({"shape": {"rect": {"size": [60, 40], "radius": 6}}, "feather": 8})));
    for index in [None, Some(0)] {
        let v = view(&p, index);
        assert_eq!(v["kind"], "rect");
        assert!(near(spot(&v, "size"), centered(&p, [30.0, 20.0]), 1e-6));
        assert!(near(spot(&v, "feather"), centered(&p, [38.0, 0.0]), 1e-6));
        assert!(near(spot(&v, "radius"), centered(&p, [24.0, -20.0]), 1e-6));
        assert_eq!(v["outer"].as_array().unwrap().len(), 1);
        assert_eq!(v["box"].as_array().unwrap().len(), 4);
    }
}

#[test]
fn dragging_a_handle_sizes_softens_and_rounds_the_shape_of_a_layer_or_an_effect_mask() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested(json!({"shape": {"rect": {"size": [60, 40], "radius": 6}}})));
    for index in [None, Some(0)] {
        hold(&mut p, index, "size", [50.0, 15.0]).unwrap();
        assert_eq!(mask(&p, index)["shape"]["rect"]["size"], json!([100.0, 30.0]));
        hold(&mut p, index, "feather", [65.0, 0.0]).unwrap();
        assert_eq!(mask(&p, index)["feather"], json!(15.0));
        hold(&mut p, index, "radius", [40.0, -15.0]).unwrap();
        assert_eq!(mask(&p, index)["shape"]["rect"]["radius"], json!(10.0));
        assert!(hold(&mut p, index, "width", [0.0, 0.0]).is_err());
    }
}

#[test]
fn rings_stars_polygons_and_lines_have_their_own_handles() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested(json!({"shape": {"ring": {"size": [80, 60], "width": 8}}})));
    hold(&mut p, None, "width", [30.0, 0.0]).unwrap();
    assert_eq!(mask(&p, None)["shape"]["ring"]["width"], json!(10.0));
    act(&mut p, json!({"effects": {"mask": {"layer": "kid", "shape": {"star": {"points": 5, "size": 60, "inner": 0.5}}}}})).unwrap();
    let v = view(&p, None);
    let _ = &v;
    let outer = 30.0f64;
    let angle = -std::f64::consts::FRAC_PI_2 + std::f64::consts::PI / 5.0;
    let r = outer * 0.6;
    hold(&mut p, None, "inner", [angle.cos() * r, angle.sin() * r]).unwrap();
    let inner = mask(&p, None)["shape"]["star"]["inner"].as_f64().unwrap();
    assert!((inner - 0.6).abs() < 0.01, "{inner}");
    act(
        &mut p,
        json!({"effects": {"mask": {"layer": "kid", "shape": {"line": {"to": [60, 0], "width": 10}}}}}),
    )
    .unwrap();
    hold(&mut p, None, "size", [40.0, 10.0]).unwrap();
    assert_eq!(mask(&p, None)["shape"]["line"]["to"], json!([80.0, 20.0]));
    hold(&mut p, None, "feather", [0.0, 25.0]).unwrap();
    assert!(mask(&p, None)["feather"].as_f64().unwrap() > 15.0);
}

#[test]
fn a_picture_mask_shows_the_channel_and_a_layer_mask_shows_the_masking_layer() {
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, nested(json!({"shape": {"ellipse": {"size": [60, 40]}}})));
    let file = p.dir.join("m.png");
    let mut picture = image::RgbaImage::new(4, 2);
    picture
        .pixels_mut()
        .enumerate()
        .for_each(|(i, px)| *px = image::Rgba([if i % 2 == 0 { 255 } else { 0 }, 100, 0, 255]));
    picture.save(&file).unwrap();
    act(&mut p, json!({"set": {"pointer": "/scene/assets/m", "value": {"path": "m.png", "linear": true}}}))
        .unwrap();
    let q = |channel: &str, invert: bool| {
        let ask = json!({"mask_picture": {"asset": "m", "channel": channel, "invert": invert, "strength": 1.0, "tint": "#3daee9"}});
        wprs_editor::rigs::query(&p.dir, &p.doc, "kid", &ask)
    };
    let path = |v: &Value| std::path::PathBuf::from(v["path"].as_str().unwrap());
    let alpha = |v: &Value, x: u32| image::open(path(v)).unwrap().to_rgba8().get_pixel(x, 0).0;
    let red = q("r", false);
    assert_eq!(alpha(&red, 0)[3], 179);
    assert_eq!(alpha(&red, 1)[3], 0);
    assert_eq!(&alpha(&red, 0)[..3], &[0x3d, 0xae, 0xe9]);
    let flipped = q("r", true);
    assert_eq!(alpha(&flipped, 0)[3], 0);
    assert_eq!(alpha(&flipped, 1)[3], 179);
    assert_ne!(path(&red), path(&flipped));
    assert!(
        q("g", false)["path"].is_string()
            && wprs_editor::rigs::query(&p.dir, &p.doc, "kid", &json!({"mask_picture": {"asset": "nope"}}))
                .is_null()
    );
    act(&mut p, json!({"effects": {"mask": {"layer": "kid", "from": "base"}}})).unwrap();
    let v = view(&p, None);
    assert_eq!(v["source"], "layer");
    assert!(near(
        [v["from_box"][0][0].as_f64().unwrap(), v["from_box"][0][1].as_f64().unwrap()],
        seat(&p, "base").corners()[0],
        1e-6
    ));
    assert!(v["holds"].as_array().unwrap().is_empty());
    assert!(view(&p, Some(5)).is_null());
}
