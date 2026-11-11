use crate::author::DIR;
use clap::Subcommand;
use std::fs;
use std::path::PathBuf;
use wprs_editor::actions::Action;

#[derive(Subcommand, Debug)]
pub enum Shading {
    #[command(about = "Copy an effect descriptor and its WGSL shader into the folder as an effect")]
    ImportEffect {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = "The descriptor, a .json file")]
        descriptor: PathBuf,
        #[arg(long, help = "The shader; the .wgsl file next to the descriptor when omitted")]
        wgsl: Option<PathBuf>,
    },
    #[command(about = "Remove an effect of the package; refused while a layer still uses it")]
    RemoveEffect {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = "Id of the effect")]
        effect: String,
    },
}

pub(crate) fn action(command: Shading) -> (PathBuf, Action) {
    match command {
        Shading::ImportEffect { dir, descriptor, wgsl } => {
            let wgsl = wgsl.map(|w| fs::canonicalize(&w).unwrap_or(w));
            (
                dir,
                Action::ImportEffect {
                    descriptor: fs::canonicalize(&descriptor).unwrap_or(descriptor),
                    wgsl,
                },
            )
        }
        Shading::RemoveEffect { dir, effect } => (dir, Action::RemoveEffect { effect }),
    }
}
