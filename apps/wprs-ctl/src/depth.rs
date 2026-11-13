use crate::editing::open;
use crate::told::{fill, told, tr};
use anyhow::{Context, Result};
use clap::Subcommand;
use std::path::{Path, PathBuf};
use wprs_core::edit::{Depth, Edit, Source};
use wprs_depth::{Device, Places, Stage, adjust, beside, coloured, lift_map, read, write};

pub use crate::looking::{Levels, Look};

const MAX_EDGE: u32 = 4096;

#[derive(Subcommand, Debug)]
pub enum Maps {
    #[command(about = "Estimate the depth map with the downloaded model, showing progress; Ctrl-C cancels")]
    Generate {
        #[arg(help = "Image file")]
        path: PathBuf,
        #[arg(long, help = "Estimate it again even when a map exists")]
        force: bool,
        #[arg(long, default_value = "auto", value_parser = ["auto", "cpu", "cuda"], help = "Where the model runs: cuda needs an ONNX Runtime built with CUDA, auto uses it when it can")]
        device: String,
        #[arg(long, value_name = "RUNS", num_args = 0..=1, default_missing_value = "3", help = "Time the model instead of saving a map")]
        bench: Option<usize>,
    },
    #[command(about = "Use a picture as the depth map, saved next to the image; white is near")]
    Import {
        #[arg(help = "Image file")]
        path: PathBuf,
        #[arg(help = "Picture to use as the depth map")]
        file: PathBuf,
    },
    #[command(
        about = "Delete the depth maps of the image: the estimated one, the imported one and the one next to it"
    )]
    Remove {
        #[arg(help = "Image file")]
        path: PathBuf,
    },
    #[command(
        about = "Save the depth map as a 16-bit grey PNG; next to the image without a file, except with --adjusted or --colour, which need one"
    )]
    Export {
        #[arg(help = "Image file")]
        path: PathBuf,
        #[arg(help = "PNG file to write")]
        file: Option<PathBuf>,
        #[arg(long, help = "With the near, far, curve, smoothing and growth of the edit")]
        adjusted: bool,
        #[arg(long, help = "In false colour, red near and blue far, as an 8-bit PNG")]
        colour: bool,
    },
    #[command(
        about = "Build, list or delete the depth layers that parallax moves; each fills what a near object hides"
    )]
    Layers(crate::stacking::Make),
    #[command(
        about = "Save one depth layer of an image as a PNG to inspect it, and the band that was filled"
    )]
    Show(crate::stacking::Peek),
    #[command(about = "Save one layer filled by LaMa on the left and by the simple fill on the right")]
    Compare(crate::stacking::Compare),
    #[command(about = "Raise or lower the depth map along a stroke of the brush; undo goes back one stroke")]
    Brush(crate::touching::Brush),
    #[command(about = "Cut the depth into layers at the given levels, or go back to the automatic cuts")]
    Cuts(crate::touching::Cuts),
    #[command(
        about = "Paint where the fill must always (refill) or never (protect) reach behind near objects"
    )]
    FillMask(crate::touching::Marks),
    #[command(
        about = "Make the regions under two or more points one region, so they share a layer and move together"
    )]
    Join(crate::touching::Join),
    #[command(about = "Cut the regions a stroke crosses in two, so each part can move on its own")]
    Split(crate::touching::Split),
    #[command(about = "Move the region under a point nearer or farther, as a whole")]
    Move(crate::touching::Move),
    #[command(about = "List the regions of the picture from the nearest, each with a point inside it")]
    Regions(crate::regioning::Show),
}

pub fn key(path: &Path, edit: &Edit) -> Result<String> {
    match edit.source.sha256.as_str() {
        "" => Ok(Source::read(path, [0; 2])?.sha256),
        sha => Ok(sha.to_owned()),
    }
}

pub fn map(path: &Path, edit: &Edit) -> Result<Option<wprs_depth::Map>> {
    let depth = edit.depth.clone().unwrap_or_default();
    Places::default().map(path, &key(path, edit)?, &depth, MAX_EDGE)
}

