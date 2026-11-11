use crate::bus;
use crate::scene::qobject::SceneEditor;
use cxx_qt_lib::QString;
use serde_json::{Value, json};
use std::path::Path;
use std::pin::Pin;

impl SceneEditor {
    pub fn simulate_media(self: Pin<&mut Self>, file: &QString) {
        let output = self.placed.to_string();
        if output.is_empty() {
            return;
        }
        let (track, cover) = match file.to_string() {
            file if file.is_empty() => (String::new(), Vec::new()),
            file => match track_from(&file) {
                Ok(found) => found,
                Err(e) => return self.fail(e),
            },
        };
        bus::spawn(async move {
            bus::call("Inputs1", "SimulateMedia", vec![json!(output), json!(track), json!(cover)]).await.ok();
        });
    }
}

fn track_from(file: &str) -> anyhow::Result<(String, Vec<u8>)> {
    let path = Path::new(file);
    let mut track = wprs_editor::media::simulated(path)?;
    let cover = match track.as_object_mut().and_then(|t| t.remove("cover")) {
        Some(Value::String(rel)) if !rel.is_empty() => {
            let cover = path.parent().unwrap_or(Path::new(".")).join(rel);
            std::fs::read(&cover).map_err(|e| anyhow::anyhow!("cannot read {}: {e}", cover.display()))?
        }
        _ => Vec::new(),
    };
    Ok((track.to_string(), cover))
}
