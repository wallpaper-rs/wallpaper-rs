mod notices;
mod queue;
mod send;
#[cfg(test)]
mod tests;

use crate::App;
use crate::db::now;
use crate::error::Res;
use crate::lang::{Lang, within};
use rusqlite::{Connection, params};
use std::collections::HashMap;
use std::fs::{self, DirBuilder, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicI64;
use tokio::sync::{Mutex, Notify};

pub use notices::{case, rewatch, watch};
pub use queue::{deliver, spawn};
pub use send::{Why, valid_from, valid_to};

#[derive(Default)]
pub struct Post {
    wake: Notify,
    sending: Mutex<()>,
    watched: AtomicI64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Security {
    StartTls,
    Tls,
    Plain,
}

impl Security {
    pub const ALL: [Security; 3] = [Security::StartTls, Security::Tls, Security::Plain];

    pub fn value(self) -> &'static str {
        match self {
            Security::StartTls => "starttls",
            Security::Tls => "tls",
            Security::Plain => "none",
        }
    }

    pub fn parse(value: &str) -> Option<Security> {
        Security::ALL.into_iter().find(|s| s.value() == value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Setup {
    pub host: String,
    pub port: u16,
    pub security: Security,
    pub user: String,
    pub from: String,
    pub to: String,
    pub lang: Lang,
}

#[derive(Clone, Debug, Default)]
pub struct Last {
    pub sent_at: Option<i64>,
    pub failed: Option<(i64, Why, String)>,
}

pub const KEYS: [&str; 7] =
    ["mail_host", "mail_port", "mail_security", "mail_user", "mail_from", "mail_to", "mail_lang"];
pub const RESULT: [&str; 4] = ["mail_sent_at", "mail_failed_at", "mail_failed_why", "mail_failed_raw"];

fn stored(c: &Connection) -> Res<HashMap<String, String>> {
    let mut statement = c.prepare("SELECT key, value FROM settings WHERE key LIKE 'mail\\_%' ESCAPE '\\'")?;
    let found = statement.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
    Ok(found.collect::<Result<_, _>>()?)
}

pub fn setup(c: &Connection) -> Res<Option<Setup>> {
    let found = stored(c)?;
    let get = |key: &str| found.get(key).cloned().unwrap_or_default();
    let (Some(host), Some(security)) = (found.get("mail_host"), Security::parse(&get("mail_security")))
    else {
        return Ok(None);
    };
    Ok(Some(Setup {
        host: host.clone(),
        port: get("mail_port").parse().unwrap_or(587),
        security,
        user: get("mail_user"),
        from: get("mail_from"),
        to: get("mail_to"),
        lang: if get("mail_lang") == "es" { Lang::Es } else { Lang::En },
    }))
}

pub fn last(c: &Connection) -> Res<Last> {
    let found = stored(c)?;
    let at = |key: &str| found.get(key).and_then(|v| v.parse::<i64>().ok());
    let failed = at("mail_failed_at").map(|when| {
        let why = Why::parse(found.get("mail_failed_why").map_or("", String::as_str));
        (when, why, found.get("mail_failed_raw").cloned().unwrap_or_default())
    });
    let sent_at = at("mail_sent_at");
    let failed = failed.filter(|(when, ..)| sent_at.is_none_or(|s| *when > s));
    Ok(Last { sent_at, failed })
}

pub fn secret_path(data: &Path) -> PathBuf {
    data.join("keys/mail.secret")
}

pub fn keep_secret(data: &Path, secret: &str) -> Res<()> {
    let path = secret_path(data);
    DirBuilder::new().recursive(true).mode(0o700).create(data.join("keys"))?;
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(path)?;
    file.write_all(secret.as_bytes())?;
    Ok(())
}

pub fn forget(c: &Connection, data: &Path) -> Res<()> {
    for key in KEYS.iter().chain(&RESULT) {
        crate::settings::drop_key(c, key)?;
    }
    c.execute("DELETE FROM mail_queue WHERE state = 'queued'", [])?;
    let _ = fs::remove_file(secret_path(data));
    Ok(())
}

pub async fn enqueue(
    app: &App,
    topic: &'static str,
    case: Option<i64>,
    write: impl FnOnce(&Setup) -> (String, String) + Send + 'static,
) -> Res<bool> {
    let queued = app.db.run(move |c| {
        let Some(setup) = setup(c)? else { return Ok(false) };
        let (subject, body) = within(setup.lang, || write(&setup));
        let sql = "INSERT INTO mail_queue (topic, case_id, subject, body, next_at, created_at) \
                   VALUES (?1, ?2, ?3, ?4, ?5, ?5)";
        c.execute(sql, params![topic, case, subject, body, now()])?;
        Ok(true)
    });
    let queued = queued.await?;
    if queued {
        app.mail.wake.notify_one();
    }
    Ok(queued)
}
