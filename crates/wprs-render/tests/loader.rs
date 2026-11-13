//! Creating the renderer and starting video on different threads must never deadlock: the
//! graphics driver and a codec plug-in take the dynamic loader's lock and the driver's own lock in
//! opposite orders unless first-time loading is serialised (`wprs_core::loader`).

use std::sync::mpsc;
use std::time::{Duration, Instant};
use wprs_render::Renderer;

const DEADLINE: Duration = Duration::from_secs(120);

#[test]
fn renderer_and_video_start_together_without_hanging() {
    let (done, finished) = mpsc::channel();
    let runner = done.clone();
    std::thread::spawn(move || {
        for _ in 0..40 {
            std::thread::scope(|s| {
                s.spawn(|| drop(Renderer::new()));
                s.spawn(|| drop(Renderer::new()));
                s.spawn(|| drop(wprs_video::init()));
                s.spawn(|| {
                    wprs_video::create_first_elements();
                });
            });
        }
        let _ = runner.send(());
    });
    drop(done);
    let start = Instant::now();
    finished
        .recv_timeout(DEADLINE)
        .unwrap_or_else(|_| panic!("renderer and video start-up hung for {:?}", start.elapsed()));
}
