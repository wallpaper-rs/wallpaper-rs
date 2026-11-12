use crate::told::{fill, told, tr};
use crate::usage;
use anyhow::Result;

pub(super) fn at_fraction(at: &str) -> Result<(f64, f64)> {
    let (x, y) = at.split_once(',').ok_or_else(|| usage(tr(told("expected X,Y"))))?;
    let parsed =
        |s: &str| s.trim().parse::<f64>().map_err(|_| usage(fill(told("%1 is not a number"), &[&s])));
    Ok((parsed(x)?, parsed(y)?))
}
