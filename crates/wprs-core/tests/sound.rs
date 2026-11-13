use wprs_core::Config;

#[test]
fn a_screen_has_its_own_sound_that_only_lowers_the_global_one() {
    let mut c = Config::default();
    assert_eq!((c.heard_at("DP-1"), c.own_sound("DP-1").muted), (1.0, None));
    c.set_option("playback.volume.DP-1", "0.4").unwrap();
    assert_eq!(c.heard_at("DP-1"), 0.4);
    assert_eq!(c.heard_at("HDMI-A-1"), 1.0, "another screen kept its sound");
    c.set_option("playback.muted.DP-1", "true").unwrap();
    assert_eq!((c.heard_at("DP-1"), c.own_sound("DP-1").volume), (0.0, Some(0.4)));
    c.set_option("playback.muted.DP-1", "false").unwrap();
    assert_eq!(c.heard_at("DP-1"), 0.4);
    c.set_option("playback.muted.default", "true").unwrap();
    assert_eq!(
        (c.heard_at("HDMI-A-1"), c.heard_at("DP-1")),
        (0.0, 0.0),
        "all screens replaces the own on/off"
    );
    assert_eq!(c.own_sound("DP-1").volume, Some(0.4), "all screens left the own volume of a screen");
    for key in ["playback.muted.DP-1", "playback.volume.DP-1", "playback.muted.default"] {
        c.set_option(key, "none").unwrap();
    }
    assert_eq!(c, Config::default(), "a sound set back to follow left a trace");
}

#[test]
fn a_bad_sound_value_or_screen_is_refused_and_changes_nothing() {
    let mut c = Config::default();
    for (key, value) in [
        ("playback.volume.DP-1", "1.5"),
        ("playback.volume.DP-1", "loud"),
        ("playback.volume.DP-1", "-0.1"),
        ("playback.muted.DP-1", "maybe"),
        ("playback.muted.", "true"),
    ] {
        assert!(c.set_option(key, value).is_err(), "{key}={value}");
    }
    assert_eq!(c, Config::default());
    assert_eq!(c.set_option("playback.volume", "0.3"), Ok(()), "the global volume still has its own key");
    assert_eq!(c.playback.volume, 0.3);
}

#[test]
fn the_sound_of_a_screen_is_kept_in_the_file_and_read_back() {
    let mut c = Config::default();
    c.set_option("playback.volume.DP-1", "0.25").unwrap();
    c.set_option("playback.muted.DP-1", "true").unwrap();
    let text = toml::to_string(&c).unwrap();
    assert!(text.contains("volume = 0.25") && text.contains("muted = true"), "{text}");
    let back: Config = toml::from_str(&text).unwrap();
    assert_eq!((back.heard_at("DP-1"), back.own_sound("DP-1").volume), (0.0, Some(0.25)));
}

#[test]
fn a_battery_level_is_a_setting_that_is_kept_in_range() {
    let mut c = Config::default();
    assert_eq!((c.pause.below_percent, c.playback.lowest_below_percent), (0, 0));
    c.set_option("pause.below_percent", "20").unwrap();
    c.set_option("playback.lowest_below_percent", "35").unwrap();
    assert_eq!((c.pause.below_percent, c.playback.lowest_below_percent), (20, 35));
    for bad in ["101", "-1", "half"] {
        assert!(c.set_option("pause.below_percent", bad).is_err(), "{bad}");
        assert!(c.set_option("playback.lowest_below_percent", bad).is_err(), "{bad}");
    }
    assert_eq!(c.pause.below_percent, 20);
}
