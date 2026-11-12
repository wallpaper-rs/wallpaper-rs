use crate::args::carries_secret;
use crate::choices::{flipped, toggled};
use serde_json::json;

const RATIOS: &str = "16x9,16x10,21x9,landscape";

#[test]
fn lists_keep_the_order_of_the_choices_and_the_others_last() {
    assert_eq!(toggled("", "21x9", RATIOS), "21x9");
    assert_eq!(toggled("21x9", "16x9", RATIOS), "16x9,21x9");
    assert_eq!(toggled("16x9,21x9", "16x9", RATIOS), "21x9");
    assert_eq!(toggled("16x9,43x18", "landscape", RATIOS), "16x9,landscape,43x18");
    assert_eq!(toggled("16x9, 43x18", "43x18", RATIOS), "16x9");
    assert_eq!(toggled("16x9", " ", RATIOS), "16x9");
}

#[test]
fn flags_flip_one_digit() {
    assert_eq!(flipped("100", 1), "110");
    assert_eq!(flipped("111", 0), "011");
    assert_eq!(flipped("100", 7), "100");
}

#[test]
fn secrets_never_travel_through_the_generic_call() {
    let add = |secret: &str| vec![json!("nas"), json!("webdav"), json!({}), json!(secret)];
    assert!(carries_secret("io.github.wallpaper_rs.Providers1", "SetSecret", &[json!("nas"), json!("x")]));
    assert!(carries_secret("Providers1", "Add", &add("hunter2")));
    assert!(!carries_secret("Providers1", "Add", &add("")));
    assert!(!carries_secret("Library1", "Add", &add("x")));
    assert!(carries_secret("Sync1", "SetEscrow", &[json!("copper lantern drifts over harbor")]));
    assert!(carries_secret("Sync1", "Restore", &[json!("copper lantern drifts over harbor")]));
    assert!(!carries_secret("Sync1", "SetEscrow", &[json!("")]));
    assert!(carries_secret("Sync1", "ApproveDevice", &[json!("BCDF-GHJK"), json!("correct horse")]));
    assert!(carries_secret("Sync1", "RevokeDevice", &[json!("0a0a"), json!("correct horse")]));
}
