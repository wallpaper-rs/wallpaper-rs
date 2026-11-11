use crate::author::{DIR, ID};
use clap::Subcommand;
use std::path::PathBuf;
use wprs_editor::actions::{Action, BoneRuleEdit};

#[derive(Subcommand, Debug)]
pub enum Ruling {
    #[command(
        about = "Add, change or remove a rule that hands a bone of a mesh or model layer to another parent, keeping its pose"
    )]
    BoneRule {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = ID)]
        layer: String,
        #[command(subcommand)]
        change: Rule,
    },
}

#[derive(Subcommand, Debug)]
pub enum Rule {
    #[command(about = "A new rule handing BONE to PARENT")]
    Add {
        #[arg(help = "The bone that is handed over")]
        bone: String,
        #[arg(help = "Its new parent")]
        parent: String,
        #[arg(long, allow_hyphen_values = true, help = "How much it moves the pose, from 0 to 1, 1 by default")]
        weight: Option<f32>,
    },
    #[command(about = "Change rule N, 0 first")]
    Set {
        #[arg(help = "The rule, 0 first")]
        index: usize,
        #[arg(long, help = "Another bone to hand over")]
        bone: Option<String>,
        #[arg(long, help = "Another parent for it")]
        parent: Option<String>,
        #[arg(long, allow_hyphen_values = true, help = "How much it moves the pose, from 0 to 1")]
        weight: Option<f32>,
    },
    #[command(about = "Remove rule N, 0 first")]
    Remove {
        #[arg(help = "The rule, 0 first")]
        index: usize,
    },
}

pub(crate) fn action(command: Ruling) -> (PathBuf, Action) {
    match command {
        Ruling::BoneRule { dir, layer, change } => {
            let edit = match change {
                Rule::Add { bone, parent, weight } => BoneRuleEdit {
                    layer,
                    index: None,
                    bone: Some(bone),
                    parent: Some(parent),
                    weight,
                    remove: false,
                },
                Rule::Set { index, bone, parent, weight } => {
                    BoneRuleEdit { layer, index: Some(index), bone, parent, weight, remove: false }
                }
                Rule::Remove { index } => BoneRuleEdit {
                    layer,
                    index: Some(index),
                    bone: None,
                    parent: None,
                    weight: None,
                    remove: true,
                },
            };
            (dir, Action::BoneRule(edit))
        }
    }
}
