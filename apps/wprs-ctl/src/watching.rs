use crate::show;
use crate::told::{fill, told};
use anyhow::Result;
use serde_json::{Value, json};
use std::pin::pin;
use wprs_client::Client;
use zbus::export::ordered_stream::{OrderedStreamExt, join};

pub async fn watch(client: &Client, json: bool) -> Result<()> {
    let daemon = &client.daemon;
    let line =
        |value: Value, text: String| if json { value.to_string() } else { show::clean(text.trim_end()) };
    let changed = daemon.receive_changed().await?.filter_map(|s| {
        let a = s.args().ok()?;
        let value = json!({"signal": "Changed", "topic": a.topic, "ids": a.ids, "serial": a.serial});
        Some(line(value, format!("{} {} {}", a.serial, a.topic, a.ids.join(" "))))
    });
    let progressed = daemon.receive_job_progress().await?.filter_map(|s| {
        let a = s.args().ok()?;
        let value = json!({"signal": "JobProgress", "job": a.job, "done": a.done, "total": a.total});
        Some(line(value, fill(told("job %1 %2/%3"), &[&a.job, &a.done, &a.total])))
    });
    let finished = daemon.receive_job_finished().await?.filter_map(|s| {
        let a = s.args().ok()?;
        let value = json!({"signal": "JobFinished", "job": a.job, "state": a.state, "error_name": a.error_name,
            "message": a.message});
        Some(line(value, format!("job {} {} {}", a.job, a.state, a.message)))
    });
    let mut events = pin!(join(join(changed, progressed), finished));
    while let Some(line) = events.next().await {
        println!("{line}");
    }
    Ok(())
}
