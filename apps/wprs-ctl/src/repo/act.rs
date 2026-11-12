use super::Repo;
use crate::input::now;
use crate::keys::{keygen, signer};
use crate::show::{either, lines, list};
use crate::told::{fill, told, tr};
use crate::usage;
use anyhow::Result;
use std::path::Path;
use std::sync::Arc;
use wprs_format::{
    Reason, RootStatus, Settings, add, init, publish, revoke, revoke_key, rotate_root, settings, sign_root,
    status, unrevoke, verify,
};
use wprs_providers::HttpFetch;

pub async fn run(command: Repo) -> Result<()> {
    match command {
        Repo::Keygen { name, no_password } => println!("{}", keygen(&name, no_password)?),
        Repo::Init { dir, name, root_keys, online_keys, policy, json } => {
            let mut charter = policy.charter()?;
            (charter.name, charter.root_keys, charter.online_keys) = (Some(name), root_keys, online_keys);
            let mut settings = Settings::default();
            charter.apply(&mut settings)?;
            root(&init(&dir, &settings, now()?)?, json)?;
            eprintln!(
                "{}",
                fill(
                    told("sign the root next: wprs-ctl repo sign-root %1 --key <root key>"),
                    &[&dir.display()]
                )
            )
        }
        Repo::SignRoot { dir, key, json } => root(&sign_root(&dir, &signer(&key)?)?, json)?,
        Repo::Status { dir, json } => root(&status(&dir)?, json)?,
        Repo::Feature { dir, id, note } => wprs_format::feature(&dir, &id, note.as_deref(), true)?,
        Repo::Unfeature { dir, id } => wprs_format::feature(&dir, &id, None, false)?,
        Repo::Unlist { dir, id, version } => wprs_format::list(&dir, &id, version.as_deref(), false)?,
        Repo::ListAgain { dir, id, version } => wprs_format::list(&dir, &id, version.as_deref(), true)?,
        Repo::Add { dir, files, author, unlisted, json } => {
            let remote = [&dir].into_iter().chain(&files).any(|p| p.to_string_lossy().contains("://"));
            if remote {
                return Err(usage(tr(told("use wprs-ctl remote add to add a repository to this computer"))));
            }
            for file in files {
                let added = add(&dir, &file, author.as_deref(), now()?)?;
                if unlisted {
                    wprs_format::list(&dir, &added.id, Some(&added.release.version), false)?;
                }
                let (release, author) = (&added.release, added.author.as_deref().unwrap_or("-"));
                let text = fill(
                    told("%1 %2 sha256=%3 author=%4"),
                    &[&added.id, &release.version, &release.sha256, &author],
                );
                either(&added, json, text)?
            }
        }
        Repo::Revoke { dir, id, version, reason, message } => {
            revoke(&dir, &id, version.as_deref(), self::reason(&reason)?, &message)?
        }
        Repo::RevokeKey { dir, fingerprint, reason } => {
            revoke_key(&dir, &fingerprint, self::reason(&reason)?)?
        }
        Repo::Publish(p) | Repo::Refresh(p) => {
            let published = publish(&p.dir, &signer(&p.key)?, p.days, now()?)?;
            let text = fill(
                told("index %1 published with %2 packages; it expires %3"),
                &[&published.version, &published.packages, &published.expires],
            );
            either(&published, p.json, text)?
        }
        Repo::RotateRoot { dir, root_keys, online_keys, removed, policy, json } => {
            let mut charter = policy.charter()?;
            (charter.root_keys, charter.online_keys, charter.removed) = (root_keys, online_keys, removed);
            let mut settings = settings(&dir)?;
            charter.apply(&mut settings)?;
            root(&rotate_root(&dir, &settings, now()?)?, json)?;
            eprintln!(
                "{}",
                tr(told("sign the new root with keys of the old and of the new root, then publish"))
            );
        }
        Repo::Unrevoke { dir, id, version } => unrevoke(&dir, &id, version.as_deref().unwrap_or("*"))?,
        Repo::Verify { url, json } => {
            let url =
                if Path::new(&url).is_dir() { std::path::absolute(&url)?.display().to_string() } else { url };
            let at = now()?;
            let info = tokio::task::spawn_blocking(move || -> Result<_> {
                Ok(verify(&HttpFetch::new(Arc::default())?, &url, at)?)
            })
            .await??;
            info.warnings.iter().for_each(|warning| eprintln!("{}", fill(told("warning: %1"), &[&warning])));
            let text = lines([
                ("repo_id", info.repo_id.clone()),
                ("name", info.name.clone()),
                ("hosts", list(&info.hosts)),
                ("root", fill(told("version %1, expires %2"), &[&info.root_version, &info.root_expires])),
                ("index", fill(told("version %1, expires %2"), &[&info.entry_version, &info.entry_expires])),
                ("packages", info.packages.to_string()),
                ("stale", info.stale.to_string()),
            ]);
            either(&info, json, text.trim_end().into())?
        }
        Repo::Author(command) => super::authors::run(command).await?,
    }
    Ok(())
}

pub(super) fn root(status: &RootStatus, json: bool) -> Result<()> {
    let text = lines([
        ("repo_id", status.repo_id.clone()),
        ("version", status.version.to_string()),
        ("signers", if status.signers.is_empty() { "-".into() } else { list(&status.signers) }),
        ("threshold", status.threshold.to_string()),
        ("complete", status.complete.to_string()),
        ("anchor", format!("?{}", status.query)),
        ("address", status.address.clone()),
    ]);
    either(status, json, text.trim_end().into())
}

pub(super) fn reason(text: &str) -> Result<Reason> {
    Ok(serde_json::from_value(text.into())?)
}
