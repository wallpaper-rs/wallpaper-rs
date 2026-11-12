use super::described;
use crate::effects::builtin;

#[test]
fn every_builtin_effect_has_a_category_and_a_description() {
    for d in builtin() {
        assert!(d.category.is_some(), "{} has no category", d.id);
        assert!(d.description.is_some(), "{} has no description", d.id);
    }
}

#[test]
fn blur_is_in_the_blur_category() {
    let d = described("blur").unwrap();
    assert_eq!(d.category.as_deref(), Some("blur"));
}

#[test]
fn perspective_is_a_draggable_quad_of_its_four_corners() {
    let d = described("perspective").unwrap();
    assert!(d.quad());
    let ids: Vec<&str> = d.points().iter().map(|(id, _)| *id).collect();
    assert_eq!(ids, ["top_left", "top_right", "bottom_right", "bottom_left"]);
    assert_eq!(d.points()[0].1, [0.04, 0.0]);
}

#[test]
fn spin_center_is_a_single_draggable_point() {
    let d = described("spin").unwrap();
    assert!(!d.quad());
    assert_eq!(d.points(), [("center", [0.5, 0.5])]);
}

#[test]
fn a_direction_vector_is_not_a_draggable_point() {
    let d = described("flow").unwrap();
    assert!(d.points().is_empty());
}

#[test]
fn handles_follow_the_kinds_and_units_of_the_descriptor() {
    use crate::effects::{Descriptor, Grab};
    let json = serde_json::json!({"id": "beam", "params": [
        {"id": "start", "type": "position", "default": [0.1, 0.5]},
        {"id": "end", "type": "position", "default": [0.9, 0.5]},
        {"id": "aim", "type": "vec2", "default": [1, 0], "min": -1, "max": 1},
        {"id": "tilt", "type": "float", "unit": "degrees", "default": 0, "min": 0, "max": 90},
        {"id": "gain", "type": "float", "default": 1, "min": 0, "max": 4}]});
    let d = Descriptor::parse(&json.to_string(), "").unwrap();
    let found: Vec<_> = d.handles().iter().map(|h| (h.param, h.kind, h.partner)).collect();
    assert_eq!(
        found,
        [
            ("start", Grab::Point, Some("end")),
            ("end", Grab::Point, Some("start")),
            ("aim", Grab::Vector, None),
            ("tilt", Grab::Angle, None),
        ]
    );
}

#[test]
fn every_builtin_angle_in_degrees_gets_a_dial_and_a_direction_a_vector() {
    use crate::effects::Grab;
    let kind = |id: &str, param: &str| {
        described(id).unwrap().handles().into_iter().find(|h| h.param == param).map(|h| h.kind)
    };
    assert_eq!(kind("spin", "angle"), Some(Grab::Angle));
    assert_eq!(kind("waves", "direction"), Some(Grab::Angle));
    assert_eq!(kind("shake", "direction"), Some(Grab::Vector));
    assert_eq!(kind("lens_flare", "source"), Some(Grab::Point));
    assert_eq!(kind("transform", "pivot"), Some(Grab::Point));
}
