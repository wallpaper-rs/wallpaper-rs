use crate::author::DIR;
use crate::told::{fill, told, tr};
use anyhow::{Context, Result};
use clap::Subcommand;
use std::path::PathBuf;
use wprs_editor::Project;
use wprs_render::{Rehearsal, Renderer, SceneAssets, Trace, Versus};
use wprs_scene::Quality;

const OUT: &str = "Picture file (.png, .jpg, .webp or .avif), an animated .gif, or with --frames the first of an image sequence named NAME-0001.EXT";

#[derive(Subcommand, Debug)]
pub enum Rendering {
    #[command(
        about = "Render the scene to a picture, an animated picture or an image sequence as the daemon would at a level, without a desktop"
    )]
    Render {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(short, long, help = OUT)]
        out: PathBuf,
        #[arg(long, default_value_t = 0.0, value_parser = crate::spanning::second, help = "Second of the scene clock of the first frame")]
        time: f64,
        #[arg(long, value_name = "TIME", value_parser = crate::wallclock::parse, help = "Wall clock the scene and its scripts see, as HH:MM[:SS] today or YYYY-MM-DD HH:MM[:SS]; the clock of this computer when omitted, so pass it to get the same picture every time")]
        time_of_day: Option<f64>,
        #[arg(long, value_name = "FILE", help = crate::media::MEDIA)]
        media: Option<PathBuf>,
        #[arg(long, default_value = "q3", value_parser = ["q0", "q1", "q2", "q3"], help = "Highest quality level")]
        quality: String,
        #[arg(long, help = "Render as on battery: a lower level and at most 60 frames a second")]
        battery: bool,
        #[arg(long, help = "Render with reduced motion")]
        calm: bool,
        #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..=600), help = "Number of frames; more than one makes a clip")]
        frames: u32,
        #[arg(long, default_value_t = 30, value_parser = clap::value_parser!(u32).range(1..=240), help = "Frames a second of the clip")]
        fps: u32,
        #[arg(long, default_value_t = 1280, value_parser = clap::value_parser!(u32).range(16..=4096), help = "Longest side of the picture in pixels")]
        size: u32,
        #[arg(long, value_name = "X,Y[:X,Y]", value_parser = pointer, help = "Make-believe pointer at these fractions of the canvas, moving to the second pair over the clip")]
        pointer: Option<([f32; 2], [f32; 2])>,
        #[arg(long, requires = "pointer", help = "Press the pointer over the middle of the clip")]
        click: bool,
        #[arg(
            long,
            value_name = "LANGUAGE",
            help = "Language the texts and the scripts see, such as es or pt-BR; the default language of the package when omitted"
        )]
        language: Option<String>,
        #[arg(long, value_parser = ["q0", "q1", "q2", "q3"], help = "Show the right part at this level instead of the left one's")]
        against: Option<String>,
        #[arg(long, help = "Show the right part without the glow of the canvas")]
        without_bloom: bool,
        #[arg(long, default_value_t = 0.5, value_parser = fraction, help = "Where the left part ends, from 0 to 1, with --against or --without-bloom")]
        split: f32,
    },
}

fn fraction(text: &str) -> Result<f32, String> {
    let v: f32 = text.parse().map_err(|_| fill(told("%1 is not a number"), &[&text]))?;
    (0.0..=1.0).contains(&v).then_some(v).ok_or_else(|| tr(told("from 0 to 1")).to_owned())
}

fn point(text: &str) -> Result<[f32; 2], String> {
    let (x, y) = text.split_once(',').ok_or("use X,Y")?;
    let at = |v: &str| fraction(v.trim());
    Ok([at(x)?, at(y)?])
}

fn pointer(text: &str) -> Result<([f32; 2], [f32; 2]), String> {
    let (from, to) = text.split_once(':').unwrap_or((text, text));
    Ok((point(from)?, point(to)?))
}

fn level(text: &str) -> Result<Quality> {
    Ok(serde_json::from_value(serde_json::json!(text))?)
}

pub fn run(command: Rendering) -> Result<()> {
    let Rendering::Render {
        dir,
        out,
        time,
        time_of_day,
        media,
        quality,
        battery,
        calm,
        frames,
        fps,
        size,
        pointer,
        click,
        language,
        against,
        without_bloom,
        split,
    } = command;
    let project = Project::open(&dir)?;
    let renderer = Renderer::new()?;
    let played = media.map(|file| wprs_editor::media::played(&file, &project.dir)).transpose()?;
    let wanted = Rehearsal {
        quality: level(&quality)?,
        battery,
        calm,
        edge: size,
        time,
        wall: time_of_day.unwrap_or_else(wprs_render::local_wall),
        script_host: None,
        media: played.as_ref().map(|p| p.state.clone()),
        lang: wprs_editor::texts::preview_locale(&project.doc, language.as_deref().unwrap_or_default()),
        frames: frames as usize,
        fps,
        pointer: pointer.map(|(from, to)| Trace { from, to, click }),
    };
    let open = |bloom: bool| -> Result<wprs_render::Opened> {
        let (scene, props) = wprs_editor::reach::open(&project.doc, bloom)?;
        let mut assets = SceneAssets::read(&scene, wprs_editor::reach::reader(&project.dir))
            .and_then(|a| a.with_scripts(&scene, wprs_editor::reach::reader(&project.dir)))
            .map_err(anyhow::Error::msg)?;
        if let Some(cover) = played.as_ref().and_then(|p| p.cover.as_ref()) {
            wprs_editor::media::dress(&mut assets, &scene, cover);
        }
        Ok((scene, assets, props))
    };
    let versus = (against.is_some() || without_bloom).then_some(Versus {
        quality: against.as_deref().map(level).transpose()?,
        without_bloom,
        split,
    });
    let reel = renderer.rehearse_versus(open, &wanted, versus.as_ref())?;
    let written = reel.write(&out).with_context(|| fill(told("cannot write %1"), &[&out.display()]))?;
    written.iter().for_each(|p: &PathBuf| println!("{}", p.display()));
    reel.left_out.iter().for_each(|l| eprintln!("{l}"));
    reel.console.iter().for_each(|l| eprintln!("{}", fill(told("script: %1"), &[&l])));
    reel.stopped.iter().for_each(|l| eprintln!("{}", fill(told("script stopped: %1"), &[&l])));
    Ok(())
}
