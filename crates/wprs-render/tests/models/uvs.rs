use super::*;

fn split(glb: &[u8]) -> (Value, Vec<u8>) {
    let word = |at: usize| u32::from_le_bytes(glb[at..at + 4].try_into().unwrap()) as usize;
    let text = word(12);
    (serde_json::from_slice(&glb[20..20 + text]).unwrap(), glb[20 + text + 8..].to_vec())
}

fn pack(doc: &Value, bin: &[u8]) -> Vec<u8> {
    let mut text = doc.to_string().into_bytes();
    text.resize(text.len().next_multiple_of(4), b' ');
    let total = 12 + 8 + text.len() + 8 + bin.len();
    let mut glb = Vec::with_capacity(total);
    for word in [0x4654_6C67, 2, total as u32, text.len() as u32, 0x4E4F_534A] {
        glb.extend_from_slice(&u32::to_le_bytes(word));
    }
    glb.extend_from_slice(&text);
    glb.extend_from_slice(&(bin.len() as u32).to_le_bytes());
    glb.extend_from_slice(&[0x42, 0x49, 0x4E, 0x00]);
    glb.extend_from_slice(bin);
    glb
}

pub fn halves(side: u32) -> Vec<u8> {
    let picture = RgbaImage::from_fn(side, 1, |x, _| match x < side / 2 {
        true => Rgba([255, 0, 0, 255]),
        false => Rgba([0, 255, 0, 255]),
    });
    let mut out = std::io::Cursor::new(Vec::new());
    picture.write_to(&mut out, image::ImageFormat::Png).unwrap();
    out.into_inner()
}

fn card(side: u32, edit: impl Fn(&mut Value, &mut Vec<u8>)) -> Vec<u8> {
    let mut m = unlit("paint", [1.0; 4]);
    m.maps[0] = Some(halves(side));
    let glb = Forge::default().material(m).part("card", ForgeShape::Plane, [0.0; 3], 3.0, 0).glb();
    let (mut doc, mut bin) = split(&glb);
    edit(&mut doc, &mut bin);
    doc["buffers"][0]["byteLength"] = json!(bin.len());
    pack(&doc, &bin)
}

fn shot(glb: Vec<u8>) -> Vec<u8> {
    let r = renderer().unwrap();
    let layer = json!([{ "id": "card", "type": "model", "model": "card" }]);
    render(r, scene(json!({}), layer, BTreeMap::from([("card", glb)])), Quality::Q3)
}

fn sides(px: &[u8]) -> ([u8; 3], [u8; 3]) {
    (rgb(px, 25, 32), rgb(px, 39, 32))
}

const RED: [u8; 3] = [255, 0, 0];
const GREEN: [u8; 3] = [0, 255, 0];

fn moved(doc: &mut Value, ext: Value) {
    doc["materials"][0]["pbrMetallicRoughness"]["baseColorTexture"]["extensions"] =
        json!({ "KHR_texture_transform": ext });
    doc["extensionsUsed"] = json!(["KHR_texture_transform"]);
    doc["extensionsRequired"] = json!(["KHR_texture_transform"]);
}

fn sampler(doc: &mut Value, s: Value) {
    doc["samplers"] = json!([s]);
    doc["textures"][0]["sampler"] = json!(0);
}

fn second(doc: &mut Value, bin: &mut Vec<u8>) {
    let uv: [f32; 8] = [1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0];
    let at = bin.len();
    bin.extend(uv.iter().flat_map(|f| f.to_le_bytes()));
    let views = doc["bufferViews"].as_array_mut().unwrap();
    views.push(json!({ "buffer": 0, "byteOffset": at, "byteLength": 32 }));
    let view = views.len() - 1;
    let accessors = doc["accessors"].as_array_mut().unwrap();
    accessors.push(json!({ "bufferView": view, "componentType": 5126, "count": 4, "type": "VEC2" }));
    doc["meshes"][0]["primitives"][0]["attributes"]["TEXCOORD_1"] = json!(accessors.len() - 1);
}

#[test]
fn a_texture_transform_moves_the_picture_on_the_model() {
    if renderer().is_none() {
        return;
    }
    let plain = sides(&shot(card(8, |_, _| {})));
    assert!(near(plain.0, RED, 8) && near(plain.1, GREEN, 8), "{plain:?}");
    let half = sides(&shot(card(8, |d, _| moved(d, json!({ "offset": [0.5, 0.0] })))));
    assert!(near(half.0, GREEN, 8) && near(half.1, RED, 8), "{half:?}");
    let turned = sides(&shot(card(8, |d, _| {
        moved(d, json!({ "rotation": std::f32::consts::PI, "offset": [1.0, 1.0] }))
    })));
    assert!(near(turned.0, GREEN, 8) && near(turned.1, RED, 8), "{turned:?}");
}

#[test]
fn a_map_reads_the_second_set_of_coordinates() {
    if renderer().is_none() {
        return;
    }
    let set = |d: &mut Value, b: &mut Vec<u8>| {
        second(d, b);
        d["materials"][0]["pbrMetallicRoughness"]["baseColorTexture"]["texCoord"] = json!(1);
    };
    let flipped = sides(&shot(card(8, set)));
    assert!(near(flipped.0, GREEN, 8) && near(flipped.1, RED, 8), "{flipped:?}");
    let picked = |d: &mut Value, b: &mut Vec<u8>| {
        second(d, b);
        moved(d, json!({ "texCoord": 1 }));
    };
    let picked = sides(&shot(card(8, picked)));
    assert!(near(picked.0, GREEN, 8) && near(picked.1, RED, 8), "{picked:?}");
}

#[test]
fn the_sampler_of_a_texture_clamps_mirrors_and_keeps_pixels_sharp() {
    if renderer().is_none() {
        return;
    }
    let past = |wrap: u32| {
        card(8, move |d, _| {
            moved(d, json!({ "offset": [0.6, 0.0] }));
            sampler(d, json!({ "wrapS": wrap }));
        })
    };
    let repeated = sides(&shot(past(10497)));
    assert!(near(repeated.1, RED, 8), "{repeated:?}");
    let clamped = sides(&shot(past(33071)));
    assert!(near(clamped.0, GREEN, 8) && near(clamped.1, GREEN, 8), "{clamped:?}");
    let mirrored = sides(&shot(past(33648)));
    assert!(near(mirrored.1, GREEN, 8), "{mirrored:?}");
    let row = |px: &[u8]| (8..56).map(|x| rgb(px, x, 32)).filter(|c| *c != [0, 0, 0]).collect::<Vec<_>>();
    let soft = row(&shot(card(2, |_, _| {})));
    assert!(soft.iter().any(|c| c[0] > 40 && c[1] > 40), "a linear sampler blends neighbours");
    let sharp = row(&shot(card(2, |d, _| sampler(d, json!({ "magFilter": 9728, "minFilter": 9728 })))));
    assert!(!sharp.is_empty() && sharp.iter().all(|c| near(*c, RED, 4) || near(*c, GREEN, 4)), "{sharp:?}");
}
