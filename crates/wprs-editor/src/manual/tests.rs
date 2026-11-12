use super::*;
use crate::ACTIONS;

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
fn schemas_name_fields_and_choices() {
    let text = describe("add_layer").unwrap();
    assert!(text.contains("kind: string (required)") && text.contains("at: integer (optional)"), "{text}");
    assert!(describe("undo").unwrap().contains("no arguments"));
    let text = describe("particles").unwrap();
    assert!(text.contains("one of:") && text.contains("emitter: integer (required)"), "{text}");
}

#[test]
fn every_query_is_listed_once() {
    let mut seen = std::collections::BTreeSet::new();
    for (name, about) in ASKS {
        assert!(seen.insert(name) && !about.is_empty(), "{name}");
    }
    let routed = crate::lamps::QUERIES.iter().chain(&crate::stage3d::QUERIES).chain(&crate::wgsl::QUERIES);
    for name in routed {
        assert!(seen.contains(name), "query {name} is not listed");
    }
}

#[test]
fn nested_variants_unit_variants_and_needed_fields_are_listed() {
    let text = describe("particles").unwrap();
    for name in ["shape:", "lit:", "collider:", "create:", "scripts:", "start:", "window:"] {
        assert!(text.contains(&format!("\n  {name}")), "{name} is not listed: {text}");
    }
    let top = text.lines().filter(|l| l.starts_with("  ") && !l.starts_with("   ") && l.contains(": object"));
    assert_eq!(top.count(), 32);
    assert!(describe("layer").unwrap().contains("clear: no arguments"));
    let text = describe("scene3d").unwrap();
    assert!(text.contains("eye: no arguments") && text.contains("center: no arguments"), "{text}");
    assert!(!text.contains(r#"default """#), "{text}");
    assert!(describe("pool").unwrap().contains("id: string (required)"));
}

#[test]
fn the_texts_of_the_listing_say_what_the_program_does() {
    assert!(about("lock").unwrap().contains("canvas handles"));
    let ask = |n: &str| ASKS.iter().find(|(name, _)| *name == n).unwrap().1;
    assert!(ask("spread").contains("screen") && !ask("spread").contains("emitter"));
    assert!(ask("clocks").contains("own clock"));
}

#[test]
fn the_particles_lighting_error_names_the_field_that_is_taken() {
    let doc = json!({ "scene": { "layers": [{ "id": "p", "type": "particles" }] } });
    let edit = crate::action::LightingEdit { layer: "p".into(), roughness: Some(0.5), ..Default::default() };
    let action = crate::Action::Lighting(edit);
    let why = crate::lamps::act(std::path::Path::new("."), &doc, &action).unwrap().unwrap_err().to_string();
    assert!(why.contains("emissive") && !why.contains("glow"), "{why}");
}
