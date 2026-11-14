use crate::App;
use crate::db::{HOUR, now, one, rows};
use crate::error::Res;
use crate::settings::Network;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Cert {
    pub days: i64,
    pub own: bool,
}

#[derive(Clone, Debug)]
pub struct Sign {
    pub next_in: i64,
    pub valid_for: i64,
}

#[derive(Clone, Debug)]
pub struct Disk {
    pub free: u64,
    pub total: u64,
    pub full: Option<(String, u64, u64)>,
}

#[derive(Clone, Debug)]
pub struct Facts {
    pub cert: Option<Cert>,
    pub sign: Option<Sign>,
    pub backup: Option<i64>,
    pub disk: Option<Disk>,
    pub cases: i64,
    pub reports: i64,
    pub invites: i64,
    pub first_invite: Option<i64>,
    pub packages: i64,
    pub published: Option<i64>,
    pub signed_at: i64,
}

pub fn not_after_der(der: &[u8]) -> Option<i64> {
    let (_, parsed) = x509_parser::parse_x509_certificate(der).ok()?;
    Some(parsed.validity().not_after.timestamp())
}

fn cert(app: &App) -> Option<Cert> {
    let seconds = now() / 1000;
    let (until, own) = match app.settings().network {
        Network::Home => (not_after_der(&fs::read(app.cfg.data.join("tls/cert.der")).ok()?)?, false),
        Network::Own { cert, .. } if !cert.is_empty() => {
            use tokio_rustls::rustls::pki_types::{CertificateDer, pem::PemObject};
            let first = CertificateDer::pem_file_iter(&cert).ok()?.next()?.ok()?;
            (not_after_der(&first)?, true)
        }
        Network::Own { .. } => return None,
    };
    Some(Cert { days: (until - seconds).div_euclid(86400), own })
}

fn sign(app: &App, info: &Value, signed_at: i64) -> Option<Sign> {
    let expires = info["entry_expires"].as_str()?;
    let until = chrono::DateTime::parse_from_rfc3339(expires).ok()?.timestamp_millis();
    let t = now();
    Some(Sign {
        next_in: (signed_at + app.cfg.resign_hours as i64 * HOUR - t) / 1000,
        valid_for: (until - t) / 1000,
    })
}

pub fn backup_age(data: &Path) -> Option<i64> {
    let newest = fs::read_dir(data.join("backups"))
        .ok()?
        .filter_map(|e| e.ok()?.metadata().ok()?.modified().ok())
        .max()?;
    Some(std::time::SystemTime::now().duration_since(newest).map_or(0, |d| d.as_secs() as i64))
}

fn disk(data: &Path, full: Option<(String, u64, u64)>) -> Option<Disk> {
    let found = rustix::fs::statvfs(data).ok()?;
    let unit = found.f_frsize;
    Some(Disk { free: found.f_bavail * unit, total: found.f_blocks * unit, full })
}

const FULL: &str = "SELECT handle, quota_bytes, (SELECT COALESCE(SUM(size), 0) FROM versions WHERE \
                    uploaded_by = users.id AND state != 'rejected') AS used FROM users WHERE status = 'active' \
                    AND quota_bytes > 0 AND used * 10 >= quota_bytes * 9 ORDER BY used * 1.0 / quota_bytes DESC";

pub async fn collect(app: &App) -> Res<Facts> {
    let (cases, reports, invites, first, packages, signed, published, full) = app
        .db
        .run(|c| {
            let n = |sql: &str| one::<i64>(c, sql, []).map(Option::unwrap_or_default);
            let first = one::<Option<i64>>(
                c,
                "SELECT MIN(expires_at) FROM invites WHERE expires_at > ?1 AND uses_left > 0",
                [now()],
            )?
            .flatten();
            let signed = one::<i64>(c, "SELECT signed_at FROM repo_state", [])?.unwrap_or_default();
            let published = one::<Option<i64>>(
                c,
                "SELECT MAX(published_at) FROM versions WHERE state = 'published'",
                [],
            )?
            .flatten();
            let full = rows(c, FULL, [])?.into_iter().next();
            Ok((
                n("SELECT COUNT(*) FROM cases WHERE state = 'open'")?,
                n("SELECT COUNT(*) FROM cases WHERE state = 'open' AND reason IN ('report', 'notice')")?,
                n(&format!("SELECT COUNT(*) FROM invites WHERE expires_at > {} AND uses_left > 0", now()))?,
                first,
                n("SELECT COUNT(*) FROM packages")?,
                signed,
                published,
                full,
            ))
        })
        .await?;
    let full = full.map(|u| {
        (
            u["handle"].as_str().unwrap_or_default().to_owned(),
            u["used"].as_u64().unwrap_or_default(),
            u["quota_bytes"].as_u64().unwrap_or_default(),
        )
    });
    let info = app.repo.info(&app.cfg);
    Ok(Facts {
        cert: cert(app),
        sign: info.as_ref().and_then(|i| sign(app, i, signed)),
        backup: backup_age(&app.cfg.data),
        disk: disk(&app.cfg.data, full),
        cases,
        reports,
        invites,
        first_invite: first,
        packages,
        published,
        signed_at: signed,
    })
}
