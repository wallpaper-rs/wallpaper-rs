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

#[test]
fn the_turbulence_phase_stays_small_after_days_of_clock() {
    let mut sim = Sim::default();
    sim.counts[2] = 1;
    sim.forces[0] = [3.0, 20.0, 0.05, 0.0];
    sim.forces[1] = [30.0, 0.2, 1.0, 1.0];
    for days in [0.0, 1.0, 3.0, 30.0] {
        sim.phases(days * 86_400.0 + 0.5);
        let phase = sim.forces[2];
        assert!((0.0..256.0).contains(&phase[0]) && (0.0..256.0).contains(&phase[1]), "{days}: {phase:?}");
    }
    sim.phases(256.0 / 0.2);
    assert!(sim.forces[2][1] < 1e-2 || sim.forces[2][1] > 255.99, "{:?}", sim.forces[2]);
}
