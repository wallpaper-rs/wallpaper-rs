use crate::bus;
use crate::scene::qobject::SceneEditor;
use cxx_qt::Threading;
use cxx_qt_lib::QString;
use serde_json::json;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

const LISTEN: Duration = Duration::from_millis(90);
const MISSES: u32 = 5;
static HEARING: AtomicU64 = AtomicU64::new(0);

impl SceneEditor {
    pub fn hear(mut self: Pin<&mut Self>, on: bool) {
        let output = self.output.to_string();
        let round = HEARING.fetch_add(1, Ordering::SeqCst) + 1;
        if !on || output.is_empty() {
            self.as_mut().set_levels(QString::default());
            return;
        }
        let qt = self.qt_thread();
        bus::spawn(async move {
            let mut misses = 0;
            while HEARING.load(Ordering::SeqCst) == round && misses < MISSES {
                match bus::call("Inputs1", "Bands", vec![json!(output)]).await {
                    Ok(reply) => {
                        misses = 0;
                        let reply = QString::from(&reply);
                        qt.queue(move |mut editor| {
                            if HEARING.load(Ordering::SeqCst) == round {
                                editor.as_mut().set_levels(reply);
                            }
                        })
                        .ok();
                    }
                    Err(_) => misses += 1,
                }
                tokio::time::sleep(LISTEN).await;
            }
        });
    }
}
