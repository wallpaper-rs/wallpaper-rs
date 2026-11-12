use super::*;

#[test]
fn a_force_follows_a_control_point_the_pointer_or_a_typed_fixed_point() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let n = system(&p)["forces"].as_array().map_or(0, Vec::len);
    let ring =
        |target: Value| json!({ "keep_distance": { "target": target, "distance": 80, "strength": 3 } });
    let pull = |target: Value| json!({ "attractor": { "target": target, "strength": 50, "radius": 200 } });
    edit(&mut p, "force", json!({ "force": ring(json!("p0")) })).unwrap();
    edit(&mut p, "force", json!({ "force": pull(json!("cursor")) })).unwrap();
    for target in [json!([412.5, 77.2]), json!("p0"), json!("cursor"), json!([0, 0])] {
        edit(&mut p, "force", json!({ "index": n, "force": ring(target.clone()) })).unwrap();
        edit(&mut p, "force", json!({ "index": n + 1, "force": pull(target.clone()) })).unwrap();
        assert_eq!(system(&p)["forces"][n], ring(target.clone()));
        let shown = handles(&p);
        let (keep, pulled) = (&shown["keeps"][0], shown["pulls"].as_array().unwrap().last().unwrap().clone());
        assert_eq!((keep["target"].clone(), pulled["ref"].clone()), (target.clone(), target.clone()));
        assert_eq!(keep["at"].is_array(), target != json!("cursor"), "{keep}");
    }
    let typed = handles(&p)["keeps"][0]["at"].clone();
    assert_eq!(typed, json!([0.0, 0.0]));
    for bad in [json!([1, "x"]), json!([1, 2, 3]), json!("p9"), json!("elsewhere")] {
        let before = p.doc.clone();
        assert!(edit(&mut p, "force", json!({ "index": n, "force": ring(bad.clone()) })).is_err(), "{bad}");
        assert_eq!(p.doc, before, "{bad} changed nothing");
    }
}
