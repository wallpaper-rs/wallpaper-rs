use serde_json::json;
use std::sync::Arc;
use wprs_scene::{Budget, Heat, Quality, Runtime, SCHEMA, Scene, Tier, Why};

fn runtime(layers: usize) -> Runtime {
    let blur = |i: usize| json!({ "id": format!("b{i}"), "type": "adjustment", "effects": [{ "effect": "blur", "params": { "radius": 30 } }] });
    let layers: Vec<_> = (0..layers).map(blur).collect();
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [960, 540] }, "layers": layers });
    Runtime::new(Arc::new(Scene::from_json(&text.to_string()).unwrap()), Default::default())
}

#[test]
fn the_level_tells_why_it_is_below_the_highest() {
    let (light, heavy) = (runtime(0), runtime(6));
    let mains = |tier| Budget::new(tier, false, 60);
    assert_eq!(mains(Tier::T3).level(&light), (Quality::Q3, Why::Full));
    assert_eq!(mains(Tier::T1).level(&light), (Quality::Q2, Why::Device));
    assert_eq!(Budget::new(Tier::T3, true, 60).level(&light), (Quality::Q2, Why::Battery));
    assert_eq!(Budget { saver: true, ..mains(Tier::T3) }.level(&light), (Quality::Q1, Why::Saver));
    assert_eq!(Budget { heat: Heat::Hot, ..mains(Tier::T3) }.level(&light), (Quality::Q0, Why::Heat));
    assert_eq!(mains(Tier::T2).level(&heavy).1, Why::Cost);
    assert!(mains(Tier::T3).raises() && !Budget::new(Tier::T3, true, 60).raises());
}
