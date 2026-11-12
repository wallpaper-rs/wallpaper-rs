use super::posing::rigged;
use super::*;

const VIEW: [f32; 4] = [0.0, 0.0, 200.0, 100.0];

fn walked(layers: Value, seconds: f64) -> Runtime {
    let clips = json!([{ "id": "walk", "animation": "walk" }, { "id": "stroll", "animation": "stroll" }]);
    let m = model("m", json!({ "clips": clips, "animations": layers, "root_motion": true }));
    let mut rt = runtime(rigged(json!({}), json!([m])));
    rt.frame(VIEW, Quality::Q3);
    rt.advance(seconds);
    rt.frame(VIEW, Quality::Q3);
    rt
}

fn moved(rt: &Runtime) -> [f32; 3] {
    let t = rt.puppet("m").unwrap().travel();
    [t[3], t[7], t[11]]
}

fn near(a: [f32; 3], b: [f32; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-3)
}

#[test]
fn root_motion_carries_height_depth_and_heading_over_each_lap() {
    let rt = walked(json!([{ "clip": "stroll" }]), 2.5);
    assert!(near(moved(&rt), [1.0, 1.25, 0.5]), "{:?}", moved(&rt));
    let t = rt.puppet("m").unwrap().travel();
    let turned = (t[0] + 0.5f32.sqrt()).abs() < 1e-3 && (t[2] + 0.5f32.sqrt()).abs() < 1e-3;
    assert!(turned, "two laps and an eighth turn it 225 degrees: {t:?}");
    let start = walked(json!([{ "clip": "stroll" }]), 0.0);
    let (a, b) = (&start.puppet("m").unwrap().posed().palette, &rt.puppet("m").unwrap().posed().palette);
    assert!(a[0].iter().zip(&b[0]).all(|(x, y)| (x - y).abs() < 1e-3), "the root keeps its first pose");
}

#[test]
fn every_playing_layer_shares_the_path_by_its_weight() {
    let mixed = walked(json!([{ "clip": "walk" }, { "clip": "stroll", "weight": 0.5 }]), 0.5);
    assert!(near(moved(&mixed), [0.25, 0.125, 0.25]), "{:?}", moved(&mixed));
    let added =
        walked(json!([{ "clip": "walk" }, { "clip": "stroll", "weight": 0.5, "additive": true }]), 0.5);
    assert!(near(moved(&added), [0.5, 0.125, 0.25]), "{:?}", moved(&added));
    let quiet = walked(json!([{ "clip": "walk" }, { "clip": "stroll", "weight": 0 }]), 0.5);
    assert!(near(moved(&quiet), [0.5, 0.0, 0.0]), "{:?}", moved(&quiet));
}

#[test]
fn the_course_ahead_starts_at_the_layer_and_walks_each_lap() {
    let rt = walked(json!([{ "clip": "walk" }]), 0.5);
    let layer = rt.scene.layer("m").unwrap();
    let path = rt.puppet("m").unwrap().course(layer, 2.0);
    assert_eq!(path.len(), COURSE_STEPS + 1);
    assert!(near(path[0], [0.0; 3]) && near(path[COURSE_STEPS], [2.0, 0.0, 0.0]), "{path:?}");
    let still = walked(json!([{ "clip": "walk", "weight": 0 }]), 0.5);
    let path = still.puppet("m").unwrap().course(still.scene.layer("m").unwrap(), 2.0);
    assert!(path.iter().all(|p| near(*p, [0.0; 3])), "{path:?}");
}

fn still(weights: [f32; 2]) -> Scene {
    let glb = ForgeRig { twist: true, weights, ..ForgeRig::new(2.0, 0.5) }.glb();
    let doc = json!({ "schema": SCHEMA, "canvas": { "size": [200, 100] },
        "assets": { "toy": { "path": "toy.glb" } }, "layers": [model("m", json!({}))] });
    let mut scene = Scene::from_json(&doc.to_string()).unwrap();
    scene.adopt(&BTreeMap::new(), move |_: &str, _: usize| Ok::<_, String>(glb.clone())).unwrap();
    scene
}

#[test]
fn a_still_model_bakes_the_blend_shapes_its_file_sets() {
    let (thin, wide) = (still([0.0, 0.0]), still([1.0, 0.0]));
    assert!(thin.solids["toy"].rig.is_none() && wide.solids["toy"].rig.is_none());
    let (a, b) = (thin.solids["toy"].bounds[1][0], wide.solids["toy"].bounds[1][0]);
    assert!((a - 0.25).abs() < 1e-3 && (b - 0.35).abs() < 1e-3, "{a} {b}");
    let tangent = |s: &Scene| s.solids["toy"].vertices[8..11].to_vec();
    let (flat, turned) = (tangent(&thin), tangent(&still([0.0, 1.0])));
    assert!(flat.iter().zip(&turned).all(|(x, y)| (x + y).abs() < 1e-4), "{flat:?} {turned:?}");
}

#[test]
fn sculpted_shapes_beside_a_model_join_its_blend_shapes() {
    let carved: Vec<Carved> = (0..36)
        .map(|v| Carved { node: 2, primitive: 0, vertex: v, at: [0.5, 0.0, 0.0], normal: [0.0; 3] })
        .collect();
    let (glb, shape) = (ForgeRig::new(2.0, 0.5).glb(), carve_encode(&carved));
    assert_eq!(carve_decode(&shape).unwrap(), carved);
    assert!(carve_decode(&shape[..20]).is_err());
    let doc = json!({ "schema": SCHEMA, "canvas": { "size": [200, 100] },
        "assets": { "toy": { "path": "toy.glb", "shapes": { "smile": "toy-smile-1.wpsh" } } },
        "layers": [model("m", json!({ "shapes": { "smile": 1 } }))] });
    let mut scene = Scene::from_json(&doc.to_string()).unwrap();
    assert!(scene.paths().contains("toy-smile-1.wpsh"));
    let read = move |path: &str, _: usize| {
        Ok::<_, String>(if path.ends_with(".glb") { glb.clone() } else { shape.clone() })
    };
    scene.adopt(&BTreeMap::new(), read).unwrap();
    let m = &scene.solids["toy"];
    let rig = m.rig.as_ref().unwrap();
    assert_eq!(rig.shapes, ["bulge", "smile"]);
    let base = rig.pieces[0].targets[1].1 as usize;
    assert_eq!(m.morphs[base], [0.5, 0.0, 0.0, 0.0]);
    let rt = runtime(scene);
    rt.frame([0.0, 0.0, 200.0, 100.0], Quality::Q3);
    let slots = rt.puppet("m").unwrap().posed().morphs[0];
    assert!(slots.contains(&(base as u32, 1.0)), "{slots:?}");
}
