use super::kit::arm;
use wprs_scene::{BoneSet, RigControl, Runtime, ScriptOutputs, Values};
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
    assert!(scene.sheets.get("sheet_2").map(String::as_str) == Some("fox") && !scene.sheets.contains_key("sheet_0"));
    let again = scene.clone();
    scene.adopt(&BTreeMap::new(), |_: &str, _: usize| Ok::<_, String>(arm().bytes())).unwrap();
    assert_eq!(scene.layers, again.layers, "a second adoption changes nothing");
}

fn inked(scene: serde_json::Value) -> Runtime {
    let assets = json!({ "arm": { "path": "arm.glb" }, "ink": { "path": "ink.png" } });
    let mut doc = json!({ "schema": SCHEMA, "canvas": { "size": [64, 64] }, "assets": assets, "layers": [] });
    scene.as_object().unwrap().iter().for_each(|(k, v)| doc[k] = v.clone());
    let mut scene = Scene::from_json(&doc.to_string()).unwrap();
    let glb = arm().bytes();
    scene.adopt(&BTreeMap::new(), move |_: &str, _: usize| Ok::<_, String>(glb.clone())).unwrap();
    Runtime::new(std::sync::Arc::new(scene), Values::new())
}

fn inked_fox(id: &str) -> serde_json::Value {
    json!({ "id": id, "type": "mesh", "mesh": "arm", "size": [64, 64], "mask": { "texture": "ink" } })
}

fn turned(ids: &[&str]) -> ScriptOutputs {
    let bone = BoneSet { rotation: Some(50.0), ..BoneSet::default() };
    let set = RigControl { bones: [("arm".to_string(), bone)].into(), ..RigControl::default() };
    ScriptOutputs { rigs: ids.iter().map(|id| (id.to_string(), set.clone())).collect(), ..Default::default() }
}

fn palette(rt: &Runtime, id: &str) -> Vec<[f32; 12]> {
    rt.puppet(id).unwrap_or_else(|| panic!("no puppet {id}")).posed().palette.clone()
}

#[test]
fn bones_moved_by_scripts_reach_the_hidden_copy_of_a_mask() {
    let mut rt = inked(json!({ "layers": [inked_fox("fox")] }));
    rt.frame([0.0, 0.0, 64.0, 64.0], Quality::Q3);
    let before = palette(&rt, "sheet_0");
    rt.set_script(turned(&["fox"]));
    rt.advance(0.1);
    rt.frame([0.0, 0.0, 64.0, 64.0], Quality::Q3);
    let (fox, sheet) = (palette(&rt, "fox"), palette(&rt, "sheet_0"));
    assert_ne!(before, sheet);
    assert_eq!(fox, sheet);
}

#[test]
fn masks_of_templates_get_hidden_copies_that_follow_each_spawn() {
    let group = json!({ "id": "pack", "type": "group", "layers": [inked_fox("body")] });
    let mut rt = inked(json!({ "templates": { "solo": inked_fox("solo"), "pack": group } }));
    let spawned = vec![
        ("a".to_string(), "solo".to_string()),
        ("b".into(), "pack".into()),
        ("c".into(), "solo".into()),
    ];
    rt.set_script(ScriptOutputs { spawned: spawned.clone(), ..Default::default() });
    rt.set_script(ScriptOutputs { spawned, ..turned(&["a", "b.body"]) });
    for _ in 0..3 {
        rt.advance(0.1);
        rt.frame([0.0, 0.0, 64.0, 64.0], Quality::Q3);
    }
    let frame = rt.frame([0.0, 0.0, 64.0, 64.0], Quality::Q3);
    let drawn: Vec<&str> = frame.draws.iter().map(|d| d.layer.id.as_str()).collect();
    assert!(drawn.contains(&"a.sheet_1") && drawn.contains(&"b.sheet_0"), "{drawn:?}");
    assert_ne!(palette(&rt, "a"), palette(&rt, "c"));
    assert_ne!(palette(&rt, "a.sheet_1"), palette(&rt, "c.sheet_1"));
    let masked = |id: &str| match rt.layer(id).and_then(|l| l.mask.as_ref()) {
        Some(m) => m.layer.clone(),
        None => None,
    };
    assert_eq!(masked("a").as_deref(), Some("a.sheet_1"));
    assert_eq!(masked("b.body").as_deref(), Some("b.sheet_0"));
    assert_eq!(palette(&rt, "a"), palette(&rt, "a.sheet_1"));
    assert_eq!(palette(&rt, "b.body"), palette(&rt, "b.sheet_0"));
    rt.set_script(ScriptOutputs { spawned: Vec::new(), ..Default::default() });
    assert!(rt.layer("a.sheet_1").is_none() && rt.layer("b.sheet_0").is_none());
}

