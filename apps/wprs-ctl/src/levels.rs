use crate::show::{clean, table};
use crate::told::{fill, told};
use anyhow::Result;
use clap::Args;
pub use notes::{measured, note, notes, sandbox_lines};
use serde_json::json;
use wprs_client::Client;
use wprs_core::model::{OutputState, State};
use wprs_core::{HdrChoice, QualityChoice, TransitionKind};

const SETTLE: std::time::Duration = std::time::Duration::from_millis(300);

#[derive(Args, Debug)]
pub struct QualityArgs {
    #[arg(
        help = "auto, a fixed level q0 to q3, or min-q1 or min-q2 to keep the automatic choice from going lower; with --output, none follows the general setting; show the levels without it"
    )]
    pub level: Option<String>,
    #[arg(
        short,
        long,
        default_value = "",
        help = "Output name such as DP-1 for its own level; every output when omitted"
    )]
    pub output: String,
    #[arg(long, value_parser = ["auto", "on", "off", "none"], help = HDR)]
    pub hdr: Option<String>,
    #[arg(long, help = crate::cli::JSON)]
    pub json: bool,
}

const HDR: &str = "Show colors brighter than white on HDR screens: auto (on mains power at the highest level), on or off; with --output, none follows the general setting";

fn hdr(o: &OutputState) -> String {
    let h = &o.hdr;
    let peak = h.peak_nits.map(|n| fill(told(", %1 nits"), &[&n])).unwrap_or_default();
    let way = match (h.active, h.encoding.as_str(), h.max_cll) {
        (true, "pq-own", Some(cll)) => fill(told(", PQ with its own description, MaxCLL %1"), &[&cll]),
        (true, "pq", _) => ", PQ".to_string(),
        (true, "scrgb", _) => ", scRGB".to_string(),
        _ => String::new(),
    };
    format!("{} {} ({}{peak}{way})", h.choice, if h.active { "on" } else { "off" }, h.reason)
}

fn general(state: &State) -> String {
    state.options.get(QualityChoice::KEY).and_then(|v| v.as_str()).unwrap_or("auto").to_owned()
}

pub async fn run(client: &Client, args: QualityArgs) -> Result<()> {
    let output = args.output.trim();
    if let Some(level) = &args.level {
        let key = if output.is_empty() {
            QualityChoice::KEY.to_owned()
        } else {
            format!("{}{output}", QualityChoice::OWN)
        };
        client.daemon.set_option(&key, level).await?;
        tokio::time::sleep(SETTLE).await;
    }
    if let Some(choice) = &args.hdr {
        let key =
            if output.is_empty() { HdrChoice::KEY.to_owned() } else { format!("{}{output}", HdrChoice::OWN) };
        client.daemon.set_option(&key, choice).await?;
        tokio::time::sleep(SETTLE).await;
    }
    let state: State = serde_json::from_str(&client.daemon.get_state().await?)?;
    let shown = state.outputs.iter().filter(|(name, _)| output.is_empty() || name.as_str() == output);
    if args.json {
        let rows = shown.map(|(name, o)| {
            let t = &o.quality_timing;
            json!({ "output": name, "choice": o.quality_choice, "quality": o.quality, "reason": o.quality_reason,
                "cost": o.quality_cost, "budget": o.quality_budget, "frame_ms": t.quality_frame_ms,
                "period_ms": t.quality_period_ms, "step": t.quality_step, "pace": t.quality_pace,
                "note": note(o), "hdr": o.hdr })
        });
        let own = &state.qualities;
        println!(
            "{}",
            json!({ "quality": general(&state), "own": own, "hdr": state.options.get(HdrChoice::KEY),
                "own_hdr": state.hdrs, "outputs": rows.collect::<Vec<_>>() })
        );
        return Ok(());
    }
    let rows: Vec<_> = shown.collect();
    let cells = rows.iter().map(|(name, o)| {
        let chosen = o.quality_choice.clone().unwrap_or_else(|| general(&state));
        vec![name.to_string(), chosen, o.quality.clone(), o.quality_reason.clone(), measured(o), hdr(o)]
    });
    print!("{}", table(&["OUTPUT", "CHOICE", "LEVEL", "WHY", "MEASURED", "HDR"], cells));
    println!("{} = {}", QualityChoice::KEY, clean(&general(&state)));
    if !output.is_empty() && !state.outputs.contains_key(output) {
        let own = state.qualities.get(output).map_or("none", String::as_str);
        println!("{}", fill(told("%1%2 = %3, not connected"), &[&QualityChoice::OWN, &output, &clean(own)]));
    }
    rows.iter().filter_map(|(name, o)| Some((name, note(o)?))).for_each(|(name, n)| println!("{name}: {n}"));
    Ok(())
}

pub fn own(state: &serde_json::Value, key: &str, json: bool) -> bool {
    let quality = key.strip_prefix(QualityChoice::OWN).map(|o| ("qualities", o));
    let passage = || key.strip_prefix(TransitionKind::OWN).map(|o| ("transitions", o));
    let hdr = || key.strip_prefix(HdrChoice::OWN).map(|o| ("hdrs", o));
    let Some((list, output)) = quality.or_else(hdr).or_else(passage) else {
        return false;
    };
    let own = state[list].get(output).cloned().unwrap_or_default();
    match json {
        true => println!("{}", serde_json::Value::Object([(key.to_owned(), own)].into_iter().collect())),
        false => println!("{key} = {}", own.as_str().unwrap_or(crate::passing::unset(list))),
    }
    if list == "transitions" && !json {
        crate::passing::lines(state, Some(output));
    }
    true
}

pub async fn notice(client: &Client, key: &str) -> Result<()> {
    if !key.starts_with(QualityChoice::KEY) {
        return Ok(());
    }
    tokio::time::sleep(SETTLE).await;
    let state: State = serde_json::from_str(&client.daemon.get_state().await?)?;
    state
        .outputs
        .iter()
        .filter_map(|(name, o)| Some((name, note(o)?)))
        .for_each(|(name, n)| println!("{name}: {n}"));
    Ok(())
}

mod notes;
#[cfg(test)]
mod tests;
