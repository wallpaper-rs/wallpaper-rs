use wprs_core::locale::{CONTEXT_MARK, parse};

#[test]
fn the_same_english_word_keeps_one_translation_per_context() {
    let po = "msgid \"Rate\"\nmsgstr \"Ritmo\"\n\nmsgctxt \"rating\"\nmsgid \"Rate\"\nmsgstr \"Valorar\"\n";
    let map = parse(po);
    assert_eq!(map["Rate"], "Ritmo");
    assert_eq!(map[&format!("rating{CONTEXT_MARK}Rate")], "Valorar");
}

#[test]
fn a_fuzzy_or_empty_entry_is_not_used() {
    let po = "#, fuzzy\nmsgid \"A\"\nmsgstr \"B\"\n\nmsgid \"C\"\nmsgstr \"\"\n";
    assert!(parse(po).is_empty());
}

#[test]
fn a_system_locale_becomes_a_valid_language_tag() {
    use wprs_core::locale::bcp47;
    assert_eq!(bcp47("es_AR.UTF-8"), "es-AR");
    assert_eq!(bcp47("sr_RS@latin"), "sr-RS");
    assert_eq!(bcp47("de"), "de");
    for plain in ["", "C", "POSIX", "C.UTF-8", "x", "en_US_extra_long_part_x"] {
        assert_eq!(bcp47(plain), "en-US", "{plain}");
    }
}

#[test]
fn a_language_is_tried_whole_and_then_by_its_base_whatever_its_separator() {
    use wprs_core::locale::{valid, variants};
    assert_eq!(variants("es_AR.UTF-8"), ["es_AR", "es"]);
    assert_eq!(variants("pt-BR"), ["pt-BR", "pt"]);
    assert_eq!(variants("en"), ["en", "en"]);
    assert!(valid("pt-BR") && valid("zh-Hans") && !valid("english") && !valid("e") && !valid("es_AR"));
}
