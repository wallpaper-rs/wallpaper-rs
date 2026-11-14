use crate::scene::qobject::SceneEditor;
use cxx_qt_lib::QString;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use wprs_render::{Rehearsal, Renderer, SceneAssets, Trace, Versus};
use wprs_scene::Quality;

fn quality(spec: &Value, key: &str) -> Option<Quality> {
    serde_json::from_value(spec[key].clone()).ok()
}

fn pair(value: &Value) -> Option<[f32; 2]> {
    let at = |i: usize| value.get(i)?.as_f64().map(|v| v.clamp(0.0, 1.0) as f32);
    Some([at(0)?, at(1)?])
}

pub fn wanted(spec: &Value, doc: &Value) -> (Rehearsal, Option<Versus>) {
    let number =
        |key: &str, default: f64, low: f64, high: f64| spec[key].as_f64().unwrap_or(default).clamp(low, high);
    let pointer = pair(&spec["pointer"]["from"]).map(|from| Trace {
        from,
        to: pair(&spec["pointer"]["to"]).unwrap_or(from),
        click: spec["pointer"]["click"] == true,
    });
    let rehearsal = Rehearsal {
        quality: quality(spec, "quality").unwrap_or_default(),
        battery: spec["battery"] == true,
        calm: spec["calm"] == true,
        edge: number("size", 1280.0, 16.0, 4096.0) as u32,
        time: number("time", 0.0, 0.0, 86_400.0),
        wall: spec["wall"]
            .as_f64()
            .or_else(|| spec["time_of_day"].as_f64().map(wprs_render::on_today))
            .unwrap_or_else(wprs_render::local_wall),
        script_host: None,
        media: None,
        lang: wprs_editor::texts::preview_locale(doc, spec["language"].as_str().unwrap_or_default()),
        frames: number("frames", 1.0, 1.0, 600.0) as usize,
        fps: number("fps", 30.0, 1.0, 240.0) as u32,
        pointer,
    };
    let against = quality(spec, "against");
    let without_bloom = spec["without_bloom"] == true;
    let split = number("split", 0.5, 0.0, 1.0) as f32;
    let versus =
        (against.is_some() || without_bloom).then_some(Versus { quality: against, without_bloom, split });
    (rehearsal, versus)
}

pub fn shoot(dir: &Path, doc: &Value, out: &Path, spec: &Value) -> anyhow::Result<String> {
    if let Some(parent) = out.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    if crate::scenecross::wanted(spec) {
        return crate::scenecross::cross(dir, doc, out, spec);
    }
    let (mut rehearsal, versus) = wanted(spec, doc);
    let played = match &spec["media"] {
        Value::Object(_) => Some(wprs_editor::media::played_from(spec["media"].clone(), dir)?),
        Value::String(file) if !file.is_empty() => Some(wprs_editor::media::played(Path::new(file), dir)?),
        _ => None,
    };
    rehearsal.media = played.as_ref().map(|p| p.state.clone());
    let open = |bloom: bool| -> anyhow::Result<wprs_render::Opened> {
        let tried = wprs_editor::tried::rendered(doc, spec);
        let (scene, props) = wprs_editor::reach::open_with(doc, dir, bloom, &tried)?;
        let mut assets = SceneAssets::read(&scene, wprs_editor::reach::reader(dir))
            .and_then(|a| a.with_scripts(&scene, wprs_editor::reach::reader(dir)))
            .map_err(anyhow::Error::msg)?;
        if let Some(cover) = played.as_ref().and_then(|p| p.cover.as_ref()) {
            wprs_editor::media::dress(&mut assets, &scene, cover);
        }
        Ok((scene, assets, props))
    };
    let reel = Renderer::shared()?.rehearse_versus(open, &rehearsal, versus.as_ref())?;
    let files: Vec<PathBuf> = reel.write(out)?;
    Ok(files.iter().map(|f| f.to_string_lossy()).collect::<Vec<_>>().join("\n"))
}

impl SceneEditor {
    pub fn render(self: Pin<&mut Self>, out: &QString, spec: &QString) {
        let Some(project) = self.project.as_ref() else { return };
        let (dir, doc) = (project.dir.clone(), project.doc.clone());
        let out = PathBuf::from(out.to_string());
        let spec = serde_json::from_str::<Value>(&spec.to_string()).unwrap_or_default();
        self.work(
            move || shoot(&dir, &doc, &out, &spec),
            Box::new(|mut editor, files| editor.as_mut().rendered(QString::from(&files))),
        );
    }
}

#[cfg(test)]
mod tests;
