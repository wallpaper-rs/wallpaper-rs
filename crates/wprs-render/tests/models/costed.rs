use super::*;
use wprs_scene::Tier;

const WIDE: (u32, u32) = (1920, 1080);

fn balls(count: usize, segments: u32) -> (Value, BTreeMap<&'static str, Vec<u8>>) {
    let ball = Forge::default()
        .material(ForgeMaterial::new("skin", [0.8, 0.5, 0.3, 1.0]))
        .part("ball", ForgeShape::Sphere(segments), [0.0; 3], 0.8, 0)
        .glb();
    let side = (count as f32).sqrt().ceil() as usize;
    let place =
        |k: usize| [(k % side) as f32 - side as f32 / 2.0, (k / side) as f32 - side as f32 / 2.0, 0.0];
    let models: Vec<Value> = (0..count)
        .map(|k| json!({ "id": format!("b{k}"), "type": "model", "model": "ball", "position": place(k) }))
        .collect();
    (json!(models), BTreeMap::from([("ball", ball)]))
}

fn staged(count: usize, segments: u32, shadows: bool) -> Scene {
    let (models, files) = balls(count, segments);
    let far = 2.0 + count as f32;
    let key = |t: f32, x: f32| json!({ "t": t, "eye": [x, 1, far], "center": [0, 0, 0] });
    let camera = json!({ "paths": [{ "id": "a", "keys": [key(0.0, -3.0), key(10.0, 3.0)] }] });
    let sun = json!({ "id": "sun", "type": "light", "light": "directional", "intensity": 3,
        "transform": { "tilt": [45, 0] }, "shadows": shadows });
    let layers = json!([sun, { "id": "world", "type": "group", "space": "3d", "layers": models }]);
    let canvas = json!({ "size": [WIDE.0, WIDE.1], "background": "#000000" });
    scene(json!({ "camera3d": camera, "canvas": canvas }), layers, files)
}

fn passes(r: &Renderer, scene: Scene) -> (u64, Vec<(String, f64)>) {
    let triangles = scene.model_triangles(Quality::Q3);
    let bench = wprs_render::Bench {
        frames: 240,
        size: WIDE,
        passes: true,
        level: wprs_render::Level::Fixed(Quality::Q3),
        tier: Some(Tier::T3),
        ..Default::default()
    };
    let runtime = Runtime::new(Arc::new(scene), Values::new());
    let report = wprs_render::scene_bench(r, runtime, Default::default(), &bench).unwrap();
    let rows = report["pass_ms"]["passes"].as_array().cloned().unwrap_or_default();
    let read = |p: &Value| (p[0].as_str().unwrap_or_default().to_owned(), p[1].as_f64().unwrap_or(0.0));
    (triangles, rows.iter().map(read).collect())
}

fn spent(rows: &[(String, f64)], what: &str) -> f64 {
    rows.iter().filter(|(l, _)| l.starts_with(what)).map(|(_, ms)| ms).sum()
}

#[test]
#[ignore]
fn the_3d_stage_costs_on_this_gpu() {
    let Some(r) = renderer() else { return };
    r.force_tier(Some(Tier::T3));
    let unit = r.unit_ms(WIDE, 16).unwrap();
    eprintln!("adapter {:?}: a unit is {unit:.4} ms", r.adapter());
    for (count, segments) in [(4, 64), (16, 64), (16, 128), (16, 256)] {
        for shadows in [false, true] {
            let (triangles, rows) = passes(r, staged(count, segments, shadows));
            let (models, maps) = (spent(&rows, "3d models"), spent(&rows, "3d shadow maps"));
            let per = models / unit / (triangles as f64 / f64::from(wprs_scene::TRIANGLE_UNIT));
            eprintln!(
                "{count} balls of {segments} segments, {triangles} triangles, shadows {shadows}: models {models:.4} ms, shadow maps {maps:.4} ms, {per:.3} units per {} triangles",
                wprs_scene::TRIANGLE_UNIT
            );
        }
    }
    r.force_tier(None);
}
