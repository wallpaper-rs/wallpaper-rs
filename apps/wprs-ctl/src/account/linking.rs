use crate::cli::Wait;
use crate::daemon::job;
use crate::input::typed;
use crate::show;
use crate::told::{fill, told, tr};
use anyhow::{Result, bail};
use futures_util::TryFutureExt;
use std::io::{self, IsTerminal};
use std::time::Duration;
use wprs_client::{Client, explain, parse};
use wprs_core::model::{AccountReply, JobReply, JobState};

pub async fn join(client: &Client, device: &str, wait: &Wait) -> Result<()> {
    let id = client.sync.join(device).await.map_err(explain)?;
    if !wait.wait {
        println!("{id}");
        return Ok(());
    }
    let mut shown = false;
    loop {
        let reply: AccountReply = parse(&client.sync.get_status().await.map_err(explain)?)?;
        if let Some(code) = reply.account.and_then(|a| a.pairing?.code).filter(|_| !shown) {
            println!("{}", fill(told("code = %1"), &[&code]));
            eprintln!(
                "{}",
                fill(told("Confirm this code on %1, and here the code that %2 shows."), &[&device, &device])
            );
            shown = true;
            match io::stdin().is_terminal() {
                true => {
                    eprint!(
                        "{}",
                        fill(told("Type the code that %1 shows, or nothing to stop: "), &[&device])
                    );
                    let typed = typed().await?;
                    if typed.trim().is_empty() {
                        client.daemon.cancel_job(id).await.map_err(explain)?;
                        bail!("{}", tr(told("the key was not taken")));
                    }
                    client.sync.confirm_share(device, typed.trim()).await.map_err(explain)?;
                }
                false => eprintln!(
                    "{}",
                    fill(
                        told("Confirm with: wprs-ctl server join %1 --code <the code shown on %2>"),
                        &[&format!("{device:?}"), &device]
                    )
                ),
            }
        }
        let JobReply { job, .. } = parse(&client.daemon.get_job(id).await.map_err(explain)?)?;
        match job.state {
            JobState::Done => {
                println!("{}", if wait.json { serde_json::to_string(&job)? } else { show::summary(&job) });
                return Ok(());
            }
            JobState::Failed | JobState::Cancelled => {
                bail!("{}", job.error.map_or(tr(told("the join stopped")), |e| e.message))
            }
            _ => tokio::time::sleep(Duration::from_millis(300)).await,
        }
    }
}

pub async fn share(client: &Client, device: &str, wait: &Wait) -> Result<()> {
    let sync = &client.sync;
    let (_, compared) = client.run_job(sync.share_key(device).map_ok(|id| ((), id)), |_, _| {}).await?;
    let code = compared.output.as_ref().and_then(|o| o["code"].as_str()).unwrap_or_default().to_owned();
    println!("{}", fill(told("code = %1"), &[&code]));
    if !io::stdin().is_terminal() {
        eprintln!(
            "{}",
            fill(
                told("Send the key with: wprs-ctl server share %1 --code <the code shown on %2>"),
                &[&format!("{device:?}"), &device]
            )
        );
        return Ok(());
    }
    eprint!("{}", fill(told("Type the code that %1 shows, or nothing to stop: "), &[&device]));
    let typed = typed().await?;
    if typed.trim().is_empty() {
        bail!("{}", tr(told("the key was not shared")));
    }
    job(client, sync.confirm_share(device, typed.trim()).map_ok(|id| ((), id)), wait).await
}
