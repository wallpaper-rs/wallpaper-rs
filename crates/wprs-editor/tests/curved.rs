use serde_json::{Value, json};
use wprs_editor::keys::value_at;
use wprs_scene::{Color, Key, Vec2, Vec3, sample};

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-3
}

fn rendered(keys: &Value, t: f32) -> Vec2 {
    let keys: Vec<Key<Vec2>> = serde_json::from_value(keys.clone()).unwrap();
    sample(&keys, t)
}

#[test]
fn number_keys_of_a_pair_bend_the_same_in_the_editor_and_the_render() {
    for keys in [
        json!([[0, 1, {"out": [1, 0.9]}], [2, 3]]),
        json!([[0, 1], [2, 3, {"in": [0.5, -1]}]]),
        json!([[0, [1, 1], {"out": [1, 0.9]}], [2, [3, 3]]]),
        json!([[0, [1, 2], {"out": [1, 0.2]}], [2, [3, -1]]]),
    ] {
        for t in [0.25, 0.5, 1.0, 1.5, 1.9] {
            let edited = value_at(&json!({ "keys": keys }), f64::from(t));
            let edited = match &edited {
                Value::Number(n) => [n.as_f64().unwrap(); 2],
                list => [list[0].as_f64().unwrap(), list[1].as_f64().unwrap()],
            };
            let shown = rendered(&keys, t);
            assert!(
                near(edited[0], f64::from(shown.0[0])) && near(edited[1], f64::from(shown.0[1])),
                "{keys} at {t}: editor {edited:?}, render {:?}",
                shown.0
            );
        }
    }
}

#[test]
fn a_number_key_of_a_pair_is_written_back_as_the_number_it_was() {
    let keys: Vec<Key<Vec2>> =
        serde_json::from_value(json!([[0, 1, {"out": [1, 0.5]}], [2, [3, 3]]])).unwrap();
    assert_eq!(
        serde_json::to_value(&keys).unwrap(),
        json!([[0.0, 1.0, {"out": [1.0, 0.5]}], [2.0, [3.0, 3.0]]])
    );
    let triple: Vec<Key<Vec3>> = serde_json::from_value(json!([[0, 0.5], [1, [1, 2, 3]]])).unwrap();
    assert_eq!((triple[0].one, triple[0].v.0, triple[1].one), (Some(0.5), [0.5; 3], None));
    assert!(serde_json::from_value::<Vec<Key<Color>>>(json!([[0, 0.5]])).is_err(), "a color is no number");
}

#[test]
fn colors_lists_and_eased_keys_sample_as_the_render_draws_them() {
    let colors = json!([[0, "#ff0000", {"out": [0.5, 0.8]}], [1, "#0000ff80"], [2, "#00ff00", "in_out_sine"], [3, "#ffffff"]]);
    let shown: Vec<Key<Color>> = serde_json::from_value(colors.clone()).unwrap();
    let triples = json!([[0, [0, 1, 2], {"out": [0.3, 0.1]}], [2, [4, -1, 2], {"in": [1, 0.6]}], [3, [1, 1, 1], "out_quad"], [4, [0, 0, 0]]]);
    let drawn: Vec<Key<Vec3>> = serde_json::from_value(triples.clone()).unwrap();
    for t in [0.0, 0.2, 0.5, 0.9, 1.4, 2.3, 2.9, 3.5, 5.0] {
        let edited =
            Color::parse(value_at(&json!({ "keys": colors }), f64::from(t)).as_str().unwrap()).unwrap();
        let render = sample(&shown, t);
        let close = edited.0.iter().zip(render.0).all(|(a, b)| (a - b.clamp(0.0, 1.0)).abs() <= 1.0 / 255.0);
        assert!(close, "color at {t}: editor {:?}, render {:?}", edited.0, render.0);
        let edited = value_at(&json!({ "keys": triples }), f64::from(t));
        let render = sample(&drawn, t).0;
        let same = (0..3).all(|i| near(edited[i].as_f64().unwrap(), f64::from(render[i])));
        assert!(same, "triple at {t}: editor {edited}, render {render:?}");
    }
    let steps = json!({"keys": [[0, true], [1, false]]});
    assert_eq!((value_at(&steps, 0.5), value_at(&steps, 1.0)), (json!(true), json!(false)));
    let eased = json!({"keys": [[0, 0, "in_quad"], [2, 10]]});
    let render: Vec<Key<f32>> = serde_json::from_value(eased["keys"].clone()).unwrap();
    assert!(near(value_at(&eased, 1.0).as_f64().unwrap(), f64::from(sample(&render, 1.0))));
}

#[test]
fn handles_per_part_bend_the_same_in_the_editor_and_the_render() {
    let triples = json!([[0, [0, 1, 2], {"out": [[0.5, 3], [1, -2], [0.2, 0]]}], [2, [4, 1, 2], {"in": [[0.5, 1], [0.5, 0.5], [1, 1]]}], [3, [0, 0, 0]]]);
    let drawn: Vec<Key<Vec3>> = serde_json::from_value(triples.clone()).unwrap();
    let colors = json!([[0, "#000000", {"out": [[1, 0.9], [0.5, 0], [0.5, 0.2], [0.5, 0]]}], [2, "#ffffff"]]);
    let shown: Vec<Key<Color>> = serde_json::from_value(colors.clone()).unwrap();
    for t in [0.1, 0.5, 1.0, 1.6, 2.4] {
        let edited = value_at(&json!({ "keys": triples }), f64::from(t));
        let render = sample(&drawn, t).0;
        assert!(
            (0..3).all(|i| near(edited[i].as_f64().unwrap(), f64::from(render[i]))),
            "{t}: {edited} {render:?}"
        );
        let edited =
            Color::parse(value_at(&json!({ "keys": colors }), f64::from(t)).as_str().unwrap()).unwrap();
        let render = sample(&shown, t);
        let close = edited.0.iter().zip(render.0).all(|(a, b)| (a - b.clamp(0.0, 1.0)).abs() <= 1.0 / 255.0);
        assert!(close, "color at {t}: editor {:?}, render {:?}", edited.0, render.0);
    }
    let red = sample(&shown, 1.0).0;
    assert!(red[0] > red[1] + 0.2 && red[2] > red[1], "each channel follows its own handle: {red:?}");
}
