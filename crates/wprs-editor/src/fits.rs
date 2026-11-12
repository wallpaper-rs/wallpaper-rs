use crate::actions::Action;
use crate::ops::{Op, apply};
use anyhow::{Result, bail};
use serde_json::Value;
use wprs_core::message::said;

pub(crate) fn scene(doc: &Value, action: &Action, ops: &[Op]) -> Result<()> {
    let (pointer, value) = match action {
        Action::Set { pointer, value } | Action::Add { pointer, value } => (pointer, value),
        _ => return Ok(()),
    };
    if !pointer.starts_with("/scene") {
        return Ok(());
    }
    let mut staged = doc.clone();
    if apply(&mut staged, ops).is_err() || crate::keys::reads(&staged) || !crate::keys::reads(doc) {
        return Ok(());
    }
    let like = match doc.pointer(pointer) {
        Some(like) => Some(like.clone()),
        None => crate::keys::taken(doc, pointer),
    };
    match like.filter(|like| differs(like, value)) {
        Some(like) => bail!(said(
            "editor.value_unfit",
            "%1 cannot take the value %2; give %3",
            &[pointer, value, &crate::blend::shape(&like)]
        )),
        None => Ok(()),
    }
}

fn differs(like: &Value, value: &Value) -> bool {
    match (like, value) {
        (Value::Number(_), Value::Number(_)) | (Value::Bool(_), Value::Bool(_)) => false,
        (Value::String(_), Value::String(_)) => false,
        (Value::Array(a), Value::Array(b)) => a.len() != b.len(),
        (_, Value::Object(_)) | (Value::Object(_), _) | (Value::Null, _) | (_, Value::Null) => false,
        _ => true,
    }
}

pub(crate) fn ease(field: &str, ease: &Value) -> Result<()> {
    if serde_json::from_value::<wprs_scene::Ease>(ease.clone()).is_ok() {
        return Ok(());
    }
    let names = wprs_scene::easings().join(", ");
    bail!(said(
        "editor.ease_shape",
        "%1 cannot take %2; give one of %3, or four numbers [x1, y1, x2, y2] of a cubic Bézier",
        &[&field, ease, &names]
    ))
}
