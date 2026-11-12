use std::path::PathBuf;
use wprs_core::{Config, Fit};

fn shown(c: &Config, output: &str) -> Option<PathBuf> {
    c.resolve(output).image
}

#[test]
fn clearing_every_output_blanks_all_and_restoring_brings_them_back() {
    let mut c = Config::default();
    c.set_image("", "/a.png".into(), None);
    c.set_image("DP-1", "/b.png".into(), Some(Fit::Fit));
    c.clear_content("");
    assert!(c.is_blank("") && c.is_blank("DP-1") && c.is_blank("HDMI-1"));
    assert!(c.has_previous("") && c.has_previous("DP-1"));
    assert_eq!(c.resolve("DP-1").fit, Fit::Fit, "the placement was lost");
    assert!(c.restore_content(""));
    assert_eq!(shown(&c, ""), Some("/a.png".into()));
    assert_eq!(shown(&c, "DP-1"), Some("/b.png".into()));
    assert!(!c.has_previous(""));
}

#[test]
fn clearing_one_output_leaves_the_others() {
    let mut c = Config::default();
    c.set_image("", "/a.png".into(), None);
    c.clear_content("DP-1");
    assert!(c.is_blank("DP-1") && !c.is_blank("HDMI-1"));
    assert!(c.restore_content("DP-1"));
    assert_eq!(shown(&c, "DP-1"), Some("/a.png".into()));
}

#[test]
fn clearing_twice_keeps_the_first_previous_and_a_new_wallpaper_forgets_it() {
    let mut c = Config::default();
    c.set_image("", "/a.png".into(), None);
    c.clear_content("");
    c.clear_content("");
    assert!(c.restore_content(""));
    assert_eq!(shown(&c, ""), Some("/a.png".into()));
    c.clear_content("");
    c.set_image("", "/c.png".into(), None);
    assert!(!c.has_previous("") && !c.restore_content(""));
    assert_eq!(shown(&c, ""), Some("/c.png".into()));
}

#[test]
fn clearing_without_a_wallpaper_has_nothing_to_restore() {
    let mut c = Config::default();
    c.clear_content("");
    assert!(c.is_blank("") && !c.has_previous("") && !c.restore_content(""));
}

#[test]
fn the_previous_wallpaper_survives_the_file() {
    let mut c = Config::default();
    c.set_playlist("", "01J0000000000000000000LIST".into());
    c.clear_content("");
    let text = toml::to_string(&c).unwrap();
    let mut back: Config = toml::from_str(&text).unwrap();
    assert!(back.restore_content(""));
    assert_eq!(back.resolve("").playlist.as_deref(), Some("01J0000000000000000000LIST"));
}
