use crate::ops::escape;
use serde_json::Value;

const PREFIXES: [&str; 4] = ["layer:", "click:", "enter:", "leave:"];

#[derive(Debug, Clone, PartialEq)]
pub enum Hold {
    Parent,
    Attach,
    Mask,
    Param,
    Binding,
    Target,
    Particles,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ref {
    pub at: String,
    pub id: String,
    pub prefix: &'static str,
    pub hold: Hold,
    pub owner: String,
}

fn up(pointer: &str) -> &str {
    pointer.rsplit_once('/').map_or("", |(a, _)| a)
}

fn last(pointer: &str) -> &str {
    pointer.rsplit_once('/').map_or("", |(_, b)| b)
}

fn visit(v: &Value, at: &str, out: &mut Vec<Ref>) {
    match v {
        Value::Object(map) => {
            let layer =
                map.get("type").is_some_and(Value::is_string) && map.get("id").is_some_and(Value::is_string);
            for (key, inner) in map {
                let here = format!("{at}/{}", escape(key));
                match (key.as_str(), inner) {
                    ("parent", Value::String(id)) if layer => out.push(Ref {
                        at: here,
                        id: id.clone(),
                        prefix: "",
                        hold: Hold::Parent,
                        owner: at.to_owned(),
                    }),
                    ("layer", Value::String(id)) => {
                        let hold = match (last(at), last(up(at))) {
                            ("attach", _) => Hold::Attach,
                            ("mask", _) => Hold::Mask,
                            (_, "params") => Hold::Param,
                            _ => Hold::Binding,
                        };
                        let owner = if hold == Hold::Binding { here.clone() } else { at.to_owned() };
                        out.push(Ref { at: here, id: id.clone(), prefix: "", hold, owner });
                    }
                    ("layer", Value::Object(inner)) if inner.get("id").is_some_and(Value::is_string) => {
                        let id = inner["id"].as_str().unwrap_or_default().to_owned();
                        let owner = here.clone();
                        out.push(Ref {
                            at: format!("{here}/id"),
                            id,
                            prefix: "",
                            hold: Hold::Particles,
                            owner,
                        });
                    }
                    _ => visit(inner, &here, out),
                }
            }
        }
        Value::Array(list) => {
            list.iter().enumerate().for_each(|(i, item)| visit(item, &format!("{at}/{i}"), out))
        }
        Value::String(text) if ["target", "trigger", "play_on"].contains(&last(at)) => {
            if let Some(prefix) = PREFIXES.iter().find(|p| text.starts_with(**p)) {
                let id = text[prefix.len()..].to_owned();
                out.push(Ref { at: at.to_owned(), id, prefix, hold: Hold::Target, owner: at.to_owned() });
            }
        }
        _ => {}
    }
}

pub fn refs(doc: &Value) -> Vec<Ref> {
    let mut out = Vec::new();
    visit(&doc["scene"], "/scene", &mut out);
    out.retain(|r| !r.at.starts_with("/scene/ext/"));
    out
}

pub fn users(doc: &Value, id: &str) -> Vec<Ref> {
    refs(doc).into_iter().filter(|r| r.id == id).collect()
}

pub fn swap(value: &mut Value, names: &std::collections::BTreeMap<String, String>) {
    let mut found = Vec::new();
    visit(value, "", &mut found);
    for r in found {
        let Some(new) = names.get(&r.id) else { continue };
        if let Some(slot) = value.pointer_mut(&r.at) {
            *slot = Value::String(format!("{}{new}", r.prefix));
        }
    }
}
