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
    for name in ["shape:", "lit:", "collider:", "create:", "scripts:", "start:", "window:", "kit:"] {
        assert!(text.contains(&format!("\n  {name}")), "{name} is not listed: {text}");
    }
    let top = text.lines().filter(|l| l.starts_with("  ") && !l.starts_with("   ") && l.contains(": object"));
    assert_eq!(top.count(), 33);
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

#[test]
fn every_query_has_a_schema_and_every_example_fits_it() {
    for (name, _) in ASKS {
        assert!(ask_schema(name).is_some(), "{name} has no argument schema");
        assert!(describe_ask(name).is_some_and(|d| d.starts_with(name)), "{name}");
        for line in example_lines("ask", name).unwrap_or_default() {
            let Some(rest) = line.strip_prefix(&format!("wprs-ctl scene ask DIR {name} ")) else { continue };
            let text = rest.split(" --layer").next().unwrap_or(rest).trim_matches('\'');
            let args: Value = serde_json::from_str(text).unwrap();
            let layer = if rest.contains("--layer") { "x" } else { "" };
            check_ask(name, &args, layer).unwrap_or_else(|e| panic!("{name} {text}: {e}"));
        }
    }
    assert!(ASKS.iter().all(|(n, _)| ask_args::known(n)));
}

#[test]
fn query_arguments_are_checked() {
    let why = |name: &str, args: Value| check_ask(name, &args, "").unwrap_err().to_string();
    assert!(
        why("pose", json!({ "clip": "a", "tim": 1 })).contains("tim is not an argument of the query pose")
    );
    assert!(why("pose", json!({})).contains("needs the argument clip"));
    assert!(
        why("pose", json!({ "clip": "a", "time": "1" }))
            .contains("time of the query pose must be a JSON number")
    );
    assert!(why("wind", json!({ "x": 1 })).contains("it takes: none"));
    assert!(why("keys", json!({})).contains("keys"));
    assert!(check_ask("rig", &json!({}), "").is_ok() && check_ask("weights", &json!("arm"), "").is_ok());
    assert!(check_ask("shape_handles", &json!({}), "form").is_ok());
    assert_eq!(ask_schema("pose").unwrap()["required"], json!(["clip"]));
    assert!(describe_ask("pose").unwrap().contains("  clip: string (required) - Clip to pose"));
}

#[test]
fn every_description_has_a_spanish_text() {
    let po = include_str!("../../../../po/es/wprs-messages.po");
    for (name, about) in ABOUT.iter().chain(ASKS.iter()) {
        assert!(po.contains(&format!("msgid {about:?}\n")), "{name} is not translated");
    }
}
