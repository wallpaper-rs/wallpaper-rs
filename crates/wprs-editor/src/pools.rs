use crate::action::PoolEdit;
use crate::layers::walk;
use crate::ops::{Op, escape, set};
use anyhow::{Result, bail, ensure};
use serde_json::{Map, Value, json};
use std::path::Path;
use wprs_core::message::said;
use wprs_scene::{MAX_POOLS, ScriptApi};

const LIST: &str = "/scene/pools";

fn valid(id: &str) -> Result<()> {
    let fits = (1..=32).contains(&id.len())
        && id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
    ensure!(fits, said("editor.pool_id", "a pool id has 1 to 32 small letters, digits or _", &[]));
    Ok(())
}

fn named(video: &Value, id: &str) -> bool {
    match video {
        Value::String(v) => v == id,
        v => v["map"].as_object().is_some_and(|m| m.values().any(|n| n == id)),
    }
}

pub fn drawn_by(doc: &Value, id: &str) -> Vec<String> {
    let templates = doc["scene"]["templates"].as_object().into_iter().flat_map(|t| t.values());
    let walked = walk(doc).into_iter().map(|(_, l)| l);
    let layers = walked.chain(templates).filter(|l| l["type"] == "video" && named(&l["video"], id));
    layers.filter_map(|l| l["id"].as_str().map(String::from)).collect()
}

pub fn builders(doc: &Value) -> Vec<String> {
    let scripts = doc["scene"]["scripts"].as_array().into_iter().flatten();
    let declared = |s: &&Value| s["uses"].as_array().is_some_and(|u| u.contains(&json!(ScriptApi::Geometry)));
    scripts.filter(declared).filter_map(|s| s["path"].as_str()).map(String::from).collect()
}

pub fn list(doc: &Value) -> Value {
    let pools = doc["scene"]["pools"].as_object().into_iter().flatten();
    let builders = builders(doc);
    let rows = pools.map(|(id, p)| {
        json!({ "id": id, "pointer": format!("{LIST}/{}", escape(id)), "max": p["max"],
            "max_vertices": p["max_vertices"], "texture": p["texture"], "size": p["size"],
            "layers": drawn_by(doc, id), "scripts": builders })
    });
    json!({ "pools": rows.collect::<Vec<_>>(), "limit": MAX_POOLS })
}

fn fresh(e: &PoolEdit) -> Map<String, Value> {
    let mut pool = Map::new();
    pool.insert("type".into(), json!("mesh"));
    pool.insert("max".into(), json!(e.max.unwrap_or(16)));
    pool.insert("max_vertices".into(), json!(e.max_vertices.unwrap_or(1024)));
    pool
}

pub(crate) fn edit(dir: &Path, doc: &Value, e: &PoolEdit) -> Result<Vec<Op>> {
    let id = e.id.as_str();
    let pools = doc["scene"]["pools"].as_object().cloned().unwrap_or_default();
    ensure!(!(e.add && e.remove), said("editor.pool_add_remove", "add or remove a pool, not both", &[]));
    if e.remove {
        ensure!(pools.contains_key(id), said("editor.there_no_pool", "there is no pool \"%1\"", &[&id]));
        let users = drawn_by(doc, id);
        if !users.is_empty() {
            bail!(said(
                "editor.pool_drawn",
                "the pool \"%1\" is drawn by %2; remove those layers first",
                &[&id, &users.join(", ")]
            ));
        }
        let path = if pools.len() == 1 { LIST.to_owned() } else { format!("{LIST}/{}", escape(id)) };
        return crate::shaping::refused(dir, doc, vec![Op::Remove { path }]);
    }
    let mut pool = match (pools.get(id), e.add) {
        (Some(_), true) => bail!(said("editor.pool_taken", "there is already a pool \"%1\"", &[&id])),
        (None, true) => {
            valid(id)?;
            ensure!(
                pools.len() < MAX_POOLS,
                said("editor.pools_most", "a scene has at most %1 pools", &[&MAX_POOLS])
            );
            fresh(e)
        }
        (None, false) => bail!(said("editor.there_no_pool", "there is no pool \"%1\"", &[&id])),
        (Some(old), false) => old.as_object().cloned().unwrap_or_default(),
    };
    if let Some(max) = e.max {
        pool.insert("max".into(), json!(max));
    }
    if let Some(vertices) = e.max_vertices {
        pool.insert("max_vertices".into(), json!(vertices));
    }
    if let Some(texture) = e.texture.as_deref().filter(|t| !t.is_empty()) {
        pool.insert("texture".into(), json!(texture));
    }
    if let Some(size) = e.size {
        pool.insert("size".into(), json!(size));
    }
    for key in e.clear.iter().filter(|k| matches!(k.as_str(), "texture" | "size")) {
        pool.remove(key);
    }
    let path = format!("{LIST}/{}", escape(id));
    crate::shaping::refused(dir, doc, set(doc, &path, Value::Object(pool)))
}
