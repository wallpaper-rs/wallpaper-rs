use super::Passing;
use serde_json::json;
use std::collections::BTreeMap;
use std::sync::Arc;
use wprs_scene::{Quality, Ready, SCHEMA, Scene};

const WIPE: &str = "fn transition(c: Crossing) -> Crossing {
    var o = c;
    o.color = mix(before(c.uv), after(c.uv), step(c.uv.x, c.progress));
    return o;
}";

fn passages() -> (Arc<Ready>, Arc<Ready>) {
    let way = |ms: u32| json!({ "descriptor": "t/wipe.json", "wgsl": "t/wipe.wgsl", "duration_ms": ms });
    let transition = json!({ "out": way(1000), "in": way(3000) });
    let text = json!({ "schema": SCHEMA, "canvas": { "size": [16, 9] }, "transition": transition });
    let mut scene = Scene::load(&text.to_string(), &BTreeMap::new()).expect("scene");
    let descriptor = json!({ "id": "wipe", "stage": "transition", "cost": { "taps": 1 } });
    let files = |path: &str, _: usize| match path {
        "t/wipe.json" => Ok(descriptor.to_string().into_bytes()),
        "t/wipe.wgsl" => Ok(WIPE.as_bytes().to_vec()),
        _ => Err("missing"),
    };
    scene.adopt(&BTreeMap::new(), files).expect("adopted");
    let ready = |r: Option<wprs_scene::Readied>| r.expect("declared").expect("ready");
    (ready(scene.passages.out.clone()), ready(scene.passages.coming.clone()))
}

#[test]
fn the_leaving_transition_plays_first_and_the_coming_one_after() {
    let (out, coming) = passages();
    let both = Passing {
        steps: vec![(out.clone(), Quality::Q3), (coming, Quality::Q3)],
        pipes: Vec::new(),
        built: None,
        background: [0; 3],
    };
    assert_eq!(both.duration(), 4.0);
    assert_eq!(both.phase(0.0), (0, 0.0));
    assert_eq!(both.phase(0.125), (0, 0.5));
    assert_eq!(both.phase(0.625), (1, 0.5));
    assert_eq!(both.phase(1.0), (1, 1.0));
    let alone =
        Passing { steps: vec![(out, Quality::Q3)], pipes: Vec::new(), built: None, background: [0; 3] };
    assert_eq!((alone.duration(), alone.phase(0.5)), (1.0, (0, 0.5)));
}

#[test]
fn the_choice_of_the_person_rules_over_the_transition_of_the_wallpaper() {
    use super::super::output::Shown;
    use wprs_core::TransitionKind::{Fade, None, Own};
    let shown = || Shown {
        content: super::super::Content::Color,
        fit: wprs_core::Fit::Stretch,
        background: [0; 3],
        image: Option::None,
        transition: Option::None,
        uv: Option::None,
        relief: Option::None,
        scene: Option::None,
        tone: Default::default(),
    };
    let told = |kind, reduced| {
        let (passing, told) = super::choose(&shown(), &shown(), kind, reduced);
        (passing.is_some(), told.plays, told.why)
    };
    assert_eq!(told(Own, false), (false, "fade", "plain"));
    assert_eq!(told(Own, true), (false, "fade", "plain"));
    assert_eq!(told(Fade, false), (false, "fade", "chosen"));
    assert_eq!(told(None, true), (false, "none", "chosen"));
}
