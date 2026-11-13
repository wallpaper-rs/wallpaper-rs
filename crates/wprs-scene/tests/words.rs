use serde_json::{Value as Json, json};
use std::collections::BTreeMap;
use std::sync::Arc;
use wprs_scene::*;

const VIEW: [f32; 4] = [0.0, 0.0, 100.0, 50.0];

fn scene(text: Json) -> Scene {
    let layers = json!([{ "id": "hello", "type": "text", "text": text }]);
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": layers });
    Scene::load(&text.to_string(), &BTreeMap::new()).unwrap_or_else(|e| panic!("{e}"))
}

fn shown(scene: &Scene, languages: &[&str]) -> String {
    let mut rt = Runtime::new(Arc::new(scene.clone()), Values::new());
    let languages: Vec<String> = languages.iter().map(|l| l.to_string()).collect();
    rt.set_languages(&languages);
    match &rt.frame(VIEW, Quality::Q3).draws[0].content {
        Content::Text { text, .. } => text.to_string(),
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_text_layer_says_what_its_language_says_and_falls_back_to_the_default() {
    let both = scene(json!({ "en": "Good morning", "es": "Buenos días", "pt-BR": "Bom dia" }));
    assert_eq!(shown(&both, &["es"]), "Buenos días");
    assert_eq!(shown(&both, &["es-AR"]), "Buenos días");
    assert_eq!(shown(&both, &["es_MX.UTF-8", "en"]), "Buenos días");
    assert_eq!(shown(&both, &["pt-BR"]), "Bom dia");
    assert_eq!(shown(&both, &["pt-PT"]), "Bom dia");
    assert_eq!(shown(&both, &["fr", "es"]), "Buenos días");
    assert_eq!(shown(&both, &["fr", "en"]), "Good morning");
    assert_eq!(shown(&both, &[]), "Good morning");
    let plain = scene(json!("Hola"));
    assert_eq!(shown(&plain, &["en"]), "Hola");
    assert!(matches!(&plain.layers[0].kind, Kind::Text(t) if t.text.said().is_some()));
}

#[test]
fn texts_in_several_languages_name_their_feature_and_are_checked() {
    let told = scene(json!({ "en": "Hi", "es": "Hola" }));
    assert!(told.features().contains("text_languages"));
    assert!(told.validate(&BTreeMap::new()).is_empty());
    assert!(!scene(json!("Hi")).features().contains("text_languages"));
    let refuse = |text: Json| {
        let layers = json!([{ "id": "hello", "type": "text", "text": text }]);
        let text = json!({ "schema": SCHEMA, "canvas": { "size": [100, 50] }, "layers": layers });
        Scene::load(&text.to_string(), &BTreeMap::new()).err().map(|e| e.to_string()).unwrap_or_default()
    };
    assert!(refuse(json!({ "en": "Hi", "english": "Hello" })).contains("/text/english"));
    assert!(refuse(json!({ "en": "Hi", "es": " " })).contains("/text/es"));
    let clock = scene(json!({ "clock": "HH:mm" }));
    assert!(matches!(&clock.layers[0].kind, Kind::Text(t) if t.text.said().is_some()));
    let text = serde_json::to_value(&told).unwrap();
    assert_eq!(text["layers"][0]["text"], json!({ "en": "Hi", "es": "Hola" }));
}

#[test]
fn scripts_read_the_text_in_the_language_shown_and_what_they_set_wins_in_every_language() {
    let both = scene(json!({ "en": "Good morning", "es": "Buenos días" }));
    let mut rt = Runtime::new(Arc::new(both), Values::new());
    rt.set_languages(&["es".to_string(), "en".to_string()]);
    assert_eq!(rt.read("hello").unwrap().text.as_deref(), Some("Buenos días"));
    let set = LayerOverride { text: Some("Hi".into()), ..LayerOverride::default() };
    rt.set_script(ScriptOutputs { layers: [("hello".into(), set)].into(), ..ScriptOutputs::default() });
    assert_eq!(rt.read("hello").unwrap().text.as_deref(), Some("Hi"));
    let frame = rt.frame(VIEW, Quality::Q3);
    assert!(matches!(&frame.draws[0].content, Content::Text { text, .. } if text == "Hi"));
}
