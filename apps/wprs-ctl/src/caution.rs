use crate::input::confirm;
use crate::told::{fill, told};
use anyhow::Result;
use wprs_client::{Client, error_name};
use wprs_core::dbus::error::UNCONFIRMED;

pub const FLASHING: &str = "Warning: this wallpaper flashes or shows fast patterns that can trigger seizures \
    in people with photosensitive epilepsy.";

pub async fn apply(client: &Client, uid: &str, output: &str, fit: &str, preset: &str) -> Result<()> {
    let lib = &client.library;
    notice(client, uid).await?;
    let applied = crate::inputs::asking(client, output, lib.apply(uid, output, fit, preset)).await;
    match applied {
        Err(e) if error_name(&e) == Some(UNCONFIRMED) => {
            eprintln!("{FLASHING}");
            anyhow::ensure!(
                confirm("Show it anyway? You will not be asked again for this package.").await?,
                "not applied; run wprs-ctl lib confirm photosensitive {uid} to allow it"
            );
            lib.confirm(&[uid], "photosensitive").await?;
            crate::inputs::asking(client, output, lib.apply(uid, output, fit, preset)).await
        }
        done => done,
    }
}

pub fn installed_line(job: &wprs_core::model::Job) -> Option<String> {
    let output = job.output.as_ref()?;
    let text = |key: &str| output.get(key)?.as_str().map(str::to_owned);
    Some(fill(told("Installed %1 %2."), &[&text("id")?, &text("version")?]))
}

pub async fn installed(client: &Client, job: Option<&wprs_core::model::Job>) -> Result<()> {
    if let Some(line) = job.and_then(installed_line) {
        eprintln!("{line}");
    }
    let item = job.and_then(|j| j.output.as_ref()).and_then(|o| o.get("item")).and_then(|i| i.as_str());
    match item {
        Some(uid) => notice(client, uid).await,
        None => Ok(()),
    }
}

pub const COMPUTE: &str = "Warning: this wallpaper needs compute shaders, which the graphics of this computer lack \
    or run in software, so it may draw slowly or leave parts out.";

async fn notice(client: &Client, uid: &str) -> Result<()> {
    let details: wprs_core::model::ItemDetails = wprs_client::parse(&client.library.get_item(uid).await?)?;
    let compute = details.item.package.as_ref().is_some_and(|p| p.facets.iter().any(|f| f == "compute"));
    let state: wprs_core::model::State = wprs_client::parse(&client.daemon.get_state().await?)?;
    if compute && !state.compute {
        eprintln!("{COMPUTE}");
    }
    let exposes = details.item.package.map(|p| p.exposes).unwrap_or_default();
    if let Some(notice) = wprs_format::web_notice(&exposes) {
        eprintln!("{}\n{}", notice.local(), wprs_format::web_restrict(uid).local());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::installed_line;
    use serde_json::json;

    #[test]
    fn a_finished_install_says_what_was_installed() {
        let job = |output: serde_json::Value| {
            serde_json::from_value(json!({ "id": 1, "kind": "install", "target": "ab/org.example.dusk@1.1.0",
                "state": "done", "done": 1, "total": 1, "started_at": 0, "finished_at": 1, "error": null,
                "output": output }))
            .unwrap()
        };
        let line = installed_line(&job(json!({ "id": "org.example.dusk", "version": "1.1.0" })));
        assert_eq!(line.as_deref(), Some("Installed org.example.dusk 1.1.0."));
        assert_eq!(installed_line(&job(json!(null))), None);
    }
}
