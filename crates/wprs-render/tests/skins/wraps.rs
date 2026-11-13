use super::kit::{mesh, near, render, renderer, rgb, scene};
use image::{Rgba, RgbaImage};
use serde_json::json;
use wprs_scene::Quality;

fn model(wrap: u32) -> Vec<u8> {
    let picture =
        RgbaImage::from_fn(4, 1, |x, _| Rgba(if x < 2 { [255, 0, 0, 255] } else { [0, 0, 255, 255] }));
    let mut png = std::io::Cursor::new(Vec::new());
    picture.write_to(&mut png, image::ImageFormat::Png).unwrap();
    let mut bin = Vec::new();
    let mut views = Vec::new();
    let mut add = |values: Vec<u8>| {
        views.push(json!({ "buffer": 0, "byteOffset": bin.len(), "byteLength": values.len() }));
        bin.extend(&values);
        bin.resize(bin.len().div_ceil(4) * 4, 0);
        views.len() - 1
    };
    let f = |v: &[f32]| v.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
    let position = add(f(&[-32.0, -32.0, 0.0, 32.0, -32.0, 0.0, 32.0, 32.0, 0.0, -32.0, 32.0, 0.0]));
    let uv = add(f(&[0.0, 1.0, 2.0, 1.0, 2.0, 0.0, 0.0, 0.0]));
    let list = add([0u32, 1, 2, 0, 2, 3].iter().flat_map(|i| i.to_le_bytes()).collect());
    let image = add(png.into_inner());
    let accessor = |view: usize, kind: &str, component: u32, count: usize| json!({ "bufferView": view, "type": kind, "componentType": component, "count": count });
    let doc = json!({ "asset": { "version": "2.0" },
        "nodes": [{ "name": "quad", "mesh": 0 }],
        "meshes": [{ "name": "quad", "primitives": [{ "attributes": { "POSITION": 0, "TEXCOORD_0": 1 }, "indices": 2, "material": 0 }] }],
        "materials": [{ "pbrMetallicRoughness": { "baseColorTexture": { "index": 0 } } }],
        "textures": [{ "source": 0, "sampler": 0 }],
        "samplers": [{ "wrapS": wrap, "wrapT": 33071, "magFilter": 9729, "minFilter": 9729 }],
        "images": [{ "bufferView": image, "mimeType": "image/png" }],
        "accessors": [accessor(position, "VEC3", 5126, 4), accessor(uv, "VEC2", 5126, 4), accessor(list, "SCALAR", 5125, 6)],
        "bufferViews": views, "buffers": [{ "byteLength": bin.len() }] });
    let mut text = doc.to_string().into_bytes();
    text.resize(text.len().div_ceil(4) * 4, b' ');
    let mut out = Vec::new();
    for w in [0x4654_6C67u32, 2, (28 + text.len() + bin.len()) as u32, text.len() as u32, 0x4E4F_534A] {
        out.extend(w.to_le_bytes());
    }
    out.extend(text);
    out.extend((bin.len() as u32).to_le_bytes());
    out.extend(0x004E_4942u32.to_le_bytes());
    out.extend(bin);
    out
}

fn row(wrap: u32) -> [[u8; 3]; 4] {
    let mut layer = mesh(json!({}));
    layer.as_object_mut().unwrap().remove("texture");
    let px = render(renderer().unwrap(), scene(layer, model(wrap)), 0.0, Quality::Q3);
    [8, 24, 40, 56].map(|x| rgb(&px, x, 32))
}

#[test]
fn the_wrap_mode_of_the_texture_sampler_decides_what_lies_beyond_the_picture() {
    if renderer().is_none() {
        return;
    }
    let (red, blue) = ([255, 0, 0], [0, 0, 255]);
    let check = |wrap: u32, want: [[u8; 3]; 4]| {
        let got = row(wrap);
        assert!(got.iter().zip(want).all(|(a, b)| near(*a, b)), "{wrap}: {got:?}");
    };
    check(10497, [red, blue, red, blue]);
    check(33648, [red, blue, blue, red]);
    check(33071, [red, blue, blue, blue]);
}
