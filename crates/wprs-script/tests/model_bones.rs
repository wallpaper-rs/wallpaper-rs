mod common;

use common::{sources, text};
use serde_json::json;
use std::collections::BTreeMap;
use wprs_scene::{BoneSet, ForgeRig, LayerRead, Scene};
use wprs_script::{Frame, Limits, Script, Setup};

fn modelled(source: &str) -> Script {
    let doc = json!({ "schema": wprs_scene::SCHEMA, "canvas": { "size": [200, 100] },
        "assets": { "toy": { "path": "toy.glb" } },
        "scripts": [{ "path": "main.js", "uses": ["animations", "layers"] }],
        "layers": [{ "id": "a", "type": "text", "text": "", "size": 10, "font": "serif" },
            { "id": "m", "type": "model", "model": "toy", "clips": [{ "id": "bend", "animation": "bend" }], "hit": ["tip"] }] });
    let mut scene = Scene::from_json(&doc.to_string()).unwrap();
    let glb = ForgeRig::new(2.0, 0.5).glb();
    scene.adopt(&BTreeMap::new(), move |_: &str, _: usize| Ok::<_, String>(glb.clone())).unwrap();
    let setup = Setup::new(&scene, &sources(&[("main.js", source)]), 1, ("en", &json!({})));
    Script::load(&setup, Limits::default()).unwrap()
}

const SOURCE: &str = r#"import { scene } from "wprs";
    const m = scene.layer("m");
    export function update(f) {
        scene.layer("a").text = [m.bones.join(","), m.bone("tip").parent, m.hit(100, 50)].join(" ");
        if (f.time === 0) {
            m.bone("tip").rotation = 20;
            m.bone("tip").position = [10, 5];
            m.impulse("tip", [3, 4]);
        }
    }"#;

#[test]
fn scripts_list_and_drive_the_bones_of_a_model_layer() {
    let mut script = modelled(SOURCE);
    let reads = [(
        "m".to_string(),
        LayerRead { bones: vec![[1.0, 0.0, 0.0, 1.0, 100.0, 50.0]; 3], ..LayerRead::default() },
    )];
    let out = script.update(&Frame { reads: &reads, ..Frame::at(0.0) }).unwrap();
    let said = [out.console.clone(), out.stopped.clone()].concat().join(" ");
    assert!(text(&out, "a").starts_with("root,tip,body root"), "{} {said}", text(&out, "a"));
    let rig = &out.rigs["m"];
    assert_eq!(
        rig.bones["tip"],
        BoneSet { rotation: Some(20.0), position: Some([10.0, 5.0]), ..BoneSet::default() }
    );
    assert_eq!(rig.impulses, [("tip".to_string(), [3.0, 4.0])]);
}
