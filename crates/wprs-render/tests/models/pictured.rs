use super::*;

const SAMPLED: &str =
    "fn material(s: Surface) -> Surface {\n    var o = s;\n    o.albedo = image0(s.uv);\n    return o;\n}\n";

fn coat() -> String {
    json!({ "id": "coat", "stage": "material", "cost": {},
        "params": [{ "id": "base", "type": "texture", "default": "white" }] })
    .to_string()
}

fn pictured(params: Value) -> Scene {
    let layers = json!([{ "id": "m", "type": "model", "model": "ball",
        "materials": { "skin": { "shader": "coat", "params": params } } }]);
    let doc = json!({ "schema": SCHEMA, "canvas": { "size": [SIDE, SIDE], "background": "#000000" },
        "assets": { "ball": { "path": "ball.glb" }, "red": { "path": "red.png" }, "blue": { "path": "blue.png" } },
        "layers": layers, "effects": { "coat": { "descriptor": "effects/coat.json", "wgsl": "effects/coat.wgsl" } } });
    let mut scene = Scene::from_json(&doc.to_string()).unwrap();
    let glb = Forge::default()
        .material(unlit("skin", [1.0; 4]))
        .part("ball", ForgeShape::Sphere(12), [0.0; 3], 2.0, 0)
        .glb();
    let read = move |path: &str, _: usize| match path {
        "ball.glb" => Ok(glb.clone()),
        "effects/coat.json" => Ok(coat().into_bytes()),
        "effects/coat.wgsl" => Ok(SAMPLED.as_bytes().to_vec()),
        p => Err(format!("no {p}")),
    };
    scene.adopt(&BTreeMap::new(), read).unwrap_or_else(|e| panic!("{e}"));
    scene
}

fn images() -> [(&'static str, Arc<Pixels>); 2] {
    let flat = |c: [u8; 4]| Pixels::new(RgbaImage::from_pixel(4, 4, Rgba(c)));
    [("red", flat([255, 0, 0, 255])), ("blue", flat([0, 0, 255, 255]))]
}

#[test]
fn a_package_material_shader_samples_the_images_its_material_names() {
    let Some(r) = renderer() else { return };
    let shot = |params: Value| shoot(r, pictured(params), &images()).frame(r, 0.0, Quality::Q3);
    let red = rgb(&shot(json!({ "base": "red" })), 32, 32);
    let blue = rgb(&shot(json!({ "base": "blue" })), 32, 32);
    let white = rgb(&shot(json!({})), 32, 32);
    assert!(red[0] > 200 && red[1] < 20 && red[2] < 20, "{red:?}");
    assert!(blue[2] > 200 && blue[0] < 20 && blue[1] < 20, "{blue:?}");
    assert!(white.iter().all(|c| *c > 200), "{white:?}");
}

#[test]
fn a_material_shader_the_card_cannot_build_is_left_out_with_a_note_and_shades_with_pbr() {
    let Some(r) = renderer() else { return };
    let mut scene = pictured(json!({ "base": "red" }));
    let mut unchecked = (*scene.loaded["coat"]).clone();
    unchecked.key ^= 2;
    unchecked.wgsl.push_str("\nfn broken( {\n");
    scene.loaded.insert("coat".into(), Arc::new(unchecked));
    let mut shot = shoot(r, scene, &images());
    let px = shot.frame(r, 0.0, Quality::Q3);
    let notes = shot.view.left_out();
    assert!(notes.iter().any(|n| n.contains("coat") && n.contains("shaded with pbr")), "{notes:?}");
    let c = rgb(&px, 32, 32);
    assert!(c.iter().all(|v| *v > 200), "the forged white skin shows instead: {c:?}");
}