pub fn run(command: Maps) -> Result<()> {
    let command = match command {
        Maps::Layers(args) => return crate::stacking::make(args),
        Maps::Show(args) => return crate::stacking::peek(args),
        Maps::Compare(args) => return crate::stacking::compare(args),
        Maps::Brush(args) => return crate::touching::brush(args),
        Maps::Cuts(args) => return crate::touching::cuts(args),
        Maps::FillMask(args) => return crate::touching::marks(args),
        Maps::Join(args) => return crate::touching::join(args),
        Maps::Split(args) => return crate::touching::split(args),
        Maps::Move(args) => return crate::touching::shift(args),
        Maps::Regions(args) => return crate::regioning::show(args),
        other => other,
    };
    let places = Places::default();
    let (path, edit) = match &command {
        Maps::Generate { path, .. }
        | Maps::Import { path, .. }
        | Maps::Export { path, .. }
        | Maps::Remove { path } => (path, open(path)?),
        _ => return Ok(()),
    };
    let (sha, depth) = (key(path, &edit)?, edit.depth.clone().unwrap_or_default());
    let found = places.find(path, &sha, &depth);
    let shown = match command {
        Maps::Generate { path, device, bench: Some(runs), .. } => {
            return bench(&places, &path, &depth, &device, runs);
        }
        Maps::Generate { force: false, .. } if found.is_some() => found,
        Maps::Generate { path, device, .. } => Some(generate(&places, &path, &sha, &depth, device.parse()?)?),
        Maps::Import { path, file } => Some(places.import(&file, &path, &sha)?),
        Maps::Remove { path } => {
            let removed = places.unmap(&path, &sha, &depth);
            anyhow::ensure!(removed > 0, "{}", tr(told("the image has no depth map")));
            println!("{}", fill(told("%1 depth maps removed"), &[&removed]));
            None
        }
        Maps::Export { path, file, adjusted, colour } => {
            let from = found.context(tr(told("the image has no depth map; generate or import one first")))?;
            let to = match file {
                Some(file) => file,
                None if adjusted || colour => {
                    return Err(crate::usage(tr(told(
                        "name the file: --adjusted and --colour never replace the map",
                    ))));
                }
                None => beside(&path),
            };
            let mut map = if adjusted { adjust(&read(&from)?, &depth) } else { read(&from)? };
            if adjusted {
                lift_map(&mut map, &depth.strokes);
            }
            match colour {
                true => coloured(&map, true).save_with_format(&to, image::ImageFormat::Png)?,
                false => write(&map, &to)?,
            }
            Some(to)
        }
        _ => return Ok(()),
    };
    shown.iter().for_each(|p| println!("{}", p.display()));
    Ok(())
}

fn bench(places: &Places, path: &Path, depth: &Depth, device: &str, runs: usize) -> Result<()> {
    let found = wprs_depth::bench(places, path, depth, device.parse()?, runs)?;
    let mut times: Vec<f64> = found.runs.iter().map(|t| t.as_secs_f64()).collect();
    times.sort_by(f64::total_cmp);
    let (median, fastest) = (times[times.len() / 2], times[0]);
    println!(
        "{}",
        fill(
            told("%1: loaded in %2 s, estimated at %3 in %4 s (median of %5, fastest %6 s)"),
            &[
                &found.device.name(),
                &format!("{:.2}", found.load.as_secs_f64()),
                &depth.input,
                &format!("{:.3}", median),
                &times.len(),
                &format!("{:.3}", fastest)
            ]
        )
    );
    Ok(())
}

fn generate(places: &Places, path: &Path, sha: &str, depth: &Depth, device: Device) -> Result<PathBuf> {
    wprs_depth::background();
    let started = std::time::Instant::now();
    let made = wprs_depth::generate(places, (path, sha), depth, (None, device), &mut |stage| match stage {
        Stage::Loading => eprintln!("{}", tr(told("loading the depth model"))),
        Stage::Estimating => eprintln!("{}", tr(told("estimating depth"))),
        Stage::Saving => eprintln!("{}", tr(told("saving the depth map"))),
    })?;
    eprintln!("{}", fill(told("done in %1 s"), &[&format!("{:.1}", started.elapsed().as_secs_f32())]));
    Ok(made)
}

pub fn fallback() -> String {
    fill(
        told(
            "LaMa or ONNX Runtime is not installed, so the simple fill is used. To get the better fill run wprs-ctl models download lama-big-fp32 (see wprs-ctl models list); nothing is downloaded until you ask",
        ),
        &[],
    )
}
