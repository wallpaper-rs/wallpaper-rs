use crate::capturing::Capture;
use crate::steps::edit;
use anyhow::{Context, Result, bail};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use wprs_client::Client;
use wprs_editor::Project;
use wprs_editor::actions::{Action, ShotFormat};

const POLL: Duration = Duration::from_millis(200);
const START: Duration = Duration::from_secs(25);

async fn frame(client: &Client, output: &str, within: Duration) -> Result<image::RgbaImage> {
    let end = Instant::now() + within;
    while Instant::now() < end {
        let path = client.daemon.capture_page(output).await?;
        if !path.is_empty() {
            let picture = image::open(&path).context("the captured frame cannot be read");
            std::fs::remove_file(&path).ok();
            return Ok(picture?.to_rgba8());
        }
        tokio::time::sleep(POLL).await;
    }
    bail!("the page drew no frame in {} s; is it loading or hidden?", within.as_secs())
}

async fn pick(client: &Client, wanted: Option<&str>) -> Result<String> {
    if let Some(name) = wanted {
        return Ok(name.to_owned());
    }
    let state: wprs_core::model::State = wprs_client::parse(&client.daemon.get_state().await?)?;
    state.outputs.keys().next().cloned().context("no screen to show the page on; name one with --output")
}

fn web_base(shot: &Capture) -> Option<wprs_format::Package<std::io::BufReader<std::fs::File>>> {
    let package = wprs_format::open(shot.base.as_ref()?, &wprs_format::Limits::default()).ok()?;
    (package.manifest.kind == wprs_format::Kind::Web).then_some(package)
}

pub fn wanted(dir: &Path, shot: &Capture) -> bool {
    let Ok(project) = Project::open(dir) else { return false };
    wprs_editor::pages::paged(&project.doc)
        || (wprs_editor::pages::kind(&project.doc) == "preset" && web_base(shot).is_some())
}

pub fn run(dir: &Path, animated: bool, shot: &Capture) -> Result<()> {
    let project = Project::open(dir)?;
    let (shown, document) = match web_base(shot) {
        Some(mut base) if !wprs_editor::pages::paged(&project.doc) => {
            let folder = std::env::temp_dir().join(format!("wprs-base-{}", std::process::id()));
            std::fs::remove_dir_all(&folder).ok();
            wprs_format::extract(&mut base, &folder)?;
            let manifest = serde_json::from_slice::<serde_json::Value>(&std::fs::read(
                folder.join(wprs_format::MANIFEST),
            )?)?;
            let values = project.doc["preset"]["values"].clone();
            let document = json!({"manifest": manifest, "preview": {"values": if values.is_object() { values } else { json!({}) }}});
            (folder, document.to_string())
        }
        _ => (dir.to_path_buf(), wprs_editor::pages::preview_document(&project.doc, &json!({})).to_string()),
    };
    let path = std::fs::canonicalize(&shown)?.to_str().context("the path is not UTF-8")?.to_owned();
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    let made = runtime.block_on(async {
        let client = Client::connect().await?;
        let output = pick(&client, shot.output.as_deref()).await?;
        client.daemon.preview_scene(&output, &path, &document, -1.0).await?;
        let made = taken(&client, &output, animated, shot, &project).await;
        client.daemon.preview_scene(&output, &path, "", 0.0).await.ok();
        made
    });
    if shown != dir {
        std::fs::remove_dir_all(&shown).ok();
    }
    let made = made?;
    edit(dir, Action::Thumbnail { file: made.clone(), animated })?;
    std::fs::remove_file(made).ok();
    Ok(())
}

async fn taken(
    client: &Client,
    output: &str,
    animated: bool,
    shot: &Capture,
    project: &Project,
) -> Result<PathBuf> {
    let part = project.dir.join("assets").join(".web-shot.part");
    std::fs::create_dir_all(project.dir.join("assets"))?;
    let first = frame(client, output, START).await?;
    if !animated {
        first.save_with_format(&part, image::ImageFormat::Png)?;
        return Ok(part);
    }
    let seconds = shot.seconds.unwrap_or(4.0).clamp(0.1, wprs_editor::shots::MOST_SECONDS);
    let (started, mut images) = (Instant::now(), vec![first]);
    while started.elapsed().as_secs_f64() < seconds {
        images.push(frame(client, output, START).await?);
    }
    let fps = ((images.len() as f64 / started.elapsed().as_secs_f64()).round() as u32).clamp(1, 30);
    let images: Vec<_> = images
        .into_iter()
        .map(|i| {
            let edge = i.width().max(i.height());
            let scale = f64::from(wprs_editor::shots::MOST_EDGE.min(edge)) / f64::from(edge);
            let size = |n: u32| ((f64::from(n) * scale).round() as u32).max(1);
            image::imageops::resize(
                &i,
                size(i.width()),
                size(i.height()),
                image::imageops::FilterType::Triangle,
            )
        })
        .collect();
    let format = match shot.format.as_deref() {
        Some("webp") => ShotFormat::Webp,
        _ => ShotFormat::Gif,
    };
    let part = part.with_extension(if format == ShotFormat::Webp { "webp" } else { "gif" });
    std::fs::write(&part, wprs_editor::shots::encode(&images, fps, format)?)?;
    Ok(part)
}
