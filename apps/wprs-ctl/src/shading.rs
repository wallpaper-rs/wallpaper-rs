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
    #[command(
        name = "new-effect",
        about = "Make an effect or a material shader of the package from a template"
    )]
    Make {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = "Id of the new effect, [a-z0-9_]{1,32}")]
        name: String,
        #[arg(long, value_parser = wprs_editor::wgsl::TEMPLATES, default_value = "color",
            help = "color (a fused tint), distort (a fused bend around a point), passes (two passes, a blur along an angle), material (a material shader that samples a picture) or particle (a particle shader)")]
        template: String,
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
        Shading::Make { dir, name, template } => {
            (dir, Action::Shader(wprs_editor::actions::ShaderEdit::New { name, template: Some(template) }))
        }
    }
}
