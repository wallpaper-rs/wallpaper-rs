use crate::bus;
use crate::scene::qobject::SceneEditor;
use serde_json::json;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

const STEP: Duration = Duration::from_millis(33);
static SWEEPING: AtomicU64 = AtomicU64::new(0);

impl SceneEditor {
    pub fn sweep(mut self: Pin<&mut Self>, on: bool) {
        let output = self.output.to_string();
        let round = SWEEPING.fetch_add(1, Ordering::SeqCst) + 1;
        let running = on && !output.is_empty();
        self.as_mut().set_sweeping(running);
        if !running {
            return self.stop_pointer(output);
        }
        bus::spawn(async move {
            let start = Instant::now();
            while SWEEPING.load(Ordering::SeqCst) == round {
                let [x, y] = wprs_editor::wander::at(start.elapsed().as_secs_f64());
                let args = vec![json!(output), json!(x), json!(y), json!(0)];
                if bus::call("Inputs1", "SimulateTouch", args).await.is_err() {
                    break;
                }
                tokio::time::sleep(STEP).await;
            }
        });
    }

    fn stop_pointer(&self, output: String) {
        if !output.is_empty() {
            bus::spawn(async move {
                let args = vec![json!(output), json!(-1.0), json!(-1.0), json!(0)];
                bus::call("Inputs1", "SimulateTouch", args).await.ok();
            });
        }
    }
}
