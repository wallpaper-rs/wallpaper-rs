mod about;
mod asks;
mod examples;
mod shape;

pub use about::ABOUT;
pub use asks::ASKS;
pub use examples::lines as example_lines;
use serde_json::{Value, json};
use std::sync::OnceLock;

pub(crate) const DEPTH: usize = 6;

fn root() -> &'static Value {
    static ROOT: OnceLock<Value> = OnceLock::new();
    ROOT.get_or_init(|| serde_json::to_value(schemars::schema_for!(crate::Action)).unwrap_or_default())
}

fn variants() -> impl Iterator<Item = &'static Value> {
    ["oneOf", "anyOf"].into_iter().filter_map(|k| root()[k].as_array()).flatten()
}

pub fn names() -> Vec<String> {
    let keys = |v: &Value| v["properties"].as_object().map(|o| o.keys().cloned().collect::<Vec<_>>());
    variants().filter_map(keys).flatten().collect()
}

pub fn about(name: &str) -> Option<&'static str> {
    ABOUT.iter().find(|(n, _)| *n == name).map(|(_, a)| *a)
}

pub fn schema(name: &str) -> Option<Value> {
    let args = variants().find_map(|v| v["properties"].get(name))?;
    Some(json!({ "$defs": root()["$defs"], "title": name, "allOf": [args] }))
}

pub fn describe(name: &str) -> Option<String> {
    let schema = schema(name)?;
    let defs = &schema["$defs"];
    let mut out = vec![format!("{name}: {}", about(name).unwrap_or_default())];
    let args = shape::resolved(&schema["allOf"][0], defs);
    let before = out.len();
    shape::lines(&args, defs, name, DEPTH, &mut out);
    if out.len() == before {
        out.push("  no arguments".into());
    }
    Some(out.join("\n"))
}

pub fn ask_about(name: &str) -> Option<&'static str> {
    ASKS.iter().find(|(n, _)| *n == name).map(|(_, a)| *a)
}

pub fn question(name: &str, args: Value) -> Value {
    json!({ name: args })
}

#[cfg(test)]
mod tests;
