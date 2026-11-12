use super::super::steer::Steer;
use super::*;
use wprs_video::Cut;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/magenta-64x32.webm");
const FRAME: Duration = Duration::from_millis(60);

fn heard() -> (Settings, bool) {
    let (transition, max_fps, volume, muted) = (Duration::ZERO, 60, 1.0, false);
    let (software_parallax, flash_guard, scene_sound) = (false, false, true);
    (
        Settings {
            transition,
            max_fps,
            volume,
            muted,
            software_parallax,
            flash_guard,
            scene_sound,
            duck: 1.0,
            quality: Default::default(),
        },
        false,
    )
}

fn frames(slot: &PlayerSlot, span: Duration) -> Vec<(Instant, Duration)> {
    let (start, mut seen) = (Instant::now(), Vec::<(Instant, Duration)>::new());
    while start.elapsed() < span {
        let pts = lock(&slot.inbox.frame).take().and_then(|f| f.pts);
        let fresh = pts.filter(|pts| seen.last().is_none_or(|(_, last)| last != pts));
        fresh.into_iter().for_each(|pts| seen.push((Instant::now(), pts)));
        std::thread::sleep(Duration::from_millis(4));
    }
    seen
}

fn playing(slot: &PlayerSlot, within: impl Fn(Duration) -> bool) {
    let (start, mut last) = (Instant::now(), None::<Duration>);
    while start.elapsed() < Duration::from_secs(30) {
        let pts = lock(&slot.inbox.frame).take().and_then(|f| f.pts).filter(|pts| within(*pts));
        if let (Some(pts), Some(was)) = (pts, last)
            && pts > was
        {
            return;
        }
        last = pts.or(last);
        std::thread::sleep(Duration::from_millis(4));
    }
    panic!("the video did not play within 30 s");
}

#[test]
fn a_steered_part_and_speed_reach_a_video_with_sound_opened_plain() {
    if let Err(e) = wprs_video::init() {
        return assert!(std::env::var_os("WPRS_REQUIRE_VIDEO").is_none(), "video required: {e}");
    }
    let (reel, (tx, _rx), now) = (Reel::plain(FIXTURE.into()), mpsc::channel(), Instant::now());
    let mut slot = PlayerSlot::new(((64, 32), true, 60, 100), now);
    slot.shares = false;
    slot.tick(&reel, heard(), &tx, now);
    assert!(slot.player.as_ref().is_some_and(Player::audio_enabled), "the video opened without sound");
    playing(&slot, |_| true);
    let cut = Cut { speed: 200, start: 300, end: 700 };
    let steer = Steer { cut, jumps: 0, at: 0.0, wants: true, volume: 0.25 };
    assert_eq!((super::super::steer::loud(80, Some(&steer)), super::super::steer::loud(80, None)), (20, 80));
    slot.steer(Some(steer));
    assert_eq!(slot.steered, Some((cut, 0)));
    let range = Duration::from_millis(300) - FRAME..Duration::from_millis(700) + FRAME;
    playing(&slot, |pts| range.contains(&pts));
    frames(&slot, Duration::from_millis(400));
    let seen = frames(&slot, Duration::from_secs(2));
    assert!(seen.len() > 10, "only {} frames after steering", seen.len());
    assert!(seen.iter().all(|(_, pts)| range.contains(pts)), "{seen:?}");
    let speed = rate(&seen, Duration::from_millis(400), 2.0);
    assert!((speed / 2.0 - 1.0).abs() < 0.35, "played at {:.0} %", speed * 100.0);
}

fn rate(seen: &[(Instant, Duration)], lap: Duration, expected: f64) -> f64 {
    let mut laps = 0u32;
    let stream: Vec<(Instant, f64)> = std::iter::once(None)
        .chain(seen.iter().map(Some))
        .zip(seen)
        .map(|(before, &(at, pts))| {
            laps += u32::from(before.is_some_and(|b| pts < b.1));
            (at, (pts + lap * laps).as_secs_f64())
        })
        .collect();
    let Some(&(start, _)) = stream.first() else { return 0.0 };
    let late = |(at, t): &(Instant, f64)| (*at - start).as_secs_f64() - t / expected;
    let third = (stream.len() / 3).max(1);
    let on_time = |part: &[(Instant, f64)]| part.iter().copied().min_by(|a, b| late(a).total_cmp(&late(b)));
    let ends = (on_time(&stream[..third]), on_time(&stream[stream.len() - third..]));
    let (Some(a), Some(b)) = ends else { return 0.0 };
    (b.1 - a.1) / (b.0 - a.0).as_secs_f64()
}
