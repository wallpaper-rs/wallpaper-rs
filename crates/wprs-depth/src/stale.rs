use crate::guided::Size;
use crate::regions::regions;
use serde::Serialize;
use wprs_core::edit::Depth;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Note {
    pub kind: &'static str,
    pub index: usize,
    pub why: &'static str,
    pub at: [f32; 2],
}

pub(crate) fn label_at(label: &[u32], (w, h): Size, p: [f32; 2]) -> u32 {
    let (x, y) = ((p[0] * w as f32) as usize, (p[1] * h as f32) as usize);
    label[y.min(h - 1) * w + x.min(w - 1)]
}

pub fn stale(levels: &[f32], size: Size, depth: &Depth) -> Vec<Note> {
    let mut notes = Vec::new();
    for (i, join) in depth.joins.iter().enumerate() {
        let before = Depth { joins: depth.joins[..i].to_vec(), moves: Vec::new(), ..depth.clone() };
        let found = regions(levels, size, &before);
        if label_at(&found.label, size, join.a) == label_at(&found.label, size, join.b) {
            notes.push(Note { kind: "join", index: i + 1, why: "same_region", at: join.a });
        }
    }
    let all = regions(levels, size, depth);
    for i in 0..depth.splits.len() {
        let mut without = depth.clone();
        let gone = without.splits.remove(i);
        if regions(levels, size, &without).means.len() == all.means.len() {
            notes.push(Note { kind: "split", index: i + 1, why: "crosses_nothing", at: gone.points[0] });
        }
    }
    let mut seen = std::collections::HashSet::new();
    for (i, shift) in depth.moves.iter().enumerate() {
        if !seen.insert(label_at(&all.label, size, shift.at)) {
            notes.push(Note { kind: "move", index: i + 1, why: "same_region", at: shift.at });
        }
    }
    notes
}
