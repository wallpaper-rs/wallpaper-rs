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
