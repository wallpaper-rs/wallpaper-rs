use crate::args::OUTPUT;
use crate::cli::JSON;
use crate::inputs::asking_changed;
use crate::told::{fill, told, tr};
use anyhow::Result;
use clap::{Args, Subcommand};
use wprs_client::{Client, Playlists1Proxy, parse};
use wprs_core::model::{BrowseEnd, BrowseSource, BrowseState};

const FROM: &str = "Browse the playlist of the screen or the whole library; by default the playlist when the screen plays one";
const PREVIEW: &str =
    "Preview it without applying it: starts browsing, or moves on while browsing; browse apply keeps it";
const TRANSITION: &str = "fade, none, or empty for the usual transition";

#[derive(Args, Debug)]
pub struct Only {
    #[arg(short, long, default_value = "", help = OUTPUT)]
    pub output: String,
    #[arg(long, help = JSON)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct Step {
    #[arg(short, long, default_value = "", help = OUTPUT)]
    pub output: String,
    #[arg(long, help = PREVIEW)]
    pub preview: bool,
    #[arg(long, default_value = "", requires = "preview", help = FROM)]
    pub from: String,
    #[arg(long, requires = "preview", help = JSON)]
    pub json: bool,
}

#[derive(Subcommand, Debug)]
pub enum Browsing {
    #[command(about = "Show the next item of the playlist of an output")]
    Next(Step),
    #[command(about = "Show the previous item of the playlist of an output")]
    Prev(Step),
    #[command(subcommand, about = "The wallpaper being previewed: apply it, stop, or choose its transition")]
    Browse(Browse),
}

#[derive(Subcommand, Debug)]
pub enum Browse {
    #[command(about = "Apply the wallpaper being previewed and stop browsing")]
    Apply {
        #[command(flatten)]
        only: Only,
        #[arg(long, default_value = "", help = TRANSITION)]
        transition: String,
    },
    #[command(about = "Choose the transition used when the previewed wallpaper is applied")]
    Transition {
        #[arg(help = TRANSITION)]
        transition: String,
        #[command(flatten)]
        only: Only,
    },
    #[command(about = "Stop browsing and leave the wallpaper as it was")]
    Cancel(Only),
    #[command(about = "Show what is being previewed")]
    Show(Only),
}

fn text(s: &BrowseState) -> String {
    let screen = if s.output.is_empty() { tr(told("every screen")) } else { s.output.clone() };
    match (s.active, s.ended) {
        (false, Some(BrowseEnd::Applied)) => return fill(told("applied on %1"), &[&screen]) + "\n",
        (false, Some(BrowseEnd::Cancelled)) => return fill(told("browsing ended on %1"), &[&screen]) + "\n",
        (false, _) => return fill(told("nothing is being browsed on %1"), &[&screen]) + "\n",
        _ => {}
    }
    let name = s.candidate.as_ref().map_or_else(|| tr(told("(none)")), |c| c.title.clone());
    let from = match s.source {
        Some(BrowseSource::Playlist) => fill(told("playlist %1"), &[&s.playlist_name]),
        _ => tr(told("library")),
    };
    let at = fill(told("%1 of %2"), &[&(s.position + 1), &s.count]);
    let mut lines = vec![fill(told("candidate on %1: %2"), &[&screen, &name]), format!("{from}, {at}")];
    if !s.transition.is_empty() {
        lines.push(fill(told("transition: %1"), &[&s.transition]));
    }
    lines.join("\n") + "\n"
}

pub async fn run(client: &Client, command: Browsing) -> Result<()> {
    let p = &client.playlists;
    let (reply, json) = match command {
        Browsing::Next(Step { output, preview: false, .. }) => {
            return asking_changed(client, &output, p.next(&output)).await;
        }
        Browsing::Prev(Step { output, preview: false, .. }) => {
            return asking_changed(client, &output, p.previous(&output)).await;
        }
        Browsing::Next(at) => (p.browse(&at.output, &at.from, 1).await?, at.json),
        Browsing::Prev(at) => (p.browse(&at.output, &at.from, -1).await?, at.json),
        Browsing::Browse(command) => browse(p, command).await?,
    };
    match json {
        true => println!("{reply}"),
        false => print!("{}", text(&parse::<BrowseState>(&reply)?)),
    }
    Ok(())
}

async fn browse(p: &Playlists1Proxy<'static>, command: Browse) -> Result<(String, bool)> {
    Ok(match command {
        Browse::Apply { only, transition } => (p.browse_apply(&only.output, &transition).await?, only.json),
        Browse::Transition { transition, only } => {
            (p.browse_transition(&only.output, &transition).await?, only.json)
        }
        Browse::Cancel(only) => (p.browse_cancel(&only.output).await?, only.json),
        Browse::Show(only) => (p.get_browse(&only.output).await?, only.json),
    })
}
