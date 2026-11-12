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
    bail!(match like {
        Some(like) => said(
            "editor.value_unfit",
            "%1 cannot take the value %2; give %3",
            &[pointer, value, &crate::blend::shape(&like)]
        ),
        None => said("editor.value_unread", "%1 cannot take the value %2", &[pointer, value]),
    })
}
