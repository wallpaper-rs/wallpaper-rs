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
