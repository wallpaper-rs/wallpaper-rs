use serde_json::{Value, json};
use tempfile::TempDir;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::rigs::query;
use wprs_editor::starters::starter;

fn project(root: &TempDir) -> Project {
    let state = root.path().join("state");
    unsafe { std::env::set_var("XDG_STATE_HOME", &state) };
    let made = starter("empty", "org.example.level", "Level", "en").unwrap();
    let mut p = Project::create(&root.path().join("level"), made).unwrap();
    let layer = json!([{ "id": "fill", "type": "solid", "color": "#ffffff",
        "effects": [{ "effect": "blur" }, { "effect": "god_rays", "optional": true }] }]);
    let set = json!({ "set": { "pointer": "/scene/layers", "value": layer } });
    p.act(&serde_json::from_value::<Action>(set).unwrap()).unwrap();
    p
}

fn level(p: &Project, quality: &str, battery: bool, calm: bool) -> Value {
    let ask = json!({ "level": { "quality": quality, "battery": battery, "calm": calm } });
    query(&p.dir, &p.doc, "", &ask)
}

#[test]
fn a_level_names_what_each_effect_keeps_and_costs_there() {
    let root = tempfile::tempdir().unwrap();
    let p = project(&root);
    let full = level(&p, "q3", false, false);
    assert_eq!(full["quality"], "q3");
    assert_eq!(full["left_out"], 0);
    assert_eq!(full["effects"].as_array().unwrap().len(), 2);
    assert!(full["effects"].as_array().unwrap().iter().all(|e| e["kept"] == true && e["units"].is_number()));
    let lowest = level(&p, "q0", false, false);
    let left: Vec<_> = lowest["effects"].as_array().unwrap().iter().filter(|e| e["kept"] == false).collect();
    assert_eq!(lowest["left_out"].as_u64().unwrap() as usize, left.len());
    assert!(left.iter().all(|e| e["units"].is_null() && e["full_units"].is_number()));
    assert!(lowest["render_scale"].as_f64().unwrap() < full["render_scale"].as_f64().unwrap());
}

#[test]
fn battery_lowers_the_level_the_daemon_reaches_and_reduced_motion_is_reported() {
    let root = tempfile::tempdir().unwrap();
    let p = project(&root);
    let drained = level(&p, "q3", true, true);
    assert_eq!((drained["asked"].as_str(), drained["quality"].as_str()), (Some("q3"), Some("q2")));
    assert_eq!((drained["battery"].clone(), drained["calm"].clone()), (json!(true), json!(true)));
    assert_eq!(level(&p, "q1", true, false)["quality"], "q1", "a lower cap is kept");
    assert_eq!(level(&p, "q2", false, false)["quality"], "q2");
}

#[test]
fn the_glare_of_the_bloom_is_kept_only_where_the_daemon_draws_it() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let set = json!({ "set": { "pointer": "/scene/canvas/bloom", "value": { "hdr": [1.0, 0.5] } } });
    p.act(&serde_json::from_value::<Action>(set).unwrap()).unwrap();
    let glare = |q: &str, battery| level(&p, q, battery, false)["bloom"].clone();
    assert_eq!(glare("q3", false), json!({ "on": true, "hdr": true, "glare": true }));
    assert_eq!(glare("q2", false), json!({ "on": true, "hdr": true, "glare": true }));
    assert_eq!(glare("q2", true), json!({ "on": true, "hdr": true, "glare": false }));
    assert_eq!(glare("q1", false), json!({ "on": true, "hdr": true, "glare": false }));
}

#[test]
fn the_preview_carries_battery_and_reduced_motion_only_when_asked() {
    let staged = |look: Value| wprs_editor::rigs::staged(&look);
    assert_eq!(staged(json!({ "battery": false, "calm": false })), None);
    let both = staged(json!({ "quality": "q2", "battery": true, "calm": true })).unwrap();
    assert_eq!((both["battery"].clone(), both["calm"].clone()), (json!(true), json!(true)));
    let one = staged(json!({ "calm": true })).unwrap();
    assert!(one.get("battery").is_none() && one["calm"] == true);
}

#[test]
fn effects_and_emitters_that_follow_the_pointer_are_named() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let layers = json!([{ "id": "fill", "type": "solid", "color": "#ffffff",
        "effects": [{ "effect": "blur" }, { "effect": "water_ripple" }] },
        { "id": "dust", "type": "particles", "system": "dust" }]);
    let particles = json!({ "dust": { "capacity": 100, "emitters": [{ "shape": "edge", "rate": 10, "follow": "cursor" }] } });
    for (pointer, value) in [("/scene/layers", layers), ("/scene/particles", particles)] {
        let set = json!({ "set": { "pointer": pointer, "value": value } });
        p.act(&serde_json::from_value::<Action>(set).unwrap()).unwrap();
    }
    let shown = level(&p, "q3", false, false);
    let follows: Vec<_> = shown["effects"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| (e["effect"].clone(), e["follows"].clone()))
        .collect();
    assert_eq!(follows, [(json!("blur"), json!(false)), (json!("water_ripple"), json!(true))]);
    assert_eq!(shown["pointer"]["emitters"], json!(["dust"]));
}

#[test]
fn a_render_draws_what_the_preview_draws_with_one_alone_and_a_pinned_seed() {
    let doc = json!({ "scene": { "clock": { "seed": null }, "canvas": { "bloom": { "strength": 1.0 } },
        "layers": [{ "id": "a", "type": "solid", "color": "#ffffff" }, { "id": "b", "type": "solid", "color": "#ff0000" }],
        "ext": { "editor": { "solo": "b" } } } });
    let shown = wprs_editor::reach::shown(&doc, true);
    assert!(shown["clock"]["seed"].as_u64().is_some_and(|s| s > 0), "the seed stayed random: {shown}");
    assert_eq!(shown["layers"][0]["visible"], false);
    assert!(shown["layers"][1]["visible"].is_null());
    assert!(shown["canvas"]["bloom"].is_object());
    assert_eq!(shown["clock"]["seed"], wprs_editor::reach::shown(&doc, false)["clock"]["seed"]);
    assert!(wprs_editor::reach::shown(&doc, false)["canvas"].get("bloom").is_none());
}
