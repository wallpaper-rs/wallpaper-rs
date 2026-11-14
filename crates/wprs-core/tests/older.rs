use wprs_core::{BatteryPause, Config, Platform};

const DEFAULT: &str = include_str!("older/master-default.toml");
const CHANGED: &str = include_str!("older/master-changed.toml");

fn on(text: &str, platform: Platform) -> Config {
    Config::parse_on(text, platform).unwrap()
}

#[test]
fn a_file_of_the_version_before_is_read_with_the_same_meaning_on_the_desktop() {
    let old = on(DEFAULT, Platform::Desktop);
    assert_eq!((old.pause.on_battery, old.playback.allow_flashing), (BatteryPause::Always, false));
    let old = on(CHANGED, Platform::Desktop);
    assert_eq!((old.pause.on_battery, old.playback.allow_flashing), (BatteryPause::Below(20), false));
    let both = CHANGED.replace("on_battery = false", "on_battery = true");
    assert_eq!(on(&both, Platform::Desktop).pause.on_battery, BatteryPause::Always);
    let off = CHANGED.replace("below_percent = 20", "below_percent = 0");
    assert_eq!(on(&off, Platform::Desktop).pause.on_battery, BatteryPause::Never);
}

#[test]
fn the_phone_reads_the_charge_threshold_alone_as_it_did_before() {
    assert_eq!(on(DEFAULT, Platform::Phone).pause.on_battery, BatteryPause::Never);
    let both = CHANGED.replace("on_battery = false", "on_battery = true");
    assert_eq!(on(&both, Platform::Phone).pause.on_battery, BatteryPause::Below(20));
}

#[test]
fn a_guard_left_off_keeps_flashing_wallpapers_disabled_and_one_left_on_stays_on() {
    for text in [DEFAULT, CHANGED] {
        assert!(!on(text, Platform::Desktop).playback.allow_flashing);
    }
    let both = "[playback]\nflash_guard = false\nallow_flashing = true\n";
    assert!(on(both, Platform::Desktop).playback.allow_flashing);
}

#[test]
fn the_converted_file_is_written_back_without_the_old_keys_at_the_first_change() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    std::fs::write(&path, CHANGED).unwrap();
    let mut config = Config::load(&path).unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), CHANGED);
    config.set_option("playback.volume", "0.7").unwrap();
    config.save(&path).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    for gone in ["flash_guard", "\nbelow_percent", "on_battery = false"] {
        assert!(!text.contains(gone), "{gone} in {text}");
    }
    assert!(text.contains("on_battery = \"below:20\"") && text.contains("allow_flashing = false"));
    assert_eq!(toml::from_str::<Config>(&text).unwrap(), config);
}

#[test]
fn a_file_that_still_cannot_be_read_keeps_its_own_error() {
    let newer = DEFAULT.replace("[pause]\n", "[pause]\nfrom_a_newer_version = 1\n");
    let error = Config::parse_on(&newer, Platform::Desktop).unwrap_err().to_string();
    assert!(error.contains("from_a_newer_version"), "{error}");
    assert!(Config::parse_on("[pause\n", Platform::Phone).is_err());
}
