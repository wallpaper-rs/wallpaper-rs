use super::*;

const SAMPLED: &str = "fn material(s: Surface) -> Surface {\n    var o = s;\n    o.albedo = image0(s.uv) * param(0u);\n    return o;\n}";

fn coat(params: Json) -> String {
    json!({ "id": "coat", "stage": "material", "cost": {}, "params": params }).to_string()
}

fn picture(id: &str) -> Json {
    json!({ "id": id, "type": "texture", "default": "white" })
}

#[test]
fn material_shaders_sample_up_to_four_images_and_count_their_reads() {
    let params = json!([{ "id": "tint", "type": "color", "default": "#ffffff" }, picture("base")]);
    let d = Descriptor::packaged("coat", &coat(params), SAMPLED).unwrap();
    assert!(d.reads >= 1.0, "{}", d.reads);
    assert_eq!(material_images(&d).collect::<Vec<_>>(), ["base"]);
    let five: Vec<Json> = (0..5).map(|i| picture(&format!("p{i}"))).collect();
    let many = Descriptor::packaged("coat", &coat(json!(five)), SAMPLED).unwrap_err();
    assert!(many.contains("at most 4 images"), "{many}");
    let lut = json!([{ "id": "grade", "type": "lut", "default": "" }]);
    assert!(Descriptor::packaged("coat", &coat(lut), SAMPLED).unwrap_err().contains("no lut"));
    let nine: Vec<Json> =
        (0..9).map(|i| json!({ "id": format!("n{i}"), "type": "float", "default": 0 })).collect();
    let crowded = Descriptor::packaged("coat", &coat(json!(nine)), SAMPLED).unwrap_err();
    assert!(crowded.contains("at most 8 parameters"), "{crowded}");
}

#[test]
fn material_shaders_that_read_what_only_layer_effects_have_are_refused_at_their_line() {
    let wgsl = "fn material(s: Surface) -> Surface {\n    var o = s;\n    o.albedo = sample(input_tex, s.uv);\n    return o;\n}";
    let why = Descriptor::packaged("coat", &coat(json!([])), wgsl).unwrap_err();
    assert!(why.contains("does not build as a material shader"), "{why}");
    let at = why.split("wgsl:").nth(1).and_then(|r| r.split(':').next()).and_then(|l| l.parse().ok());
    assert_eq!(at.and_then(shader_line), Some(3), "{why}");
    let image = "fn material(s: Surface) -> Surface {\n    var o = s;\n    o.albedo = image0(s.uv);\n    return o;\n}";
    let layer = json!({ "id": "flat", "cost": {}, "passes": [{ "entry": "fs_flat" }] }).to_string();
    let wgsl = format!(
        "{image}\n@fragment\nfn fs_flat(in: VertexOut) -> @location(0) vec4<f32> {{\n    return image0(in.uv);\n}}"
    );
    assert!(Descriptor::packaged("flat", &layer, &wgsl).is_err(), "only materials have images");
}

#[test]
fn material_images_name_pictures_of_the_package() {
    let given = r#""glass": { "shader": "glass", "params": { "base": "nowhere" }, "#;
    let doc = DEEP.replacen(r#""glass": { "shader": "glass", "#, given, 1);
    let mut scene = Scene::from_json(&doc).unwrap();
    let read = |path: &str, max: usize| match path {
        "effects/glass.json" => {
            Ok(json!({ "id": "glass", "stage": "material", "cost": {}, "params": [picture("base")] })
                .to_string()
                .into_bytes())
        }
        "effects/glass.wgsl" => Ok(SAMPLED.replace("* param(0u)", "").into_bytes()),
        p => files(&figure())(p, max),
    };
    let found = scene.adopt(&BTreeMap::new(), read).unwrap_err().to_string();
    assert!(found.contains("params/base"), "{found}");
}
