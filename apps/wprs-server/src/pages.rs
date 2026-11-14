use crate::auth::Auth;
use crate::creds;
use crate::error::Res;
use crate::{App, Router, St};
use axum::extract::State;
use axum::http::header;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/login", get(crate::entry::login_page).post(crate::entry::login))
        .route("/logout", post(logout))
        .route("/register", get(crate::entry::register_page).post(crate::entry::register))
        .route("/link", get(crate::entry::link))
        .route("/account", get(crate::own::account))
}

#[derive(Deserialize, Default)]
pub struct Next {
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub invite: String,
}

pub fn local(path: &str) -> &str {
    let plain = path.bytes().all(|b| b.is_ascii_graphic() && b != b'\\');
    if plain && path.starts_with('/') && !path.starts_with("//") { path } else { "/account" }
}

pub async fn start_session(app: &App, user: i64, next: &str) -> Res<Response> {
    let value = app.db.run(move |c| creds::session(c, user)).await?;
    let cookie = creds::cookie(&app.cfg, &value, 30 * 86400);
    Ok(([(header::SET_COOKIE, cookie)], Redirect::to(local(next))).into_response())
}

async fn logout(State(app): St, auth: Option<Auth>) -> Res<Response> {
    let session = auth.and_then(|a| a.session);
    app.db.run(move |c| Ok(c.execute("DELETE FROM sessions WHERE id_hash = ?1", [session])?)).await?;
    Ok(([(header::SET_COOKIE, creds::cookie(&app.cfg, "", 0))], Redirect::to("/")).into_response())
}
