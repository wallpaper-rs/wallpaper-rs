use crate::guided::Size;
use crate::regions::UNSET;
use std::collections::HashMap;

const BUCKETS: usize = 1024;

fn around(k: usize, (w, h): Size) -> impl Iterator<Item = usize> {
    let (x, y) = (k % w, k / w);
    [(x > 0, k.wrapping_sub(1)), (x + 1 < w, k + 1), (y > 0, k.wrapping_sub(w)), (y + 1 < h, k + w)]
        .into_iter()
        .filter_map(|(inside, q)| inside.then_some(q))
}

pub(crate) fn grow(label: &mut [u32], levels: &[f32], size: Size) {
    let mut queue: Vec<Vec<(usize, u32)>> = vec![Vec::new(); BUCKETS];
    let bucket = |a: f32, b: f32| (((a - b).abs() * (BUCKETS - 1) as f32) as usize).min(BUCKETS - 1);
    let push = |queue: &mut Vec<Vec<(usize, u32)>>, label: &[u32], k: usize| {
        for q in around(k, size).filter(|q| label[*q] == UNSET) {
            queue[bucket(levels[q], levels[k])].push((q, label[k]));
        }
    };
    for k in 0..label.len() {
        if label[k] != UNSET && around(k, size).any(|q| label[q] == UNSET) {
            push(&mut queue, label, k);
        }
    }
    let mut at = 0;
    while at < BUCKETS {
        let Some((q, l)) = queue[at].pop() else {
            at += 1;
            continue;
        };
        if label[q] != UNSET {
            continue;
        }
        label[q] = l;
        for n in around(q, size).filter(|n| label[*n] == UNSET) {
            let b = bucket(levels[n], levels[q]);
            queue[b].push((n, l));
            at = at.min(b);
        }
    }
    label.iter_mut().filter(|l| **l == UNSET).for_each(|l| *l = 0);
}

fn root(parent: &mut [u32], mut a: u32) -> u32 {
    while parent[a as usize] != a {
        parent[a as usize] = parent[parent[a as usize] as usize];
        a = parent[a as usize];
    }
    a
}

pub(crate) fn merge_small(label: &mut [u32], size: Size, least: usize) {
    let count = label.iter().copied().max().map_or(0, |m| m as usize + 1);
    let mut sizes = vec![0usize; count];
    let mut near: Vec<HashMap<u32, usize>> = vec![HashMap::new(); count];
    for k in 0..label.len() {
        sizes[label[k] as usize] += 1;
        for q in around(k, size).filter(|q| *q > k && label[*q] != label[k]) {
            let (a, b) = (label[k], label[q]);
            *near[a as usize].entry(b).or_default() += 1;
            *near[b as usize].entry(a).or_default() += 1;
        }
    }
    let mut parent: Vec<u32> = (0..count as u32).collect();
    let mut merged = true;
    while merged {
        merged = false;
        let mut order: Vec<u32> = (0..count as u32)
            .filter(|r| {
                parent[*r as usize] == *r && sizes[*r as usize] < least && !near[*r as usize].is_empty()
            })
            .collect();
        order.sort_by_key(|r| (sizes[*r as usize], *r));
        for r in order {
            if parent[r as usize] != r || sizes[r as usize] >= least {
                continue;
            }
            let pick = near[r as usize].iter().max_by_key(|(b, n)| (**n, std::cmp::Reverse(**b)));
            let Some((&to, _)) = pick else { continue };
            (parent[r as usize], merged) = (to, true);
            sizes[to as usize] += sizes[r as usize];
            let moved = std::mem::take(&mut near[r as usize]);
            for (x, n) in moved.into_iter().filter(|(x, _)| *x != to) {
                let row = &mut near[x as usize];
                row.remove(&r);
                *row.entry(to).or_default() += n;
                *near[to as usize].entry(x).or_default() += n;
            }
            near[to as usize].remove(&r);
        }
    }
    for l in label.iter_mut() {
        *l = root(&mut parent, *l);
    }
}
