use serde_json::json;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::inspect::fields_with;
use wprs_editor::starters::starter;

#[test]
fn fluid_emitters_are_a_list_placed_on_the_canvas_that_may_be_keyframed() {
    let root = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("XDG_STATE_HOME", root.path().join("state")) };
    let dir = root.path().join("fluid");
    let mut project =
        Project::create(&dir, starter("empty", "org.example.fluid", "Fluid", "en").unwrap()).unwrap();
    let layer = json!({"add_layer": {"kind": "solid", "id": "fill"}});
    project.act(&serde_json::from_value::<Action>(layer).unwrap()).unwrap();
    let pointer = wprs_editor::layers::find(&project.doc, "fill").unwrap();
    let effect = json!({"add": {"pointer": format!("{pointer}/effects"), "value": [{ "effect": "fluid" }]}});
    project.act(&serde_json::from_value::<Action>(effect).unwrap()).unwrap();
    let at = format!("{pointer}/effects/0");
    let fields = fields_with(&project.doc, &at, 0.0, &project.effects());
    let emitters = fields.iter().find(|f| f.key == "emitters").unwrap();
    assert_eq!(
        (emitters.widget.as_str(), emitters.bindable, &emitters.default),
        ("emitters", true, &json!([]))
    );
    assert!(fields.iter().find(|f| f.key == "strength").unwrap().bindable);
    let list = json!([{ "from": [0.2, 0.9], "strength": 4 }, { "from": [0.1, 1], "to": [0.9, 1], "angle": -80, "strength": 6 }]);
    let set = json!({"set": {"pointer": format!("{at}/params/emitters"), "value": list}});
    project.act(&serde_json::from_value::<Action>(set).unwrap()).unwrap();
    assert!(wprs_editor::check::review(&project.doc, Some(&dir)).iter().all(|p| p.warning));
    let far = json!({"set": {"pointer": format!("{at}/params/emitters/0/strength"), "value": 99}});
    project.act(&serde_json::from_value::<Action>(far).unwrap()).unwrap();
    let found = wprs_editor::check::review(&project.doc, Some(&dir));
    assert!(found.iter().any(|p| !p.warning && p.message.contains("emitters")), "{found:?}");
}
