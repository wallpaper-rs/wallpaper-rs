use super::*;
use std::time::{Duration, Instant};

#[test]
fn seeking_a_full_day_into_a_particle_scene_stays_quick() {
    let mut rt = runtime(&scene(sparks(), true));
    let start = Instant::now();
    rt.seek(f64::from(MAX_POSTER_TIME));
    let took = start.elapsed();
    assert!(took < Duration::from_secs(20), "seeking {MAX_POSTER_TIME} s took {took:?}");
    assert!(rt.frame(VIEW, Quality::Q3).time >= 86_000.0);
}
