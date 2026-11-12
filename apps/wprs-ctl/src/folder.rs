use crate::author::DIR;
use crate::media::VALUES;
use anyhow::{Context, Result, ensure};
use clap::Subcommand;
use std::fs;
use std::path::PathBuf;
use wprs_client::Client;
use wprs_editor::{Project, assets};

mod parts;

#[derive(Subcommand, Debug)]
pub enum Folder {
    #[command(about = "Delete the files in assets/ that the scene does not use")]
    Prune {
        #[arg(help = DIR)]
        dir: PathBuf,
    },
    #[command(about = "List the changes that undo and redo walk through")]
    History {
        #[arg(help = DIR)]
        dir: PathBuf,
    },
    #[command(about = "Restore the unsaved changes the app kept for a folder, or drop them with --discard")]
    Recover {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = "Drop the kept changes instead of restoring them", long)]
        discard: bool,
    },
    #[command(about = "Check the scene and its files; exits with 1 when something is wrong")]
    Validate {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(long, help = "Print one JSON object per problem")]
        json: bool,
    },
    #[command(about = "Build the .wpak package, signed with --key, as wprs-ctl pack build does")]
    Build {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(short, long, help = "Package file; <id>-<version>.wpak next to the scene folder when omitted")]
        out: Option<PathBuf>,
        #[arg(long, help = "Secret key file of the author")]
        key: Option<PathBuf>,
        #[arg(long, help = "Print the result as JSON")]
        json: bool,
    },
    #[command(about = "Show the scene of the folder on the desktop for 60 seconds without applying it")]
    Preview {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(short, long, default_value = "", help = crate::cli::OUTPUT)]
        output: String,
        #[arg(long, default_value_t = -1.0, allow_hyphen_values = true, value_parser = crate::spanning::second, help = "Hold this second, up to 86400; plays when negative, and with --layout draws this second instead of the poster time")]
        time: f64,
        #[arg(long, help = "End the preview")]
        stop: bool,
        #[arg(long, value_parser = ["q0", "q1", "q2", "q3"], help = "Highest quality level of the preview")]
        quality: Option<String>,
        #[arg(long, help = "Start the particle systems and the scene again")]
        restart: bool,
        #[arg(long, help = "Show it as the daemon does on battery: a lower level and the battery pace")]
        battery: bool,
        #[arg(long, help = "Show it as the daemon does with reduced motion")]
        calm: bool,
        #[arg(long, help = "Show it without the glow of the canvas, to compare it with the glow")]
        without_bloom: bool,
        #[arg(long, value_name = "FILE", help = crate::media::MEDIA)]
        media: Option<PathBuf>,
        #[arg(long, value_name = "FILE", help = VALUES)]
        values: Option<PathBuf>,
        #[arg(long, value_name = "ID", help = "Start from this preset of the package before --values")]
        preset: Option<String>,
        #[arg(
            long,
            conflicts_with = "stop",
            help = "Print the console of its scripts and report its error as they change, until stopped. The scene runs only while something consumes frames on that output (a screen showing it or a stream of it), so without a consumer nothing is printed"
        )]
        console: bool,
        #[command(flatten)]
        span: crate::spanning::Span,
    },
    #[command(about = "Build the scene into the library and show it on an output")]
    Apply {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(short, long, default_value = "", help = crate::cli::OUTPUT)]
        output: String,
        #[arg(long, help = "Spread it over every screen, each one showing its part")]
        span: bool,
    },
    #[command(
        about = "Print the JSON Schema of scenes, of user properties with --property or of manifests with --manifest"
    )]
    Schema {
        #[arg(help = "Print the schema of user properties", long)]
        property: bool,
        #[arg(help = "Print the schema of manifests", long, conflicts_with = "property")]
        manifest: bool,
    },
    #[command(
        about = "List the built-in effects and the effects of the package, with category, cost and description"
    )]
    Catalog {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(long, help = "Only the effects of this category")]
        category: Option<String>,
        #[arg(long, help = "Print one JSON object per effect")]
        json: bool,
    },
}

fn runtime() -> Result<tokio::runtime::Runtime> {
    Ok(tokio::runtime::Builder::new_current_thread().enable_all().build()?)
}

pub(crate) fn run(command: Folder) -> Result<()> {
    match command {
        Folder::Prune { dir } => {
            assets::prune(&dir, &Project::open(&dir)?.doc)?.iter().for_each(|p| println!("{p}"))
        }
        Folder::History { dir } => {
            let project = Project::open(&dir)?;
            let (done, undone) = project.history.labels();
            let step = |label: &&str| wprs_editor::history::step(label).local();
            done.iter().for_each(|label| println!("{}", step(label)));
            undone.iter().for_each(|label| println!("{} (undone)", step(label)));
        }
        Folder::Recover { dir, discard } => {
            let mut project = Project::open(&dir)?;
            ensure!(project.pending.is_some(), "there are no unsaved changes to recover");
            project.recover(!discard)?;
            if !discard {
                project.save()?;
            }
        }
        Folder::Validate { dir, json } => parts::validate(&dir, json)?,
        Folder::Build { dir, out, key, json } => parts::build(&dir, out, key, json)?,
        Folder::Preview { dir, span: span @ crate::spanning::Span { layout: Some(_), .. }, time, .. } => {
            crate::spanning::preview(&dir, &span, (time >= 0.0).then_some(time))?
        }
        Folder::Preview {
            dir,
            output,
            time,
            stop,
            quality,
            restart,
            battery,
            calm,
            without_bloom,
            media,
            console,
            values,
            preset,
            ..
        } => {
            let project = Project::open(&dir)?;
            let document = crate::media::preview_document(
                &project.dir,
                &project.doc,
                quality,
                restart,
                media,
                [battery, calm, without_bloom],
                (values, preset),
            )?
            .to_string();
            let (dir, document) = (fs::canonicalize(&dir)?, if stop { String::new() } else { document });
            let path = dir.to_str().context("the path is not UTF-8")?.to_owned();
            runtime()?.block_on(async {
                let client = Client::connect().await?;
                let before = parts::drafted(&client).await?;
                client.daemon.preview_scene(&output, &path, &document, time).await?;
                parts::previewed(&client, (&before, &output), stop, &project.doc).await?;
                if console {
                    crate::console::watch(&client, &output).await?;
                }
                Ok::<_, anyhow::Error>(())
            })?;
        }
        Folder::Apply { dir, output, span } => parts::apply_folder(&dir, &output, span)?,
        Folder::Schema { property, manifest } => {
            let schema = match (property, manifest) {
                (true, _) => wprs_scene::property_schema(),
                (_, true) => wprs_format::manifest_schema(),
                _ => wprs_scene::schema(),
            };
            println!("{}", serde_json::to_string_pretty(&schema)?);
        }
        Folder::Catalog { dir, category, json } => crate::catalog::run(&dir, category.as_deref(), json)?,
    }
    Ok(())
}
