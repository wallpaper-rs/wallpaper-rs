mod sheet;
#[cfg(test)]
mod tests;

use crate::db::one;
use crate::error::{Problem, Res};
use crate::{App, Router, St};
use axum::extract::Path as Part;
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use rusqlite::{Connection, OptionalExtension};
use std::path::Path;
use std::sync::{Arc, PoisonError, RwLock};

pub struct Shown {
    pub sha: String,
    pub title: String,
    pub by: Option<String>,
}

#[derive(Default)]
pub struct Band(RwLock<Option<Arc<Shown>>>);

const NEWEST: &str = "SELECT v.id, v.sha256, v.title, u.handle, v.band_sheet FROM versions v \
                      JOIN packages p ON p.id = v.package_id LEFT JOIN users u ON u.id = p.owner_id \
                      WHERE v.state = 'published' AND v.listed = 1 AND v.content = 'general' \
                      AND p.hidden IS NULL AND v.id = (SELECT w.id FROM versions w \
                      WHERE w.package_id = v.package_id AND w.state = 'published' AND w.listed = 1 \
                      ORDER BY w.published_at DESC, w.id DESC LIMIT 1) \
                      ORDER BY v.published_at DESC, v.id DESC LIMIT 1";

type Row = (i64, String, String, Option<String>, Option<String>);

pub fn pick(conn: &Connection, thumbs: &Path) -> Res<Option<Shown>> {
    let row = |r: &rusqlite::Row| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?));
    let found: Option<Row> = conn.query_row(NEWEST, [], row).optional()?;
    let Some((id, sha, title, by, stored)) = found else { return Ok(None) };
    let made = match stored {
        Some(made) => made,
        None => {
            let made = sheet::of(&thumbs.join(format!("{sha}.jpg"))).unwrap_or_default();
            conn.execute("UPDATE versions SET band_sheet = ?2 WHERE id = ?1", rusqlite::params![id, made])?;
            made
        }
    };
    Ok((!made.is_empty()).then_some(Shown { sha, title, by }))
}

impl Band {
    pub fn open(conn: &Connection, data: &Path) -> Band {
        let shown = pick(conn, &data.join("thumbs")).unwrap_or_else(|problem| {
            tracing::warn!(detail = %problem.detail, "choosing the cover of the band failed");
            None
        });
        Band(RwLock::new(shown.map(Arc::new)))
    }

    pub fn shown(&self) -> Option<Arc<Shown>> {
        self.0.read().unwrap_or_else(PoisonError::into_inner).clone()
    }
}

impl App {
    pub async fn changed(self: &Arc<App>) {
        let thumbs = self.cfg.data.join("thumbs");
        match self.db.run(move |c| pick(c, &thumbs)).await {
            Ok(shown) => *self.band.0.write().unwrap_or_else(PoisonError::into_inner) = shown.map(Arc::new),
            Err(problem) => tracing::warn!(detail = %problem.detail, "choosing the cover of the band failed"),
        }
        self.dirty.notify_one();
    }

    pub fn band(&self) -> Option<Arc<Shown>> {
        self.band.shown()
    }
}

pub fn routes() -> Router {
    Router::new().route("/band/{file}", get(stylesheet))
}

async fn stylesheet(axum::extract::State(app): St, Part(file): Part<String>) -> Res<impl IntoResponse> {
    let hex = |s: &&str| s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    let sha =
        file.strip_suffix(".css").filter(hex).ok_or_else(|| Problem::missing("stylesheet"))?.to_string();
    let sql =
        "SELECT band_sheet FROM versions WHERE sha256 = ?1 AND state = 'published' AND band_sheet != ''";
    let css = app
        .db
        .run(move |c| one::<String>(c, sql, [sha]))
        .await?
        .ok_or_else(|| Problem::missing("stylesheet"))?;
    let headers =
        [(header::CONTENT_TYPE, "text/css"), (header::CACHE_CONTROL, "public, max-age=31536000, immutable")];
    Ok((headers, css))
}
