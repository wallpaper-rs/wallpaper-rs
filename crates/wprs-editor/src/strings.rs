use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use wprs_format::{ENGINE, LOCALE, locale_file, locale_names};

pub(crate) type Table = BTreeMap<String, String>;

pub(crate) fn valid_key(key: &str) -> bool {
    (1..=64).contains(&key.len()) && key.bytes().all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
}

pub(crate) fn language_of(path: &str) -> Option<String> {
    let name = path.strip_prefix(LOCALE)?.strip_suffix(".json")?;
    locale_file(path).then(|| name.to_owned())
}

pub(crate) fn file_name(language: &str) -> String {
    format!("{LOCALE}{language}.json")
}

fn table(value: &Value) -> Table {
    let pairs = value.as_object().into_iter().flatten();
    pairs.filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_owned()))).collect()
}

pub(crate) fn inline(doc: &Value) -> BTreeMap<String, Table> {
    let tables = doc["manifest"]["strings"].as_object().into_iter().flatten();
    tables.map(|(language, t)| (language.clone(), table(t))).collect()
}

pub(crate) fn files(dir: &Path) -> BTreeMap<String, Table> {
    let read = |name: String| {
        let text = fs::read_to_string(dir.join(&name)).ok()?;
        Some((language_of(&name)?, table(&serde_json::from_str(&text).ok()?)))
    };
    locale_names(dir).into_iter().filter_map(read).collect()
}

fn walk(value: &Value, out: &mut BTreeSet<String>) {
    match value {
        Value::String(s) => {
            if let Some(key) = s.strip_prefix('@').filter(|k| valid_key(k)) {
                out.insert(key.to_owned());
            }
        }
        Value::Array(list) => list.iter().for_each(|v| walk(v, out)),
        Value::Object(map) => map.iter().filter(|(k, _)| *k != "strings").for_each(|(_, v)| walk(v, out)),
        _ => {}
    }
}

fn used(doc: &Value) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    walk(&doc["manifest"], &mut out);
    walk(&doc["scene"], &mut out);
    out
}

pub fn query(dir: &Path, doc: &Value) -> Value {
    let (held, loose) = (inline(doc), files(dir));
    let mut tables: BTreeMap<String, (Table, bool)> = BTreeMap::new();
    for (language, t) in held {
        tables.insert(language, (t, false));
    }
    for (language, t) in loose {
        tables.entry(language).and_modify(|e| e.0.extend(t.clone())).or_insert((t, true));
    }
    let default = doc["manifest"]["default_locale"].as_str();
    let mut languages: BTreeSet<String> = tables.keys().cloned().collect();
    languages.extend(default.map(String::from));
    let spoken = wprs_format::Manifest::deserialize(&doc["manifest"]).map(|m| m.locales());
    languages.extend(spoken.unwrap_or_default());
    let referenced = used(doc);
    let mut names: BTreeSet<String> =
        referenced.iter().filter(|k| !k.starts_with("wprs.")).cloned().collect();
    names.extend(tables.values().flat_map(|(t, _)| t.keys().cloned()));
    let rows = names.iter().map(|key| {
        let texts: BTreeMap<&String, &String> =
            tables.iter().filter_map(|(l, (t, _))| Some((l, t.get(key)?))).collect();
        let missing: Vec<&String> = languages.iter().filter(|l| !texts.contains_key(l)).collect();
        json!({ "key": key, "used": referenced.contains(key), "texts": texts, "missing": missing })
    });
    let engine = ENGINE.iter().map(|(k, en, es)| json!({ "key": format!("wprs.{k}"), "en": en, "es": es }));
    let places = tables.iter().map(|(l, (_, file))| (l, if *file { "file" } else { "manifest" }));
    json!({
        "default": default,
        "languages": languages,
        "places": places.collect::<BTreeMap<_, _>>(),
        "keys": rows.collect::<Vec<_>>(),
        "engine": engine.collect::<Vec<_>>(),
        "reads_files": doc["manifest"]["requires"]["features"].as_array().is_some_and(|f| f.contains(&json!("locale_files"))),
    })
}
