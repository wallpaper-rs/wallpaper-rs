use super::{act, project, refused};
use serde_json::json;

#[test]
fn unreadable_keyframe_fields_say_which_field_and_the_shape_it_needs() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let opacity = "/scene/layers/0/opacity";
    act(&mut p, json!({"keyframe": {"pointer": opacity, "time": 0, "value": 0}})).unwrap();
    let eased =
        refused(&mut p, json!({"keyframe": {"pointer": opacity, "time": 1, "value": 1, "ease": "bouncy"}}));
    assert!(eased.contains("ease cannot take \"bouncy\"") && eased.contains("in_out_sine"), "{eased}");
    assert!(eased.contains("[x1, y1, x2, y2]"), "{eased}");
    let short =
        refused(&mut p, json!({"keyframe": {"pointer": opacity, "time": 1, "value": 1, "ease": [0.1, 0.2]}}));
    assert!(short.contains("ease cannot take"), "{short}");
    let valued = refused(&mut p, json!({"keyframe": {"pointer": opacity, "time": 1, "value": "high"}}));
    assert!(valued.contains("give a number such as 0.5"), "{valued}");
    let based = refused(&mut p, json!({"keyframe": {"pointer": opacity, "time": 0, "base": "#ff0000"}}));
    assert!(based.contains("the base of"), "{based}");
    let lined = refused(&mut p, json!({"keyframe": {"pointer": opacity, "time": 0, "timeline": "intro"}}));
    assert!(lined.contains("no timeline \"intro\""), "{lined}");
    let bent = refused(&mut p, json!({"keyframe": {"pointer": opacity, "time": 0, "out_handle": [-1, 0]}}));
    assert!(bent.contains("Bézier handle reaches"), "{bent}");
    let planned: wprs_editor::actions::Action =
        serde_json::from_value(json!({"keyframe": {"pointer": opacity, "time": 1, "ease": "bouncy"}}))
            .unwrap();
    let dry = wprs_editor::actions::plan(&p.doc, &planned).unwrap_err().to_string();
    assert_eq!(dry, eased, "the plan checks like the project");
}
