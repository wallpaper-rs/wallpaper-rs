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
