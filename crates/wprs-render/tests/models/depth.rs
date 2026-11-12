use super::*;

fn model(id: &str, asset: &str, at: [f32; 3]) -> Value {
    json!({ "id": id, "type": "model", "model": asset, "position": at })
}

#[test]
fn an_unlit_cube_fills_the_middle_and_leaves_the_corners_empty() {
    let Some(r) = renderer() else { return };
    let models = BTreeMap::from([("red", cube(unlit("paint", [1.0, 0.0, 0.0, 1.0])))]);
    let px = render(r, scene(json!({}), json!([model("cube", "red", [0.0; 3])]), models), Quality::Q3);
    assert!(near(rgb(&px, 32, 32), [255, 0, 0], 3), "{:?}", rgb(&px, 32, 32));
    assert_eq!(rgb(&px, 1, 1), [0, 0, 0]);
}

#[test]
fn the_nearer_model_hides_the_farther_one_in_a_3d_group_whatever_their_order() {
    let Some(r) = renderer() else { return };
    let models = BTreeMap::from([
        ("red", cube(unlit("paint", [1.0, 0.0, 0.0, 1.0]))),
        ("blue", cube(unlit("paint", [0.0, 0.0, 1.0, 1.0]))),
    ]);
    for flip in [false, true] {
        let (near_one, far_one) =
            (model("near", "red", [0.0, 0.0, 1.5]), model("far", "blue", [0.6, 0.0, -2.0]));
        let list = if flip { json!([near_one, far_one]) } else { json!([far_one, near_one]) };
        let group = json!([{ "id": "world", "type": "group", "space": "3d", "layers": list }]);
        let px = render(r, scene(json!({}), group, models.clone()), Quality::Q3);
        assert!(near(rgb(&px, 32, 32), [255, 0, 0], 3), "{flip}: {:?}", rgb(&px, 32, 32));
    }
}

#[test]
fn flat_layers_stay_in_front_unless_they_test_depth() {
    let Some(r) = renderer() else { return };
    let models = BTreeMap::from([("red", cube(unlit("paint", [1.0, 0.0, 0.0, 1.0])))]);
    let card = |deep: bool| json!({ "id": "card", "type": "solid", "color": "#00ff00", "size": [16, 16], "depth_test": deep });
    for (deep, want) in [(false, [0, 255, 0]), (true, [255, 0, 0])] {
        let group = json!([{ "id": "world", "type": "group", "space": "3d",
            "layers": [card(deep), model("cube", "red", [0.0, 0.0, 1.0])] }]);
        let px = render(r, scene(json!({}), group, models.clone()), Quality::Q3);
        assert!(near(rgb(&px, 32, 32), want, 3), "{deep}: {:?}", rgb(&px, 32, 32));
        assert!(near(rgb(&px, 2, 32), [0, 0, 0], 3));
    }
}

#[test]
fn a_wider_field_of_view_shows_the_model_smaller() {
    let Some(r) = renderer() else { return };
    let models = BTreeMap::from([("red", cube(unlit("paint", [1.0, 0.0, 0.0, 1.0])))]);
    let wide = |fov: f32| {
        let s = scene(
            json!({ "camera3d": { "fov": fov } }),
            json!([model("cube", "red", [0.0; 3])]),
            models.clone(),
        );
        let px = render(r, s, Quality::Q3);
        (0..SIDE).filter(|x| rgb(&px, *x, 32)[0] > 128).count()
    };
    let (narrow, broad) = (wide(40.0), wide(90.0));
    assert!(narrow > broad + 8, "{narrow} {broad}");
}

#[test]
fn a_3d_group_inside_another_shares_its_depth() {
    let Some(r) = renderer() else { return };
    let models = BTreeMap::from([
        ("red", cube(unlit("paint", [1.0, 0.0, 0.0, 1.0]))),
        ("blue", cube(unlit("paint", [0.0, 0.0, 1.0, 1.0]))),
    ]);
    let card = json!({ "id": "card", "type": "solid", "color": "#00ff00", "size": [256, 256], "depth_test": true, "transform": { "z": -40 } });
    let inner = json!({ "id": "inner", "type": "group", "space": "3d",
        "layers": [model("near", "blue", [0.0, 0.0, 1.5]), card] });
    let mut far = model("far", "red", [1.5, 0.0, -0.5]);
    far["scale"] = json!(1.5);
    let group = json!([{ "id": "world", "type": "group", "space": "3d",
        "layers": [inner, far] }]);
    let px = render(r, scene(json!({}), group, models), Quality::Q3);
    assert!(near(rgb(&px, 42, 32), [0, 0, 255], 3), "{:?}", rgb(&px, 42, 32));
    assert!(near(rgb(&px, 58, 32), [255, 0, 0], 3), "{:?}", rgb(&px, 58, 32));
    assert!(near(rgb(&px, 2, 32), [0, 255, 0], 3), "{:?}", rgb(&px, 2, 32));
}

fn sparks(depth: f32) -> Value {
    json!({ "particles": { "s": { "capacity": 4, "seed": 1, "perspective": { "focal": 100 },
        "emitters": [{ "shape": { "point": [32, 32] }, "bursts": [{ "count": 1 }] }],
        "init": { "life": [50, 50], "speed": [0, 0], "size": [20, 20], "color": ["#00ff00", "#00ff00"], "depth": [depth, depth] } } } })
}

#[test]
fn particles_in_a_3d_group_hide_one_by_one_behind_the_models_in_front_of_them() {
    let Some(r) = renderer() else { return };
    let models = BTreeMap::from([("red", cube(unlit("paint", [1.0, 0.0, 0.0, 1.0])))]);
    let group = json!([{ "id": "world", "type": "group", "space": "3d", "layers": [
        model("cube", "red", [0.0; 3]),
        { "id": "p", "type": "particles", "system": "s", "depth_test": true }] }]);
    for (depth, quality, want) in [
        (-50.0, Quality::Q3, [0, 255, 0]),
        (50.0, Quality::Q3, [255, 0, 0]),
        (-50.0, Quality::Q0, [255, 0, 0]),
    ] {
        let mut shot = shoot(r, scene(sparks(depth), group.clone(), models.clone()), &[]);
        let px = (0..3).map(|_| shot.frame(r, 0.05, quality)).last().unwrap();
        assert!(near(rgb(&px, 32, 32), want, 8), "{depth} {quality:?}: {:?}", rgb(&px, 32, 32));
    }
}
