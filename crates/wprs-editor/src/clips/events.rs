use crate::action::ClipEventEdit;
use crate::ops::{Op, set};
use anyhow::{Result, anyhow, ensure};
use serde_json::{Value, json};
use wprs_core::message::said;
use wprs_scene::MAX_EVENTS;

fn named(name: &str) -> Result<()> {
    let fits = (1..=64).contains(&name.chars().count()) && !name.contains(['/', '#']);
    ensure!(fits, said("editor.event_name", "an event has a name of 1 to 64 characters without / or #", &[]));
    Ok(())
}

fn rounded(t: f64) -> f64 {
    (t * 1e4).round() / 1e4
}

pub(crate) fn event(doc: &Value, e: &ClipEventEdit) -> Result<Vec<Op>> {
    let ClipEventEdit { layer, clip: id, name, at, time, to, remove } = e;
    let (path, l) = crate::actions::layer(doc, layer)?;
    let (i, found) = super::found(l, id)?;
    let mut clip = found.clone();
    named(name)?;
    let span = match (clip["to"].as_f64(), clip["from"].as_f64().unwrap_or(0.0)) {
        (Some(end), from) => (end - from).max(0.0),
        (None, _) => super::MAX_LENGTH as f64,
    };
    if let Some(t) = time {
        ensure!(
            t.is_finite() && (0.0..=span).contains(t),
            said("editor.event_time", "an event is within the clip, from 0 to %1 s", &[&rounded(span)])
        );
    }
    let mut list = clip["events"].as_array().cloned().unwrap_or_default();
    let near = |v: &Value, t: f64| (v["t"].as_f64().unwrap_or(0.0) - t).abs() < 1e-3;
    let hit = list.iter().position(|v| v["name"] == name.as_str() && at.is_none_or(|t| near(v, t)));
    match (hit, remove) {
        (Some(i), true) => {
            list.remove(i);
        }
        (None, true) => {
            return Err(anyhow!(said("editor.no_event", "the clip has no event \"%1\"", &[name])));
        }
        (Some(i), false) => {
            if let Some(t) = time {
                list[i]["t"] = json!(rounded(*t));
            }
            if let Some(new) = to {
                named(new)?;
                list[i]["name"] = json!(new);
            }
        }
        (None, false) => {
            let Some(t) = time else {
                return Err(anyhow!(said("editor.no_event", "the clip has no event \"%1\"", &[name])));
            };
            ensure!(
                list.len() < MAX_EVENTS,
                said("editor.events_most", "a clip has at most %1 events", &[&MAX_EVENTS])
            );
            list.push(json!({ "t": rounded(*t), "name": to.as_deref().unwrap_or(name) }));
        }
    }
    list.sort_by(|a, b| a["t"].as_f64().unwrap_or(0.0).total_cmp(&b["t"].as_f64().unwrap_or(0.0)));
    match list.is_empty() {
        true => drop(clip.as_object_mut().map(|m| m.remove("events"))),
        false => clip["events"] = Value::Array(list),
    }
    let at = format!("{path}/clips/{i}");
    Ok(match clip.get("events") {
        Some(events) => set(doc, &format!("{at}/events"), events.clone()),
        None if found.get("events").is_some() => vec![Op::Remove { path: format!("{at}/events") }],
        None => Vec::new(),
    })
}
