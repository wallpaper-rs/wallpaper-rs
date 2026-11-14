use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::sync::OnceLock;
use tempfile::TempDir;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::passing::{TransitionEdit, Way};
use wprs_editor::starters::starter;

fn state() {
    static STATE: OnceLock<TempDir> = OnceLock::new();
    STATE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("XDG_STATE_HOME", dir.path()) };
        dir
    });
}

fn project(root: &TempDir) -> Project {
    state();
    let dir = root.path().join("snow");
    Project::create(&dir, starter("empty", "org.example.snow", "Snow", "en").unwrap()).unwrap()
}

fn edit(way: Way) -> TransitionEdit {
    TransitionEdit { way, ..Default::default() }
}

fn act(p: &mut Project, e: TransitionEdit) -> anyhow::Result<()> {
    p.act(&Action::Transition(e)).map(|_| ())
}

fn asked(p: &Project) -> Value {
    p.ask("", &json!({ "transition": {} }))
}

#[test]
fn making_a_transition_writes_a_starter_effect_that_plays_at_every_level() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, TransitionEdit { make: true, ..edit(Way::In) }).unwrap();
    assert!(p.dir.join("transitions/opening.wgsl").exists());
    let made = asked(&p);
    assert_eq!(made["in"]["descriptor"], "transitions/opening.json");
    assert_eq!(made["in"]["plays"], json!([true, true, true, true]));
    assert_eq!(made["in"]["problem"], Value::Null);
    assert_eq!(made["in"]["shape"].as_array().unwrap().len(), 49);
    assert_eq!(made["out"], Value::Null);
    assert_eq!(made["effects"], json!(["transitions/opening.json"]));
    act(&mut p, TransitionEdit { make: true, ..edit(Way::Out) }).unwrap();
    assert_eq!(asked(&p)["out"]["descriptor"], "transitions/opening.json");
}

#[test]
fn a_curve_a_duration_and_a_value_change_and_undo_takes_them_back() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, TransitionEdit { make: true, ..edit(Way::In) }).unwrap();
    let curve = json!([0.2, 0.8, 0.4, 1.0]);
    let values = serde_json::from_value(json!({ "center": [0.3, 0.6], "edge": 0.5 })).unwrap();
    let change = TransitionEdit {
        duration_ms: Some(2500),
        curve: Some(curve.clone()),
        params: Some(values),
        ..edit(Way::In)
    };
    act(&mut p, change).unwrap();
    let way = &p.doc["scene"]["transition"]["in"];
    assert_eq!((way["duration_ms"].clone(), way["curve"].clone()), (json!(2500), curve));
    assert_eq!(way["params"]["center"], json!([0.3, 0.6]));
    let shape = asked(&p)["in"]["shape"].clone();
    assert!(shape[24].as_f64().unwrap() > 0.5, "{shape}");
    p.act(&Action::Undo {}).unwrap();
    assert_eq!(p.doc["scene"]["transition"]["in"]["duration_ms"], 1200);
    let cleared = serde_json::from_value(json!({ "edge": null })).unwrap();
    act(&mut p, TransitionEdit { params: Some(cleared), ..edit(Way::In) }).unwrap();
    assert!(p.doc["scene"]["transition"]["in"].get("params").is_none());
}

#[test]
fn what_breaks_a_rule_is_refused_and_the_scene_stays_as_it_was() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let why = act(&mut p, edit(Way::In)).unwrap_err().to_string();
    assert!(why.contains("make it first"), "{why}");
    act(&mut p, TransitionEdit { make: true, ..edit(Way::In) }).unwrap();
    let before = p.doc.clone();
    let long = act(&mut p, TransitionEdit { duration_ms: Some(20_000), ..edit(Way::In) }).unwrap_err();
    assert!(long.to_string().contains("from 0.1 to 10 s"), "{long}");
    let odd = serde_json::from_value(json!({ "nothing": 1 })).unwrap();
    let unknown = act(&mut p, TransitionEdit { params: Some(odd), ..edit(Way::In) }).unwrap_err();
    assert!(unknown.to_string().contains("nothing"), "{unknown}");
    assert!(act(&mut p, TransitionEdit { curve: Some(json!("wobbly")), ..edit(Way::In) }).is_err());
    fs::write(p.dir.join("plain.json"), json!({ "id": "plain", "cost": { "taps": 1 } }).to_string()).unwrap();
    fs::write(p.dir.join("plain.wgsl"), "fn transition(c: Crossing) -> Crossing { return c; }").unwrap();
    let stage =
        act(&mut p, TransitionEdit { effect: Some("plain.json".into()), ..edit(Way::In) }).unwrap_err();
    assert!(stage.to_string().contains("plain"), "{stage}");
    assert_eq!(p.doc, before);
}

#[test]
fn removing_one_way_keeps_the_other_and_removing_both_leaves_no_transition() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, TransitionEdit { make: true, ..edit(Way::In) }).unwrap();
    act(&mut p, TransitionEdit { make: true, ..edit(Way::Out) }).unwrap();
    act(&mut p, TransitionEdit { remove: true, ..edit(Way::In) }).unwrap();
    assert!(p.doc["scene"]["transition"].get("in").is_none());
    act(&mut p, TransitionEdit { remove: true, ..edit(Way::Out) }).unwrap();
    assert!(p.doc["scene"].get("transition").is_none());
    p.act(&Action::Undo {}).unwrap();
    assert!(p.doc["scene"]["transition"]["out"].is_object());
    assert!(act(&mut p, TransitionEdit { remove: true, ..edit(Way::In) }).is_err());
}

#[test]
fn what_the_tool_writes_passes_the_validation_of_the_scene() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    act(&mut p, TransitionEdit { make: true, duration_ms: Some(800), ..edit(Way::In) }).unwrap();
    act(&mut p, TransitionEdit { make: true, curve: Some(json!("in_out_sine")), ..edit(Way::Out) }).unwrap();
    p.save().unwrap();
    let text = fs::read_to_string(p.dir.join("scene.json")).unwrap();
    let mut scene = wprs_scene::Scene::load(&text, &BTreeMap::new()).unwrap();
    let dir = p.dir.clone();
    scene.read_passages(&mut |f: &str, _: usize| fs::read(dir.join(f)).map_err(|e| e.to_string()));
    assert!(scene.passages.problems().is_empty(), "{:?}", scene.passages.problems());
    assert!(scene.passages.coming.is_some() && scene.passages.out.is_some());
}
