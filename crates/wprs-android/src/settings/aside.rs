use crate::Error;
use std::fs;
use std::path::Path;
use wprs_core::{Config, ConfigError, Platform};

pub(crate) const SET_ASIDE: &str = "settings_set_aside";
const BROKEN: &str = "broken";
const SEEN: &str = "unread";

pub(super) fn load(path: &Path) -> Result<Config, Error> {
    match Config::load_on(path, Platform::Phone) {
        Err(ConfigError::Parse { line, column, message }) => {
            log::warn!("settings file set aside at line {line}, column {column}: {message}");
            fs::rename(path, path.with_added_extension(BROKEN)).ok();
            Ok(Config::default())
        }
        other => Ok(other?),
    }
}

pub(super) fn told(path: &Path) -> bool {
    let broken = path.with_added_extension(BROKEN);
    broken.exists() && fs::rename(&broken, path.with_added_extension(SEEN)).is_ok()
}

#[cfg(test)]
#[test]
fn an_unreadable_file_is_set_aside_once_and_the_settings_start_from_the_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    fs::write(&path, "[pause]\nfrom_a_newer_version = 1\n").unwrap();
    assert_eq!(load(&path).unwrap(), Config::default());
    assert!(!path.exists() && told(&path) && !told(&path));
    assert!(path.with_added_extension(SEEN).exists());
    fs::write(&path, "[pause]\non_battery = true\nbelow_percent = 15\n").unwrap();
    assert_eq!(load(&path).unwrap().pause.on_battery, wprs_core::BatteryPause::Below(15));
    assert!(path.exists() && !told(&path));
}
