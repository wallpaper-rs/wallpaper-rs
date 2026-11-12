use super::super::{SceneSpec, SceneState};
use crate::compositor::power::Power;
use crate::scenes::ScenePack;
use std::sync::Arc;
use std::time::{Duration, Instant};
use wprs_core::{AutoPace, QualityChoice};
use wprs_scene::{Scene, Tier};

fn snowfall(quality: Option<QualityChoice>) -> SceneSpec {
    let scene = Scene::from_json(include_str!("../../../../../../scenes/snowfall/scene.json")).unwrap();
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("../../../../../../scenes/snowfall/manifest.json")).unwrap();
    let properties = serde_json::from_value(manifest["properties"].clone()).unwrap();
    let pack = Arc::new(ScenePack::bare(scene, properties, Vec::new()));
    let values = pack.props.values(&[]);
    SceneSpec {
        values,
        pack,
        extra: Vec::new(),
        clips: Vec::new(),
        hold: None,
        drafted: false,
        moving: false,
        quality,
        span: None,
    }
}

fn level(state: &SceneState) -> (&'static str, &'static str) {
    (state.quality().quality, state.quality().reason)
}

struct Fed {
    state: SceneState,
    at: Instant,
    sample: u64,
}

impl Fed {
    fn new(spec: SceneSpec, power: Power) -> Fed {
        let mut state = SceneState::new(spec, power);
        state.quality = state.level(power);
        Fed { state, at: Instant::now(), sample: 0 }
    }

    fn feed(&mut self, seconds: u64, ms: f64, power: Power) -> (&'static str, &'static str) {
        for _ in 0..seconds * 60 {
            (self.at, self.sample) = (self.at + Duration::from_micros(16_667), self.sample + 1);
            self.state.judge((self.sample, ms), power, 60, self.at);
            self.state.quality = self.state.level(power);
        }
        level(&self.state)
    }
}

#[test]
fn slow_gpu_frames_lower_the_reported_quality_and_say_why() {
    let smooth = Power { pace: AutoPace::Smooth, ..Power::new(Tier::T3, false, 0) };
    assert_eq!(level(&SceneState::new(snowfall(None), Power::new(Tier::T3, true, 0))), ("q2", "battery"));
    let mut fed = Fed::new(snowfall(None), smooth);
    assert_eq!(level(&fed.state), ("q3", "full"));
    assert_eq!(fed.feed(3, 50.0, smooth), ("q2", "slow"));
}

#[test]
fn measured_frames_raise_the_level_past_what_the_model_of_the_card_allows() {
    let mains = Power::new(Tier::T1, false, 0);
    let mut fed = Fed::new(snowfall(None), mains);
    assert_eq!(level(&fed.state), ("q1", "cost"));
    assert_eq!(fed.feed(20, 1.0, mains), ("q3", "full"), "short frames did not raise the level");
    let battery = Power::new(Tier::T1, true, 0);
    let mut fed = Fed::new(snowfall(None), battery);
    assert_eq!(fed.feed(20, 1.0, battery), ("q1", "battery"), "on battery the level rose or lost its reason");
}

#[test]
fn a_heavy_scene_first_halves_its_pace_and_never_under_a_fixed_level() {
    let mains = Power::new(Tier::T3, false, 0);
    let mut fed = Fed::new(snowfall(None), mains);
    assert_eq!(fed.feed(4, 25.0, mains), ("q3", "full"));
    assert_eq!((fed.state.pilot.pace(), fed.state.fps(mains, 60)), (2, 30), "the pace was not halved");
    let smooth = Power { pace: AutoPace::Smooth, ..mains };
    fed.feed(1, 25.0, smooth);
    assert_eq!((fed.state.pilot.pace(), fed.state.fps(smooth, 60)), (1, 60), "smooth kept half the pace");
    let fixed = Some(QualityChoice::Fixed(3));
    let mut fed = Fed::new(snowfall(fixed), mains);
    assert_eq!(fed.state.quality().automatic, "", "frames not measured yet were told as too slow");
    assert_eq!(fed.feed(30, 90.0, mains), ("q3", "user"), "slow frames moved a fixed level");
    assert_eq!((fed.state.pilot.pace(), fed.state.fps(mains, 60)), (1, 60), "a fixed level lost its pace");
    assert_eq!(fed.state.quality().automatic, "lower", "measured slow frames at a fixed level were not told");
}

#[test]
fn a_fixed_level_over_the_estimate_warns_only_until_its_frames_are_measured_to_fit() {
    let software = Power::new(Tier::T0, false, 0);
    let mut fed = Fed::new(snowfall(Some(QualityChoice::Fixed(3))), software);
    assert!(fed.state.over.is_some_and(|(cost, budget)| cost > budget), "{:?}", fed.state.over);
    assert_eq!(fed.state.quality().automatic, "lower");
    assert_eq!(fed.feed(2, 1.0, software), ("q3", "user"));
    assert_eq!(fed.state.quality().automatic, "", "frames measured to fit still warned from the estimate");
}

#[test]
fn a_scene_measured_far_too_slow_at_the_lowest_level_stays_still_only_in_automatic() {
    let mains = Power { pace: AutoPace::Smooth, ..Power::new(Tier::T3, false, 0) };
    let mut fed = Fed::new(snowfall(None), mains);
    assert_eq!(fed.feed(40, 200.0, mains).0, "q0");
    assert!(fed.state.still && fed.state.quality().automatic == "still", "a stalled scene kept moving");
    let fixed = Power { choice: QualityChoice::Fixed(0), ..mains };
    fed.state.level(fixed);
    assert!(!fed.state.still && fed.state.quality().automatic.is_empty(), "a fixed level stayed still");
}
