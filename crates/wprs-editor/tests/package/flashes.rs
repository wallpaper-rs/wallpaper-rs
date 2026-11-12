use crate::{act, ask, project, warned};
use image::{Rgba, RgbaImage};
use serde_json::json;
use wprs_editor::flashes::{Report, grid, judge, remember};

const CELLS: usize = 32 * 18;

fn frames(fps: u32, seconds: u32, every: u32) -> Vec<(f64, Vec<f32>)> {
    (0..fps * seconds)
        .map(|n| {
            (
                f64::from(n) / f64::from(fps),
                vec![if (n / every).is_multiple_of(2) { 0.9 } else { 0.05 }; CELLS],
            )
        })
        .collect()
}

#[test]
fn fast_alternation_flashes_and_slow_pulses_do_not() {
    let fast = judge(frames(15, 4, 1), (0.0, 4.0, 15));
    assert!(fast.flashing && fast.rate >= 3.0 && fast.at.is_some(), "{fast:?}");
    let slow = judge(frames(15, 6, 8), (0.0, 6.0, 15));
    assert!(!slow.flashing && slow.at.is_none(), "{slow:?}");
}

#[test]
fn the_grid_averages_the_brightness_of_each_cell() {
    let mut image = RgbaImage::from_pixel(64, 36, Rgba([0, 0, 0, 255]));
    for y in 0..36 {
        for x in 32..64 {
            image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }
    let cells = grid(&image);
    assert_eq!(cells.len(), CELLS);
    assert!(cells[0] < 0.01 && cells[31] > 0.99 && cells[CELLS - 1] > 0.99, "{:?}", &cells[..4]);
}

#[test]
fn a_measured_flash_suggests_the_flag_until_the_scene_changes_or_the_flag_is_set() {
    let root = tempfile::tempdir().unwrap();
    let mut p = project(&root, "flashes");
    let report = Report { flashing: true, rate: 7.0, at: Some(1.5), from: 0.0, seconds: 8.0, fps: 15 };
    assert!(ask(&p, json!({ "flashes": {} })).is_null());
    remember(&p.dir, &p.doc, &report);
    assert_eq!(ask(&p, json!({ "flashes": {} }))["at"], json!(1.5));
    assert_eq!(warned(&p, "/manifest/photosensitive").len(), 1);
    act(&mut p, json!({ "set": { "pointer": "/manifest/photosensitive", "value": true } })).unwrap();
    assert!(warned(&p, "/manifest/photosensitive").is_empty());
    act(&mut p, json!({ "undo": {} })).unwrap();
    remember(&p.dir, &p.doc, &report);
    act(&mut p, json!({ "add_layer": { "kind": "solid", "id": "box" } })).unwrap();
    assert!(warned(&p, "/manifest/photosensitive").is_empty(), "the measure is stale once the scene changes");
    assert!(ask(&p, json!({ "flashes": {} })).is_null());
}
