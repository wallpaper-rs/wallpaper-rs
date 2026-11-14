use super::{ask, renderer};
use crate::stage::Stage;
use crate::stage::report::wall;
use crate::{counted, pack, power};
use serde_json::json;
use std::path::Path;
use std::time::{Duration, Instant};

fn wall_clock() {
    let local = chrono::Local::now().naive_local().and_utc().timestamp_millis() as f64 / 1e3;
    assert!((wall() - local).abs() < 1.0, "the scene clock is {} and the local time {local}", wall());
    let start = Instant::now();
    counted::watch(true);
    while start.elapsed() < Duration::from_millis(1100) {
        std::hint::black_box(wall());
    }
    let (made, _) = counted::watch(false);
    assert_eq!(made, 0, "reading the local time for each frame allocated {made} times in a second");
}

pub(super) fn frames() {
    wall_clock();
    let Some(r) = renderer() else { return };
    let listed = ask(json!({ "op": "items", "query": "kind:scene" }));
    let software = r.adapters().iter().any(|a| a.used && a.kind == "software");
    let bound = if software { GUARD_ON_SOFTWARE_DRAWING } else { GUARD_ON_A_GRAPHICS_CARD };
    for item in listed.as_array().unwrap() {
        let title = item["title"].as_str().unwrap();
        crate::settings::setting("playback.allow_flashing", Some("false")).unwrap();
        let (guarded, guarded_gpu, scripted) = steady(r, item);
        crate::settings::setting("playback.allow_flashing", Some("true")).unwrap();
        let (free, free_gpu, _) = steady(r, item);
        eprintln!(
            "{title}: {guarded} and {free} allocations in 60 steady frames besides the GPU calls, \
             {guarded_gpu} and {free_gpu} in them, guarded and not"
        );
        for (state, made) in [("guarded", guarded), ("unguarded", free)] {
            assert!(
                made <= 60 * scripted,
                "{title} made {made} allocations in 60 {state} frames besides the GPU calls"
            );
        }
        let guard = guarded_gpu.saturating_sub(free_gpu);
        assert!(
            scripted > 0 || guard <= 60 * bound,
            "the pass, copy and read back of the flash guard made {guard} allocations inside the GPU calls \
             of 60 frames of {title}, more than {bound} a frame"
        );
    }
    crate::settings::setting("playback.allow_flashing", Some("false")).unwrap();
}

fn steady(r: &wprs_render::Renderer, item: &serde_json::Value) -> (u64, u64, u64) {
    let power = power::Power {
        charging: true,
        saver: false,
        low: false,
        thermal: 0,
        headroom: 0.0,
        still: false,
        percent: -1,
    };
    let (uid, path) = (item["uid"].as_str().unwrap(), item["path"].as_str().unwrap());
    let mut stage = Stage::new(uid.into(), pack::load(Path::new(path)).unwrap());
    let (mut made, mut gpu) = (0, 0);
    for n in 0..90 {
        counted::watch(true);
        let nanos = n * 1_000_000_000 / 60;
        stage.render(r, ((64, 128), nanos, 60), (power, wprs_scene::Tier::T1), &[], true).unwrap();
        let own = counted::watch(false).0;
        let calls = counted::in_gpu_calls();
        if n >= 30 {
            (made, gpu) = (made + own, gpu + calls);
        }
    }
    let scripted = if stage.pack.scripts.is_empty() { 0 } else { STEPS };
    (made, gpu, scripted)
}

const STEPS: u64 = 16;
const GUARD_ON_SOFTWARE_DRAWING: u64 = 30;
const GUARD_ON_A_GRAPHICS_CARD: u64 = 50;
