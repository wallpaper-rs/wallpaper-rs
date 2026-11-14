mod done;
mod forms;
mod later;
mod steps;
mod stored;
mod view;

use crate::auth::Auth;
use crate::creds::{self, digest};
use crate::db::{self, now, one};
use crate::error::{Problem, Res};
use crate::{App, Router, St};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, header};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use rusqlite::params;
use serde::Deserialize;

#[derive(Default)]
pub struct Held {
    key: Option<(Vec<u8>, String, String)>,
    invite: Option<(Vec<u8>, String)>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    Code,
    Network,
    Account,
    Repository,
    Key,
    Access,
    Done,
}

impl Stage {
    pub fn number(self) -> usize {
        match self {
            Stage::Code | Stage::Network => 0,
            Stage::Account => 1,
            Stage::Repository | Stage::Key => 2,
            Stage::Access | Stage::Done => 3,
        }
    }
}

pub fn routes() -> Router {
    Router::new()
        .route("/setup", get(show).post(steps::code))
        .route("/setup/network", post(steps::network))
        .route("/setup/account", post(steps::account))
        .route("/setup/repository", post(later::repository))
        .route("/setup/saved", post(later::saved))
        .route("/setup/again", post(later::again_key))
        .route("/setup/access", post(later::access))
        .route("/api/v1/admin/settings", get(stored::read).patch(stored::change))
}

pub fn cookie_name(app: &App) -> String {
    format!("{}-setup", app.cfg.cookie())
}

pub async fn visit(app: &App, headers: &HeaderMap) -> Res<Option<Vec<u8>>> {
    let name = cookie_name(app);
    let found = headers.get_all(header::COOKIE).iter().filter_map(|v| v.to_str().ok()).find_map(|v| {
        v.split(';').find_map(|pair| pair.trim().strip_prefix(name.as_str())?.strip_prefix('=').map(digest))
    });
    let Some(hash) = found else { return Ok(None) };
    let sql = "SELECT 1 FROM setup_visits WHERE hash = ?1 AND expires_at > ?2";
    let kept = hash.clone();
    let alive: Option<i64> = app.db.run(move |c| one(c, sql, params![kept, now()])).await?;
    Ok(alive.map(|_| hash))
}

pub async fn stage(app: &App, visit: bool) -> Res<Stage> {
    let settings = app.settings();
    if settings.done {
        return Ok(Stage::Done);
    }
    if !visit {
        return Ok(Stage::Code);
    }
    if !settings.network_chosen {
        return Ok(Stage::Network);
    }
    let admin =
        app.db.run(|c| one::<i64>(c, "SELECT id FROM users WHERE role = 'admin' LIMIT 1", [])).await?;
    Ok(match (admin, app.repo.ready(), settings.key_pending) {
        (None, _, _) => Stage::Account,
        (_, false, _) => Stage::Repository,
        (_, true, true) => Stage::Key,
        _ => Stage::Access,
    })
}

#[derive(Deserialize, Default)]
pub struct Again {
    #[serde(default)]
    step: String,
}

async fn show(
    State(app): St,
    auth: Option<Auth>,
    headers: HeaderMap,
    Query(q): Query<Again>,
) -> Res<Response> {
    let visit = visit(&app, &headers).await?;
    let found = stage(&app, visit.is_some()).await?;
    let page = match (found, q.step.as_str()) {
        (Stage::Done, _) => return done::screen(&app, auth).await,
        (s, "network") if s.number() > 0 => view::network(&app, &app.settings(), None),
        (s, "account") if s.number() > 1 => view::account(&app, app.settings().admin_handle.as_deref(), None),
        (Stage::Code, _) => view::code(&app, None),
        (Stage::Network, _) => view::network(&app, &app.settings(), None),
        (Stage::Account, _) => view::account(&app, None, None),
        (Stage::Repository, _) => view::repository(&app, None),
        (Stage::Key, _) => view::key(&app, held_key(&app, visit.as_deref()).as_ref(), None),
        (Stage::Access, _) => view::access(&app, &later::first_invite(&app, visit.as_deref()).await?),
    };
    Ok(([(header::CACHE_CONTROL, "no-store")], page).into_response())
}

fn held_key(app: &App, visit: Option<&[u8]>) -> Option<(String, String)> {
    let held = app.held.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    held.key.as_ref().filter(|(v, _, _)| Some(v.as_slice()) == visit).map(|(_, n, s)| (n.clone(), s.clone()))
}

pub fn again() -> Response {
    Redirect::to("/setup").into_response()
}

impl App {
    pub async fn setup_token(&self) -> anyhow::Result<String> {
        let token = creds::token('s');
        let hash = creds::digest(&token);
        let sql = "INSERT INTO setup_tokens (hash, expires_at) VALUES (?1, ?2)";
        let stored = self.db.run(move |c| Ok(c.execute(sql, params![hash, db::now() + db::DAY])?));
        stored.await.map_err(|e| anyhow::anyhow!(e.detail))?;
        Ok(token)
    }

    pub(crate) async fn announce(&self) -> anyhow::Result<()> {
        let sql = "SELECT COUNT(*) FROM users WHERE role = 'admin'";
        let admins = self.db.run(move |c| Ok(one::<i64>(c, sql, [])?.unwrap_or_default())).await;
        if !self.settings().done && admins.map_err(|e| anyhow::anyhow!(e.detail))? > 0 && self.repo.ready() {
            self.change(|c| crate::settings::put(c, &[("setup", "done")]))
                .await
                .map_err(|e| anyhow::anyhow!(e.detail))?;
        }
        if !self.settings().done {
            let token = self.setup_token().await?;
            tracing::warn!(
                "the server is not set up yet: open {} and enter the one-time setup token {token}",
                self.cfg.url("/setup")
            );
        }
        Ok(())
    }
}

pub fn finished() -> Problem {
    Problem::forbidden("the server is already set up")
}
