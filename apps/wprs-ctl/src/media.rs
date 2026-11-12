use crate::cli::JSON;
use crate::show::list;
use crate::told::{fill, told, tr};
use anyhow::Result;
use clap::Subcommand;
use serde_json::{Value, json};
use wprs_client::{Client, parse};
use wprs_core::model::{InputsReply, MediaStatus};
use wprs_editor::media::simulated;

#[derive(Subcommand, Debug)]
pub enum Media {
    #[command(
        about = "Show whether the media player is read and for which screens; track data is never shown here"
    )]
    Status {
        #[arg(long, help = JSON)]
        json: bool,
    },
    #[command(
        about = "Show a made-up track on the wallpaper of a screen instead of what a media player plays, to try media values, triggers and scripts; --stop ends it"
    )]
    Simulate {
        #[arg(help = "The screen")]
        output: String,
        #[arg(required_unless_present = "stop", help = MEDIA)]
        file: Option<std::path::PathBuf>,
        #[arg(long, conflicts_with = "file", help = "Stop the simulated playback")]
        stop: bool,
    },
}

fn status(s: &MediaStatus) -> String {
    let yes = |on: bool| if on { "yes" } else { "no" };
    let mut lines = vec![match s.listening {
        true => fill(told("media player: read for %1"), &[&list(s.using.iter())]),
        false => {
            tr(told("media player: not read (no shown wallpaper allowed to use it needs it)")).to_owned()
        }
    }];
    if s.listening {
        lines.push(format!("players: {}", s.players));
        lines.push(fill(
            told("track: %1, playing: %2, cover: %3"),
            &[&yes(s.track), &yes(s.playing), &yes(s.cover)],
        ));
    }
    let remote = if s.remote_covers { "on" } else { "off" };
    lines.push(fill(told("covers from the internet (media.remote_covers): %1"), &[&remote]));
    lines.join("\n") + "\n"
}

pub async fn run(client: &Client, command: Media) -> Result<()> {
    let json = match command {
        Media::Status { json } => json,
        Media::Simulate { output, file: None, .. } => {
            return Ok(client.inputs.simulate_media(&output, "", &[]).await?);
        }
        Media::Simulate { output, file: Some(file), .. } => {
            let mut track = simulated(&file)?;
            let cover = match track.as_object_mut().and_then(|t| t.remove("cover")) {
                Some(serde_json::Value::String(path)) if !path.is_empty() => {
                    let path = file.parent().unwrap_or(std::path::Path::new(".")).join(path);
                    std::fs::read(&path).map_err(|e| {
                        anyhow::anyhow!("{}", fill(told("cannot read %1: %2"), &[&path.display(), &e]))
                    })?
                }
                _ => Vec::new(),
            };
            return Ok(client.inputs.simulate_media(&output, &track.to_string(), &cover).await?);
        }
    };
    let reply: InputsReply = parse(&client.inputs.get_inputs().await?)?;
    match json {
        true => println!("{}", serde_json::to_string(&reply.media)?),
        false => print!("{}", status(&reply.media)),
    }
    Ok(())
}

pub const VALUES: &str =
    "JSON file of setting values to try in the preview only, such as {\"speed\": 2}; nothing is saved";

pub const MEDIA: &str = "JSON file of a made-up track shown instead of a media player: title, artist, album, album_artist, subtitle, genres, playing, position, duration, rate and cover (a picture of the folder)";

pub(crate) fn preview_document(
    dir: &std::path::Path,
    doc: &Value,
    quality: Option<String>,
    restart: bool,
    media: Option<std::path::PathBuf>,
    [battery, calm, without_bloom]: [bool; 3],
    tried: (Option<std::path::PathBuf>, Option<String>),
) -> Result<Value> {
    let nonce = restart.then(|| std::time::UNIX_EPOCH.elapsed().map_or(1, |d| d.as_millis() as u64));
    let media = media.map(|f| simulated(&f)).transpose()?;
    let values = tried.0.map(|f| read_values(&f)).transpose()?;
    let look = json!({ "quality": quality, "restart": nonce, "media": media, "battery": battery, "calm": calm,
        "values": values, "preset": tried.1 });
    if wprs_editor::pages::paged(doc) {
        return Ok(wprs_editor::pages::preview_document(doc, &look));
    }
    if wprs_editor::reels::media(doc) {
        return Ok(wprs_editor::reels::preview_document(dir, doc, &look));
    }
    let mut document = json!({"manifest": doc["manifest"], "scene": wprs_editor::actions::visible(doc)});
    if without_bloom {
        document["scene"]["canvas"].as_object_mut().into_iter().for_each(|c| drop(c.remove("bloom")));
    }
    wprs_editor::scripts::stamped(&mut document["scene"], dir, doc);
    wprs_editor::rigs::staged(&look).into_iter().for_each(|s| document["preview"] = s);
    wprs_editor::tried::stage(doc, &look, &mut document);
    Ok(document)
}

fn read_values(file: &std::path::Path) -> Result<Value> {
    let text = std::fs::read_to_string(file)
        .map_err(|e| anyhow::anyhow!("{}", fill(told("cannot read %1: %2"), &[&file.display(), &e])))?;
    let values: Value =
        serde_json::from_str(&text).map_err(|e| anyhow::anyhow!("{}: {e}", file.display()))?;
    anyhow::ensure!(
        values.is_object(),
        "{}",
        fill(told("%1 is not a JSON object of setting values"), &[&file.display()])
    );
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_preview_without_the_glow_drops_the_bloom_of_the_canvas_only_when_asked() {
        let doc =
            json!({ "manifest": {}, "scene": { "canvas": { "bloom": { "strength": 1.0 } }, "layers": [] } });
        let dir = std::path::Path::new(".");
        let with = preview_document(dir, &doc, None, false, None, [false; 3], (None, None)).unwrap();
        assert!(with["scene"]["canvas"]["bloom"].is_object());
        let without =
            preview_document(dir, &doc, None, false, None, [false, false, true], (None, None)).unwrap();
        assert!(without["scene"]["canvas"].get("bloom").is_none());
    }
}
