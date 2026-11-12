use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::time::Duration;
use wprs_core::message::{Message, said};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Screen {
    pub name: String,
    pub quality: String,
    pub error: Option<String>,
}

pub fn build(dir: &Path, doc: &Value, out: &Path) -> Result<()> {
    let before = fs::metadata(out).and_then(|m| Ok((m.modified()?, m.len()))).ok();
    crate::package::build_saved(dir, doc, out, None)?;
    let after = fs::metadata(out)?;
    if let Some((time, len)) = before {
        let same = |a: std::time::SystemTime, b: std::time::SystemTime| {
            let secs =
                |t: std::time::SystemTime| t.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs());
            secs(a).ok() == secs(b).ok()
        };
        if len == after.len() && same(time, after.modified()?) {
            fs::File::options().write(true).open(out)?.set_modified(time + Duration::from_secs(1))?;
        }
    }
    Ok(())
}

fn titled(manifest: &Value) -> String {
    let text = |v: &Value| v.as_str().map(str::to_owned);
    match &manifest["title"] {
        Value::Object(map) => {
            let own = wprs_core::locale::languages().into_iter().find_map(|l| text(map.get(&l)?));
            let default = manifest["default_locale"].as_str().and_then(|l| text(map.get(l)?));
            own.or(default).or_else(|| map.values().find_map(text)).unwrap_or_default()
        }
        other => text(other).unwrap_or_default(),
    }
}

pub fn title(doc: &Value) -> String {
    Some(titled(&doc["manifest"])).filter(|t| !t.is_empty()).unwrap_or_else(|| "scene".into())
}

pub fn screens(state: &Value, uid: &str, output: &str) -> Vec<Screen> {
    let Some(outputs) = state["outputs"].as_object() else { return Vec::new() };
    let shown = outputs.iter().filter(|(name, o)| {
        o["item"] == uid
            && (output.is_empty() || name.as_str() == output)
            && !o["drafted"].as_bool().unwrap_or(false)
    });
    shown
        .map(|(name, o)| Screen {
            name: name.clone(),
            quality: o["quality"].as_str().unwrap_or_default().to_uppercase(),
            error: o["error"].as_str().map(str::to_owned),
        })
        .collect()
}

pub fn settled(screens: &[Screen]) -> bool {
    !screens.is_empty() && screens.iter().all(|s| !s.quality.is_empty() || s.error.is_some())
}

pub fn done(doc: &Value, shown: &[Screen]) -> Message {
    let title = title(doc);
    let version = doc["manifest"]["version"].as_str().unwrap_or_default();
    let version = version.to_owned();
    match shown {
        [] => said("editor.applied_unseen", "applied: %1 %2; no screen shows it yet", &[&title, &version]),
        [one] if !one.quality.is_empty() => said(
            "editor.applied_one",
            "applied: %1 %2 on %3, quality %4",
            &[&title, &version, &one.name, &one.quality],
        ),
        _ => {
            let each = |s: &Screen| match s.quality.is_empty() {
                true => s.name.clone(),
                false => format!("{} ({})", s.name, s.quality),
            };
            let list = shown.iter().map(each).collect::<Vec<_>>().join(", ");
            said("editor.applied_many", "applied: %1 %2 on %3", &[&title, &version, &list])
        }
    }
}

pub fn failed(shown: &[Screen]) -> Option<Message> {
    let why = shown.iter().find_map(|s| s.error.as_deref().map(|e| (s.name.as_str(), e)))?;
    Some(said("editor.applied_failed", "the screen %1 could not show it: %2", &[&why.0, &why.1]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn doc() -> Value {
        json!({"manifest": {"id": "org.example.clock", "version": "1.0.2", "title": "My clock"}})
    }

    #[test]
    fn the_message_names_the_scene_its_version_the_screen_and_the_level() {
        let state = json!({"outputs": {
            "DP-1": {"item": "u1", "quality": "q3", "error": null},
            "DP-2": {"item": "u2", "quality": "q1", "error": null},
            "DP-3": {"item": "u1", "quality": "q2", "error": null, "drafted": true}}});
        let one = screens(&state, "u1", "");
        assert_eq!(one.len(), 1, "a screen busy with a preview, or showing another wallpaper, was listed");
        assert_eq!(done(&doc(), &one).to_string(), "applied: My clock 1.0.2 on DP-1, quality Q3");
        let state =
            json!({"outputs": {"A": {"item": "u", "quality": "q3"}, "B": {"item": "u", "quality": ""}}});
        let two = screens(&state, "u", "");
        assert!(!settled(&two));
        assert_eq!(done(&doc(), &two).to_string(), "applied: My clock 1.0.2 on A (Q3), B");
        assert!(screens(&state, "u", "B").len() == 1 && screens(&state, "u", "C").is_empty());
        assert_eq!(done(&doc(), &[]).to_string(), "applied: My clock 1.0.2; no screen shows it yet");
    }

    #[test]
    fn a_screen_that_could_not_show_it_is_said() {
        let state = json!({"outputs": {"A": {"item": "u", "quality": "", "error": "boom"}}});
        let shown = screens(&state, "u", "");
        assert!(settled(&shown));
        assert_eq!(failed(&shown).unwrap().to_string(), "the screen A could not show it: boom");
        assert!(failed(&screens(&json!({"outputs": {}}), "u", "")).is_none());
    }
}
