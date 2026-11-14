use crate::account::insert_user;
use crate::auth::Auth;
use crate::creds::{self, check_handle, check_password, digest};
use crate::db::now;
use crate::error::{Problem, Res};
use crate::frame::Frame;
use crate::lang::{t, tm};
use crate::layout::setup_screen;
use crate::net::{Client, LOGIN};
use crate::ui::{Kind, button, field, form_actions};
use crate::{App, Router, St};
use axum::extract::{Form, State};
use axum::http::header;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use maud::{Markup, html};
use rusqlite::params;
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/login", get(crate::entry::login_page).post(crate::entry::login))
        .route("/logout", post(logout))
        .route("/register", get(crate::entry::register_page).post(crate::entry::register))
        .route("/setup", get(setup_page).post(setup))
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

async fn setup_page(State(app): St) -> Markup {
    let body = html! {
        p { (tm("Enter the one-time setup token printed in the server log, or made with {}.",
            &[html! { code { "wprs-server setup-token" } }])) }
        form.formulario method="post" action="/setup" {
            (field(t("Setup token"), html! { input name="token" autocomplete="off" required; }, None, None))
            (field(t("Handle"), html! { input name="handle" autocomplete="username" required; }, None, None))
            (field(t("Password (10 characters or more)"), html! {
                input type="password" name="password" autocomplete="new-password" required minlength="10"; },
                None, None))
            (form_actions(button(Kind::Primary, t("Create administrator"))))
        }
    };
    setup_screen(&app, Frame::new(t("Create the first administrator")), body)
}

#[derive(Deserialize)]
struct Setup {
    token: String,
    handle: String,
    password: String,
}

async fn setup(State(app): St, client: Client, Form(s): Form<Setup>) -> Res<Response> {
    app.limits.take(&LOGIN, &client.key())?;
    check_handle(&s.handle)?;
    check_password(&s.handle, &s.password)?;
    let (hash, token, quota) =
        (creds::hash(&app, s.password).await?, digest(s.token.trim()), app.cfg.user_quota);
    let user = app.db.run(move |c| {
        let tx = c.transaction()?;
        let used = tx
            .execute("DELETE FROM setup_tokens WHERE hash = ?1 AND expires_at > ?2", params![token, now()])?;
        if used == 0 {
            return Err(Problem::forbidden("the setup token is wrong, used or expired"));
        }
        let id = insert_user(&tx, &s.handle, Some(&hash), "admin", quota)?;
        tx.execute("DELETE FROM setup_tokens", [])?;
        tx.commit()?;
        Ok(id)
    });
    let user = user.await?;
    start_session(&app, user, "/admin").await
}
