use image::{Luma, Rgb, RgbImage};
use std::sync::atomic::AtomicBool;
use wprs_core::Fill;
use wprs_core::edit::{Depth, Join, Shift, Touch};
use wprs_depth::{Layers, Map, Preview, Settings, chart, layers};

const W: u32 = 160;
const H: u32 = 100;

fn boxes(x: u32, y: u32) -> f32 {
    match (x, y) {
        (20..60, 20..80) => 0.9,
        (90..140, 30..70) => 0.55,
        _ => 0.1,
    }
}

fn scene() -> (RgbImage, Map) {
    let colour = |x, y| match boxes(x, y) {
        0.9 => [230, 20, 20],
        0.55 => [20, 20, 230],
        _ => [20, 160, 40],
    };
    let image = RgbImage::from_fn(W, H, |x, y| Rgb(colour(x, y)));
    (image, Map::from_fn(W, H, |x, y| Luma([(boxes(x, y) * 65535.0) as u16])))
}

fn built(depth: &Depth, shift: f32) -> Layers {
    let (image, map) = scene();
    let set = Settings { fill: Fill::Simple, count: 5, shift, edge: 1920 };
    layers(&image, &map, (depth, &set), None, &mut |_| {}, &AtomicBool::new(false)).unwrap()
}

#[test]
fn the_chart_lists_the_regions_from_the_nearest_with_a_point_inside_each() {
    let found = chart(&scene().1, &Depth::default());
    assert_eq!(found.zones.len(), 3);
    assert_eq!(found.zones.iter().map(|z| z.order).collect::<Vec<_>>(), [1, 2, 3]);
    assert!(found.zones.windows(2).all(|z| z[0].depth > z[1].depth), "{:?}", found.zones);
    assert!((found.zones.iter().map(|z| z.share).sum::<f32>() - 1.0).abs() < 1e-3);
    assert_eq!((found.zones[0].colour.as_str(), found.zones[2].colour.as_str()), ("#ef4444", "#4287f5"));
    for zone in &found.zones {
        let (x, y) = ((zone.at[0] * W as f32) as usize, (zone.at[1] * H as f32) as usize);
        assert_eq!(found.order[y * W as usize + x] as usize, zone.order - 1, "{zone:?}");
    }
    assert_eq!(found.order[50 * W as usize + 40], 0);
    let (gw, gh, cells) = found.grid();
    assert_eq!(cells.len(), gw * gh);
    assert!(gw.max(gh) <= 128 && cells.iter().all(|c| *c < 3));
}

#[test]
fn the_painted_regions_run_from_blue_far_to_red_near_with_a_line_at_each_border() {
    let image = chart(&scene().1, &Depth::default()).painted();
    let (near, far) = (image.get_pixel(30, 50).0, image.get_pixel(5, 5).0);
    assert!(near[0] > near[2] && far[2] > far[0], "{near:?} {far:?}");
    assert_eq!(image.get_pixel(59, 50)[3], 255);
    assert!(image.get_pixel(30, 50)[3] < 255);
}

#[test]
fn joining_two_regions_and_splitting_one_change_the_chart() {
    let join = Join { a: [0.25, 0.5], b: [0.7, 0.5] };
    let joined = chart(&scene().1, &Depth { joins: vec![join], ..Depth::default() });
    assert_eq!(joined.zones.len(), 2);
    let line = Touch { points: vec![[0.0, 0.5], [1.0, 0.5]], radius: 0.01, falloff: 0.0, by: 1.0 };
    let cut = chart(&scene().1, &Depth { splits: vec![line], ..Depth::default() });
    assert!(cut.zones.len() > 3, "{:?}", cut.zones);
}

#[test]
fn moving_a_region_changes_its_place_in_the_order_and_in_the_layers() {
    let farther = Shift { at: [0.25, 0.5], by: -0.6 };
    let moved = Depth { moves: vec![farther], ..Depth::default() };
    let found = chart(&scene().1, &moved);
    assert!((found.zones[0].depth - 0.55).abs() < 0.02, "{:?}", found.zones);
    assert!((found.zones[1].depth - 0.3).abs() < 0.02, "{:?}", found.zones);
    let plain = built(&Depth::default(), 0.02);
    let after = built(&moved, 0.02);
    assert!(plain.info.depths.iter().any(|d| *d > 0.85) && after.info.depths.iter().all(|d| *d < 0.6));
    let key = |d: &Depth| serde_json::to_string(d).unwrap();
    assert_ne!(key(&moved), key(&Depth::default()));
}

