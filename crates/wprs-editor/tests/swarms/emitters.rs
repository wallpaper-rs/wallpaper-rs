use super::*;

#[test]
fn emitters_are_added_at_the_next_index_and_removed_by_index() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root);
    let count =
        |p: &Project| p.doc.pointer("/scene/particles/flakes/emitters").unwrap().as_array().unwrap().len();
    let before = count(&p);
    let circle = json!({ "circle": { "center": [0, 0], "radius": 10 } });
    edit(&mut p, "shape", json!({ "emitter": before, "shape": circle })).unwrap();
    assert_eq!(count(&p), before + 1);
    edit(&mut p, "shape", json!({ "emitter": before, "remove": true })).unwrap();
    assert_eq!(count(&p), before);
    assert!(edit(&mut p, "shape", json!({ "emitter": before + 3, "remove": true })).is_err());
    for _ in 1..before {
        edit(&mut p, "shape", json!({ "emitter": 0, "remove": true })).unwrap();
    }
    assert!(
        edit(&mut p, "shape", json!({ "emitter": 0, "remove": true })).is_err(),
        "the last emitter stays"
    );
}
