use super::hands::{hand, put};
use super::parts::parts;
use super::{index, list};
use crate::action::{KeyPick, KeysEdit};
use anyhow::{Result, bail};
use serde_json::Value;
use wprs_core::message::said;

pub(super) fn sized(e: &KeysEdit) -> bool {
    e.lift != 0.0 || e.value.is_some() || e.value_scale.is_some()
}

fn like(doc: &Value, pick: &KeyPick) -> (Option<String>, String, usize) {
    let (at, keys) = list(doc, &pick.pointer).unwrap_or_default();
    let unit = crate::schema::at(doc, &pick.pointer).and_then(|s| s["x-unit"].as_str().map(String::from));
    let width = keys.first().and_then(|k| parts(&k[1])).map_or(0, |p| p.len());
    (crate::kinded::kind(doc, &at), unit.unwrap_or_default(), width)
}

pub(super) fn alike(doc: &Value, picks: &[KeyPick]) -> Result<()> {
    let first = like(doc, &picks[0]);
    if let Some(other) = picks.iter().skip(1).find(|p| like(doc, p) != first) {
        bail!(said(
            "editor.keys_unlike",
            "lift or scale the keys of several values only when they are of one kind and unit; %1 and %2 are not",
            &[&picks[0].pointer, &other.pointer]
        ));
    }
    Ok(())
}

pub(super) fn middles(doc: &Value, picks: &[KeyPick]) -> Vec<f64> {
    let (mut lo, mut hi) = (Vec::<f64>::new(), Vec::<f64>::new());
    for pick in picks {
        let Ok((_, keys)) = list(doc, &pick.pointer) else { continue };
        let found = pick.times.iter().filter_map(|t| index(&keys, &pick.pointer, *t).ok());
        for all in found.filter_map(|i| parts(&keys[i][1])) {
            for (i, x) in all.into_iter().enumerate() {
                if i >= lo.len() {
                    lo.push(x);
                    hi.push(x);
                }
                (lo[i], hi[i]) = (lo[i].min(x), hi[i].max(x));
            }
        }
    }
    lo.into_iter().zip(hi).map(|(a, b)| (a + b) / 2.0).collect()
}

pub(super) fn valued(v: &Value, e: &KeysEdit, mids: &[f64]) -> Option<Value> {
    let k = e.value_scale.unwrap_or(1.0);
    super::parts::changed(v, e.part, |i, x| {
        let pivot = e.value_pivot.or_else(|| mids.get(i).copied()).unwrap_or(0.0);
        pivot + (e.value.unwrap_or(x) - pivot) * k + e.lift
    })
}

pub(super) fn risen(key: &mut Value, e: &KeysEdit) {
    let Some(k) = e.value_scale else { return };
    let color = key[1].is_string();
    let touched = |i: usize| e.part.map_or(!(color && i == 3), |p| p == i);
    for side in ["in", "out"] {
        let Some(mut h) = hand(key, side) else { continue };
        let units = h.split || key[1].is_number();
        h.pairs
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| units && (!h.split || touched(*i)))
            .for_each(|(_, p)| p[1] *= k);
        put(key, side, Some(h));
    }
}
