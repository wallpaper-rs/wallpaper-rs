use serde_json::{Value, json};
use wprs_scene::{ScriptApi, ScriptEvent, imported};

const CALLS: [(&str, ScriptApi); 5] = [
    (".bloom", ScriptApi::Bloom),
    (".spawn(", ScriptApi::Layers),
    (".order(", ScriptApi::Layers),
    (".effect(", ScriptApi::Effects),
    ("console.", ScriptApi::Console),
];

pub(super) const BLIND: [&str; 5] = ["clicks", "geometry", "animations", "shortcuts", "hierarchy"];

const STAGE: [&str; 7] = ["camera", "camera3d", "parallax", "bloom", "background", "ambient", "sky"];

fn staged(source: &str) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_' || c == '$';
    let left = |before: &str| match before.chars().next_back() {
        Some('.') => before.ends_with("wprs."),
        Some(c) => !word(c),
        None => true,
    };
    STAGE.iter().any(|key| {
        let call = format!("scene.{key}");
        let mut found = source.match_indices(&call);
        found.any(|(i, _)| left(&source[..i]) && !source[i + call.len()..].starts_with(word))
    })
}

fn unpacked(source: &str) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_' || c == '$';
    source.match_indices('}').any(|(i, _)| {
        let rest = source[i + 1..].trim_start();
        let Some(rest) = rest.strip_prefix('=').map(str::trim_start) else { return false };
        let rest = rest.strip_prefix("wprs.").unwrap_or(rest);
        let named = rest.strip_prefix("scene").is_some_and(|after| !after.starts_with(word));
        let open = source[..i].rfind('{').unwrap_or(i);
        named && source[open + 1..i].split(|c: char| !word(c)).any(|w| STAGE.contains(&w))
    })
}

pub(super) fn exported(source: &str) -> Vec<String> {
    let mut found = Vec::new();
    for line in source.lines().map(str::trim_start).filter_map(|l| l.strip_prefix("export ")) {
        let line = line.trim_start_matches("async ");
        let named = ["function ", "const ", "let ", "var "].iter().find_map(|k| line.strip_prefix(k));
        let names: Vec<&str> = match named {
            Some(rest) => rest.split(|c: char| !(c.is_alphanumeric() || c == '_')).take(1).collect(),
            None => line.trim_start_matches('{').split('}').next().unwrap_or_default().split(',').collect(),
        };
        let last = |n: &str| n.split_whitespace().last().unwrap_or_default().to_owned();
        found.extend(names.into_iter().map(last).filter(|n| !n.is_empty()));
    }
    found
}

pub fn declaration(source: &str) -> Value {
    let names = exported(source);
    let mut events: Vec<ScriptEvent> =
        names.iter().filter_map(|n| serde_json::from_value(json!(n)).ok()).collect();
    for touch in ["click", "enter", "leave", "move", "down", "up"] {
        if [format!(".on(\"{touch}\""), format!(".on('{touch}'")].iter().any(|t| source.contains(t.as_str()))
        {
            events.extend(serde_json::from_value::<ScriptEvent>(json!(touch)));
        }
    }
    events.sort();
    events.dedup();
    let mut uses = imported(source);
    uses.extend(CALLS.iter().filter(|(call, _)| source.contains(call)).map(|(_, api)| *api));
    uses.extend((staged(source) || unpacked(source)).then_some(ScriptApi::Camera));
    uses.sort();
    uses.dedup();
    let mut entry = json!({});
    if events.iter().any(|e| *e != ScriptEvent::Update) {
        entry["events"] = json!(events);
    }
    if !uses.is_empty() {
        entry["uses"] = json!(uses);
    }
    entry
}

pub fn functions(source: &str) -> Vec<String> {
    let mut names = exported(source);
    names.retain(|n| serde_json::from_value::<ScriptEvent>(json!(n)).is_err());
    names.sort();
    names.dedup();
    names
}

pub fn used_params(source: &str) -> Vec<String> {
    let word = |c: char| c.is_alphanumeric() || c == '_' || c == '$';
    let mut found = Vec::new();
    for (at, _) in source.match_indices("params") {
        let before = source[..at].chars().next_back();
        let rest = &source[at + "params".len()..];
        if before.is_some_and(word) || rest.starts_with(word) {
            continue;
        }
        let name = match rest.chars().next() {
            Some('.') => rest[1..].split(|c: char| !word(c)).next().unwrap_or_default().to_owned(),
            Some('[') => rest[1..].split(']').next().unwrap_or_default().trim_matches(['"', '\'']).to_owned(),
            _ => {
                found.extend(destructured(&source[..at], rest));
                continue;
            }
        };
        found.push(name);
    }
    found.retain(|n| valid_param(n));
    found.sort();
    found.dedup();
    found
}

pub fn valid_param(id: &str) -> bool {
    (1..=32).contains(&id.len())
        && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn braced(inner: &str) -> Vec<String> {
    let parts = inner.split(',').map(|p| p.trim().trim_start_matches("..."));
    let names = parts.filter_map(|p| p.split(|c: char| c == ':' || c == '=' || c.is_whitespace()).next());
    names.map(String::from).collect()
}

fn destructured(before: &str, rest: &str) -> Vec<String> {
    let word = |c: char| c.is_alphanumeric() || c == '_' || c == '$';
    if let Some(inner) =
        rest.trim_start().strip_prefix(':').map(str::trim_start).and_then(|r| r.strip_prefix('{'))
    {
        return braced(inner.split('}').next().unwrap_or_default());
    }
    let head = before.trim_end_matches(|c: char| word(c) || c == '.').trim_end();
    let Some(close) = head.strip_suffix('=').map(str::trim_end).and_then(|h| h.strip_suffix('}')) else {
        return Vec::new();
    };
    close.rfind('{').map(|open| braced(&close[open + 1..])).unwrap_or_default()
}
