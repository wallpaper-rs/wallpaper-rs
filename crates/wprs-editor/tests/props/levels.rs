use super::*;
use wprs_editor::levels::effective;

fn levels(doc: &Value, id: &str) -> Vec<Field> {
    fields(doc, &format!("/manifest/properties/{id}"))
        .into_iter()
        .filter(|f| f.pointer.contains("/quality"))
        .collect()
}

#[test]
fn a_property_has_a_control_per_quality_level_with_its_own_widget() {
    let mut doc = sample();
    let speed = levels(&doc, "speed");
    let keys: Vec<&str> = speed.iter().map(|f| f.key.as_str()).collect();
    assert_eq!(keys, ["quality", "q0", "q1", "q2", "q3"]);
    assert_eq!(speed[0].widget, "group");
    let q1 = &speed[2];
    assert_eq!((q1.widget.as_str(), q1.range.clone(), q1.unit.as_str()), ("number", json!([0.5, 4.0]), "x"));
    assert_eq!(
        (q1.pointer.as_str(), q1.set, q1.default.clone()),
        ("/manifest/properties/speed/quality/q1", false, json!(1.0))
    );
    let mood = levels(&doc, "mood");
    assert_eq!(
        (mood[1].widget.as_str(), mood[1].options.clone()),
        ("ref:option", vec![json!("calm"), json!("storm")])
    );
    assert_eq!(mood[1].names, ["Calm", "Storm"]);
    assert!(
        levels(&doc, "tint").is_empty() && levels(&doc, "go").is_empty(),
        "colors and buttons do not change"
    );
    run(&mut doc, json!({"set": {"pointer": "/manifest/properties/speed/quality/q1", "value": 2}}));
    run(&mut doc, json!({"set": {"pointer": "/manifest/properties/mood/quality/q0", "value": "calm"}}));
    let set = levels(&doc, "speed");
    assert_eq!((set[2].set, set[2].value.clone()), (true, json!(2)));
    assert!(errors(&doc).is_empty(), "{:?}", errors(&doc));
    let mut bad = doc.clone();
    run(&mut bad, json!({"set": {"pointer": "/manifest/properties/mood/quality/q2", "value": "snow"}}));
    assert!(!errors(&bad).is_empty(), "a choice the property does not offer is a problem");
    run(&mut doc, json!({"remove": {"pointer": "/manifest/properties/speed/quality/q1"}}));
    assert!(!levels(&doc, "speed")[2].set);
}

#[test]
fn the_value_each_level_keeps_is_the_one_the_runtime_uses() {
    let mut doc = sample();
    run(&mut doc, json!({"set": {"pointer": "/manifest/properties/speed/quality/q1", "value": 0.5}}));
    run(&mut doc, json!({"set": {"pointer": "/manifest/properties/mood/quality/q2", "value": "storm"}}));
    let speed = effective(&doc, "speed");
    let values: Vec<Value> = speed["levels"].as_array().unwrap().iter().map(|l| l["value"].clone()).collect();
    assert_eq!(
        values,
        [json!(0.5), json!(0.5), json!(1.0), json!(1.0)],
        "a cap holds for the levels below it"
    );
    assert_eq!(speed["levels"][1]["own"], true);
    assert_eq!(speed["levels"][0]["own"], false);
    let mood = effective(&doc, "mood");
    let values: Vec<Value> = mood["levels"].as_array().unwrap().iter().map(|l| l["value"].clone()).collect();
    assert_eq!(values, [json!("storm"), json!("storm"), json!("storm"), json!("calm")]);
    assert_eq!(effective(&doc, "nope"), Value::Null);
}
