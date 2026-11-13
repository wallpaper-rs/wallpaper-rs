use super::depth::setup;
use crate::reliefwork::{build, frame, regions, survey};
use serde_json::Value;
use std::sync::atomic::AtomicBool;
use wprs_core::Fill;
use wprs_core::edit::{Depth, Join, Shift};

fn zones(text: &str) -> Vec<Value> {
    serde_json::from_str::<Value>(text).unwrap()["zones"].as_array().unwrap().clone()
}

#[test]
fn the_regions_are_painted_listed_and_followed_by_joins_and_moves() {
    let (_dir, places, target) = setup(Depth::default());
    let (image, text) = regions(&places, &target).unwrap();
    assert_eq!(image.dimensions(), (64, 48));
    let found = zones(&text);
    assert_eq!(found.len(), 2);
    assert!(found[0]["depth"].as_f64() > found[1]["depth"].as_f64() && found[0]["order"] == 1);
    let grid: Value = serde_json::from_str(&text).unwrap();
    let cells = grid["grid"]["cells"].as_array().unwrap().len() as u64;
    assert_eq!(cells, grid["grid"]["w"].as_u64().unwrap() * grid["grid"]["h"].as_u64().unwrap());
    let join = Join { a: [0.4, 0.45], b: [0.05, 0.05] };
    let (_dir, places, target) = setup(Depth { joins: vec![join], ..Depth::default() });
    assert_eq!(zones(&regions(&places, &target).unwrap().1).len(), 1);
    let nearer = Shift { at: [0.05, 0.05], by: 0.7 };
    let (_dir, places, target) = setup(Depth { moves: vec![nearer], ..Depth::default() });
    assert_eq!(
        zones(&regions(&places, &target).unwrap().1)[0]["depth"].as_f64().map(|d| d > 0.9),
        Some(true)
    );
}

#[test]
fn the_preview_waits_for_the_layers_then_moves_with_the_pointer_and_the_survey_gives_the_limit() {
    let (_dir, places, target) = setup(Depth::default());
    assert!(frame(&places, &target, [1.0, 0.0]).unwrap().is_none());
    let before: Value = serde_json::from_str(&survey(&places, &target).unwrap()).unwrap();
    assert!(before["reach"].is_null() && before["moves"].is_null());
    let stop = AtomicBool::new(false);
    build(&places, &target, Some(Fill::Simple), &stop, &mut |_| {}).unwrap();
    let after: Value = serde_json::from_str(&survey(&places, &target).unwrap()).unwrap();
    assert!(after["reach"].as_f64().is_some_and(|r| r > 0.0) && after["moves"].is_boolean(), "{after}");
    let (left, right) = (frame(&places, &target, [-1.0, 0.0]), frame(&places, &target, [1.0, 0.0]));
    let (left, right) = (left.unwrap().unwrap(), right.unwrap().unwrap());
    assert_eq!(left.dimensions(), (64, 48));
    assert!(left != right, "the picture does not move with the pointer");
}
