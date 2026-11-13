use super::check::bounded;
use super::parse::{numbers, usage};
use super::touch::{self, Touch};
use super::{Edit, Error};
use crate::message::{Message, said};
use serde::{Deserialize, Serialize};

pub const MOST_JOINS: usize = 64;
pub const MOST_SPLITS: usize = 64;
pub const MOST_MOVES: usize = 64;
pub const MOST_TOGETHER: usize = 8;
pub const SPLIT_RADIUS: f32 = 0.004;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Join {
    pub a: [f32; 2],
    pub b: [f32; 2],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shift {
    pub at: [f32; 2],
    pub by: f32,
}

pub(super) fn check(joins: &[Join], splits: &[Touch], moves: &[Shift]) -> Result<(), Message> {
    let few = joins.len() <= MOST_JOINS && splits.len() <= MOST_SPLITS && moves.len() <= MOST_MOVES;
    let crowded = || {
        let counts: [&dyn std::fmt::Display; 3] = [&MOST_JOINS, &MOST_SPLITS, &MOST_MOVES];
        said("edit.regions", "use at most %1 joins, %2 splits and %3 moves", &counts)
    };
    few.then_some(()).ok_or_else(crowded)?;
    for join in joins {
        join.a.iter().chain(&join.b).try_for_each(|v| bounded("depth.joins", *v, (0.0, 1.0)))?;
    }
    for shift in moves {
        shift.at.iter().try_for_each(|v| bounded("depth.moves", *v, (0.0, 1.0)))?;
        bounded("depth.moves", shift.by, (-1.0, 1.0))?;
    }
    touch::check("depth.splits", splits)
}

impl Edit {
    pub(super) fn join(&mut self, points: Option<String>, clear: bool) -> Result<(), Error> {
        let found = points.as_deref().map(touch::points).transpose()?;
        if found.is_none() && !clear {
            return Err(usage(said(
                "edit.join_none",
                "give two or more points, or clear to remove the joins",
                &[],
            )));
        }
        let few = found.as_ref().is_none_or(|p| (2..=MOST_TOGETHER).contains(&p.len()));
        let said =
            || said("edit.join_count", "a join takes 2 to %1 points, one in each region", &[&MOST_TOGETHER]);
        few.then_some(()).ok_or_else(|| usage(said()))?;
        let d = self.depth.get_or_insert_with(super::Depth::default);
        if clear {
            d.joins.clear();
        }
        let first = found.iter().flat_map(|p| p.iter().skip(1).map(|b| Join { a: p[0], b: *b }));
        d.joins.extend(first.collect::<Vec<_>>());
        Ok(())
    }

    pub(super) fn split(
        &mut self,
        points: Option<String>,
        radius: Option<f32>,
        clear: bool,
    ) -> Result<(), Error> {
        let found = points.as_deref().map(touch::points).transpose()?;
        if found.is_none() && !clear {
            return Err(usage(said("edit.split_none", "give a stroke, or clear to remove the splits", &[])));
        }
        let radius = radius.unwrap_or(SPLIT_RADIUS);
        let d = self.depth.get_or_insert_with(super::Depth::default);
        if clear {
            d.splits.clear();
        }
        d.splits.extend(found.map(|points| Touch { points, radius, falloff: 0.0, by: 1.0 }));
        Ok(())
    }

    pub(super) fn shift(&mut self, at: Option<String>, by: Option<f32>, clear: bool) -> Result<(), Error> {
        let at = at.as_deref().map(numbers::<2>).transpose()?;
        if at.is_none() && !clear {
            return Err(usage(said(
                "edit.move_none",
                "give a point of the region, or clear to put the regions back",
                &[],
            )));
        }
        let by = at.map(|_| {
            by.ok_or_else(|| {
                usage(said(
                    "edit.move_by",
                    "say how far with by: above 0 moves it nearer, below 0 farther",
                    &[],
                ))
            })
        });
        let by = by.transpose()?;
        let d = self.depth.get_or_insert_with(super::Depth::default);
        if clear {
            d.moves.clear();
        }
        d.moves.extend(at.zip(by).map(|(at, by)| Shift { at, by }));
        Ok(())
    }
}
