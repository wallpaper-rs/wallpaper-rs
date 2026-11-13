use super::*;

pub const IRIS: &str = r#"
fn transition(c: Crossing) -> Crossing {
    var o = c;
    let soft = param(0u).x;
    let aspect = c.resolution.x / max(c.resolution.y, 1.0);
    let d = length((c.uv - vec2<f32>(0.5)) * vec2<f32>(aspect, 1.0)) / length(vec2<f32>(aspect, 1.0) * 0.5);
    let k = smoothstep(c.progress * (1.0 + soft) - soft, c.progress * (1.0 + soft), d);
    o.color = mix(after(c.uv), before(c.uv), k);
    return o;
}
"#;

fn iris() -> Json {
    json!({ "id": "porthole", "stage": "transition", "cost": { "taps": 1, "passes": 1, "scale": 1 },
        "params": [{ "id": "soft", "type": "float", "default": 0.1, "min": 0, "max": 1 }] })
}

fn crossing(transition: Json, descriptor: Json, wgsl: &str) -> Scene {
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [64, 32] }, "transition": transition });
    let mut scene = Scene::load(&text.to_string(), &BTreeMap::new()).unwrap_or_else(|e| panic!("{e}"));
    let files = |path: &str, _: usize| match path {
        "t/iris.json" => Ok(descriptor.to_string().into_bytes()),
        "t/iris.wgsl" => Ok(wgsl.as_bytes().to_vec()),
        _ => Err("missing"),
    };
    scene.adopt(&BTreeMap::new(), files).unwrap_or_else(|e| panic!("{e}"));
    scene
}

fn way(params: Json) -> Json {
    json!({ "descriptor": "t/iris.json", "wgsl": "t/iris.wgsl", "duration_ms": 1200, "curve": "in_out_sine",
        "params": params })
}

fn refusal(descriptor: Json, wgsl: &str) -> String {
    let scene = crossing(json!({ "in": way(json!({})) }), descriptor, wgsl);
    scene.passages.coming.clone().expect("declared").expect_err("refused")
}

#[test]
fn a_scene_brings_its_transition_in_and_out() {
    let scene = crossing(json!({ "in": way(json!({ "soft": 0.25 })), "out": way(json!({})) }), iris(), IRIS);
    assert!(scene.features().contains("transition"));
    let coming = scene.passages.way(Way::In).cloned().unwrap().unwrap();
    assert_eq!((coming.way, coming.duration.as_millis()), (Way::In, 1200));
    assert_eq!((coming.slots[0][0], coming.at(0.0), coming.at(1.0)), (0.25, 0.0, 1.0));
    assert!((coming.at(0.5) - 0.5).abs() < 1e-3 && coming.plays_at(Quality::Q0));
    let mut medium = iris();
    medium["cost"]["taps"] = json!(4);
    let medium = crossing(json!({ "in": way(json!({})) }), medium, IRIS).passages.coming.unwrap().unwrap();
    assert!(!medium.plays_at(Quality::Q1) && medium.plays_at(Quality::Q2));
    let out = scene.passages.way(Way::Out).cloned().unwrap().unwrap();
    assert_eq!(out.slots[0][0], 0.1);
    let module = transition_shader(&coming.effect).unwrap();
    let entries: Vec<_> = module.entry_points.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(entries, ["wprs_vs", "wprs_fs"]);
    assert!(scene.passages.problems().is_empty());
    let back = Scene::from_json(&scene.to_json().unwrap()).unwrap();
    assert_eq!(back.transition, scene.transition);
}

#[test]
fn scenes_without_a_transition_read_as_before() {
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [64, 32] } });
    let mut scene = Scene::load(&text.to_string(), &BTreeMap::new()).unwrap();
    scene.adopt(&BTreeMap::new(), |_: &str, _: usize| Err::<Vec<u8>, _>("none")).unwrap();
    assert!(scene.transition.is_none() && scene.passages == Passages::default());
    assert!(!scene.features().contains("transition") && !scene.to_json().unwrap().contains("transition"));
}

#[test]
fn a_bad_transition_falls_back_without_refusing_the_scene() {
    let mut heavy = iris();
    heavy["cost"] = json!({ "taps": 12, "passes": 1, "scale": 1 });
    let mut layered = iris();
    layered["stage"] = json!("layer");
    let mut pictured = iris();
    pictured["params"] = json!([{ "id": "soft", "type": "texture", "default": "white" }]);
    let tapped = IRIS.replace("after(c.uv), before", "tap(c.uv), before");
    for (descriptor, wgsl, problem) in [
        (heavy, IRIS, "more than the 8 a transition may cost"),
        (layered, IRIS, "a layer effect needs passes"),
        (pictured, IRIS, "takes no picture"),
        (iris(), tapped.as_str(), "does not build as a transition shader"),
        (iris(), "fn other() {}", "needs fn transition(value: Crossing) -> Crossing"),
        (iris(), "fn transition(c: Crossing) -> Crossing { loop { } return c; }", "loop"),
    ] {
        let found = refusal(descriptor, wgsl);
        assert!(found.contains(problem), "{found}");
    }
    let wrong = crossing(json!({ "in": way(json!({ "rings": 3 })) }), iris(), IRIS);
    assert_eq!(wrong.passages.problems(), [(Way::In, "its descriptor has no parameter rings")]);
    let missing =
        crossing(json!({ "out": { "descriptor": "t/none.json", "wgsl": "t/iris.wgsl" } }), iris(), IRIS);
    assert!(missing.passages.problems()[0].1.starts_with("t/none.json"));
    let typed = crossing(json!({ "in": way(json!({ "soft": true })) }), iris(), IRIS);
    assert!(typed.passages.problems()[0].1.contains("does not fit"));
}

#[test]
fn the_shape_of_a_transition_is_checked_with_the_scene() {
    let text =
        |t: Json| json!({ "schema": SCHEMA, "canvas": { "size": [64, 32] }, "transition": t }).to_string();
    for (t, problem) in [
        (
            json!({ "in": { "descriptor": "t/a.json", "wgsl": "t/a.wgsl", "duration_ms": 20 } }),
            "/transition/in/duration_ms",
        ),
        (json!({ "out": { "descriptor": "t/a.txt", "wgsl": "t/a.wgsl" } }), "/transition/out/descriptor"),
        (
            json!({ "in": { "descriptor": "t/a.json", "wgsl": "t/a.wgsl", "curve": "wobbly" } }),
            "unknown easing",
        ),
        (json!({ "sideways": {} }), "unknown field"),
    ] {
        let fault = Scene::load(&text(t), &BTreeMap::new()).unwrap_err().to_string();
        assert!(fault.contains(problem), "{fault}");
    }
}
