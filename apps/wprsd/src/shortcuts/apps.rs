use super::launch::{entry, folders, installed, launchable};
use super::{Target, target};
use serde_json::Value as Json;
use std::path::PathBuf;
use wprs_core::model::Application as App;

const MOST: usize = 4000;

pub(crate) fn listed() -> Vec<App> {
    listed_in(&folders())
}

pub(super) fn listed_in(folders: &[PathBuf]) -> Vec<App> {
    let mut found: Vec<App> = Vec::new();
    let files = folders.iter().filter_map(|d| std::fs::read_dir(d).ok()).flatten().flatten();
    for file in files {
        let path = file.path();
        let id = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_owned();
        let named = path.extension().is_some_and(|e| e == "desktop") && super::launch::valid_id(&id);
        if !named || found.iter().any(|a| a.id == id) || found.len() >= MOST {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let entry = entry(&text);
        if launchable(&id, &entry).is_err() || entry.get("NoDisplay") == Some(&"true") {
            continue;
        }
        let name = entry.get("Name").filter(|n| !n.is_empty()).map_or(id.clone(), |n| (*n).to_owned());
        found.push(App { id, name, icon: entry.get("Icon").copied().unwrap_or_default().to_owned() });
    }
    found.sort_by_key(|a| (a.name.to_lowercase(), a.id.clone()));
    found
}

fn address(text: &str) -> Result<(), String> {
    let rest = text.strip_prefix("https://").ok_or("an address starts with https://")?;
    let host = rest.split(['/', '?', '#']).next().unwrap_or_default();
    let spaced = text.chars().any(|c| c.is_whitespace() || c.is_control());
    (!host.is_empty() && !host.starts_with(['.', ':', '@']) && !spaced && text.len() <= 2048)
        .then_some(())
        .ok_or_else(|| "this is not an address of at most 2048 characters".into())
}

pub(crate) fn check(value: &Json) -> Result<(), String> {
    if value.is_null() {
        return Ok(());
    }
    let Some(target) = target(value) else {
        let uri = value.get("uri").and_then(Json::as_str);
        return Err(match uri.map(address) {
            Some(Err(e)) => e,
            _ => "choose an https:// address, a file, a folder or an application id".into(),
        });
    };
    match target {
        Target::Uri(uri) => address(&uri),
        Target::File(path) => path.is_file().then_some(()).ok_or_else(|| "no such file".into()),
        Target::Folder(path) => path.is_dir().then_some(()).ok_or_else(|| "no such folder".into()),
        Target::App(id) => {
            if crate::flatpak::app_id().is_some() {
                return Err("applications open only outside the Flatpak".into());
            }
            installed(&id).map(drop).map_err(|e| format!("{e}"))
        }
    }
}
