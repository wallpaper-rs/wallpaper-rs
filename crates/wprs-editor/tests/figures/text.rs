use super::{act, at, near, scene, seat};
use serde_json::{Value, json};
use wprs_scene::{Color, Layout};
use wprs_type::Fonts;

fn label(text: &str, extra: Value) -> Value {
    let mut layer = json!({"id": "t", "type": "text", "text": text, "size": 60,
        "transform": {"position": [500, 400], "rotation": 12}});
    for (k, v) in extra.as_object().cloned().unwrap_or_default() {
        layer[k] = v;
    }
    layer
}

fn fonted() -> bool {
    !wprs_type::families().is_empty()
}

fn drawn(text: &str, size: f32, layout: &Layout) -> [f64; 2] {
    let chain = Fonts::from_faces([]).chain(None, &[]);
    let image = wprs_type::raster(&chain, text, size, wprs_scene::Align::Left, 1.0, layout).unwrap();
    [f64::from(image.width()), f64::from(image.height())]
}

fn box_of(p: &wprs_editor::Project) -> Value {
    let q = json!({"text_box": {"layer": "t", "time": 0.0}});
    wprs_editor::rigs::query(&p.dir, &p.doc, "t", &q)
}

#[test]
fn the_box_of_a_text_is_the_size_the_renderer_draws() {
    if !fonted() {
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let p = scene(&root, json!([label("Hello world", json!({}))]));
    let want = drawn("Hello world", 60.0, &Layout::default());
    assert!(near(seat(&p, "t").size, want, 1e-6), "{:?} {want:?}", seat(&p, "t").size);
    let found = box_of(&p);
    assert_eq!(found["lines"], 1);
    assert!(near([found["size"][0].as_f64().unwrap(), found["size"][1].as_f64().unwrap()], want, 1e-6));
}

#[test]
fn a_line_break_adds_a_line_to_the_box() {
    if !fonted() {
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let one = scene(&root, json!([label("Hello", json!({}))]));
    let two = scene(&root, json!([label("Hello\nworld", json!({}))]));
    assert_eq!(box_of(&two)["lines"], 2);
    let (a, b) = (seat(&one, "t").size, seat(&two, "t").size);
    assert!(b[1] > a[1] * 1.8 && b[1] < a[1] * 2.2, "{a:?} {b:?}");
    let layout = Layout::default();
    assert!(near(b, drawn("Hello\nworld", 60.0, &layout), 1e-6));
}

#[test]
fn the_box_counts_the_outline_the_padding_and_the_wrap() {
    if !fonted() {
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let extra = json!({"wrap": {"width": 300, "lines": 2}, "padding": 8, "outline": {"width": 3},
        "line_height": 1.5, "letter_spacing": 2});
    let p = scene(&root, json!([label("one two three four five six seven", extra)]));
    let layout = Layout {
        wrap: Some(wprs_scene::Wrap { width: 300.0, lines: Some(2), ellipsis: false }),
        line_height: Some(1.5),
        padding: 8.0,
        backing: None::<(Color, [f32; 2], f32)>,
        spacing: 2.0,
        reach: 8.0,
    };
    let want = drawn("one two three four five six seven", 60.0, &layout);
    assert!(near(seat(&p, "t").size, want, 1e-6), "{:?} {want:?}", seat(&p, "t").size);
    assert_eq!(box_of(&p)["lines"], 2);
}

#[test]
fn dragging_the_width_handle_sets_the_wrap_and_keeps_the_left_edge() {
    if !fonted() {
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let mut p = scene(&root, json!([label("one two three four five six seven eight", json!({}))]));
    let before = seat(&p, "t");
    let edge = before.at([0.0, 0.5]);
    let found = box_of(&p);
    assert!(near(
        [found["handle"][0].as_f64().unwrap(), found["handle"][1].as_f64().unwrap()],
        before.at([1.0, 0.5]),
        1e-6
    ));
    let to = before.at([0.5, 0.5]);
    act(&mut p, json!({"layer": {"figure": {"text_width": {"layer": "t", "to": to}}}})).unwrap();
    let width = at(&p, "/scene/layers/0/wrap/width").as_f64().unwrap();
    assert!(width > 100.0 && width < before.size[0], "{width}");
    let after = seat(&p, "t");
    assert!(after.size[1] > before.size[1]);
    assert!(near(after.at([0.0, 0.5]), edge, 0.6), "{:?} {edge:?}", after.at([0.0, 0.5]));
    act(&mut p, json!({"layer": {"figure": {"text_width": {"layer": "t", "remove": true}}}})).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/wrap"), Value::Null);
}

#[test]
fn a_clock_text_measures_what_it_will_show() {
    if !fonted() {
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let mut layer = label("", json!({}));
    layer["text"] = json!({"clock": "HH:mm"});
    let p = scene(&root, json!([layer]));
    let size = seat(&p, "t").size;
    let digits = drawn("00:00", 60.0, &Layout::default());
    assert!(size[0] > digits[0] * 0.8 && size[0] < digits[0] * 1.2, "{size:?} {digits:?}");
}
