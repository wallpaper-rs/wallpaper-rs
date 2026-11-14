use super::send::{Failure, send};
use super::{secret_path, setup};
use crate::App;
use crate::db::{HOUR, MINUTE, now, one};
use crate::error::Res;
use crate::settings::{drop_key, put};
use rusqlite::params;
use std::sync::Arc;
use std::time::Duration;

pub const WAITS: [i64; 5] = [MINUTE, 5 * MINUTE, 30 * MINUTE, 2 * HOUR, 8 * HOUR];

pub fn spawn(app: Arc<App>) {
    tokio::spawn(async move {
        loop {
            let next = match deliver(&app, now()).await {
                Ok(next) => next,
                Err(problem) => {
                    tracing::warn!(detail = %problem.detail, "sending mail failed");
                    Some(MINUTE)
                }
            };
            match next {
                Some(wait) => {
                    let wait = Duration::from_millis(wait.max(1000) as u64);
                    tokio::select! {
                        () = app.mail.wake.notified() => {}
                        () = tokio::time::sleep(wait) => {}
                    }
                }
                None => app.mail.wake.notified().await,
            }
        }
    });
}

type Due = (i64, String, String, i64);

pub async fn deliver(app: &App, at: i64) -> Res<Option<i64>> {
    let _one = app.mail.sending.lock().await;
    let due = app.db.run(move |c| {
        let Some(setup) = setup(c)? else { return Ok(None) };
        let sql = "SELECT id, subject, body, tries FROM mail_queue \
                   WHERE state = 'queued' AND next_at <= ?1 ORDER BY id LIMIT 20";
        let mut statement = c.prepare(sql)?;
        let found = statement.query_map([at], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?;
        Ok(Some((setup, found.collect::<Result<Vec<Due>, _>>()?)))
    });
    let Some((setup, due)) = due.await? else { return Ok(None) };
    let secret = std::fs::read_to_string(secret_path(&app.cfg.data)).unwrap_or_default();
    let hello = crate::lan::host(&app.cfg.public_url);
    for (id, subject, body, tries) in due {
        let sent = send(&setup, &secret, &hello, &subject, &body).await;
        let tries = tries + 1;
        app.db.run(move |c| record(c, id, tries, at, sent)).await?;
    }
    let next = "SELECT MIN(next_at) FROM mail_queue WHERE state = 'queued'";
    let next = app.db.run(move |c| one::<Option<i64>>(c, next, [])).await?.flatten();
    Ok(next.map(|n| (n - now()).max(0)))
}

fn record(c: &rusqlite::Connection, id: i64, tries: i64, at: i64, sent: Result<(), Failure>) -> Res<()> {
    match sent {
        Ok(()) => {
            let sql = "UPDATE mail_queue SET state = 'sent', tries = ?2, sent_at = ?3 WHERE id = ?1";
            c.execute(sql, params![id, tries, at])?;
            put(c, &[("mail_sent_at", &at.to_string())])?;
            for key in ["mail_failed_at", "mail_failed_why", "mail_failed_raw"] {
                drop_key(c, key)?;
            }
        }
        Err(failure) => {
            tracing::warn!(detail = %failure.raw, tries, "a mail did not go out");
            let wait = WAITS.get(tries as usize - 1).copied();
            let sql = "UPDATE mail_queue SET tries = ?2, next_at = ?3, \
                       state = IIF(?4, 'queued', 'failed') WHERE id = ?1";
            c.execute(sql, params![id, tries, at + wait.unwrap_or(0), wait.is_some()])?;
            let when = at.to_string();
            let pairs = [
                ("mail_failed_at", when.as_str()),
                ("mail_failed_why", failure.why.value()),
                ("mail_failed_raw", failure.raw.as_str()),
            ];
            put(c, &pairs)?;
        }
    }
    Ok(())
}
