use crate::input::{one, path_arg};
use crate::told::{told, tr};
use anyhow::{Result, anyhow};
use std::path::Path;
use wprs_client::Client;

pub async fn file_or_item(client: &Client, text: &str) -> Result<String> {
    if !Path::new(text).is_file() {
        return one(client, text).await;
    }
    if let Ok(uid) = one(client, text).await {
        return Ok(uid);
    }
    let path = path_arg(text)?;
    let imported =
        client.run_job(async { Ok(((), client.library.import(&[path.as_str()], false).await?)) }, |_, _| {});
    let (_, job) = imported.await?;
    let report = job.output.unwrap_or_default();
    let refused = report["failed"][0]["error"]
        .as_str()
        .map_or_else(|| tr(told("the library refused the file")), str::to_owned);
    report["imported"][0]["uid"].as_str().map(String::from).ok_or_else(|| anyhow!(refused))
}
