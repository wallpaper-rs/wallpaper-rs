use anyhow::{Result, ensure};
use wprs_core::message::said;
use wprs_scene::MAX_FRAMES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Order {
    #[default]
    Rows,
    Columns,
    RowsReversed,
    ColumnsReversed,
}

pub fn cells(
    size: (u32, u32),
    rows: u32,
    columns: u32,
    margin: u32,
    spacing: u32,
    order: Order,
) -> Result<Vec<[u32; 4]>> {
    let fits = (1..=MAX_FRAMES).contains(&rows) && (1..=MAX_FRAMES).contains(&columns);
    ensure!(
        fits && rows * columns <= MAX_FRAMES,
        said("editor.sprite_count", "a sprite sheet has at most %1 frames", &[&MAX_FRAMES])
    );
    let room = |side: u32, n: u32| side.saturating_sub(2 * margin + spacing * (n - 1)) / n;
    let (w, h) = (room(size.0, columns), room(size.1, rows));
    ensure!(w > 0 && h > 0, said("editor.sprite_cells", "the cells would have no size", &[]));
    let at = |r: u32, c: u32| [margin + c * (w + spacing), margin + r * (h + spacing), w, h];
    let rows_first = || (0..rows).flat_map(|r| (0..columns).map(move |c| (r, c)));
    let columns_first = || (0..columns).flat_map(|c| (0..rows).map(move |r| (r, c)));
    let mut list: Vec<[u32; 4]> = match order {
        Order::Rows | Order::RowsReversed => rows_first().map(|(r, c)| at(r, c)).collect(),
        Order::Columns | Order::ColumnsReversed => columns_first().map(|(r, c)| at(r, c)).collect(),
    };
    if matches!(order, Order::RowsReversed | Order::ColumnsReversed) {
        list.reverse();
    }
    Ok(list)
}

pub(crate) fn inside(rect: [u32; 4], size: (u32, u32)) -> Result<()> {
    let [x, y, w, h] = rect;
    ensure!(
        w > 0
            && h > 0
            && u64::from(x) + u64::from(w) <= u64::from(size.0)
            && u64::from(y) + u64::from(h) <= u64::from(size.1),
        said("editor.sprite_inside", "a frame lies inside the picture and has a size", &[])
    );
    Ok(())
}
