use image::{Rgba, RgbaImage};
use serde_json::json;
use wprs_render::{Rehearsal, Renderer, SceneAssets, Trace, split};
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
        console: Vec::new(),
        stopped: Vec::new(),
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

fn gpu() -> Option<&'static Renderer> {
    match Renderer::shared() {
        Ok(r) => Some(r),
        Err(e) if std::env::var_os("WPRS_REQUIRE_GPU").is_none() => {
            eprintln!("skipping GPU test: {e}");
            None
        }
        Err(e) => panic!("GPU required but unavailable: {e}"),
    }
}

fn clocked(source: &str) -> (wprs_scene::Scene, SceneAssets) {
    let doc = json!({ "schema": wprs_scene::SCHEMA, "canvas": { "size": [32, 18], "background": "#000000" },
        "scripts": [{ "path": "scripts/hour.js", "events": ["update"], "uses": ["layers", "console"] }],
        "layers": [{ "id": "lamp", "type": "solid", "color": "#ffffff", "visible": false }] });
    let scene = wprs_scene::Scene::load(&doc.to_string(), &Default::default()).unwrap();
    let assets =
        SceneAssets { scripts: vec![("scripts/hour.js".into(), source.into())], ..Default::default() };
    (scene, assets)
}

const HOUR: &str = "import { scene, time } from 'wprs';
export function update() {
  scene.layer('lamp').visible = time.format('HH') === '15';
  console.log('hour ' + time.format('HH'));
}";

#[test]
fn a_render_runs_the_scripts_at_the_clock_it_was_given() {
    let Some(r) = gpu() else { return };
    let (scene, assets) = clocked(HOUR);
    let at = |hours: f64| Rehearsal {
        wall: 1_700_000_000.0 - 1_700_000_000.0 % 86_400.0 + hours * 3600.0,
        edge: 32,
        ..Rehearsal::default()
    };
    let day = r.rehearse(&scene, &assets, &Default::default(), &at(15.5)).unwrap();
    let night = r.rehearse(&scene, &assets, &Default::default(), &at(3.5)).unwrap();
    assert!(!day.console.is_empty(), "{:?} {:?}", day.console, day.stopped);
    assert_eq!(day.frames[0].get_pixel(16, 9).0[0], 255, "{:?}", day.console);
    assert_eq!(night.frames[0].get_pixel(16, 9).0[0], 0);
    assert!(day.console.iter().any(|l| l == "log scripts/hour.js: hour 15"), "{:?}", day.console);
    assert!(night.console.iter().any(|l| l == "log scripts/hour.js: hour 03"), "{:?}", night.console);
}

#[test]
fn a_script_that_fails_in_a_render_says_where() {
    let Some(r) = gpu() else { return };
    let bad = "import { scene } from 'wprs';\n\nexport function update() {\n  scene.layer('lamp').opacity = 'high';\n}";
    let (scene, assets) = clocked(bad);
    let reel = r
        .rehearse(&scene, &assets, &Default::default(), &Rehearsal { edge: 32, ..Rehearsal::default() })
        .unwrap();
    let said = reel.console.iter().find(|l| l.starts_with("error scripts/hour.js")).expect("no error line");
    assert!(said.ends_with("(scripts/hour.js line 4)"), "{said}");
}
