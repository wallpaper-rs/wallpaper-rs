use super::{act, at, scene};
use serde_json::{Value, json};

fn stop(p: &mut wprs_editor::Project, stop: Value) -> anyhow::Result<()> {
    act(p, json!({"layer": {"figure": {"stop": {"layer": "g", "stop": stop}}}}))
}

#[test]
fn stops_are_added_moved_recoloured_and_removed_in_order() {
    let root = tempfile::tempdir().unwrap();
    let layer = json!({"id": "g", "type": "shape", "shape": {"rect": {"size": [100, 100]}}});
    let mut p = scene(&root, json!([layer]));
    assert!(stop(&mut p, json!({"add": {"at": 0.5}})).is_err());
    stop(&mut p, json!({"make": {"radial": true}})).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/gradient/radial"), json!(true));
    stop(&mut p, json!({"add": {"at": 0.4}})).unwrap();
    assert_eq!(
        at(&p, "/scene/layers/0/gradient/stops"),
        json!([[0.0, "#ffffff"], [0.4, "#000000"], [1.0, "#000000"]])
    );
    stop(&mut p, json!({"color": {"index": 1, "color": "#00ff00"}})).unwrap();
    stop(&mut p, json!({"move": {"index": 1, "at": 1.5}})).unwrap();
    assert_eq!(
        at(&p, "/scene/layers/0/gradient/stops"),
        json!([[0.0, "#ffffff"], [1.0, "#00ff00"], [1.0, "#000000"]])
    );
    stop(&mut p, json!({"add": {"at": 0.25, "color": "#112233"}})).unwrap();
    assert!(stop(&mut p, json!({"add": {"at": 0.75}})).is_err());
    stop(&mut p, json!({"remove": {"index": 0}})).unwrap();
    stop(&mut p, json!({"remove": {"index": 0}})).unwrap();
    assert!(stop(&mut p, json!({"remove": {"index": 0}})).is_err());
    assert!(stop(&mut p, json!({"move": {"index": 9, "at": 0.5}})).is_err());
    assert!(stop(&mut p, json!({"color": {"index": 0, "color": "nonsense"}})).is_err());
    stop(&mut p, json!("clear")).unwrap();
    assert_eq!(at(&p, "/scene/layers/0/gradient"), Value::Null);
}
