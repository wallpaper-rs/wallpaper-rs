use crate::authoring::count;
use anyhow::{Result, anyhow, ensure};
use serde_json::json;
use std::path::{Path, PathBuf};
use wprs_client::Client;
use wprs_editor::{Project, package};

pub(super) fn validate(dir: &Path, json: bool) -> Result<()> {
    let project = Project::open(dir)?;
    let problems = package::problems(&project.dir, &project.doc);
    for p in &problems {
        match json {
            true => println!("{}", serde_json::to_string(p)?),
            false => {
                println!("{}{}: {}", if p.warning { "warning: " } else { "" }, p.pointer, p.said.local())
            }
        }
    }
    let errors = problems.iter().filter(|p| !p.warning).count();
    ensure!(errors == 0, "{}", count(errors, "problem"));
    Ok(())
}

pub(super) fn build(dir: &Path, out: Option<PathBuf>, key: Option<PathBuf>, json: bool) -> Result<()> {
    let mut project = Project::open(dir)?;
    let out = out.map_or_else(|| package::output(dir, &project.doc), Ok)?;
    let signer = key.map(|key| crate::keys::signer(&key)).transpose()?;
    let (sha256, size) = package::build(&mut project, &out, signer.as_ref())?;
    match json {
        true => println!("{}", json!({"path": out, "sha256": sha256, "size": size})),
        false => println!("{sha256} {size} {}", out.display()),
    }
    Ok(())
}

pub(super) async fn apply(path: &str, output: &str, fit: &str) -> Result<()> {
    let client = Client::connect().await?;
    let (_, job) =
        client.run_job(async { Ok(((), client.library.import(&[path], false).await?)) }, |_, _| {}).await?;
    let report = job.output.unwrap_or_default();
    let uid = report["imported"][0]["uid"].as_str().map(String::from);
    let failed =
        report["failed"][0]["error"].as_str().unwrap_or("the library refused the package").to_owned();
    let uid = uid.ok_or_else(|| anyhow!(failed))?;
    client.library.apply(&uid, output, fit, "").await?;
    println!("{uid}");
    Ok(())
}
