use super::kit::*;
use super::*;

#[test]
fn every_node_that_places_a_mesh_counts_its_triangles() {
    let ball = Forge::default()
        .material(ForgeMaterial::new("m", [1.0; 4]))
        .part("b", ForgeShape::Sphere(130), [0.0; 3], 1.0, 0)
        .glb();
    let (mut doc, bin) = split(&ball);
    let one = ModelFile::parse(&ball).unwrap().triangles;
    let copies = (MAX_TRIANGLES / one + 1) as usize;
    doc["nodes"] = json!(vec![json!({ "mesh": 0 }); 3]);
    let three = pack(&doc, &bin);
    assert_eq!(ModelFile::parse(&three).unwrap().triangles, one * 3);
    assert_eq!(Model::decode(&three).unwrap().triangles, one * 3);
    doc["nodes"] = json!(vec![json!({ "mesh": 0 }); copies]);
    let many = pack(&doc, &bin);
    assert!(ModelFile::parse(&many).unwrap_err().contains("triangles"));
    assert!(Model::decode(&many).unwrap_err().contains("triangles"));
    assert!(Model::decode_posed(&many).unwrap_err().contains("triangles"));
}

#[test]
fn a_scene_refuses_models_that_together_pass_the_triangle_budget() {
    let ball = Forge::default()
        .material(ForgeMaterial::new("m", [1.0; 4]))
        .part("b", ForgeShape::Sphere(130), [0.0; 3], 1.0, 0)
        .glb();
    let (mut doc, bin) = split(&ball);
    doc["nodes"] = json!(vec![json!({ "mesh": 0 }); 20]);
    let heavy = pack(&doc, &bin);
    let assets = json!({ "a": { "path": "a.glb" }, "b": { "path": "b.glb" } });
    let layers =
        json!([{ "id": "x", "type": "model", "model": "a" }, { "id": "y", "type": "model", "model": "b" }]);
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [8, 8] }, "assets": assets, "layers": layers });
    let mut scene = Scene::from_json(&text.to_string()).unwrap();
    let read = scene.adopt(&BTreeMap::new(), |_: &str, _: usize| Ok::<_, String>(heavy.clone()));
    assert!(format!("{read:?}").contains("triangles the models of a scene may hold"), "{read:?}");
}

#[test]
fn a_tier_shows_the_poster_of_models_past_its_triangles_counting_fur_shells() {
    let ball = Forge::default()
        .material(ForgeMaterial::new("m", [1.0; 4]))
        .part("b", ForgeShape::Sphere(130), [0.0; 3], 1.0, 0)
        .glb();
    let made = |materials: Value| {
        let text = json!({ "schema": SCHEMA, "canvas": { "size": [8, 8] }, "assets": { "a": { "path": "a.glb" } },
            "layers": [{ "id": "x", "type": "model", "model": "a", "materials": materials }] });
        let mut scene = Scene::from_json(&text.to_string()).unwrap();
        scene.adopt(&BTreeMap::new(), |_: &str, _: usize| Ok::<_, String>(ball.clone())).unwrap();
        runtime(scene)
    };
    let plain = made(json!({}));
    let one = plain.scene.model_triangles(Quality::Q0);
    assert!(one > 50_000 && one < 75_000, "{one}");
    assert!(plain.cost(Quality::Q0) < Tier::T0.budget(), "the cost alone would let it play");
    assert!(Budget::new(Tier::T0, false, 60).still(&plain), "past the 50 000 triangles of T0");
    assert!(!Budget::new(Tier::T1, false, 60).still(&plain));
    let furry = made(json!({ "m": { "shader": "fur", "params": { "layers": 8 } } }));
    assert_eq!(furry.scene.model_triangles(Quality::Q0), one * 3);
    assert_eq!(furry.scene.model_triangles(Quality::Q3), one * 9);
    assert!(furry.cost(Quality::Q3) > furry.cost(Quality::Q0));
    assert!(Budget::new(Tier::T1, false, 60).still(&furry), "the shells count at the lowest quality");
    assert!(!Budget::new(Tier::T2, false, 60).still(&furry));
    let fixed = Budget { choice: wprs_core::QualityChoice::Fixed(0), ..Budget::new(Tier::T0, false, 60) };
    assert!(fixed.heavy(&plain) && !fixed.still(&plain), "a fixed level plays it anyway");
    for choice in ["q0", "q3", "min-q1"] {
        let chosen = Budget { choice: wprs_core::QualityChoice::parse(choice).unwrap(), ..fixed };
        let level = chosen.level(&plain).0;
        assert_eq!(chosen.over(&plain, Quality::Q0), None, "{choice}");
        assert_eq!(chosen.automatic(&plain, level), "stop", "{choice} hid that automatic stops it");
    }
    let floor = Budget {
        choice: wprs_core::QualityChoice::parse("min-q1").unwrap(),
        ..Budget::new(Tier::T1, false, 60)
    };
    let (level, why) = floor.level(&furry);
    assert!(level >= Quality::Q1 && why != wprs_scene::Why::User, "the floor kept the automatic level");
    assert_eq!(floor.automatic(&furry, level), "stop", "a floor that plays it said it stays still");
    assert_eq!(Budget::new(Tier::T1, false, 60).automatic(&furry, Quality::Q0), "still");
}

#[test]
fn texture_features_models_skip_are_refused_when_required_and_noted_otherwise() {
    let (mut doc, bin) = painted(16);
    assert!(ModelFile::parse(&pack(&doc, &bin)).unwrap().notes.is_empty());
    doc["extensionsUsed"] = json!(["KHR_texture_transform"]);
    doc["materials"][0]["pbrMetallicRoughness"]["baseColorTexture"]["texCoord"] = json!(1);
    doc["samplers"] = json!([{ "wrapS": 33071 }]);
    let noted = pack(&doc, &bin);
    let notes = ModelFile::parse(&noted).unwrap().notes;
    assert_eq!(notes.len(), 3, "{notes:?}");
    assert!(Model::decode(&noted).is_ok());
    doc["extensionsRequired"] = json!(["KHR_texture_transform"]);
    let required = pack(&doc, &bin);
    assert!(ModelFile::parse(&required).is_ok(), "mesh layers keep reading it");
    assert!(Model::decode(&required).unwrap_err().contains("KHR_texture_transform"));
}
