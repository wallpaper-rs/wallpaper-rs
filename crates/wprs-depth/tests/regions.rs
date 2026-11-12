use image::{Luma, Rgb, RgbImage};
use std::sync::atomic::AtomicBool;
use wprs_core::Fill;
use wprs_core::edit::{Depth, Join, Touch};
use wprs_depth::{BEND, Layers, Map, Settings, field_size, layers, regions};

const W: usize = 160;
const H: usize = 100;

fn picture(depth: impl Fn(usize, usize) -> f32, colour: impl Fn(usize, usize) -> [u8; 3]) -> (RgbImage, Map) {
    let image = RgbImage::from_fn(W as u32, H as u32, |x, y| Rgb(colour(x as usize, y as usize)));
    let map =
        Map::from_fn(W as u32, H as u32, |x, y| Luma([(depth(x as usize, y as usize) * 65535.0) as u16]));
    (image, map)
}

fn plane(depth: impl Fn(usize, usize) -> f32) -> Vec<f32> {
    (0..W * H).map(|k| depth(k % W, k / W)).collect()
}

fn built(scene: &(RgbImage, Map), depth: &Depth, shift: f32) -> Layers {
    let set = Settings { fill: Fill::Simple, count: 5, shift, edge: 1920 };
    layers(&scene.0, &scene.1, (depth, &set), None, &mut |_| {}, &AtomicBool::new(false)).unwrap()
}

fn boxes(x: usize, y: usize) -> f32 {
    match (x, y) {
        (20..60, 20..80) => 0.9,
        (90..140, 30..70) => 0.55,
        _ => 0.1,
    }
}

fn hue(x: usize, y: usize) -> [u8; 3] {
    match boxes(x, y) {
        0.9 => [230, 20, 20],
        0.55 => [20, 20, 230],
        _ => [20, 160, 40],
    }
}

fn floor(_: usize, y: usize) -> f32 {
    0.1 + 0.8 * y as f32 / (H - 1) as f32
}

fn owner(made: &Layers, x: u32, y: u32) -> usize {
    (0..made.images.len()).rev().find(|i| made.images[*i].get_pixel(x, y)[3] == 255).unwrap_or(0)
}

#[test]
fn shapes_at_their_depths_are_whole_regions_on_ordered_layers() {
    let found = regions(&plane(boxes), (W, H), &Depth::default());
    assert_eq!(found.means.len(), 3, "{:?}", found.means);
    let made = built(&picture(boxes, hue), &Depth::default(), 0.02);
    assert_eq!(made.images.len(), 3);
    assert!(made.info.depths.windows(2).all(|d| d[0] < d[1]), "{:?}", made.info.depths);
    assert_eq!((owner(&made, 40, 50), owner(&made, 115, 50), owner(&made, 5, 5)), (2, 1, 0));
    assert!(made.images[0].pixels().all(|p| p[3] == 255), "the far layer is complete");
}

#[test]
fn a_continuous_floor_stays_one_region_whose_depth_moves_gently() {
    let found = regions(&plane(floor), (W, H), &Depth::default());
    assert_eq!(found.means.len(), 1);
    let shift = 0.02;
    let made = built(&picture(floor, |x, _| [(x % 256) as u8, 90, 90]), &Depth::default(), shift);
    let (fw, fh) = field_size((W, H));
    let field = &made.fields[0];
    let at = |x: usize, y: usize| f32::from(field.get_pixel(x as u32, y as u32)[0]) / 65535.0;
    assert!(at(fw / 2, fh - 1) - at(fw / 2, 0) > 0.3, "the floor is not a rigid card");
    let steep = (0..fh - 1).flat_map(|y| (0..fw - 1).map(move |x| (x, y)));
    let most = steep
        .map(|(x, y)| (at(x + 1, y) - at(x, y)).abs().max((at(x, y + 1) - at(x, y)).abs()))
        .fold(0.0, f32::max);
    let scale = most / wprs_depth::FIELD as f32 * shift * W as f32;
    assert!(scale <= BEND * 1.05, "the local scale moves by {scale}");
    assert!(made.info.reach.high - made.info.reach.low > 0.3 && made.info.reach.moves(1920));
}

