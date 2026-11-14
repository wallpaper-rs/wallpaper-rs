use crate::auth::{Auth, Role};
use crate::db::{audit, changed, now};
use crate::error::{Input, Problem, require};
use crate::{Reply, Router, St};
use axum::Json;
use axum::extract::{Path, State};
use axum::routing::{delete, get};
use rusqlite::params;
use serde::Deserialize;
use serde_json::json;

pub fn routes() -> Router {
    Router::new()
        .route(
            "/api/v1/mod/hidden",
            get(hidden).post(|s: St, a: Auth, Input(h): Input<Hide>| hide(s, a, h, true)),
        )
        .route(
            "/api/v1/mod/hidden/{id}",
            delete(|s: St, a: Auth, Path(id)| hide(s, a, Hide { id, reason: String::new() }, false)),
        )
}

#[derive(Deserialize)]
pub struct Hide {
    id: String,
    #[serde(default)]
    reason: String,
}

async fn hidden(State(app): St, auth: Auth) -> Reply {
    auth.at_least(Role::Moderator)?;
    let sql = "SELECT id, hidden AS reason, hidden_at FROM packages WHERE hidden IS NOT NULL ORDER BY hidden_at DESC";
    app.db.items(sql, vec![]).await
}

async fn hide(State(app): St, auth: Auth, h: Hide, on: bool) -> Reply {
    auth.at_least(Role::Moderator)?;
    let (id, reason) = (h.id, h.reason.trim().to_string());
    require(!on || reason.chars().count() >= 10, "reason: at least 10 characters for the author")?;
    let (key, shared, _turn) = (id.clone(), app.clone(), app.deciding.lock().await);
    let sql = "UPDATE packages SET hidden = ?2, hidden_at = ?3 WHERE id = ?1 AND (hidden IS NULL) = ?4";
    app.db
        .run(move |c| {
            let (text, at) = (on.then_some(reason.clone()), on.then(now));
            let what = if on { "visible package" } else { "hidden package" };
            changed(c.execute(sql, params![key, text, at, on])?, what)?;
            audit(c, Some(auth.user), if on { "hide" } else { "unhide" }, &key, &reason)?;
            Ok(c.execute("UPDATE repo_state SET dirty = 1", [])?)
        })
        .await?;
    let key = id.clone();
    let marked =
        tokio::task::spawn_blocking(move || shared.repo.hide(&key, on)).await.map_err(Problem::internal)?;
    if let Err(e) = marked {
        tracing::info!("a hidden package is not in the repository yet: {e}");
    }
    app.dirty.notify_one();
    Ok(Json(json!({"id": id, "hidden": on})))
}
