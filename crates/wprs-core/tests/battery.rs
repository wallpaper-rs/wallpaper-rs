use wprs_core::Config;

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
