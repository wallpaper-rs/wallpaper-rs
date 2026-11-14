use wprs_core::{Config, LessMotion, TransitionKind};

#[test]
fn the_wallpaper_brings_its_transition_unless_the_person_chooses_another() {
    let mut config = Config::default();
    assert_eq!(config.transition("DP-1"), TransitionKind::Own);
    assert_eq!(config.options()["transition.kind"].as_str(), Some("own"));
    config.set_option("transition.kind", "fade").unwrap();
    assert_eq!(config.transition("DP-1"), TransitionKind::Fade);
    config.set_option("transition.kind.DP-1", "none").unwrap();
    assert_eq!(
        (config.transition("DP-1"), config.own_transition("DP-1")),
        (TransitionKind::None, Some(TransitionKind::None))
    );
    assert_eq!(config.transition("HDMI-A-1"), TransitionKind::Fade);
    assert_eq!(config.own_transitions()["DP-1"], "none");
    let saved: Config = toml::from_str(&toml::to_string(&config).unwrap()).unwrap();
    assert_eq!(saved.own_transition("DP-1"), Some(TransitionKind::None));
    config.set_option("transition.kind.DP-1", "general").unwrap();
    assert_eq!(
        (config.transition("DP-1"), config.outputs.contains_key("DP-1")),
        (TransitionKind::Fade, false)
    );
    assert!(config.set_option("transition.kind.DP-1", "spin").unwrap_err().contains("own, fade, none"));
    assert!(config.set_option("transition.kind.", "fade").is_err());
    assert!(config.set_option("transition.kind", "general").is_err());
}

#[test]
fn less_motion_follows_the_system_unless_the_person_says_always_or_never() {
    let mut config = Config::default();
    assert_eq!(config.transition.less_motion, LessMotion::System);
    assert!(LessMotion::System.reduces(true) && !LessMotion::System.reduces(false));
    config.set_option("transition.less_motion", "never").unwrap();
    assert!(!config.transition.less_motion.reduces(true));
    config.set_option("transition.less_motion", "always").unwrap();
    assert!(config.transition.less_motion.reduces(false));
    assert!(config.set_option("transition.less_motion", "sometimes").is_err());
}
