use crate::cli::JSON;
use crate::told::{fill, told, tr};
use anyhow::Result;
use clap::Subcommand;
use serde_json::{Value, json};
use wprs_client::Client;
use wprs_core::rgb::Covered;

const COVERED: &str = "follow keeps the lights following the scene without drawing it while the server has devices connected, pause sends nothing and leaves the lights as they are until the wallpaper shows again; both apply while the covered wallpaper pauses (pause.when_covered)";

const PLACES: &str = "x y w h as parts of the picture from 0 to 1, then right, left, down or up for the way a line of lights runs; off leaves the device alone (a zone stays dark); none takes the whole picture again";

fn covered(s: &Value) -> String {
    let choice = s["when_covered"].as_str().unwrap_or("pause");
    let now = if s["held"] == true { tr(told(", paused now")) } else { String::new() };
    fill(told("while covered (%1): %2%3"), &[&Covered::KEY, &choice, &now])
}

#[derive(Subcommand, Debug)]
pub enum Rgb {
    #[command(
        about = "Show whether RGB lighting is on, its OpenRGB server, the connection and what sends colors"
    )]
    Status {
        #[arg(long, help = JSON)]
        json: bool,
    },
    #[command(about = "Connect to the OpenRGB server now and list its devices and lights")]
    Devices {
        #[arg(long, help = JSON)]
        json: bool,
    },
    #[command(about = "Show a moving rainbow for 3 seconds on one device, or on all of them")]
    Test {
        #[arg(help = "Index from wprs-ctl rgb devices; every device when omitted")]
        device: Option<u32>,
    },
    #[command(
        about = "Show or choose what the lights do while a window covers the wallpaper: follow it or pause"
    )]
    Covered {
        #[arg(value_parser = Covered::WORDS, help = COVERED)]
        choice: Option<String>,
        #[arg(long, help = JSON)]
        json: bool,
    },
    #[command(
        about = "Show or choose the part of the picture each device or zone takes; list every place without a device"
    )]
    Place {
        #[arg(
            help = "The device name as wprs-ctl rgb devices shows it, with its serial or number when two share a name"
        )]
        device: Option<String>,
        #[arg(help = PLACES)]
        place: Option<String>,
        #[arg(long, help = "Place one zone of the device, counted from 0 in wprs-ctl rgb devices --json")]
        zone: Option<u16>,
        #[arg(long, help = JSON)]
        json: bool,
    },
    #[command(about = "Print the colors a screen sends to RGB lighting, at most 64 by 16")]
    Strip {
        #[arg(short, long, help = crate::cli::OUTPUT)]
        output: String,
        #[arg(long, help = JSON)]
        json: bool,
    },
}

fn waiting(why: &str) -> String {
    match why {
        "locked" => tr(told("the session is locked and the wallpaper pauses while locked")),
        "battery" => tr(told("the computer is on battery and the wallpaper pauses on battery")),
        "colorless" => tr(told("the covered output shows something that sends no colors")),
        "ungranted" => tr(told("the package shown there does not hold the rgb grant")),
        _ => tr(told("the wallpaper is paused")),
    }
}

fn status(s: &Value) -> String {
    let on = if s["enabled"] == true { "on" } else { "off" };
    let place = if s["local"] == true {
        tr(told("this computer"))
    } else {
        tr(told("another computer on the network"))
    };
    let mut lines = vec![fill(
        told("RGB lighting: %1, OpenRGB server %2 (%3)"),
        &[&on, &s["server"].as_str().unwrap_or("?"), &place],
    )];
    lines.push(match (s["connected"] == true, s["error"].as_str()) {
        (true, _) => fill(told("connected, %1 devices"), &[&s["devices"].as_array().map_or(0, Vec::len)]),
        (false, Some(e)) => fill(told("not connected: %1"), &[&e]),
        (false, None) => tr(told("not connected")),
    });
    lines.push(covered(s));
    for (output, why) in s["waiting"].as_object().into_iter().flatten() {
        lines.push(fill(
            told("%1: the lights wait because %2"),
            &[&output, &waiting(why.as_str().unwrap_or(""))],
        ));
    }
    if let Some(source) = s["source"].as_str() {
        lines.push(fill(told("colors from %1, %2 frames sent"), &[&source, &s["frames"]]));
    }
    lines.join("\n")
}

fn devices(s: &Value) -> String {
    let rows = s["devices"].as_array().into_iter().flatten().map(|d| {
        let kind = d["kind"].as_str().unwrap_or("?");
        let own = d["own_mode"].as_str().map_or(String::new(), |m| match d["taken"] == true {
            true => fill(told(", own mode %1 comes back when the lights stop"), &[&m]),
            false => fill(told(", own mode %1"), &[&m]),
        });
        let name = d["key"].as_str().or(d["name"].as_str()).unwrap_or("?");
        format!("{:>3}  {kind:<14} {name}  ({} lights{own})", d["index"], d["leds"])
    });
    let list: Vec<String> = rows.collect();
    if list.is_empty() { tr(told("no devices")) } else { list.join("\n") }
}

pub async fn run(client: &Client, command: Rgb) -> Result<()> {
    let rgb = &client.rgb;
    match command {
        Rgb::Status { json } => {
            let s: Value = serde_json::from_str(&rgb.get_status().await?)?;
            println!("{}", if json { s.to_string() } else { status(&s) });
        }
        Rgb::Devices { json } => {
            rgb.connect().await?;
            let s: Value = serde_json::from_str(&rgb.get_status().await?)?;
            println!("{}", if json { s["devices"].to_string() } else { devices(&s) });
        }
        Rgb::Test { device } => rgb.test(device.map_or(-1, |d| d as i32)).await?,
        Rgb::Covered { choice, json } => {
            if let Some(choice) = choice {
                client.daemon.set_option(Covered::KEY, &choice).await?;
            }
            let s: Value = serde_json::from_str(&rgb.get_status().await?)?;
            let shown = json!({ "when_covered": s["when_covered"], "held": s["held"] });
            println!("{}", if json { shown.to_string() } else { covered(&s) });
        }
        Rgb::Place { device, place, zone, json } => {
            let key = device.map(|d| zone.map_or(d.clone(), |z| format!("{d}#{z}")));
            if let (Some(key), Some(place)) = (&key, &place) {
                client.daemon.set_option(&format!("{}{key}", wprs_core::rgb::PLACE), place).await?;
            }
            let s: Value = serde_json::from_str(&rgb.get_status().await?)?;
            let places = s["places"].as_object().cloned().unwrap_or_default();
            let shown: Vec<(String, Value)> = match &key {
                Some(key) => vec![(key.clone(), places.get(key).cloned().unwrap_or(Value::Null))],
                None => places.into_iter().collect(),
            };
            match json {
                true => println!("{}", Value::Object(shown.into_iter().collect())),
                false if shown.is_empty() => println!("{}", tr(told("every device takes the whole picture"))),
                false => shown
                    .iter()
                    .for_each(|(k, v)| println!("{k}: {}", v.as_str().unwrap_or(&tr(told("whole picture"))))),
            }
        }
        Rgb::Strip { output, json } => {
            let (w, h, colors) = rgb.get_strip(&output).await?;
            let hex: Vec<String> =
                colors.chunks(3).map(|c| format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])).collect();
            match json {
                true => println!("{}", json!({ "width": w, "height": h, "colors": hex })),
                false => hex.chunks(w.max(1) as usize).for_each(|row| println!("{}", row.join(" "))),
            }
        }
    }
    Ok(())
}
