use super::asking::Filter;
use super::at_fraction;

#[test]
fn a_position_parses_as_two_numbers_split_on_a_comma() {
    assert_eq!(at_fraction("0.25,0.75").unwrap(), (0.25, 0.75));
    assert_eq!(at_fraction("1,0").unwrap(), (1.0, 0.0));
    assert_eq!(at_fraction(" 0.1 , 0.2 ").unwrap(), (0.1, 0.2));
}

#[test]
fn a_position_without_a_comma_or_with_a_number_that_does_not_parse_is_refused() {
    assert!(at_fraction("0.5").is_err());
    assert!(at_fraction("x,0.5").is_err());
    assert!(at_fraction("0.5,y").is_err());
}

#[test]
fn applying_a_scene_asks_only_for_that_scene_even_when_another_screen_waits_for_its_own() {
    let reply: wprs_core::model::InputsReply = serde_json::from_value(serde_json::json!({
        "model": 1, "serial": 1, "capturing": false, "tracking": false, "grants": {},
        "shown": {
            "DP-1": {"scene": "org.example.pulse", "audio": true, "pointer": false, "title": "Pulse"},
            "DP-2": {"scene": "org.example.lights", "audio": false, "pointer": true, "title": "Lights"}
        }
    }))
    .unwrap();
    let every = |_: &String| true;
    let all = super::asking::requests(&reply, &every, &Filter { only: None, seen: None });
    assert_eq!(all.len(), 2);
    let lights = super::asking::requests(
        &reply,
        &every,
        &Filter { only: Some("org.example.lights".into()), seen: None },
    );
    assert_eq!(lights.len(), 1);
    assert!(lights[0].starts_with("Lights follows the pointer"), "{lights:?}");
    let seen = Some([("DP-1".to_string(), String::new())].into());
    let changed = super::asking::requests(&reply, &every, &Filter { only: None, seen });
    assert_eq!(changed.len(), 1, "a package that was already shown asked again: {changed:?}");
    assert!(changed[0].starts_with("Lights"), "{changed:?}");
}
