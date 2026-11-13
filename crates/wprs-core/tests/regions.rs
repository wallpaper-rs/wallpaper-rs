use serde_json::{Value, json};
use wprs_core::edit::{ACTIONS, Edit, SPLIT_RADIUS};

fn act(edit: &mut Edit, action: Value) -> Result<(), String> {
    edit.act(serde_json::from_value(action).unwrap()).map_err(|e| e.to_string())
}

#[test]
fn the_region_actions_are_listed() {
    for name in ["depth_join", "depth_split", "depth_move"] {
        assert!(ACTIONS.contains(&name), "{name}");
    }
}

#[test]
fn a_join_pairs_the_first_point_with_each_other_and_clear_removes_them() {
    let mut e = Edit::default();
    act(&mut e, json!({"depth_join": {"points": "0.1,0.2;0.8,0.5;0.5,0.9"}})).unwrap();
    let joins = e.depth.clone().unwrap().joins;
    assert_eq!((joins.len(), joins[1].a, joins[1].b), (2, [0.1, 0.2], [0.5, 0.9]));
    assert!(act(&mut e, json!({"depth_join": {"points": "0.1,0.2"}})).unwrap_err().contains("2 to 8"));
    assert!(act(&mut e, json!({"depth_join": {}})).unwrap_err().contains("two or more"));
    assert!(act(&mut e, json!({"depth_join": {"points": "0.1,0.2;1.4,0.5"}})).is_err());
    assert_eq!(e.depth.as_ref().map(|d| d.joins.len()), Some(2));
    act(&mut e, json!({"depth_join": {"points": "0.3,0.3;0.6,0.6", "clear": true}})).unwrap();
    assert_eq!(e.depth.as_ref().map(|d| d.joins.len()), Some(1));
    act(&mut e, json!({"depth_join": {"clear": true}})).unwrap();
    assert!(e.depth.unwrap().joins.is_empty());
}

#[test]
fn a_split_is_a_thin_hard_stroke() {
    let mut e = Edit::default();
    act(&mut e, json!({"depth_split": {"points": "0.0,0.5;1.0,0.5"}})).unwrap();
    let stroke = e.depth.clone().unwrap().splits[0].clone();
    assert_eq!((stroke.radius, stroke.falloff, stroke.points.len()), (SPLIT_RADIUS, 0.0, 2));
    act(&mut e, json!({"depth_split": {"points": "0.5,0.1;0.5,0.9", "radius": 0.01}})).unwrap();
    assert_eq!(e.depth.as_ref().map(|d| (d.splits.len(), d.splits[1].radius)), Some((2, 0.01)));
    assert!(act(&mut e, json!({"depth_split": {}})).unwrap_err().contains("give a stroke"));
    assert!(act(&mut e, json!({"depth_split": {"points": "0.5,2.0"}})).is_err());
    act(&mut e, json!({"depth_split": {"clear": true}})).unwrap();
    assert!(e.depth.as_ref().unwrap().splits.is_empty());
}

#[test]
fn a_move_names_a_region_by_a_point_and_how_far() {
    let mut e = Edit::default();
    act(&mut e, json!({"depth_move": {"at": "0.25,0.5", "by": 0.2}})).unwrap();
    act(&mut e, json!({"depth_move": {"at": "0.7,0.5", "by": -0.1}})).unwrap();
    let moves = e.depth.clone().unwrap().moves;
    assert_eq!((moves.len(), moves[1].by, moves[0].at), (2, -0.1, [0.25, 0.5]));
    assert!(act(&mut e, json!({"depth_move": {"at": "0.25,0.5"}})).unwrap_err().contains("by"));
    assert!(act(&mut e, json!({"depth_move": {"by": 0.1}})).unwrap_err().contains("point of the region"));
    assert!(act(&mut e, json!({"depth_move": {"at": "0.25,0.5", "by": 1.5}})).is_err());
    assert_eq!(e.depth.as_ref().map(|d| d.moves.len()), Some(2));
    assert!(Edit::parse(&e.to_json()).unwrap() == e);
    act(&mut e, json!({"depth_move": {"clear": true}})).unwrap();
    assert!(e.depth.unwrap().moves.is_empty());
}
