use wprs_scene::{Text, builtin};

#[test]
fn every_built_in_effect_describes_itself_in_english_and_spanish() {
    for d in builtin() {
        let Some(Text::Localized(texts)) = &d.description else {
            panic!("{} has no localized description", d.id)
        };
        let (en, es) = (texts.get("en"), texts.get("es"));
        assert!(en.is_some_and(|t| !t.is_empty()) && es.is_some_and(|t| !t.is_empty()), "{}", d.id);
        assert_ne!(en, es, "{}", d.id);
    }
}
