use super::check::bounded;
use super::touch::{self, Touch};
use crate::message::{Message, said};
use serde::{Deserialize, Serialize};

pub const MOST_JOINS: usize = 64;
pub const MOST_SPLITS: usize = 64;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Join {
    pub a: [f32; 2],
    pub b: [f32; 2],
}

pub(super) fn check(joins: &[Join], splits: &[Touch]) -> Result<(), Message> {
    let few = joins.len() <= MOST_JOINS && splits.len() <= MOST_SPLITS;
    let crowded = || said("edit.regions", "use at most %1 joins and %2 splits", &[&MOST_JOINS, &MOST_SPLITS]);
    few.then_some(()).ok_or_else(crowded)?;
    for join in joins {
        join.a.iter().chain(&join.b).try_for_each(|v| bounded("depth.joins", *v, (0.0, 1.0)))?;
    }
    touch::check("depth.splits", splits)
}
