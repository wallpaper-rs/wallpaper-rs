use super::*;

fn followed(name: &str, source: &str, entry: serde_json::Value, call: &str, time: f64) -> serde_json::Value {
    let path = format!("scripts/{name}.js");
    let mut entry = entry;
    entry["path"] = json!(path);
    let value = json!({ "script": path, "fn": call });
    let scene = json!({ "scripts": [entry], "layers": [{ "id": "dot", "type": "solid", "color": "#ffffff", "opacity": value }] });
    let mut script = staged(scene, &[(&path, source)]);
    let out = script.update(&Frame { dt: 0.016, ..Frame::at(time) }).unwrap();
    assert!(out.console.is_empty(), "{name}: {:?}", out.console);
    out.values[&script_key(&path, Some(call), None)].clone()
}

#[test]
fn the_value_templates_of_the_editor_run_and_answer_what_they_export() {
    let wave = include_str!("../../../wprs-editor/src/molds/wave.js");
    let params =
        json!({ "speed": { "type": "number", "default": 1 }, "size": { "type": "number", "default": 20 } });
    let shown = followed("wave", wave, json!({ "params": params }), "wave", 1.0);
    assert!((shown.as_f64().unwrap() - 20.0 * 1f64.sin()).abs() < 1e-3, "{shown}");
    let daytime = include_str!("../../../wprs-editor/src/molds/daytime.js");
    let light = followed("daytime", daytime, json!({}), "daylight", 0.0);
    assert!((0.0..=1.0).contains(&light.as_f64().unwrap()), "{light}");
    let params = json!({ "prop": { "type": "text", "default": "" } });
    let setting = include_str!("../../../wprs-editor/src/molds/setting.js");
    assert_eq!(followed("setting", setting, json!({ "params": params }), "setting", 0.0), json!(0));
    let launches = include_str!("../../../wprs-editor/src/molds/launches.js");
    let counted =
        followed("launches", launches, json!({ "events": ["init"], "uses": ["storage"] }), "launches", 0.0);
    assert_eq!(counted, json!(1));
    let audio = include_str!("../../../wprs-editor/src/molds/audio.js");
    let level = followed("audio", audio, json!({ "uses": ["audio"] }), "level", 0.0);
    assert_eq!(level, json!(0));
}

#[test]
fn the_spawn_template_draws_copies_of_a_template_and_takes_them_away() {
    let source = include_str!("../../../wprs-editor/src/molds/spawn.js");
    let params = json!({ "count": { "type": "number", "default": 3 }, "gap": { "type": "number", "default": 50 },
        "template": { "type": "text", "default": "copy" } });
    let scene = json!({ "scripts": [{ "path": "scripts/spawn.js", "events": ["init", "destroy"], "uses": ["layers"], "params": params }],
        "templates": { "copy": { "id": "copy", "type": "solid", "color": "#ffffff", "size": [8, 40] } },
        "layers": [{ "id": "back", "type": "solid", "color": "#000000" }] });
    let mut script = staged(scene, &[("scripts/spawn.js", source)]);
    let out = at(&mut script, 0.0).unwrap();
    assert!(out.console.is_empty(), "{:?}", out.console);
    let kinds: Vec<&str> = out.spawned.iter().map(|(_, t)| t.as_str()).collect();
    assert_eq!(kinds, ["copy", "copy", "copy"]);
    assert_eq!(
        (out.layers["copy_1"].position, out.layers["copy_2"].position),
        (Some([160.0, 540.0]), Some([210.0, 540.0]))
    );
}
