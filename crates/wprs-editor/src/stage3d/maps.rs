use crate::ops::{Op, escape};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use wprs_core::message::said;
use wprs_scene::{LINEAR_MAP_SLOTS, MAP_SLOTS};

fn picture(doc: &Value, id: &str) -> bool {
    let path = doc.pointer(&format!("/scene/assets/{}/path", escape(id))).and_then(Value::as_str);
    let ext = path.and_then(|p| p.rsplit_once('.')).map(|(_, e)| e.to_ascii_lowercase());
    ext.is_some_and(|e| {
        ["png", "jpg", "jpeg", "webp", "gif", "bmp", "ktx2", "hdr", "exr"].contains(&e.as_str())
    })
}

pub(crate) fn maps(doc: &Value, block: &mut Value, given: &BTreeMap<String, Option<String>>) -> Result<()> {
    for (slot, id) in given {
        slot_ok(slot)?;
        match id {
            Some(id) => {
                ensure!(
                    picture(doc, id),
                    said("editor.map_picture", "%1 is not a picture of the scene", &[id])
                );
                block["maps"][slot] = json!(id);
            }
            None => {
                let maps = block.get_mut("maps").and_then(Value::as_object_mut);
                drop(maps.and_then(|m| m.remove(slot)));
            }
        }
    }
    if block["maps"].as_object().is_some_and(|m| m.is_empty()) {
        drop(block.as_object_mut().and_then(|b| b.remove("maps")));
    }
    Ok(())
}

pub(crate) fn data_slot(slot: &str) -> bool {
    slot == "comb" || MAP_SLOTS.iter().position(|s| *s == slot).is_some_and(|k| LINEAR_MAP_SLOTS[k])
}

pub(crate) fn chosen(doc: &Value, set: &Value) -> Vec<Value> {
    MAP_SLOTS
        .iter()
        .filter_map(|slot| {
            let id = set["maps"][slot].as_str()?;
            let at = format!("/scene/assets/{}", escape(id));
            let linear = doc.pointer(&at).is_some_and(|a| a["linear"] == json!(true));
            Some(json!({ "slot": slot, "asset": id, "path": doc.pointer(&format!("{at}/path")),
                "warn": data_slot(slot) && !linear }))
        })
        .collect()
}

fn slot_ok(slot: &str) -> Result<()> {
    ensure!(
        MAP_SLOTS.contains(&slot) || slot == "comb",
        said(
            "editor.material_map_slot",
            "%1 is not a map of a material; use base, orm, normal, occlusion, emissive or comb",
            &[&slot]
        )
    );
    Ok(())
}

pub(crate) fn files(
    dir: &Path,
    doc: &Value,
    block: &mut Value,
    given: &BTreeMap<String, PathBuf>,
) -> Result<Vec<Op>> {
    let mut ops = Vec::new();
    for (slot, file) in given {
        slot_ok(slot)?;
        let (id, path, fresh) = crate::assets::imported(dir, doc, file, None)?;
        let made = crate::assets::sheet(dir, doc, &id, &path, (data_slot(slot), None));
        ops.extend(crate::assets::undone(dir, (&path, fresh), made)?);
        block["maps"][slot] = json!(id);
    }
    Ok(ops)
}
