use serde_json::json;
use wprs_editor::tried::{stage, values};

fn doc() -> serde_json::Value {
    json!({"manifest": {"presets": [{"id": "calm", "values": {"speed": 0.2, "tint": "#102030"}}]}})
}

#[test]
fn a_preset_and_values_merge_with_the_values_winning() {
    let tried = values(&doc(), &json!({"preset": "calm", "values": {"speed": 3, "gone": null}})).unwrap();
    assert_eq!(tried, json!({"speed": 3, "tint": "#102030"}));
    assert_eq!(values(&doc(), &json!({"preset": "none"})).unwrap(), json!({}));
    assert!(values(&doc(), &json!({"quality": "q1"})).is_none());
}

#[test]
fn the_values_join_the_preview_part_of_the_document() {
    let mut document = json!({"manifest": {}});
    stage(&doc(), &json!({"preset": "calm"}), &mut document);
    assert_eq!(document["preview"]["values"]["speed"], 0.2);
    let mut kept = json!({"preview": {"quality": "q2"}});
    stage(&doc(), &json!({"values": {"a": 1}}), &mut kept);
    assert_eq!(
        (kept["preview"]["quality"].clone(), kept["preview"]["values"]["a"].clone()),
        (json!("q2"), json!(1))
    );
}

#[test]
fn the_form_shows_defaults_under_the_tried_values_without_secrets_or_buttons() {
    let doc = json!({"manifest": {"properties": {
        "speed": {"type": "slider", "min": 0, "max": 10, "default": 1},
        "key": {"type": "text", "secret": true},
        "go": {"type": "button"}
    }, "presets": [{"id": "fast", "label": "Fast", "values": {"speed": 8}}]}});
    let plain = wprs_editor::tried::query(&doc, &json!({}));
    assert_eq!(plain["values"]["speed"].as_f64(), Some(1.0));
    let ids: Vec<_> =
        plain["fields"].as_array().unwrap().iter().map(|f| f["key"].as_str().unwrap()).collect();
    assert_eq!(ids, ["speed"]);
    let fast = wprs_editor::tried::query(&doc, &json!({"preset": "fast"}));
    assert_eq!(fast["values"]["speed"].as_f64(), Some(8.0));
    assert_eq!(fast["presets"][0]["label"], "Fast");
}

#[test]
fn a_capture_with_a_preset_chosen_renders_the_preset_values() {
    let doc = json!({"manifest": {"properties": {
        "speed": {"type": "slider", "min": 0, "max": 10, "default": 1}
    }, "presets": [{"id": "fast", "label": "Fast", "values": {"speed": 8}}]}});
    let spec = json!({"time": 0, "values": {}, "preset": "fast"});
    let shown = wprs_editor::reach::tried(&doc, &wprs_editor::tried::rendered(&doc, &spec));
    assert_eq!(shown["speed"].as_f64(), Some(8.0));
    let spec = json!({"values": {"speed": 3}, "preset": "fast"});
    assert_eq!(wprs_editor::tried::rendered(&doc, &spec)["speed"], 3);
    assert!(wprs_editor::tried::rendered(&doc, &json!({"time": 0})).is_empty());
}

#[test]
fn the_form_reads_the_texts_of_each_language_instead_of_their_keys() {
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("../../../scenes/dusk/manifest.json")).unwrap();
    let doc = json!({ "manifest": manifest });
    let label = |language: &str| {
        let form = wprs_editor::tried::query(&doc, &json!({"languages": [language]}));
        let mood = form["fields"].as_array().unwrap().iter().find(|f| f["key"] == "mood").cloned().unwrap();
        (mood["label"].as_str().unwrap().to_owned(), mood["options"][0]["label"].as_str().unwrap().to_owned())
    };
    assert_eq!(label("en"), ("Time of day".to_owned(), "Dusk".to_owned()));
    assert!(!label("es").0.starts_with('@'));
}
