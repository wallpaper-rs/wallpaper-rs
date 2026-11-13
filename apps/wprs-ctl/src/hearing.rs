use crate::cli::OUTPUT;
use crate::input::one;
use crate::told::{fill, told, tr};
use anyhow::{Result, bail};
use clap::Args;
use wprs_client::Client;
use wprs_core::model::{ItemDetails, State};

pub const SOUND: &str = "on to play the sound of the wallpaper wherever it is shown, off to silence it, follow \
    to follow the global volume again; the global mute stays the master";
pub const LEVEL: &str = "Volume of the wallpaper in percent, below the global volume";

#[derive(Args, Debug)]
pub struct Hearing {
    #[arg(value_parser = ["on", "off", "follow"], help = SOUND)]
    state: Option<String>,
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=100), help = LEVEL)]
    volume: Option<u8>,
    #[arg(long, value_name = "ITEM", help = crate::cli::ITEM)]
    item: Option<String>,
    #[arg(short, long, default_value = "", help = OUTPUT)]
    output: String,
}

pub async fn set(client: &Client, uid: &str, state: Option<&str>, volume: Option<u8>) -> Result<()> {
    let muted = match state {
        Some("on") => "false",
        Some("off") => "true",
        Some(_) => "none",
        None => "",
    };
    let level = match (state, volume) {
        (_, Some(percent)) => percent.to_string(),
        (Some("follow"), None) => "none".to_owned(),
        _ => String::new(),
    };
    if muted.is_empty() && level.is_empty() {
        return Ok(());
    }
    client.library.set_sound(&[uid], muted, &level).await?;
    Ok(())
}

async fn shown(client: &Client, output: &str) -> Result<String> {
    let state: State = serde_json::from_str(&client.daemon.get_state().await?)?;
    let found = state.outputs.iter().find(|(name, _)| output.is_empty() || name.as_str() == output);
    match found.and_then(|(_, o)| o.item.clone()) {
        Some(uid) => Ok(uid),
        None => bail!("{}", tr(told("no wallpaper of the library is shown there; name an item with --item"))),
    }
}

pub async fn run(client: &Client, args: Hearing) -> Result<()> {
    let uid = match &args.item {
        Some(item) => one(client, item).await?,
        None => shown(client, args.output.trim()).await?,
    };
    set(client, &uid, args.state.as_deref(), args.volume).await?;
    let details: ItemDetails = serde_json::from_str(&client.library.get_item(&uid).await?)?;
    let item = details.item;
    let sound = match item.sound_muted {
        Some(true) => "off",
        Some(false) => "on",
        None => "follow",
    };
    let volume = item.sound_volume.map_or("follow".to_owned(), |v| format!("{v}%"));
    let state: State = serde_json::from_str(&client.daemon.get_state().await?)?;
    let global = if state.muted { "muted" } else { "on" };
    let level = format!("{:.0}", state.volume * 100.0);
    println!(
        "{}",
        fill(
            told("%1: sound %2, volume %3, global %4 at %5%"),
            &[&item.title, &sound, &volume, &global, &level]
        )
    );
    Ok(())
}
