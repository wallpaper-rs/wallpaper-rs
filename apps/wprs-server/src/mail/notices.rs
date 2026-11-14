use super::{enqueue, setup};
use crate::App;
use crate::db::{MINUTE, now, one};
use crate::error::Res;
use crate::lang::{t, tf};
use crate::ui::Tone;
use rusqlite::params;
use std::sync::atomic::Ordering;

const EVERY: i64 = 15 * MINUTE;

fn tone_value(tone: Option<Tone>) -> &'static str {
    tone.map_or("none", Tone::class)
}

pub async fn case(app: &App, id: i64) -> Res<bool> {
    let sql = "SELECT c.reason, c.priority, v.title FROM cases c JOIN versions v ON v.id = c.version_id \
               WHERE c.id = ?1";
    let found = app.db.run(move |c| {
        Ok(c.query_row(sql, [id], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?, r.get::<_, String>(2)?))
        })?)
    });
    let (reason, priority, title) = found.await?;
    if priority < 1 {
        return Ok(false);
    }
    let (name, link) = (app.cfg.name.clone(), app.cfg.url(&format!("/mod/cases/{id}")));
    enqueue(app, "case", Some(id), move |_| {
        let (subject, arrived) = match reason.as_str() {
            "report" => (
                tf("{}: a new report to review", &[&name]),
                tf("A report arrived about the wallpaper “{}”.", &[&title]),
            ),
            "counter" => (
                tf("{}: a new counter-notice to review", &[&name]),
                tf("A counter-notice arrived about the wallpaper “{}”.", &[&title]),
            ),
            _ => (
                tf("{}: a new legal notice to review", &[&name]),
                tf("A legal notice arrived about the wallpaper “{}”.", &[&title]),
            ),
        };
        let body = [
            arrived,
            tf("Open it in Review: {}", &[&link]),
            t("This mail does not carry who sent it nor their address; you see what you need in the case.")
                .to_string(),
        ];
        (subject, body.join("\n\n"))
    })
    .await
}

pub fn rewatch(app: &App) {
    app.mail.watched.store(0, Ordering::Relaxed);
}

pub async fn watch(app: &App) -> Res<usize> {
    let at = now();
    if at - app.mail.watched.load(Ordering::Relaxed) < EVERY {
        return Ok(0);
    }
    app.mail.watched.store(at, Ordering::Relaxed);
    if app.db.run(|c| Ok(setup(c)?.is_none())).await? {
        return Ok(0);
    }
    let facts = crate::health::collect(app).await?;
    let seen: Vec<(&'static str, &'static str, Option<Tone>)> = crate::health::watched(&facts);
    let changed = app.db.run(move |c| {
        let mut changed = Vec::new();
        for (key, name, tone) in seen {
            let before = one::<String>(c, "SELECT tone FROM health_seen WHERE row = ?1", [key])?;
            let value = tone_value(tone);
            if before.as_deref() == Some(value) {
                continue;
            }
            let sql = "INSERT INTO health_seen (row, tone) VALUES (?1, ?2) \
                       ON CONFLICT (row) DO UPDATE SET tone = excluded.tone";
            c.execute(sql, params![key, value])?;
            if matches!(tone, Some(Tone::Warn | Tone::Bad)) {
                changed.push((name, tone == Some(Tone::Bad)));
            }
        }
        Ok(changed)
    });
    let changed = changed.await?;
    for (row, red) in &changed {
        let (row, red, name, link) = (*row, *red, app.cfg.name.clone(), app.cfg.url("/admin"));
        enqueue(app, "health", None, move |_| {
            let (subject, said) = match red {
                true => (
                    tf("{}: “{}” turned red", &[&name, &t(row)]),
                    tf("In the health page of the server, the row “{}” turned red.", &[&t(row)]),
                ),
                false => (
                    tf("{}: “{}” needs a look soon", &[&name, &t(row)]),
                    tf("In the health page of the server, the row “{}” has little time left.", &[&t(row)]),
                ),
            };
            let body = [
                said,
                tf("Open it: {}", &[&link]),
                t("You get one mail each time a row changes, not one per check.").to_string(),
            ];
            (subject, body.join("\n\n"))
        })
        .await?;
    }
    Ok(changed.len())
}
