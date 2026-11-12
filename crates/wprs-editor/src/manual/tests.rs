use super::*;
use crate::{ACTIONS, Action};

#[test]
fn every_action_variant_is_listed_and_has_a_schema() {
    let named = names();
    assert_eq!(named.len(), ACTIONS.len());
    for name in ACTIONS {
        assert!(named.iter().any(|n| n == name), "{name} has no schema");
        assert!(ABOUT.iter().any(|(n, a)| *n == name && !a.is_empty()), "{name} is not described");
        assert!(describe(name).is_some_and(|d| d.starts_with(name)), "{name}");
    }
    assert_eq!(ABOUT.map(|(n, _)| n), ACTIONS);
}

#[test]
fn every_example_is_an_action() {
    for name in ACTIONS {
        let example = example(name).unwrap();
        let action: Result<Action, _> = serde_json::from_value(json!({ name: example }));
        assert!(action.is_ok(), "{name}: {example}: {action:?}");
        assert_eq!(action.unwrap().name(), name);
    }
}

#[test]
fn schemas_name_fields_and_choices() {
    let text = describe("add_layer").unwrap();
    assert!(text.contains("kind: string (required)") && text.contains("at: integer (optional)"), "{text}");
    assert!(describe("undo").unwrap().contains("no arguments"));
    let text = describe("particles").unwrap();
    assert!(text.contains("one of:") && text.contains("emitter: integer (required)"), "{text}");
    assert!(invocation("hide").unwrap().contains(r#"'{"hidden":false,"id":"text"}'"#));
}

#[test]
fn every_query_is_listed_once() {
    let mut seen = std::collections::BTreeSet::new();
    for (name, about, args) in ASKS {
        assert!(seen.insert(name) && !about.is_empty(), "{name}");
        assert!(serde_json::from_str::<Value>(args).is_ok(), "{name}");
    }
    let routed = crate::lamps::QUERIES.iter().chain(&crate::stage3d::QUERIES);
    for name in routed {
        assert!(seen.contains(name), "query {name} is not listed");
    }
}
