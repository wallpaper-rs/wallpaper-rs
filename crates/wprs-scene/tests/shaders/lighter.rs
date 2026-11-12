use super::*;

const SOFT: &str = r#"
@fragment
fn fs_soft_a(in: VertexOut) -> @location(0) vec4<f32> {
    return (tap(in.uv) + tap(in.uv + px(1.0)) + tap(in.uv - px(1.0)) + tap(in.uv + px(2.0))) * 0.25;
}

@fragment
fn fs_soft_b(in: VertexOut) -> @location(0) vec4<f32> {
    return (tap(in.uv) + tap(in.uv + px(3.0)) + tap(in.uv - px(3.0)) + tap(in.uv + px(4.0))) * 0.25;
}

@fragment
fn fs_soft_lite(in: VertexOut) -> @location(0) vec4<f32> {
    return tap(in.uv);
}

@fragment
fn fs_soft_wide(in: VertexOut) -> @location(0) vec4<f32> {
    var c = vec4<f32>(0.0);
    for (var i = 0; i < 64; i++) {
        c += tap(in.uv + px(f32(i)));
    }
    return c / 64.0;
}
"#;

fn soft_json(lighter: Json) -> Json {
    json!({ "id": "soft", "params": [{ "id": "radius", "type": "float", "default": 2, "min": 0, "max": 8 }],
        "passes": [{ "entry": "fs_soft_a" }, { "entry": "fs_soft_b" }],
        "cost": { "taps": 2, "passes": 2, "scale": 1 }, "lighter": lighter })
}

fn soft(lighter: Json) -> Result<Descriptor, String> {
    Descriptor::packaged("soft", &soft_json(lighter).to_string(), SOFT)
}

fn single(entry: &str) -> Json {
    json!({ "passes": [{ "entry": entry }], "cost": { "taps": 1, "passes": 1, "scale": 1 } })
}

#[test]
fn a_package_lighter_variant_is_built_with_its_own_key_and_measure() {
    let d = soft(json!({ "passes": [{ "entry": "fs_soft_lite" }], "cost": { "taps": 1 }, "up_to": "q1" }))
        .unwrap();
    let lite = d.lite.as_deref().expect("a lighter variant");
    assert!(lite.light && !d.light && lite.key != d.key && lite.key & 1 == 1);
    assert_eq!(lite.passes.iter().map(|p| p.entry.as_str()).collect::<Vec<_>>(), ["fs_soft_lite"]);
    assert!(lite.units() < d.units() && d.lighter_units() == Some(lite.units()));
    assert_eq!(d.form(Quality::Q1, false), Form::Lighter);
    assert_eq!(d.form(Quality::Q2, false), Form::Full);
    assert!(soft(Json::Null).unwrap().lite.is_none());
}

#[test]
fn the_loader_refuses_lighter_variants_that_are_not_lighter_or_not_whole() {
    let refused = |lighter: Json, why: &str| {
        let e = soft(lighter).expect_err(why);
        assert!(e.contains(why), "{e}");
    };
    refused(single("fs_soft_missing"), "no entry point fs_soft_missing for its lighter variant");
    let same =
        json!({ "passes": [{ "entry": "fs_soft_lite" }], "cost": { "taps": 2, "passes": 2, "scale": 1 } });
    refused(same, "the lighter variant must cost less than the full effect");
    refused(single("fs_soft_wide"), "its lighter variant costs");
    refused(json!({ "passes": [], "cost": { "taps": 1 } }), "the lighter variant needs passes");
    let always = json!({ "passes": [{ "entry": "fs_soft_lite" }], "cost": { "taps": 1 }, "up_to": "q3" });
    refused(always, "replaces the effect only up to q0, q1 or q2");
    let many: Vec<Json> = (0..17).map(|_| json!({ "entry": "fs_soft_lite", "scale": 0.1 })).collect();
    refused(json!({ "passes": many, "cost": { "taps": 1 } }), "the lighter variant: more than 16 passes");
    let unknown =
        json!({ "passes": [{ "entry": "fs_soft_lite", "inputs": ["pass:3"] }], "cost": { "taps": 1 } });
    refused(unknown, "the lighter variant: pass 0 reads the unknown input pass:3");
    let extra = json!({ "passes": [{ "entry": "fs_soft_lite" }], "cost": { "taps": 1 }, "below": "q1" });
    assert!(soft(extra).is_err());
}

#[test]
fn a_scene_whose_effects_bring_a_lighter_variant_names_its_feature() {
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [64, 32] }, "assets": { "a": { "path": "a.png" } },
        "effects": { "soft": { "descriptor": "fx/soft.json", "wgsl": "fx/soft.wgsl" } },
        "layers": [{ "id": "l", "type": "image", "image": "a", "effects": [{ "effect": "soft" }] }] });
    let with = |lighter: Json| {
        let mut scene = Scene::load(&text.to_string(), &BTreeMap::new()).unwrap();
        let json = soft_json(lighter).to_string();
        let files = |path: &str, _: usize| match path {
            "fx/soft.json" => Ok(json.clone().into_bytes()),
            "fx/soft.wgsl" => Ok(SOFT.as_bytes().to_vec()),
            _ => Err("missing"),
        };
        scene.adopt(&BTreeMap::new(), files).unwrap();
        scene.features().contains("lighter_effects")
    };
    assert!(with(single("fs_soft_lite")) && !with(Json::Null));
}
