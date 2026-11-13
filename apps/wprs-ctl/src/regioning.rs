use crate::depth::key;
use crate::editing::open;
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
    let found = chart(&map.context("the image has no depth map; generate or import one first")?, &depth);
    if let Some(file) = &args.png {
        found.painted().save_with_format(file, image::ImageFormat::Png)?;
    }
    for note in &found.notes {
        let (kind, index) = (note.kind, note.index);
        let why = match note.why {
            "crosses_nothing" => "its stroke crosses no region",
            _ if kind == "join" => "both its points are already in one region",
            _ => "its point is in a region that an earlier move or join already took",
        };
        let (x, y) = (note.at[0], note.at[1]);
        eprintln!("{kind} {index} is not applied: {why} (at {x:.3},{y:.3}); clear the {kind}s or redo it");
    }
    if args.json {
        println!("{}", serde_json::to_string(&found.zones)?);
        return Ok(());
    }
    for zone in &found.zones {
        let (order, depth, share) = (zone.order, zone.depth, zone.share * 100.0);
        println!(
            "{order}  depth {depth:.3}  {share:.1} % of the picture  at {:.3},{:.3}",
            zone.at[0], zone.at[1]
        );
    }
    Ok(())
}
