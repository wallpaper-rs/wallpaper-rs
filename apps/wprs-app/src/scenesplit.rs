use crate::bus;
use crate::scene::qobject::SceneEditor;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use serde_json::{Value, json};
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, Ordering};

static HALVES: AtomicU32 = AtomicU32::new(0);

#[derive(Default)]
pub struct Halves {
    pub on: bool,
    pub ready: bool,
    pub output: String,
    pub pixels: (u32, u32),
}

pub fn without_glow(document: &Value) -> Value {
    let mut plain = document.clone();
    if let Some(canvas) = plain.pointer_mut("/scene/canvas").and_then(Value::as_object_mut) {
        canvas.remove("bloom");
    }
    plain
}

fn release(output: String) {
    if !output.is_empty() {
        bus::spawn(async move {
            bus::call("Daemon1", "ReleaseStream", vec![json!(output)]).await.ok();
        });
    }
}

impl SceneEditor {
    pub fn halve(mut self: Pin<&mut Self>, on: bool) {
        let old = {
            let halves = &mut self.as_mut().rust_mut().halves;
            *halves = Halves { on, ..Halves::default() };
            std::mem::take(&mut halves.output)
        };
        self.as_mut().set_split(QString::from(if on { r#"{"on":true}"# } else { "" }));
        release(old);
        if on {
            self.pair();
        }
    }

    pub(crate) fn pair(mut self: Pin<&mut Self>) {
        let wanted = self.attached;
        if !self.halves.on || wanted == (0, 0) || wanted == self.halves.pixels {
            return;
        }
        let output = format!("preview:{}-g{}", std::process::id(), HALVES.fetch_add(1, Ordering::Relaxed));
        let qt = self.qt_thread();
        let old = {
            let halves = &mut self.as_mut().rust_mut().halves;
            halves.ready = false;
            halves.pixels = wanted;
            std::mem::replace(&mut halves.output, output.clone())
        };
        self.as_mut().set_split(QString::from(r#"{"on":true}"#));
        bus::spawn(async move {
            release(old);
            let args = vec![json!(output), json!(wanted.0), json!(wanted.1)];
            let reply = bus::call("Daemon1", "RequestStream", args).await;
            let reply = reply.map(|r| serde_json::from_str::<Value>(&r).unwrap_or_default());
            qt.queue(move |mut editor| {
                if editor.halves.output != output {
                    return;
                }
                let Ok(r) = reply else { return };
                editor.as_mut().rust_mut().halves.ready = true;
                let found = json!({ "on": true, "node": r[0].as_u64().unwrap_or(0), "serial": r[1].as_f64().unwrap_or(0.0) });
                editor.as_mut().set_split(QString::from(&found.to_string()));
                editor.send();
            })
            .ok();
        });
    }

    pub(crate) fn halved(&self) -> Option<String> {
        self.halves.ready.then(|| self.halves.output.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_other_half_drops_the_glow_of_the_canvas_and_nothing_else() {
        let document =
            json!({ "scene": { "canvas": { "size": [8, 8], "bloom": { "strength": 1.0 } }, "layers": [] } });
        let plain = without_glow(&document);
        assert_eq!(plain["scene"]["canvas"], json!({ "size": [8, 8] }));
        assert_eq!(plain["scene"]["layers"], json!([]));
        assert!(document["scene"]["canvas"]["bloom"].is_object());
        let paged = json!({ "manifest": {} });
        assert_eq!(without_glow(&paged), paged);
    }
}