#[test]
fn the_preview_moves_near_things_more_than_far_ones_and_rests_at_the_middle() {
    let made = built(&Depth::default(), 0.02);
    let preview = Preview::new(&made, 80);
    assert!(preview.reach().moves(W));
    let (strength, focus, overscan) = (0.02, 0.3, 1.04);
    let rest = preview.frame([0.0, 0.0], (strength, focus, overscan));
    let left = preview.frame([-1.0, 0.0], (strength, focus, overscan));
    let right = preview.frame([1.0, 0.0], (strength, focus, overscan));
    assert_eq!(rest.dimensions(), (80, 50));
    let red =
        |image: &image::RgbaImage| (0..80).filter(|x| image.get_pixel(*x, 25)[0] > 150).collect::<Vec<u32>>();
    let (a, b) = (red(&left), red(&right));
    assert!(!a.is_empty() && !b.is_empty());
    assert!(a[0] != b[0], "the near box does not move: {a:?} {b:?}");
    assert!(left != right && rest != left);
}

fn notes(depth: &Depth) -> Vec<(&'static str, usize, &'static str)> {
    chart(&scene().1, depth).notes.iter().map(|n| (n.kind, n.index, n.why)).collect()
}

#[test]
fn marks_that_no_longer_apply_are_reported_and_never_break_the_layers() {
    assert!(notes(&Depth::default()).is_empty());
    let apart = Join { a: [0.25, 0.5], b: [0.7, 0.5] };
    let inside = Join { a: [0.2, 0.3], b: [0.3, 0.6] };
    let depth = Depth { joins: vec![apart.clone(), inside], ..Depth::default() };
    assert_eq!(notes(&depth), [("join", 2, "same_region")]);
    let again = Depth { joins: vec![apart.clone(), apart], ..Depth::default() };
    assert_eq!(notes(&again), [("join", 2, "same_region")]);
    assert_eq!(built(&again, 0.02).images.len(), 2);
}

#[test]
fn a_split_that_crosses_no_region_is_reported() {
    let stroke = |y: f32| Touch { points: vec![[0.0, y], [1.0, y]], radius: 0.01, falloff: 0.0, by: 1.0 };
    let off = Touch { points: vec![[0.02, 0.02], [0.05, 0.02]], radius: 0.004, falloff: 0.0, by: 1.0 };
    let depth = Depth { splits: vec![stroke(0.5), off], ..Depth::default() };
    assert_eq!(notes(&depth), [("split", 2, "crosses_nothing")]);
    assert!(built(&depth, 0.02).images.len() >= 3);
}

#[test]
fn a_move_into_a_region_already_moved_or_joined_is_not_applied_and_reported() {
    let first = Shift { at: [0.25, 0.5], by: -0.6 };
    let again = Shift { at: [0.3, 0.4], by: -0.2 };
    let depth = Depth { moves: vec![first.clone(), again], ..Depth::default() };
    assert_eq!(notes(&depth), [("move", 2, "same_region")]);
    let found = chart(&scene().1, &depth);
    assert!((found.zones[0].depth - 0.55).abs() < 0.02 && (found.zones[1].depth - 0.3).abs() < 0.02);
    let join = Join { a: [0.25, 0.5], b: [0.7, 0.5] };
    let second = Shift { at: [0.7, 0.5], by: 0.1 };
    let joined = Depth { joins: vec![join], moves: vec![first, second], ..Depth::default() };
    assert_eq!(notes(&joined), [("move", 2, "same_region")]);
    assert_eq!(built(&joined, 0.02).images.len(), 2);
}

#[test]
fn the_zones_know_their_bounds() {
    let found = chart(&scene().1, &Depth::default());
    let b = found.zones[0].bounds;
    assert!((b[0] - 0.125).abs() < 0.02 && (b[2] - 0.375).abs() < 0.02 && (b[1] - 0.2).abs() < 0.02, "{b:?}");
}
