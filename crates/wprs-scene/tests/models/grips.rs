use wprs_scene::Eye;
use wprs_scene::grip::{along, deepen, dolly, orbit, ray, ring, roll, slide, turn};

const SIZE: [f32; 2] = [200.0, 100.0];

fn eye() -> Eye {
    Eye {
        at: [3.0, 2.0, 5.0],
        center: [0.0; 3],
        up: [0.0, 1.0, 0.0],
        fov: 50.0,
        near: 0.1,
        far: 100.0,
        aspect: 2.0,
    }
}

fn shown(e: &Eye, p: [f32; 3]) -> [f32; 2] {
    let [x, y, _] = e.project(p, SIZE).unwrap();
    [x, y]
}

fn close(a: [f32; 2], b: [f32; 2]) -> bool {
    (a[0] - b[0]).hypot(a[1] - b[1]) < 0.05
}

#[test]
fn a_ray_passes_through_what_the_camera_shows_at_that_pixel() {
    let e = eye();
    for px in [[20.0, 10.0], [100.0, 50.0], [190.0, 80.0]] {
        let d = ray(&e, px, SIZE);
        let p = std::array::from_fn(|i| e.at[i] + d[i] * 4.0);
        assert!(close(shown(&e, p), px), "{px:?}");
    }
}

#[test]
fn slides_and_depth_keep_the_point_under_the_pointer() {
    let e = eye();
    let p = [0.5, 0.2, -0.3];
    let from = shown(&e, p);
    let to = [from[0] + 30.0, from[1] - 12.0];
    assert!(close(shown(&e, slide(&e, SIZE, p, from, to)), to));
    let deep = deepen(&e, SIZE, p, from, [from[0], from[1] - 40.0]);
    assert!(close(shown(&e, deep), from) && e.depth(deep) > e.depth(p));
    let a = [0.0, 0.0, 1.0];
    let tip = shown(&e, [p[0], p[1], p[2] + 1.0]);
    assert!((along(&e, SIZE, p, a, from, tip) - 1.0).abs() < 1e-3);
}

#[test]
fn turns_follow_the_right_hand_and_the_camera_orbits_dollies_and_rolls() {
    let e = Eye { at: [0.0, 0.0, 5.0], ..eye() };
    let c = [0.0; 3];
    let m = shown(&e, c);
    let d = turn(&e, SIZE, c, [0.0, 0.0, 1.0], [m[0] + 20.0, m[1]], [m[0], m[1] - 20.0]);
    assert!((d - 90.0).abs() < 0.5, "{d}");
    let swung = orbit(e.at, c, e.up, [90.0, 0.0]);
    assert!((swung[0] - 5.0).abs() < 1e-3 && swung[2].abs() < 1e-3, "{swung:?}");
    let back = dolly(e.at, c, SIZE, [0.0, 0.0], [0.0, 50.0]);
    assert!((back[2] - 5.0 * 1f32.exp()).abs() < 1e-2, "{back:?}");
    let up = roll([0.0, 1.0, 0.0], [0.0, 0.0, -1.0], 90.0);
    assert!((up[0] - 1.0).abs() < 1e-4, "{up:?}");
    assert!(ring(c, [0.0, 1.0, 0.0], 2.0, 8).all(|p| (p[0].hypot(p[2]) - 2.0).abs() < 1e-4 && p[1] == 0.0));
}
