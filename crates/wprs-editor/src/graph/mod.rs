mod curves;
pub(crate) mod parts;
mod paste;

pub use curves::curves;

use crate::action::{KeyPick, KeysEdit};
use crate::keys::time_of;
use crate::ops::{Op, set};
use anyhow::{Result, anyhow, bail, ensure};
use serde_json::{Value, json};
use wprs_core::message::said;

pub(crate) fn list(doc: &Value, pointer: &str) -> Result<(String, Vec<Value>)> {
    let keyed = |keys: &Vec<Value>| keys.iter().all(|k| k.get(0).is_some_and(Value::is_number));
    match doc.pointer(pointer) {
        Some(Value::Array(keys)) if keyed(keys) && keys.iter().all(Value::is_array) => {
            Ok((pointer.into(), keys.clone()))
        }
        Some(v) if v.get("keys").and_then(Value::as_array).is_some_and(keyed) => {
            Ok((format!("{pointer}/keys"), v["keys"].as_array().cloned().unwrap_or_default()))
        }
        _ => bail!(said("editor.no_keyframes", "%1 has no keyframes", &[&pointer])),
    }
}

pub(crate) fn index(keys: &[Value], pointer: &str, t: f64) -> Result<usize> {
    keys.iter()
        .position(|k| (time_of(k) - t).abs() < 1e-3)
        .ok_or_else(|| anyhow!(said("editor.no_keyframe_s", "%1 has no keyframe at %2 s", &[&pointer, &t])))
}

pub(crate) fn sorted(pointer: &str, keys: &mut [Value]) -> Result<()> {
    keys.sort_by(|a, b| time_of(a).total_cmp(&time_of(b)));
    let met = keys.windows(2).find(|w| time_of(&w[1]) - time_of(&w[0]) < 1e-3);
    if let Some(w) = met {
        bail!(said("editor.keys_meet", "two keys of %1 would meet at %2 s", &[&pointer, &time_of(&w[1])]));
    }
    parts::clamp(keys);
    Ok(())
}

pub(crate) fn edit(doc: &Value, e: &KeysEdit) -> Result<Vec<Op>> {
    ensure!(
        !e.picks.is_empty(),
        said("editor.keys_picks", "picks names the keys to change: a pointer and the times of its keys", &[])
    );
    let numbers = [e.shift, e.pivot, e.lift].into_iter().chain(e.value).chain(e.at);
    ensure!(
        numbers.into_iter().all(f64::is_finite),
        said("editor.keys_finite", "shift, pivot, lift, value and at are finite numbers", &[])
    );
    let ops = if e.picks.iter().any(|p| !p.keys.is_empty()) {
        paste::paste(doc, e)?
    } else if e.in_handle.is_some() || e.out_handle.is_some() || e.straighten {
        bend(doc, e)?
    } else {
        let mut ops = Vec::new();
        for pick in &e.picks {
            ops.extend(moved(doc, pick, e)?);
        }
        ops
    };
    fits(doc, &e.picks, &ops)?;
    Ok(ops)
}

