use super::kit::{mesh, near, render, renderer, rgb, scene};
use super::parts::colored;
use serde_json::json;
use wprs_scene::Quality;

fn shot(size: u32) -> Vec<u8> {
    let mut layer = mesh(json!({ "size": [size, size] }));
    layer.as_object_mut().unwrap().remove("texture");
    let glb = colored(0.0, [[1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0]]);
    render(renderer().unwrap(), scene(layer, glb), 0.0, Quality::Q3)
}

#[test]
fn parts_beyond_the_box_of_a_mesh_are_not_cut_at_its_edge() {
    if renderer().is_none() {
        return;
    }
    let wide = shot(32);
    assert!(near(rgb(&wide, 10, 32), [255, 0, 0]), "{:?}", rgb(&wide, 10, 32));
    assert!(near(rgb(&wide, 54, 32), [0, 255, 0]), "{:?}", rgb(&wide, 54, 32));
    assert!(near(rgb(&wide, 3, 32), [0, 0, 0]));
    let fitted = shot(64);
    assert!(near(rgb(&fitted, 10, 32), [255, 0, 0]));
}
