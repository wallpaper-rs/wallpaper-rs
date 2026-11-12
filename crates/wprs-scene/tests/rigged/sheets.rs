use super::kit::arm;
use serde_json::json;
use std::collections::BTreeMap;
use wprs_scene::*;

#[test]
fn picture_masks_of_a_mesh_are_posed_with_it_by_one_hidden_copy_per_picture() {
    let effect =
        |mask: serde_json::Value| json!({ "effect": "waves", "params": { "amplitude": 4 }, "mask": mask });
    let fox = json!({ "id": "fox", "type": "mesh", "mesh": "arm", "texture": "skin", "size": [64, 64], "color": "#ff0000",
        "clips": [{ "id": "wave", "animation": "wave" }], "animations": [{ "clip": "wave" }],
        "mask": { "texture": "ink" },
        "effects": [effect(json!({ "texture": "ink", "channel": "r" })), effect(json!({ "texture": "veil" })),
            effect(json!({ "shape": { "ellipse": { "size": [8, 8] } } }))] });
    let assets = json!({ "arm": { "path": "arm.glb" }, "skin": { "path": "skin.png" }, "ink": { "path": "ink.png" },
        "veil": { "path": "veil.png" } });
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [64, 64] }, "assets": assets,
        "layers": [{ "id": "sheet_0", "type": "solid", "color": "#000000" }, fox] });
    let mut scene = Scene::from_json(&text.to_string()).unwrap();
    let glb = arm().bytes();
    scene.adopt(&BTreeMap::new(), move |_: &str, _: usize| Ok::<_, String>(glb.clone())).unwrap();
    let ids: Vec<&str> = scene.layers.iter().map(|l| l.id.as_str()).collect();
    assert_eq!(ids, ["sheet_0", "sheet_1", "sheet_2", "fox"]);
    let fox = scene.layer("fox").unwrap();
    let masks: Vec<_> = fox.effects.iter().map(|e| e.mask.as_ref().unwrap()).collect();
    assert_eq!(fox.mask.as_ref().unwrap().layer.as_deref(), Some("sheet_1"));
    assert_eq!((masks[0].layer.as_deref(), masks[0].channel), (Some("sheet_1"), Some(MaskChannel::R)));
    assert_eq!(
        (masks[1].layer.as_deref(), masks[1].channel),
        (Some("sheet_2"), Some(MaskChannel::Luminance))
    );
    assert!(masks[2].layer.is_none() && masks[2].shape.is_some(), "shapes stay in the box");
    let copy = scene.layer("sheet_1").unwrap();
    let Kind::Mesh(m) = &copy.kind else { panic!("not a mesh") };
    assert_eq!((m.texture.clone(), m.color.clone()), (Some(Value::Fixed("ink".into())), None));
    assert!(copy.effects.is_empty() && copy.mask.is_none() && copy.visible == Value::Fixed(false));
    assert!(scene.sheets.contains("sheet_2") && !scene.sheets.contains("sheet_0"));
    let again = scene.clone();
    scene.adopt(&BTreeMap::new(), |_: &str, _: usize| Ok::<_, String>(arm().bytes())).unwrap();
    assert_eq!(scene.layers, again.layers, "a second adoption changes nothing");
}
