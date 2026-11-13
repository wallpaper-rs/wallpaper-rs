use crate::depth::key;
use crate::editing::open;
use crate::told::{fill, told, tr};
use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use wprs_depth::{Places, chart};

const EDGE: u32 = 512;

#[derive(Args, Debug)]
pub struct Show {
    #[arg(help = "Image file")]
    path: PathBuf,
    #[arg(long, value_name = "FILE", help = "Also save the regions as a PNG, blue far to red near")]
    png: Option<PathBuf>,
    #[arg(long, help = "Print the list as JSON")]
    json: bool,
}

pub fn show(args: Show) -> Result<()> {
    let edit = open(&args.path)?;
    let (sha, depth) = (key(&args.path, &edit)?, edit.depth.clone().unwrap_or_default());
    let map = Places::default().map(&args.path, &sha, &depth, EDGE)?;
    let found =
        chart(&map.context(tr(told("the image has no depth map; generate or import one first")))?, &depth);
    if let Some(file) = &args.png {
        found.painted().save_with_format(file, image::ImageFormat::Png)?;
    }
    for note in &found.notes {
        let (kind, index) = (note.kind, note.index);
        let why = match note.why {
            "crosses_nothing" => tr(told("its stroke crosses no region")),
            _ if kind == "join" => tr(told("both its points are already in one region")),
            _ => tr(told("its point is in a region that an earlier move or join already took")),
        };
        let (x, y) = (note.at[0], note.at[1]);
        let at = (format!("{x:.3}"), format!("{y:.3}"));
        let line = fill(
            told("%1 %2 is not applied: %3 (at %4,%5); clear the %1s or redo it"),
            &[&kind, &index, &why, &at.0, &at.1],
        );
        eprintln!("{line}");
    }
    if args.json {
        println!("{}", serde_json::to_string(&found.zones)?);
        return Ok(());
    }
    for zone in &found.zones {
        let (order, depth, share) = (zone.order, zone.depth, zone.share * 100.0);
        let place = (
            format!("{depth:.3}"),
            format!("{share:.1}"),
            format!("{:.3}", zone.at[0]),
            format!("{:.3}", zone.at[1]),
        );
        let line = fill(
            told("%1  depth %2  %3 % of the picture  at %4,%5"),
            &[&order, &place.0, &place.1, &place.2, &place.3],
        );
        println!("{line}");
    }
    Ok(())
}
