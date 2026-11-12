use super::*;

fn pair(debug: Option<Value>) -> Scene {
    let models = BTreeMap::from([("box", cube(unlit("skin", [0.9, 0.1, 0.1, 1.0])))]);
    let at = |id: &str, x: f32| json!({ "id": id, "type": "model", "model": "box", "position": [x, 0, 0], "scale": [0.4, 0.4, 0.4] });
    let mut s = scene(json!({}), json!([at("a", -1.2), at("b", 1.2)]), models);
    if let Some(d) = debug {
        s.ext.insert("editor".into(), json!({ "debug3d": d }));
        s.drafted();
    }
    s
}

#[test]
fn a_debug_view_of_one_material_leaves_the_others_drawn_as_they_are() {
    let Some(r) = renderer() else { return };
    let plain = render(r, pair(None), Quality::Q3);
    let red = rgb(&plain, 19, 32);
    assert!(red[0] > 200 && red[1] < 60 && rgb(&plain, 45, 32) == red, "{red:?} {:?}", rgb(&plain, 45, 32));
    let all = render(r, pair(Some(json!("matte"))), Quality::Q3);
    assert!(near(rgb(&all, 19, 32), [255; 3], 2) && near(rgb(&all, 45, 32), [255; 3], 2));
    let one = json!({ "show": "matte", "layer": "a", "material": "skin" });
    let px = render(r, pair(Some(one)), Quality::Q3);
    assert!(near(rgb(&px, 19, 32), [255; 3], 2), "{:?}", rgb(&px, 19, 32));
    assert_eq!(rgb(&px, 45, 32), red, "the other model keeps its own colors");
    let other = json!({ "show": "shadows", "layer": "b", "material": "skin" });
    let px = render(r, pair(Some(other)), Quality::Q3);
    assert_eq!(rgb(&px, 19, 32), red);
    assert!(near(rgb(&px, 45, 32), [255; 3], 2), "an unshadowed face shows white: {:?}", rgb(&px, 45, 32));
    let mut saved = pair(None);
    saved
        .ext
        .insert("editor".into(), json!({ "debug3d": { "show": "matte", "layer": "a", "material": "skin" } }));
    assert_eq!(render(r, saved, Quality::Q3), plain, "a shown wallpaper never draws a debug view");
}
