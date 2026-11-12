use crate::layers::walk;
use serde_json::Value;
use std::collections::HashMap;
use wprs_scene::MAX_DEPTH;

fn ups(doc: &Value) -> HashMap<String, String> {
    let named = walk(doc).into_iter().filter_map(|(_, l)| {
        let parent = l["parent"].as_str().filter(|p| !p.is_empty())?;
        Some((l["id"].as_str()?.to_owned(), parent.to_owned()))
    });
    named.collect()
}

pub fn chain(doc: &Value, id: &str) -> Vec<String> {
    let up = ups(doc);
    let mut out: Vec<String> = Vec::new();
    let mut at = up.get(id);
    while let Some(next) = at.filter(|n| *n != id && !out.contains(n) && out.len() <= MAX_DEPTH) {
        out.push(next.clone());
        at = up.get(next);
    }
    out
}

fn height(up: &HashMap<String, String>, id: &str, depth: usize) -> usize {
    if depth > MAX_DEPTH {
        return 0;
    }
    let below = up.iter().filter(|(_, p)| *p == id).map(|(child, _)| child);
    below.map(|child| 1 + height(up, child, depth + 1)).max().unwrap_or(0)
}

pub fn parents(doc: &Value, id: &str) -> Vec<String> {
    let up = ups(doc);
    let layers = walk(doc);
    let own = layers.iter().find(|(_, l)| l["id"] == id).map(|(p, _)| format!("{p}/"));
    let inside = |pointer: &str| own.as_deref().is_some_and(|o| pointer.starts_with(o));
    let below = height(&up, id, 0);
    layers
        .iter()
        .filter(|(pointer, l)| {
            let Some(other) = l["id"].as_str() else { return false };
            let drawn = !matches!(l["type"].as_str(), Some("sound" | "light"));
            let above = chain(doc, other);
            let fits = above.len() + 1 + below <= MAX_DEPTH;
            drawn && other != id && !above.iter().any(|a| a == id) && !inside(pointer) && fits
        })
        .filter_map(|(_, l)| l["id"].as_str().map(String::from))
        .collect()
}
