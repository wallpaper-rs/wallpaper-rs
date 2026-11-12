use super::{card, scene};
use wprs_editor::Project;
use serde_json::json;
use wprs_editor::review::report;
use wprs_editor::seats::seats;

fn broken(root: &tempfile::TempDir, bad: &str) -> Project {
    let mut p = scene(root, json!([]));
    p.doc["scene"]["layers"] = trio(bad);
    p
}

fn trio(bad: &str) -> serde_json::Value {
    json!([
        card("first", json!({"position": [200, 200]})),
        {"id": "middle", "type": "solid", "color": bad, "size": [200, 100], "transform": {"position": [500, 500]}},
        card("last", json!({"position": [800, 800]}))
    ])
}

#[test]
fn one_layer_with_a_bad_color_costs_only_itself() {
    for bad in ["#fff", "red", "#12345", ""] {
        let root = tempfile::tempdir().unwrap();
        let p = broken(&root, bad);
        let found = seats(&p.dir, &p.doc, 0.0);
        let ids: Vec<&str> = found.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(ids, ["first", "middle", "last"], "{bad}");
        let middle = &found[1];
        assert!(super::near(middle.pivot(), [500.0, 500.0], 0.01), "{bad}");
        assert_eq!(middle.size, [200.0, 100.0], "{bad}");
    }
}

#[test]
fn the_problem_list_names_the_layer_and_field_of_a_bad_color() {
    let root = tempfile::tempdir().unwrap();
    let p = broken(&root, "#fff");
    let said = report(&p.dir, &p.doc);
    let found = said["findings"].as_array().unwrap();
    let hit = found.iter().find(|f| f["pointer"] == "/scene/layers/1/color").unwrap_or_else(|| panic!("{found:?}"));
    assert_eq!(hit["level"], "error");
    assert!(hit["message"].as_str().unwrap().contains("#rrggbb"), "{hit}");
}
