use wprs_format::{Derivatives, derivatives};

fn allowed(expression: &str) -> Vec<String> {
    match derivatives(expression) {
        Derivatives::Allowed(conditions) => conditions,
        other => panic!("{expression}: {other:?}"),
    }
}

#[test]
fn free_and_conditioned_licenses_allow_derived_works() {
    assert!(allowed("CC0-1.0").is_empty());
    assert_eq!(allowed("CC-BY-4.0"), ["attribution"]);
    assert_eq!(allowed("CC-BY-SA-4.0"), ["share-alike", "attribution"]);
    assert_eq!(allowed("CC-BY-NC-SA-4.0"), ["share-alike", "non-commercial", "attribution"]);
    assert_eq!(allowed("GPL-3.0-or-later"), ["copyleft"]);
    assert_eq!(allowed("LicenseRef-studio"), ["custom terms"]);
}

#[test]
fn no_derivatives_licenses_refuse_unless_another_choice_is_free() {
    for expression in ["CC-BY-ND-4.0", "CC-BY-NC-ND-4.0", "CC-BY-4.0 AND CC-BY-ND-4.0", ""] {
        assert!(matches!(derivatives(expression), Derivatives::Refused(_)), "{expression}");
    }
    assert_eq!(allowed("CC-BY-ND-4.0 OR CC-BY-4.0"), ["attribution"]);
    assert!(matches!(derivatives("CC-BY-ND-4.0"), Derivatives::Refused(r) if r.contains("CC-BY-ND-4.0")));
}
