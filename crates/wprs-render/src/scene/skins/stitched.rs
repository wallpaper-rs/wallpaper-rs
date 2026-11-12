use super::atlas::{pack, placed};
use image::{Rgba, RgbaImage};
use wprs_scene::{Rigged, VERTEX_FLOATS};

#[test]
fn pictures_of_one_mesh_share_an_atlas_and_each_part_reads_its_own() {
    let red = RgbaImage::from_pixel(40, 20, Rgba([255, 0, 0, 255]));
    let blue = RgbaImage::from_pixel(10, 30, Rgba([0, 0, 255, 255]));
    let atlas = pack(vec![red, blue], 4096).unwrap();
    let [x, y, w, h] = atlas.rects[1];
    let (aw, ah) = atlas.rgba.dimensions();
    let middle = |r: [f32; 4]| ((r[0] + r[2] / 2.0) * aw as f32, (r[1] + r[3] / 2.0) * ah as f32);
    let (bx, by) = middle([x, y, w, h]);
    assert_eq!(atlas.rgba.get_pixel(bx as u32, by as u32).0, [0, 0, 255, 255]);
    let (rx, ry) = middle(atlas.rects[0]);
    assert_eq!(atlas.rgba.get_pixel(rx as u32, ry as u32).0, [255, 0, 0, 255]);
    assert!((w * aw as f32 - 10.0).abs() < 1e-3 && (h * ah as f32 - 30.0).abs() < 1e-3);
    let mut vertex = vec![0.0; VERTEX_FLOATS * 2];
    vertex[3..5].copy_from_slice(&[1.0, 1.0]);
    vertex[VERTEX_FLOATS + 3..VERTEX_FLOATS + 5].copy_from_slice(&[0.0, 0.5]);
    vertex[VERTEX_FLOATS * 2 - 1] = 1.0;
    let rig = Rigged { vertices: vertex, sheets: vec![0, 1], ..Rigged::default() };
    let laid = placed(&rig, &atlas.rects);
    let [r0x, r0y, r0w, r0h] = atlas.rects[0];
    assert_eq!(laid[3..5], [r0x + r0w, r0y + r0h]);
    assert_eq!(laid[VERTEX_FLOATS + 3..VERTEX_FLOATS + 5], [x, y + 0.5 * h]);
    let huge = vec![RgbaImage::new(3000, 3000), RgbaImage::new(3000, 3000)];
    let shrunk = pack(huge, 2048).unwrap();
    assert!(shrunk.rgba.width() <= 2048 && shrunk.rgba.height() <= 2048);
}
