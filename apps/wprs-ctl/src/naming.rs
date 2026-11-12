use crate::author::{Author, DIR};
use anyhow::Result;
use clap::Subcommand;
use serde_json::{Value, json};
use std::path::PathBuf;
use wprs_editor::Project;
use wprs_editor::actions::{Action, Named, PasteEdit, RenameEdit};

#[derive(Subcommand, Debug)]
pub enum Naming {
    #[command(
        about = "Give a layer, timeline, asset or particle system a new id; what names it follows, scripts are not rewritten and each line that names it is printed"
    )]
    Rename {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(value_parser = ["layer", "timeline", "asset", "system"], help = "layer, timeline, asset or system")]
        kind: String,
        #[arg(help = "The id it has")]
        id: String,
        #[arg(help = "The new id")]
        to: String,
    },
    #[command(about = "Print layers with the assets, systems and effects they use as a clip for paste")]
    Copy {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = "The layers to copy", required = true)]
        ids: Vec<String>,
    },
    #[command(
        about = "Paste a clip that copy printed, from this or another scene, with free ids and its files"
    )]
    Paste {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = "The clip as JSON, or a file that holds it")]
        clip: String,
        #[arg(long, help = "Group to paste into")]
        into: Option<String>,
        #[arg(long, help = "Position among its siblings, 0 at the bottom; on top when omitted")]
        at: Option<usize>,
    },
}

pub(crate) fn action(command: Naming) -> Result<(PathBuf, Action), Box<Author>> {
    Ok(match command {
        Naming::Rename { dir, kind, id, to } => {
            let kind = serde_json::from_value(json!(kind)).unwrap_or(Named::Layer);
            (dir, Action::Rename(RenameEdit { kind, id, to }))
        }
        Naming::Paste { dir, clip, into, at } => {
            let text = std::fs::read_to_string(&clip).unwrap_or(clip);
            let clip: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
            (dir, Action::Paste(PasteEdit { clip, into, at }))
        }
        other => return Err(Box::new(Author::Naming(other))),
    })
}

pub(crate) fn run(command: Naming) -> Result<()> {
    if let Naming::Copy { dir, ids } = command {
        let project = Project::open(&dir)?;
        println!(
            "{}",
            serde_json::to_string_pretty(&wprs_editor::ids::copy(&project.dir, &project.doc, &ids)?)?
        );
    }
    Ok(())
}
