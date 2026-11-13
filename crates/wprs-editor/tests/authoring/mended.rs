use super::{act, at, project, refused};
use serde_json::json;
use wprs_editor::flashes::{Report, cached, remember};

fn ids(p: &wprs_editor::Project, pointer: &str) -> Vec<String> {
    let list = at(p, pointer);
    list.as_array().unwrap().iter().map(|l| l["id"].as_str().unwrap().to_owned()).collect()
}

#[test]
fn a_layer_moves_into_a_group_whether_the_group_is_above_or_below_it() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, json!({"add_layer": {"kind": "group", "id": "above"}})).unwrap();
    act(&mut p, json!({"move_layer": {"id": "card", "to": 0, "into": "above"}})).unwrap();
    assert_eq!(ids(&p, "/scene/layers"), ["above"]);
    assert_eq!(ids(&p, "/scene/layers/0/layers"), ["card"]);
    act(&mut p, json!({"add_layer": {"kind": "solid", "id": "late"}})).unwrap();
    act(&mut p, json!({"add_layer": {"kind": "group", "id": "below", "at": 0}})).unwrap();
    act(&mut p, json!({"move_layer": {"id": "late", "to": 0, "into": "below"}})).unwrap();
    assert_eq!(ids(&p, "/scene/layers"), ["below", "above"]);
    act(&mut p, json!({"move_layer": {"id": "card", "to": 1, "into": "below"}})).unwrap();
    assert_eq!(ids(&p, "/scene/layers/0/layers"), ["late", "card"]);
    act(&mut p, json!({"move_layer": {"id": "below", "to": 0, "into": "above"}})).unwrap();
    assert_eq!(ids(&p, "/scene/layers/0/layers"), ["below"]);
    let why = refused(&mut p, json!({"move_layer": {"id": "above", "to": 0, "into": "below"}}));
    assert!(why.contains("itself"), "{why}");
}

#[test]
fn a_script_is_made_from_a_template_without_a_file() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, json!({"import_script": {"template": "empty"}})).unwrap();
    assert_eq!(at(&p, "/scene/scripts").as_array().map(Vec::len), Some(1));
}

#[test]
fn a_text_layer_is_translated_like_the_title_and_the_inspector_offers_its_languages() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, json!({"add_layer": {"kind": "text", "id": "words"}})).unwrap();
    let words = "/scene/layers/1/text";
    let put = |language: &str, text: &str| json!({"translate": {"pointer": words, "language": language, "text": text}});
    act(&mut p, put("en", "Good morning")).unwrap();
    assert_eq!(at(&p, words), "Good morning");
    act(&mut p, put("es", "Buenos días")).unwrap();
    assert_eq!(at(&p, words), json!({"en": "Good morning", "es": "Buenos días"}));
    let shown = wprs_editor::inspect::fields(&p.doc, "/scene/layers/1");
    let field = shown.iter().find(|f| f.pointer == words).unwrap();
    assert_eq!((field.widget.as_str(), field.options.len()), ("texts", 2));
    act(&mut p, json!({"translate": {"pointer": words, "language": "fr", "add": true}})).unwrap();
    assert_eq!(at(&p, &format!("{words}/fr")), "Good morning");
    act(&mut p, json!({"translate": {"pointer": words, "language": "fr"}})).unwrap();
    act(&mut p, json!({"translate": {"pointer": words, "language": "es"}})).unwrap();
    assert_eq!(at(&p, words), json!({"en": "Good morning"}));
    let last = refused(&mut p, json!({"translate": {"pointer": words, "language": "en"}}));
    assert!(last.contains("text layer needs a text"), "{last}");
    assert!(wprs_editor::check::review(&p.doc, None).iter().all(|q| !q.pointer.contains("words")));
}

#[test]
fn a_text_layer_without_the_default_language_is_a_problem_and_a_bound_one_has_no_languages() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, json!({"add_layer": {"kind": "text", "id": "words"}})).unwrap();
    let words = "/scene/layers/1/text";
    act(&mut p, json!({"set": {"pointer": words, "value": {"es": "Hola", "fr": "Salut"}}})).unwrap();
    let problems = wprs_editor::check::review(&p.doc, None);
    assert!(problems.iter().any(|q| q.pointer == words), "{problems:?}");
    act(&mut p, json!({"translate": {"pointer": words, "language": "en", "text": "Hi"}})).unwrap();
    assert!(wprs_editor::check::review(&p.doc, None).iter().all(|q| q.pointer != words));
    act(&mut p, json!({"set": {"pointer": words, "value": {"clock": "HH:mm"}}})).unwrap();
    let bound = refused(&mut p, json!({"translate": {"pointer": words, "language": "es", "add": true}}));
    assert!(bound.contains("follows a value"), "{bound}");
    assert_eq!(wprs_editor::texts::preview_locale(&p.doc, "es"), "en");
    act(&mut p, json!({"set": {"pointer": words, "value": {"en": "Hi", "es": "Hola"}}})).unwrap();
    assert_eq!(wprs_editor::texts::preview_locale(&p.doc, "es-AR"), "es");
    assert_eq!(wprs_editor::texts::preview_locale(&p.doc, "fr"), "en");
    let strings = p.ask("", &json!({"strings": {}}));
    assert!(strings["languages"].as_array().unwrap().contains(&json!("es")), "{strings}");
}

#[test]
fn a_light_added_with_at_sits_at_that_position_in_its_group() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, json!({"add_layer": {"kind": "group", "id": "pane"}})).unwrap();
    act(&mut p, json!({"layer": {"transform": {"layer": "pane", "position": [100, 50]}}})).unwrap();
    act(&mut p, json!({"light": {"add": {"light": "point", "id": "lamp", "into": "pane", "at": [10, 20]}}}))
        .unwrap();
    let path = wprs_editor::layers::find(&p.doc, "lamp").unwrap();
    assert_eq!(at(&p, &format!("{path}/transform/position")), json!([10.0, 20.0]));
    let seen = p.ask("", &json!({"lights": {"time": 0}}));
    let lamp = seen["lights"].as_array().unwrap().iter().find(|l| l["id"] == "lamp").unwrap();
    assert_eq!(lamp["at"], json!([110.0, 70.0]));
    act(&mut p, json!({"light": {"set": {"layer": "lamp", "at": [500, 300]}}})).unwrap();
    let seen = p.ask("", &json!({"lights": {"time": 0}}));
    let lamp = seen["lights"].as_array().unwrap().iter().find(|l| l["id"] == "lamp").unwrap();
    assert_eq!(lamp["at"], json!([500.0, 300.0]));
}

#[test]
fn the_flashes_measured_are_found_again_by_another_process() {
    let root = tempfile::tempdir().unwrap();
    let p = project(&root);
    let report = Report { flashing: true, rate: 4.5, at: Some(1.0), from: 0.0, seconds: 8.0, fps: 15 };
    assert!(cached(&p.dir, &p.doc).is_none());
    remember(&p.dir, &p.doc, &report);
    let elsewhere = p.dir.join(".");
    assert_eq!(cached(&elsewhere, &p.doc), Some(report));
    let mut changed = p.doc.clone();
    changed["scene"]["canvas"]["size"] = json!([100, 100]);
    assert_eq!(cached(&elsewhere, &changed), None);
}
