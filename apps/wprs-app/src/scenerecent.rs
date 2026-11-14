#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[auto_cxx_name]
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type Recents = super::RecentsRust;

        #[qinvokable]
        fn projects(self: &Recents) -> QString;
        #[qinvokable]
        fn forget_project(self: &Recents, folder: &QString);
        #[qinvokable]
        fn is_project(self: &Recents, folder: &QString) -> bool;
        #[qinvokable]
        fn shown(self: &Recents, folder: &QString) -> QString;
    }
}

use cxx_qt_lib::QString;
use qobject::Recents;

#[derive(Default)]
pub struct RecentsRust;

impl Recents {
    pub fn projects(&self) -> QString {
        QString::from(&serde_json::to_string(&list()).unwrap_or_default())
    }

    pub fn forget_project(&self, folder: &QString) {
        forget(Path::new(&folder.to_string()));
    }

    pub fn is_project(&self, folder: &QString) -> bool {
        project::read(Path::new(&folder.to_string())).is_ok()
    }

    pub fn shown(&self, folder: &QString) -> QString {
        let home = dirs::home_dir().map(|h| h.to_string_lossy().into_owned()).unwrap_or_default();
        QString::from(&short_path(&folder.to_string(), &home, 56))
    }
}

pub fn short_path(path: &str, home: &str, max: usize) -> String {
    let home = home.trim_end_matches('/');
    let path = match path.strip_prefix(home) {
        Some(rest) if !home.is_empty() && (rest.is_empty() || rest.starts_with('/')) => format!("~{rest}"),
        _ => path.to_owned(),
    };
    let chars: Vec<char> = path.chars().collect();
    if chars.len() <= max || max < 3 {
        return path;
    }
    let head = (max - 1) / 3;
    let tail = max - 1 - head;
    let mut out: String = chars[..head].iter().collect();
    out.push('…');
    out.extend(&chars[chars.len() - tail..]);
    out
}

use crate::scenefile::projects_dir;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use wprs_editor::project;

fn store() -> PathBuf {
    if crate::bus::checking() {
        return projects_dir().join(".recent.json");
    }
    dirs::state_dir()
        .or_else(dirs::data_local_dir)
        .unwrap_or_default()
        .join("wallpaper-rs")
        .join("recent-projects.json")
}

fn saved() -> Vec<(PathBuf, String)> {
    let text = std::fs::read_to_string(store()).unwrap_or_default();
    let list: Vec<Value> = serde_json::from_str(&text).unwrap_or_default();
    list.iter()
        .filter_map(|e| {
            Some((PathBuf::from(e["folder"].as_str()?), e["title"].as_str().unwrap_or_default().to_owned()))
        })
        .collect()
}

fn keep(list: &[(PathBuf, String)]) {
    let rows: Vec<Value> = list.iter().map(|(f, t)| json!({"folder": f, "title": t})).collect();
    if let Some(parent) = store().parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(store(), Value::Array(rows).to_string()).ok();
}

pub fn put(mut list: Vec<(PathBuf, String)>, folder: &Path, title: &str) -> Vec<(PathBuf, String)> {
    list.retain(|(f, _)| f != folder);
    list.insert(0, (folder.to_owned(), title.to_owned()));
    list.truncate(40);
    list
}

pub fn record(folder: &Path, title: &str) {
    keep(&put(saved(), folder, title));
}

pub fn forget(folder: &Path) {
    let mut list = saved();
    list.retain(|(f, _)| f != folder);
    keep(&list);
}

pub fn order(rows: &mut [Value]) {
    rows.sort_by_key(|r| (r["missing"] == true, std::cmp::Reverse(r["edited"].as_u64().unwrap_or(0))));
}

fn edited(folder: &Path, doc: &Value) -> u64 {
    let entry = project::entry(doc).unwrap_or_default();
    ["manifest.json", entry.as_str()]
        .iter()
        .filter_map(|n| {
            std::fs::metadata(folder.join(n)).ok()?.modified().ok()?.duration_since(UNIX_EPOCH).ok()
        })
        .map(|d| d.as_secs())
        .max()
        .unwrap_or(0)
}

fn poster(folder: &Path, doc: &Value) -> String {
    let named = doc["manifest"]["preview"].as_str().map(|p| folder.join(p));
    named.filter(|p| p.is_file()).map(|p| p.to_string_lossy().into_owned()).unwrap_or_default()
}

fn row(folder: &Path, title: &str) -> Value {
    let languages = wprs_core::locale::languages();
    match project::read(folder) {
        Ok(doc) => json!({
            "folder": folder,
            "title": wprs_editor::meta::title(&doc, &languages),
            "kind": wprs_editor::pages::kind(&doc),
            "version": doc["manifest"]["version"].as_str().unwrap_or_default(),
            "edited": edited(folder, &doc),
            "poster": poster(folder, &doc),
            "missing": false,
        }),
        Err(_) => {
            json!({"folder": folder, "title": title, "kind": "", "edited": 0, "poster": "", "missing": true})
        }
    }
}

pub fn list() -> Vec<Value> {
    let known = saved();
    let mut rows: Vec<Value> = known.iter().map(|(f, t)| row(f, t)).collect();
    for entry in std::fs::read_dir(projects_dir()).into_iter().flatten().flatten() {
        if !known.iter().any(|(f, _)| *f == entry.path()) {
            let found = row(&entry.path(), "");
            if found["missing"] == false {
                rows.push(found);
            }
        }
    }
    order(&mut rows);
    rows
}
