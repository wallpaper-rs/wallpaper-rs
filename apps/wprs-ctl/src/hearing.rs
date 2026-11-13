use crate::cli::OUTPUT;
use anyhow::Result;
use clap::Args;
use wprs_client::Client;
use wprs_core::dbus::DEFAULT_OUTPUT;
use wprs_core::model::State;
use wprs_core::sound::{MUTED, VOLUME};

pub const SOUND: &str = "on to play the sound of the wallpaper on that screen, off to silence it, follow to \
    follow the global volume again; the global mute stays the master";
pub const LEVEL: &str = "Volume of the wallpaper on that screen in percent, below the global volume";

#[derive(Args, Debug)]
pub struct Hearing {
    #[arg(value_parser = ["on", "off", "follow"], help = SOUND)]
    state: Option<String>,
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=100), help = LEVEL)]
    volume: Option<u8>,
    #[arg(short, long, default_value = "", help = OUTPUT)]
    output: String,
}

fn screen(output: &str) -> &str {
    if output.trim().is_empty() { DEFAULT_OUTPUT } else { output.trim() }
}

pub async fn set(client: &Client, output: &str, state: Option<&str>, volume: Option<u8>) -> Result<()> {
    let screen = screen(output);
    let (muted, level) = (format!("{MUTED}{screen}"), format!("{VOLUME}{screen}"));
    match state {
        Some("on") => client.daemon.set_option(&muted, "false").await?,
        Some("off") => client.daemon.set_option(&muted, "true").await?,
        Some(_) => {
            client.daemon.set_option(&muted, "none").await?;
            client.daemon.set_option(&level, "none").await?;
        }
        None => {}
    }
    if let Some(percent) = volume {
        client.daemon.set_option(&level, &(f64::from(percent) / 100.0).to_string()).await?;
    }
    Ok(())
}

pub async fn run(client: &Client, args: Hearing) -> Result<()> {
    set(client, &args.output, args.state.as_deref(), args.volume).await?;
    let state: State = serde_json::from_str(&client.daemon.get_state().await?)?;
    let wanted = args.output.trim();
    for (name, o) in state.outputs.iter().filter(|(n, _)| wanted.is_empty() || n.as_str() == wanted) {
        let sound = match o.sound_muted {
            Some(true) => "off",
            Some(false) => "on",
            None => "follow",
        };
        let volume = o.sound_volume.map_or("follow".to_owned(), |v| format!("{:.0}%", v * 100.0));
        println!("{name}: sound {sound}, volume {volume}, global {}", global(&state));
    }
    Ok(())
}

fn global(state: &State) -> String {
    let word = if state.muted { "muted" } else { "on" };
    format!("{word} at {:.0}%", state.volume * 100.0)
}
