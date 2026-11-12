use super::*;

fn ball() -> BTreeMap<&'static str, Vec<u8>> {
    let skin = unlit("skin", [0.9, 0.7, 0.4, 1.0]);
    BTreeMap::from([(
        "ball",
        Forge::default().material(skin).part("ball", ForgeShape::Sphere(12), [0.0; 3], 1.2, 0).glb(),
    )])
}

fn combed(map: Option<[u8; 4]>) -> Vec<u8> {
    let r = renderer().unwrap();
    let mut fur = json!({ "shader": "fur", "params": { "layers": 16, "length": 60, "density": 0.8 } });
    let mut images = Vec::new();
    if let Some(rgba) = map {
        fur["maps"] = json!({ "comb": "combs" });
        images.push(("combs", Pixels::new(RgbaImage::from_pixel(8, 8, Rgba(rgba)))));
    }
    let layers = json!([{ "id": "m", "type": "model", "model": "ball", "materials": { "skin": fur } }]);
    let assets = json!({ "assets": { "combs": { "path": "combs.png", "linear": true } } });
    shoot(r, scene(assets, layers, ball()), &images).frame(r, 0.0, Quality::Q3)
}

fn middle(px: &[u8]) -> f32 {
    let lit: Vec<u32> = (0..SIDE * SIDE)
        .filter(|i| rgb(px, i % SIDE, i / SIDE).iter().any(|c| *c > 8))
        .map(|i| i % SIDE)
        .collect();
    lit.iter().sum::<u32>() as f32 / lit.len().max(1) as f32
}

#[test]
fn a_painted_comb_map_bends_the_fur_where_it_is_painted() {
    let Some(_) = renderer() else { return };
    let plain = combed(None);
    assert_eq!(
        combed(Some([128, 128, 128, 0])),
        plain,
        "an unpainted comb map leaves the comb of the material"
    );
    let (right, left) =
        (middle(&combed(Some([255, 128, 128, 255]))), middle(&combed(Some([0, 128, 128, 255]))));
    let still = middle(&plain);
    assert!(right > still + 1.0 && left < still - 1.0, "{left} {still} {right}");
}
