use super::*;

fn balls(shadows: bool) -> Runtime {
    let ball = Forge::default()
        .material(ForgeMaterial::new("m", [1.0; 4]))
        .part("b", ForgeShape::Sphere(130), [0.0; 3], 1.0, 0)
        .glb();
    let sun = json!({ "id": "sun", "type": "light", "light": "directional", "shadows": shadows });
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [8, 8] }, "assets": { "a": { "path": "a.glb" } },
        "layers": [sun, { "id": "x", "type": "model", "model": "a" }] });
    let mut scene = Scene::from_json(&text.to_string()).unwrap();
    scene.adopt(&BTreeMap::new(), |_: &str, _: usize| Ok::<_, String>(ball.clone())).unwrap();
    runtime(scene)
}

#[test]
fn phones_and_the_levels_phones_reach_keep_the_prudent_triangle_unit() {
    for tier in [Tier::T0, Tier::T1, Tier::T2, Tier::T3] {
        assert_eq!(triangle_unit(tier, true), TRIANGLE_UNIT, "{tier:?} on a phone");
    }
    for tier in [Tier::T0, Tier::T1, Tier::T2] {
        assert_eq!(triangle_unit(tier, false), TRIANGLE_UNIT, "{tier:?} on a computer");
    }
    assert_eq!(triangle_unit(Tier::T3, false), MEASURED_TRIANGLE_UNIT);
}

#[test]
fn the_cost_of_models_follows_the_unit_of_the_device_and_its_shadow_maps() {
    let rt = balls(false);
    let triangles = rt.scene.model_triangles(Quality::Q3) as f32;
    let prudent = rt.cost(Quality::Q3);
    assert_eq!(prudent, rt.scene.cost(Quality::Q3), "a runtime without a device prices like the editor");
    rt.set_device(Tier::T3, false);
    let measured = rt.cost(Quality::Q3);
    let gap = triangles / TRIANGLE_UNIT - triangles / MEASURED_TRIANGLE_UNIT;
    assert!((prudent - measured - gap).abs() < 1e-3, "{prudent} {measured} {gap}");
    rt.set_device(Tier::T3, true);
    assert_eq!(rt.cost(Quality::Q3), prudent, "a phone keeps the prudent unit");
    rt.set_device(Tier::T2, false);
    assert_eq!(rt.cost(Quality::Q3), prudent);
    let shaded = balls(true);
    shaded.set_device(Tier::T3, false);
    let more = shaded.cost(Quality::Q3) - measured;
    assert!(
        (more - triangles / MEASURED_TRIANGLE_UNIT).abs() < 1e-3,
        "one shadow map draws them again: {more}"
    );
}
