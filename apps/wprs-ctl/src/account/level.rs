use crate::cli::Wait;
use crate::daemon::job;
use crate::input::confirm;
use crate::show;
use crate::told::{told, tr};
use anyhow::{Result, bail};
use clap::{Args, ValueEnum};
use futures_util::TryFutureExt;
use std::io::{self, IsTerminal};
use wprs_client::Client;

pub const CONNECTED: &str = told(
    "connected, not syncing: repositories, browsing, installing and votes work; nothing of the library \
     is sent. Turn syncing on with `wprs-ctl server syncing on`",
);

#[derive(Args, Debug)]
pub struct Connect {
    #[arg(
        long,
        default_value = "",
        help = "Name of this computer, seen only by your devices; the host name when empty"
    )]
    pub name: String,
    #[command(flatten)]
    pub wait: Wait,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Switch {
    On,
    Off,
}

#[derive(Args, Debug)]
pub struct Syncing {
    #[arg(
        value_enum,
        help = "on: sync the library, playlists and settings with your other devices; off: stop syncing and \
                stay connected"
    )]
    pub switch: Switch,
    #[arg(long, help = "Sync without end-to-end encryption; the server can then read every record")]
    pub plaintext: bool,
    #[command(flatten)]
    pub wait: Wait,
}

pub async fn connect(client: &Client, args: Connect) -> Result<()> {
    let start = client.sync.connect(&args.name).map_ok(|(code, url, id)| {
        print!("{}", show::lines(vec![("code", code), ("url", url)]));
        ((), id)
    });
    job(client, start, &args.wait).await.inspect_err(super::keyring_hint)
}

pub async fn syncing(client: &Client, args: Syncing) -> Result<()> {
    let on = matches!(args.switch, Switch::On);
    if on && args.plaintext {
        eprintln!("{}", crate::serving::PLAINTEXT);
        if io::stdin().is_terminal() && !confirm(&tr(told("Sync without end-to-end encryption?"))).await? {
            bail!("{}", tr(told("syncing stays off")));
        }
    }
    job(client, client.sync.set_syncing(on, args.plaintext).map_ok(|id| ((), id)), &args.wait).await
}