#[test]
fn a_thin_strand_stays_with_the_object_it_grows_from() {
    let strand = |x: usize, y: usize| match (x, y) {
        (60..100, 50..90) => 0.9,
        (79..81, 10..50) => 0.88,
        _ => 0.1,
    };
    let found = regions(&plane(strand), (W, H), &Depth::default());
    let (blob, tip) = (found.label[70 * W + 80], found.label[20 * W + 80]);
    assert_eq!(blob, tip, "the strand is cut off its object");
    assert_ne!(blob, found.label[20 * W + 20]);
}

#[test]
fn a_hole_in_an_object_shows_the_filled_background() {
    let ring = |x: usize, y: usize| {
        let d = (x as f32 - 80.0).hypot(y as f32 - 50.0);
        if (15.0..40.0).contains(&d) { 0.9 } else { 0.1 }
    };
    let scene = picture(ring, |x, y| if ring(x, y) > 0.5 { [230, 20, 20] } else { [20, 160, 40] });
    let made = built(&scene, &Depth::default(), 0.03);
    assert_eq!(made.images.len(), 2);
    assert_eq!((owner(&made, 80, 50), owner(&made, 80, 22)), (0, 1));
    let behind = made.images[0].get_pixel(80, 30).0;
    assert!(behind[1] > 120 && behind[0] < 80, "the ring is copied into the fill: {behind:?}");
}

#[test]
fn small_fragments_join_the_neighbour_they_touch_most() {
    let specks = |x: usize, y: usize| if x % 40 == 7 && y % 30 == 7 { 0.95 } else { floor(x, y) };
    let found = regions(&plane(specks), (W, H), &Depth::default());
    assert_eq!(found.means.len(), 1, "{:?}", found.sizes);
}

#[test]
fn joins_and_splits_of_the_author_change_the_regions() {
    let two = Depth { joins: vec![Join { a: [0.25, 0.5], b: [0.7, 0.5] }], ..Depth::default() };
    assert_eq!(regions(&plane(boxes), (W, H), &two).means.len(), 2);
    let line = Touch { points: vec![[0.0, 0.5], [1.0, 0.5]], radius: 0.01, falloff: 0.0, by: 1.0 };
    let cut = Depth { splits: vec![line], ..Depth::default() };
    let found = regions(&plane(floor), (W, H), &cut);
    assert_eq!(found.means.len(), 2);
    assert_ne!(found.label[10 * W + 50], found.label[90 * W + 50]);
}

#[test]
fn a_protected_band_cuts_the_movement_of_the_picture() {
    let scene = picture(boxes, hue);
    let free = built(&scene, &Depth::default(), 0.03);
    assert!((free.info.reach.shift - 0.03).abs() < 1e-6, "{:?}", free.info.reach);
    let keep = Touch { points: vec![[0.25, 0.5]], radius: 0.3, falloff: 0.0, by: -1.0 };
    let kept = built(&scene, &Depth { fills: vec![keep], ..Depth::default() }, 0.03);
    assert!(kept.info.reach.shift < 0.03 * 0.5, "{:?}", kept.info.reach);
}

#[test]
fn the_near_edge_keeps_the_pictures_own_blend_and_no_fringe() {
    let soft = |x: usize, y: usize| if (40..100).contains(&x) && (20..80).contains(&y) { 0.9 } else { 0.1 };
    let colour = |x: usize, y: usize| match (x, soft(x, y) > 0.5) {
        (40, true) => [125, 90, 30],
        (_, true) => [230, 20, 20],
        _ => [20, 160, 40],
    };
    let made = built(&picture(soft, colour), &Depth::default(), 0.03);
    let edge = made.images[1].get_pixel(40, 50).0;
    assert!((100..160).contains(&edge[3]), "the blended pixel is half covered: {edge:?}");
    assert!(edge[0] > 180 && edge[1] < 70, "the edge keeps the object's colour: {edge:?}");
    let outside = made.images[1].get_pixel(37, 50).0[3];
    assert!(outside < 30, "no fringe of the background moves with the object: {outside}");
    for x in 38..46 {
        let back = made.images[0].get_pixel(x, 50).0;
        assert!(back[1] > 120 && back[0] < 90, "no halo of the object in the fill at {x}: {back:?}");
    }
}
