use super::hands::{self, Hand};
use super::{index, list, parts};
use crate::action::KeysEdit;
use crate::keys::time_of;
use crate::ops::{Op, set};
use anyhow::{Result, anyhow, ensure};
use serde_json::Value;
use wprs_core::message::said;

pub(super) fn bend(doc: &Value, e: &KeysEdit) -> Result<Vec<Op>> {
    let one = e.picks.len() == 1 && e.picks[0].times.len() == 1;
    ensure!(one, said("editor.keys_one_bent", "handles bend one key at a time: one pick with one time", &[]));
    let (pointer, t) = (e.picks[0].pointer.as_str(), e.picks[0].times[0]);
    let (at, old) = list(doc, pointer)?;
    let i = index(&old, pointer, t)?;
    let mut keys = old.clone();
    let handled = e.in_handle.is_some() || e.out_handle.is_some();
    if e.straighten {
        ensure!(
            !handled && !e.aligned && !e.link,
            said(
                "editor.straighten_alone",
                "straighten the curve after a keyframe on its own, without other changes",
                &[]
            )
        );
    }
    ensure!(
        !e.link || !handled,
        said(
            "editor.link_alone",
            "link the parts of the handles of a keyframe on its own, without other changes",
            &[]
        )
    );
    if e.straighten {
        keys[i] = crate::straight::cut(&old[i], "out");
        if let Some(next) = old.get(i + 1) {
            keys[i + 1] = crate::straight::cut(next, "in");
        }
        return Ok(set(doc, &at, Value::Array(keys)));
    }
    let width = parts::parts(&old[i][1]).map_or(0, |p| p.len());
    match (e.link, e.part.filter(|_| width > 1)) {
        (true, part) => link(&mut keys, &old, i, part.unwrap_or(0)),
        (false, Some(p)) => {
            ensure!(
                p < width,
                said("editor.keys_unvalued", "the key of %1 at %2 s has no part %3", &[&pointer, &t, &p])
            );
            split(&mut keys, &old, i, p, e).ok_or_else(|| {
                anyhow!(said(
                    "editor.handle_no_neighbour",
                    "the keyframe of %1 at %2 s has no neighbour on that side to bend its curve towards",
                    &[&pointer, &t]
                ))
            })?;
            crate::handles::spans(None, [e.in_handle, e.out_handle])?;
        }
        (false, None) => whole(&mut keys, &old, i, e)?,
    }
    crate::handles::reach(pointer, &old, &keys)?;
    Ok(set(doc, &at, Value::Array(keys)))
}

fn whole(keys: &mut [Value], old: &[Value], i: usize, e: &KeysEdit) -> Result<()> {
    let t = time_of(&old[i]);
    let room = |j: Option<usize>| j.and_then(|j| old.get(j)).map_or(0.0, |k| (time_of(k) - t).abs());
    let (before, after) = (room(i.checked_sub(1)), room(Some(i + 1)));
    let partner = |side: &str| hands::hand(&old[i], side).map(|h| h.pairs[0]);
    let mut pair = [e.in_handle, e.out_handle];
    if e.aligned {
        match pair {
            [None, Some(out)] => pair[0] = hands::aligned(out, partner("in"), before),
            [Some(back), None] => pair[1] = hands::aligned(back, partner("out"), after),
            _ => {}
        }
    }
    crate::handles::spans(None, pair)?;
    let third = crate::handles::end(old[i].get(2), None, pair, &old[i][1])?;
    keys[i] = crate::handles::key(t, old[i][1].clone(), third);
    Ok(())
}

fn placed(keys: &mut [Value], old: &[Value], i: usize, side: &str, p: usize, h: [f64; 2]) -> Option<()> {
    let (mut pairs, _) = hands::each(old, i, side)?;
    *pairs.get_mut(p)? = h;
    let mut key = keys[i].clone();
    if !key.get(2).is_none_or(Value::is_object) {
        key = serde_json::json!([key[0], key[1]]);
    }
    hands::put(&mut key, side, Some(Hand { pairs, split: true }));
    keys[i] = key;
    Some(())
}

fn split(keys: &mut [Value], old: &[Value], i: usize, p: usize, e: &KeysEdit) -> Option<()> {
    for (side, given, other) in [("in", e.in_handle, "out"), ("out", e.out_handle, "in")] {
        let Some(h) = given else { continue };
        placed(keys, old, i, side, p, h)?;
        let free = if other == "in" { e.in_handle.is_none() } else { e.out_handle.is_none() };
        if let Some((pairs, seg)) = hands::each(old, i, other).filter(|_| e.aligned && free)
            && let Some(turned) = hands::aligned(h, pairs.get(p).copied(), seg.room)
        {
            placed(keys, old, i, other, p, turned)?;
        }
    }
    Some(())
}

fn link(keys: &mut [Value], old: &[Value], i: usize, part: usize) {
    for side in ["in", "out"] {
        let split = hands::hand(&old[i], side).is_some_and(|h| h.split);
        if let Some((pairs, seg)) = hands::each(old, i, side).filter(|_| split) {
            let one = hands::linked(&pairs, &seg, part);
            hands::put(&mut keys[i], side, Some(Hand { pairs: vec![one], split: false }));
        }
    }
}
