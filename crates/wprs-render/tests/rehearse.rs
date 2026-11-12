use image::{Rgba, RgbaImage};
use wprs_render::{Rehearsal, Trace, split};
use wprs_scene::Quality;

#[test]
fn battery_lowers_the_level_and_caps_the_rate_as_the_daemon_does() {
    let wanted = Rehearsal { quality: Quality::Q3, fps: 120, ..Rehearsal::default() };
    assert_eq!((wanted.level(), wanted.rate()), (Quality::Q3, 120));
    let drained = Rehearsal { battery: true, ..wanted.clone() };
    assert_eq!((drained.level(), drained.rate()), (Quality::Q2, 60));
    let low = Rehearsal { quality: Quality::Q1, battery: true, ..wanted };
    assert_eq!(low.level(), Quality::Q1, "a lower cap stays");
}

#[test]
fn the_made_up_pointer_crosses_the_canvas_and_presses_in_the_middle() {
    let trace = Trace { from: [0.0, 0.5], to: [1.0, 0.5], click: true };
    let wanted = Rehearsal { frames: 11, pointer: Some(trace), ..Rehearsal::default() };
    assert_eq!(wanted.pointer_at(0), (Some([0.0, 0.5]), 0));
    assert_eq!(wanted.pointer_at(5), (Some([0.5, 0.5]), 1));
    assert_eq!(wanted.pointer_at(10), (Some([1.0, 0.5]), 0));
    let pressed: Vec<_> = (0..11).filter(|i| wanted.pointer_at(*i).1 == 1).collect();
    assert_eq!(pressed, [4, 5, 6]);
    let still = Rehearsal { pointer: None, ..wanted };
    assert_eq!(still.pointer_at(5), (None, 0));
    let single = Rehearsal { frames: 1, pointer: Some(trace), ..Rehearsal::default() };
    assert_eq!(single.pointer_at(0).0, Some([0.0, 0.5]));
}

#[test]
fn a_comparison_keeps_the_left_of_the_first_and_the_right_of_the_second() {
    let (a, b) = (
        RgbaImage::from_pixel(8, 2, Rgba([255, 0, 0, 255])),
        RgbaImage::from_pixel(8, 2, Rgba([0, 0, 255, 255])),
    );
    let joined = split(&a, &b, 0.25);
    assert_eq!(joined.get_pixel(1, 0), a.get_pixel(0, 0));
    assert_eq!(joined.get_pixel(2, 1), b.get_pixel(0, 0));
    assert_eq!(split(&a, &b, 1.0), a);
    assert_eq!(split(&a, &b, 0.0), b);
}

#[test]
fn frames_are_written_as_a_picture_a_sequence_or_an_animated_picture() {
    let dir = tempfile::tempdir().unwrap();
    let frame = |c: u8| RgbaImage::from_pixel(4, 4, Rgba([c, 0, 0, 255]));
    let reel = |n: u8| wprs_render::Reel {
        frames: (0..n).map(|i| frame(i * 40)).collect(),
        quality: Quality::Q3,
        render_size: (4, 4),
        fps: 10,
        left_out: Vec::new(),
    };
    let one = reel(1).write(&dir.path().join("a.png")).unwrap();
    assert_eq!(one, [dir.path().join("a.png")]);
    let many = reel(3).write(&dir.path().join("b.png")).unwrap();
    let names: Vec<_> = many.iter().map(|p| p.file_name().unwrap().to_str().unwrap().to_owned()).collect();
    assert_eq!(names, ["b-0001.png", "b-0002.png", "b-0003.png"]);
    let gif = reel(3).write(&dir.path().join("c.gif")).unwrap();
    assert_eq!(&std::fs::read(&gif[0]).unwrap()[..3], b"GIF");
    assert!(reel(1).write(&dir.path().join("d.txt")).is_err());
}
