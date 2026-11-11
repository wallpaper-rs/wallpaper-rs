use anyhow::Result;
use serde_json::Value;
use wprs_client::Client;

const TIMEOUT_MS: u32 = 20_000;

pub async fn watch(client: &Client, output: &str) -> Result<()> {
    let mut shown = 0usize;
    let mut error = None;
    let mut serial = client.daemon.serial().await?;
    loop {
        let state: Value = serde_json::from_str(&client.daemon.get_state().await?)?;
        let lines = state["outputs"][output]["console"].as_array().cloned().unwrap_or_default();
        lines.iter().skip(shown).filter_map(Value::as_str).for_each(|line| println!("{line}"));
        shown = lines.len();
        let now = state["outputs"][output]["error"].as_str().map(str::to_owned);
        if now != error {
            if let Some(message) = &now {
                eprintln!("error: {message}");
            }
            error = now;
        }
        (serial, _) = client.daemon.wait_changed(serial, TIMEOUT_MS).await?;
    }
}
