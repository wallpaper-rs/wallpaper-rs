use image::{ImageBuffer, Luma, Rgba, RgbaImage};
use wprs_core::Fit;
use wprs_render::{DepthMap, Layer, Relief, Renderer};

const W: u32 = 64;
const H: u32 = 32;

fn renderer() -> Option<&'static Renderer> {
    match Renderer::shared() {
        Ok(r) => Some(r),
        Err(e) if std::env::var_os("WPRS_REQUIRE_GPU").is_none() => {
            eprintln!("skipping GPU test: {e}");
            None
        }
        Err(e) => panic!("GPU required but unavailable: {e}"),
    }
}

fn ground(x: u32) -> [u8; 3] {
    [(x * 4) as u8, 0, 128]
}

fn near(got: [u8; 3], want: [u8; 3]) -> bool {
    got.iter().zip(want).all(|(a, b)| a.abs_diff(b) <= 2)
}

fn square(x: u32, y: u32) -> bool {
    (24..40).contains(&x) && (8..24).contains(&y)
}

fn patch(x: u32, y: u32) -> bool {
    (8..16).contains(&x) && (4..28).contains(&y)
}

fn back() -> RgbaImage {
    RgbaImage::from_fn(W, H, |x, _| Rgba([ground(x)[0], 0, 128, 255]))
}

fn middle() -> RgbaImage {
    RgbaImage::from_fn(W, H, |x, y| if patch(x, y) { Rgba([255, 0, 0, 255]) } else { Rgba([0, 0, 0, 0]) })
}

fn front() -> RgbaImage {
    RgbaImage::from_fn(
        W,
        H,
        |x, y| if square(x, y) { Rgba([255, 255, 255, 255]) } else { Rgba([0, 0, 0, 0]) },
    )
}

fn flat() -> RgbaImage {
    RgbaImage::from_fn(W, H, |x, y| match (square(x, y), patch(x, y)) {
        (true, _) => Rgba([255; 4]),
        (_, true) => Rgba([255, 0, 0, 255]),
        _ => back().get_pixel(x, 0).to_owned(),
    })
}

fn row(
    r: &Renderer,
    relief: impl FnOnce(&wprs_render::GpuImage) -> (wprs_render::GpuImage, f32),
) -> Vec<[u8; 3]> {
    let image = r.upload_rgba(flat().as_raw(), W, H, W * 4).unwrap();
    let (depth, shift) = relief(&image);
    let relief =
        Relief { depth: &depth, shift: [shift / W as f32, 0.0], focus: 0.0, steps: 16, overscan: 1.0 };
    let layer = Layer {
        image: Some(&image),
        fit: Fit::Stretch,
        background: [0; 3],
        opacity: 1.0,
        uv: None,
        relief: Some(relief),
        flip: [false; 2],
        grade: None,
    };
    let (mut target, mut pixels) = (r.create_target(W, H).unwrap(), Vec::new());
    r.render_layers(&mut target, &[layer], &mut pixels).unwrap();
    let all: Vec<[u8; 3]> = pixels.as_chunks::<4>().0.iter().map(|p| [p[2], p[1], p[0]]).collect();
    all[(H / 2 * W) as usize..((H / 2 + 1) * W) as usize].to_vec()
}

fn level(value: f32) -> DepthMap {
    ImageBuffer::from_pixel(W / 4, H / 4, Luma([(value * 65535.0) as u16]))
}

fn stacked(r: &Renderer, shift: f32) -> Vec<[u8; 3]> {
    let fields = [level(0.0), level(0.5), level(1.0)];
    row(r, |_| (r.upload_layers(&[back(), middle(), front()], &fields, &[0.0, 0.5, 1.0]).unwrap(), shift))
}

fn ramped(r: &Renderer, shift: f32) -> Vec<[u8; 3]> {
    let slope: DepthMap = ImageBuffer::from_fn(W / 4, H / 4, |x, _| Luma([(x * 65535 / (W / 4 - 1)) as u16]));
    let marks = RgbaImage::from_fn(W, H, |x, _| Rgba([if x % 16 == 0 { 255 } else { 0 }, 0, 0, 255]));
    let image = r.upload_rgba(marks.as_raw(), W, H, W * 4).unwrap();
    let deck = r.upload_layers(&[marks], &[slope], &[0.5]).unwrap();
    let relief = Relief { depth: &deck, shift: [shift / W as f32, 0.0], focus: 0.0, steps: 1, overscan: 1.0 };
    let layer = Layer {
        image: Some(&image),
        fit: Fit::Stretch,
        background: [0; 3],
        opacity: 1.0,
        uv: None,
        relief: Some(relief),
        flip: [false; 2],
        grade: None,
    };
    let (mut target, mut pixels) = (r.create_target(W, H).unwrap(), Vec::new());
    r.render_layers(&mut target, &[layer], &mut pixels).unwrap();
    let all: Vec<[u8; 3]> = pixels.as_chunks::<4>().0.iter().map(|p| [p[2], p[1], p[0]]).collect();
    all[(H / 2 * W) as usize..((H / 2 + 1) * W) as usize].to_vec()
}

#[test]
fn what_a_near_object_uncovers_shows_the_filled_background() {
    let Some(r) = renderer() else { return };
    let moved = stacked(r, 8.0);
    for x in [25, 27, 29, 31] {
        assert!(near(moved[x], ground(x as u32)), "x {x}: {:?} wants {:?}", moved[x], ground(x as u32));
    }
    assert!(near(moved[36], [255; 3]) && near(moved[46], [255; 3]) && near(moved[50], ground(50)));
}

#[test]
fn a_layer_moves_by_its_own_gentle_depth_without_tearing() {
    let Some(r) = renderer() else { return };
    let moved = ramped(r, 4.0);
    let marks: Vec<usize> = (0..W as usize).filter(|x| moved[*x][0] > 120).collect();
    let starts: Vec<usize> =
        marks.iter().copied().filter(|x| !marks.contains(&(x.wrapping_sub(1)))).collect();
    assert_eq!(starts.len(), 4, "{marks:?}");
    let gaps: Vec<usize> = starts.windows(2).map(|w| w[1] - w[0]).collect();
    assert!(gaps.iter().all(|g| (16..=18).contains(g)), "the layer stretches evenly: {gaps:?}");
    assert!(starts[0] <= 1 && starts[3] >= 50, "the near side moves further: {starts:?}");
}

#[test]
fn each_layer_moves_by_its_own_depth_and_rest_shows_the_picture() {
    let Some(r) = renderer() else { return };
    let moved = stacked(r, 8.0);
    assert!(near(moved[18], [255, 0, 0]) && near(moved[12], [255, 0, 0]) && near(moved[9], ground(9)));
    assert!(near(moved[14], [255, 0, 0]) && !near(moved[22], [255, 0, 0]));
    let still = stacked(r, 0.0);
    assert!(near(still[30], [255; 3]) && near(still[10], [255, 0, 0]) && near(still[5], ground(5)));
}