fn moved(doc: &Value, pick: &KeyPick, e: &KeysEdit) -> Result<Vec<Op>> {
    let pointer = pick.pointer.as_str();
    let (at, old) = list(doc, pointer)?;
    ensure!(
        !pick.times.is_empty(),
        said("editor.keys_times", "times lists the seconds of the keys of %1 to change", &[&pointer])
    );
    let scale = e.scale.unwrap_or(1.0);
    ensure!(
        scale.is_finite() && scale > 0.0,
        said(
            "editor.keys_scale",
            "scale is a number over 0 that stretches the times of the picked keys",
            &[]
        )
    );
    let chosen = pick.times.iter().map(|t| index(&old, pointer, *t)).collect::<Result<Vec<_>>>()?;
    let mut keys = old.clone();
    if e.remove {
        let mut left: Vec<Value> =
            keys.into_iter().enumerate().filter(|(i, _)| !chosen.contains(i)).map(|(_, k)| k).collect();
        ensure!(
            !left.is_empty(),
            said(
                "editor.keys_last",
                "keep at least one key of %1; the keyframe action removes the last one",
                &[&pointer]
            )
        );
        parts::clamp(&mut left);
        crate::handles::reach(pointer, &old, &left)?;
        return Ok(set(doc, &at, Value::Array(left)));
    }
    let valued = e.lift != 0.0 || e.value.is_some();
    for &i in &chosen {
        let t = time_of(&keys[i]);
        let new = ((e.pivot + (t - e.pivot) * scale + e.shift) * 1000.0).round() / 1000.0;
        ensure!(
            new >= 0.0,
            said("editor.keys_before_zero", "the keys of %1 would move before 0 s", &[&pointer])
        );
        keys[i][0] = json!(new);
        parts::stretch(&mut keys[i], scale);
        if valued {
            let v = &keys[i][1];
            ensure!(
                e.value.is_none() || e.part.is_some() || v.is_number(),
                said(
                    "editor.keys_part",
                    "part names the part to set in keys of lists and colors: 0 for x or red, 1 for y or green and so on",
                    &[]
                )
            );
            let lift = |x: f64| e.value.unwrap_or(x) + e.lift;
            keys[i][1] = parts::changed(v, e.part, lift).ok_or_else(|| {
                anyhow!(said(
                    "editor.keys_unvalued",
                    "the key of %1 at %2 s has no part %3",
                    &[&pointer, &t, &e.part.unwrap_or(0)]
                ))
            })?;
        }
    }
    sorted(pointer, &mut keys)?;
    crate::handles::reach(pointer, &old, &keys)?;
    Ok(set(doc, &at, Value::Array(keys)))
}

fn bend(doc: &Value, e: &KeysEdit) -> Result<Vec<Op>> {
    let one = e.picks.len() == 1 && e.picks[0].times.len() == 1;
    ensure!(one, said("editor.keys_one_bent", "handles bend one key at a time: one pick with one time", &[]));
    let (pointer, t) = (e.picks[0].pointer.as_str(), e.picks[0].times[0]);
    let (at, old) = list(doc, pointer)?;
    let i = index(&old, pointer, t)?;
    let mut keys = old.clone();
    if e.straighten {
        let alone = e.in_handle.is_none() && e.out_handle.is_none() && !e.aligned;
        ensure!(
            alone,
            said(
                "editor.straighten_alone",
                "straighten the curve after a keyframe on its own, without other changes",
                &[]
            )
        );
        keys[i] = crate::straight::cut(&old[i], "out");
        if let Some(next) = old.get(i + 1) {
            keys[i + 1] = crate::straight::cut(next, "in");
        }
        return Ok(set(doc, &at, Value::Array(keys)));
    }
    let room = |j: Option<usize>| j.and_then(|j| old.get(j)).map_or(0.0, |k| (time_of(k) - t).abs());
    let (before, after) = (room(i.checked_sub(1)), room(Some(i + 1)));
    let mut pair = [e.in_handle, e.out_handle];
    if e.aligned {
        match pair {
            [None, Some(out)] => pair[0] = parts::aligned(out, parts::side(&old[i], "in"), before),
            [Some(back), None] => pair[1] = parts::aligned(back, parts::side(&old[i], "out"), after),
            _ => {}
        }
    }
    crate::handles::spans(None, pair)?;
    let third = crate::handles::end(old[i].get(2), None, pair, &old[i][1])?;
    keys[i] = crate::handles::key(time_of(&old[i]), old[i][1].clone(), third);
    crate::handles::reach(pointer, &old, &keys)?;
    Ok(set(doc, &at, Value::Array(keys)))
}

fn fits(doc: &Value, picks: &[KeyPick], ops: &[Op]) -> Result<()> {
    let mut staged = doc.clone();
    crate::ops::apply(&mut staged, ops)?;
    if crate::keys::reads(&staged) || !crate::keys::reads(doc) {
        return Ok(());
    }
    for pick in picks {
        let (_, old) = list(doc, &pick.pointer).unwrap_or_default();
        let (_, new) = list(&staged, &pick.pointer).unwrap_or_default();
        let fresh = new.iter().map(|k| &k[1]).find(|v| !old.iter().any(|o| &o[1] == *v));
        if let (Some(like), Some(value)) = (old.first(), fresh) {
            bail!(crate::blend::fitting(&pick.pointer, value, &like[1]));
        }
    }
    Ok(())
}
