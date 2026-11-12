use crate::ops::{Op, escape, set};
use crate::strings::{Table, file_name, files, inline, valid_key};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use wprs_core::message::said;
use wprs_format::{MAX_LOCALE_FILE, SCHEMA, valid_tag};

pub struct Write {
    pub path: PathBuf,
    pub body: Option<String>,
}

pub(crate) fn apply(writes: &[Write]) -> Result<()> {
    for w in writes {
        match &w.body {
            Some(body) => {
                fs::create_dir_all(w.path.parent().unwrap_or(Path::new(".")))?;
                fs::write(&w.path, body)?;
            }
            None => drop(fs::remove_file(&w.path)),
        }
    }
    Ok(())
}

fn body(table: &Table) -> Result<String> {
    let text = serde_json::to_string_pretty(table)? + "\n";
    ensure!(
        text.len() as u64 <= MAX_LOCALE_FILE,
        said("editor.locale_most", "a locale file has at most 512 KiB", &[])
    );
    Ok(text)
}

fn checked(doc: &Value, language: &str) -> Result<()> {
    ensure!(
        doc["manifest"]["schema"] == SCHEMA && valid_tag(language),
        said("editor.strings_need", "texts by key need %1 and a language such as en or pt-BR", &[&SCHEMA])
    );
    Ok(())
}

fn tables(doc: &Value, language: &str, next: &Table) -> Vec<Op> {
    let at = format!("/manifest/strings/{}", escape(language));
    match (next.is_empty(), doc.pointer(&at).is_some()) {
        (false, _) => set(doc, &at, json!(next)),
        (true, true) if inline(doc).len() <= 1 => vec![Op::Remove { path: "/manifest/strings".into() }],
        (true, true) => vec![Op::Remove { path: at }],
        (true, false) => Vec::new(),
    }
}

fn features(doc: &Value, on: bool) -> Vec<Op> {
    let at = "/manifest/requires/features";
    let mut list: Vec<Value> = doc.pointer(at).and_then(Value::as_array).cloned().unwrap_or_default();
    if list.contains(&json!("locale_files")) == on {
        return Vec::new();
    }
    list.retain(|f| f != "locale_files");
    if on {
        list.push(json!("locale_files"));
    }
    set(doc, at, json!(list))
}

pub fn text(
    dir: &Path,
    doc: &Value,
    (key, language): (&str, &str),
    text: Option<&str>,
) -> Result<(Vec<Op>, Vec<Write>)> {
    checked(doc, language)?;
    ensure!(
        valid_key(key) && !key.starts_with("wprs."),
        said(
            "editor.strings_key",
            "a key has up to 64 letters, digits, _, . or -, and wprs. keys belong to the engine",
            &[]
        )
    );
    let text = text.filter(|t| !t.trim().is_empty());
    let mut table = files(dir).remove(language);
    let in_file = table.is_some();
    let mut table = table.take().unwrap_or_else(|| inline(doc).remove(language).unwrap_or_default());
    match text {
        Some(t) => drop(table.insert(key.to_owned(), t.to_owned())),
        None => drop(table.remove(key)),
    }
    if in_file {
        let path = dir.join(file_name(language));
        return Ok((Vec::new(), vec![Write { path, body: Some(body(&table)?) }]));
    }
    Ok((tables(doc, language, &table), Vec::new()))
}

pub fn locale(dir: &Path, doc: &Value, language: &str, file: bool) -> Result<(Vec<Op>, Vec<Write>)> {
    checked(doc, language)?;
    let (held, loose) = (inline(doc).remove(language), files(dir).remove(language));
    let path = dir.join(file_name(language));
    if file {
        let Some(table) = held else { return Ok((Vec::new(), Vec::new())) };
        let mut merged = loose.unwrap_or_default();
        merged.extend(table);
        let mut ops = tables(doc, language, &Table::new());
        ops.extend(features(doc, true));
        return Ok((ops, vec![Write { path, body: Some(body(&merged)?) }]));
    }
    let Some(table) = loose else { return Ok((Vec::new(), Vec::new())) };
    let mut merged = held.unwrap_or_default();
    merged.extend(table);
    let mut ops = tables(doc, language, &merged);
    ops.extend(features(doc, files(dir).len() > 1));
    Ok((ops, vec![Write { path, body: None }]))
}
