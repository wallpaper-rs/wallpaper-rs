use super::kit::*;
use super::*;

#[test]
fn textures_that_share_a_picture_hold_it_once_and_their_count_is_capped() {
    let (mut doc, bin) = painted(1024);
    doc["textures"] = json!(vec![json!({ "source": 0 }); 3000]);
    doc["materials"][0]["pbrMetallicRoughness"]["baseColorTexture"]["index"] = json!(2999);
    let m = Model::decode(&pack(&doc, &bin)).unwrap();
    assert_eq!((m.textures.len(), m.materials[0].maps[0]), (1, Some(0)));
    doc["textures"] = json!(vec![json!({ "source": 0 }); 5000]);
    assert!(Model::decode(&pack(&doc, &bin)).unwrap_err().contains("textures, at most 4096"));
}

#[test]
fn the_pixels_of_all_textures_of_a_model_are_capped() {
    let (mut doc, mut bin) = painted(4096);
    let (mut images, mut textures) = (Vec::new(), Vec::new());
    for i in 0..17 {
        let png = header(4096);
        let view = json!({ "buffer": 0, "byteOffset": bin.len(), "byteLength": png.len() });
        bin.extend(png);
        doc["bufferViews"].as_array_mut().unwrap().push(view);
        let at = doc["bufferViews"].as_array().unwrap().len() - 1;
        images.push(json!({ "bufferView": at, "mimeType": "image/png" }));
        textures.push(json!({ "source": i }));
    }
    (doc["images"], doc["textures"]) = (json!(images), json!(textures));
    doc["buffers"][0]["byteLength"] = json!(bin.len());
    assert!(Model::decode(&pack(&doc, &bin)).unwrap_err().contains("texture pixels"));
    doc["images"].as_array_mut().unwrap().truncate(16);
    doc["textures"].as_array_mut().unwrap().truncate(16);
    assert_eq!(Model::decode(&pack(&doc, &bin)).unwrap().textures.len(), 16);
}

#[test]
fn pictures_are_held_once_per_byte_range_and_overlapping_ranges_cannot_pass_the_file() {
    let (mut doc, mut bin) = painted(1);
    let view = doc["images"][0]["bufferView"].as_u64().unwrap() as usize;
    let at = doc["bufferViews"][view]["byteOffset"].as_u64().unwrap_or(0) as usize;
    bin.resize(bin.len() + (64 << 10), 0);
    doc["buffers"][0]["byteLength"] = json!(bin.len());
    let made = |doc: &mut Value, lengths: &dyn Fn(usize) -> usize| {
        let first = doc["bufferViews"].as_array().unwrap().len();
        for i in 0..4096 {
            let span = json!({ "buffer": 0, "byteOffset": at, "byteLength": lengths(i) });
            doc["bufferViews"].as_array_mut().unwrap().push(span);
        }
        doc["images"] = json!((0..4096).map(|i| json!({ "bufferView": first + i })).collect::<Vec<_>>());
        doc["textures"] = json!((0..4096).map(|i| json!({ "source": i })).collect::<Vec<_>>());
        pack(doc, &bin)
    };
    let whole = bin.len() - at;
    let same = made(&mut doc.clone(), &|_| whole);
    assert!(ModelFile::parse(&same).is_ok());
    assert_eq!(Model::decode(&same).unwrap().textures.len(), 1);
    let shifted = made(&mut doc, &|i| whole - i);
    assert!(ModelFile::parse(&shifted).unwrap_err().contains("overlapping pictures"));
    assert!(Model::decode(&shifted).unwrap_err().contains("overlapping pictures"));
    assert!(Model::decode_posed(&shifted).unwrap_err().contains("overlapping pictures"));
    let (mut unused, _) = split(&shifted);
    unused["textures"] = json!([{ "source": 0 }]);
    let unused = pack(&unused, &bin);
    assert!(ModelFile::parse(&unused).is_ok(), "pictures no texture shows are never read");
    let model = Model::decode(&unused).unwrap();
    let texture = &model.textures[0];
    assert_eq!(texture.bytes().as_ptr(), texture.file[texture.range.clone()].as_ptr());
    assert_eq!(texture.file.len(), unused.len(), "textures point into the file instead of copying it");
}

#[test]
fn buffer_views_have_their_own_cap_and_leave_the_scene_entries_alone() {
    let (mut doc, bin) = painted(1);
    let padded = |doc: &mut Value, n: usize| {
        let views = doc["bufferViews"].as_array_mut().unwrap();
        views.resize(n, json!({}));
        pack(doc, &bin)
    };
    let many = padded(&mut doc, wprs_scene::MAX_MODEL_ENTRIES / 2);
    let mut left = wprs_scene::MAX_MODEL_ENTRIES / 4;
    ModelFile::parse_head(&many, many.len() as u64, &mut left).unwrap();
    assert!(left > wprs_scene::MAX_MODEL_ENTRIES / 4 - 64);
    assert!(
        ModelFile::parse(&padded(&mut doc, wprs_scene::MAX_VIEWS + 1)).unwrap_err().contains("more entries")
    );
}
