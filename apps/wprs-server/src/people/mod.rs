mod invites;
mod load;
mod view;

use crate::admin::{Edit, change_user};
use crate::auth::{Auth, ROLES, Role};
use crate::error::{Problem, Res};
use crate::invite::new_invite;
use crate::lang::t;
use crate::settings::Access;
use crate::{Router, St};
use axum::Form;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use load::{draw, load};
use serde::Deserialize;
use view::{Created, Draft};

pub fn routes() -> Router {
    Router::new()
        .route("/admin/personas", get(show))
        .route("/admin/personas/invites", post(create))
        .route("/admin/personas/invites/{id}/revoke", post(revoke))
        .route("/admin/personas/accounts/{id}", post(change))
        .route("/admin/personas/access", post(access))
}

pub const HERE: &str = "/admin/personas";

fn admin(auth: Option<Auth>) -> Option<Auth> {
    auth.filter(|a| a.role >= Role::Admin)
}

fn login() -> Response {
    Redirect::to("/login?next=/admin/personas").into_response()
}

#[derive(Deserialize)]
struct Ask {
    revoke: Option<i64>,
}

async fn show(State(app): St, auth: Option<Auth>, Query(ask): Query<Ask>) -> Res<Response> {
    let Some(auth) = admin(auth) else { return Ok(login()) };
    let people = load(&app, &auth, None, Draft::default(), ask.revoke).await?;
    draw(&app, &auth, StatusCode::OK, people).await
}

#[derive(Deserialize)]
struct Fresh {
    role: String,
    uses: String,
    days: String,
}

fn refused(f: &Fresh) -> Option<(&'static str, String, String)> {
    let range = |v: &str, max: i64| v.trim().parse::<i64>().ok().filter(|n| (1..=max).contains(n));
    if !ROLES.contains(&f.role.as_str()) {
        return Some((
            "role",
            t("Choose one of the roles.").into(),
            "role: user, trusted, moderator or admin".into(),
        ));
    }
    if range(&f.uses, 1000).is_none() {
        return Some((
            "uses",
            t("Put a number of people from 1 to 1000 and create it again.").into(),
            "422 Unprocessable Entity: uses must be between 1 and 1000".into(),
        ));
    }
    if range(&f.days, 90).is_none() {
        let message = t("An invitation works 90 days at most. Put 90 or less and create it again.");
        return Some((
            "days",
            message.into(),
            "422 Unprocessable Entity: days must be between 1 and 90".into(),
        ));
    }
    None
}

async fn create(State(app): St, auth: Option<Auth>, Form(f): Form<Fresh>) -> Res<Response> {
    let Some(auth) = admin(auth) else { return Ok(login()) };
    if let Some(error) = refused(&f) {
        let draft = Draft { role: f.role, uses: f.uses, days: f.days, error: Some(error) };
        let people = load(&app, &auth, None, draft, None).await?;
        return draw(&app, &auth, StatusCode::UNPROCESSABLE_ENTITY, people).await;
    }
    let (uses, days) = (f.uses.trim().parse().unwrap_or(1), f.days.trim().parse().unwrap_or(7));
    let (code, id, expires) = new_invite(&app, auth.user, &f.role, uses, days).await?;
    let link = app.cfg.url(&format!("/register?invite={code}"));
    let svg = crate::qr::own_connect_svg(&app, Some(&code)).unwrap_or_default();
    let created = Created { id, link, svg, role: f.role, uses, expires };
    let people = load(&app, &auth, Some(created), Draft::default(), None).await?;
    draw(&app, &auth, StatusCode::CREATED, people).await
}

#[derive(Deserialize)]
struct Sure {
    #[serde(default)]
    confirm: bool,
}

async fn revoke(
    State(app): St,
    auth: Option<Auth>,
    Path(id): Path<i64>,
    Form(s): Form<Sure>,
) -> Res<Response> {
    let Some(_) = admin(auth) else { return Ok(login()) };
    if s.confirm {
        let sql = "DELETE FROM invites WHERE id = ?1";
        app.db.run(move |c| crate::db::changed(c.execute(sql, [id])?, "invite")).await?;
    }
    Ok(Redirect::to(HERE).into_response())
}

#[derive(Deserialize)]
struct Change {
    role: Option<String>,
    status: Option<String>,
    quota_gb: Option<String>,
}

async fn change(
    State(app): St,
    auth: Option<Auth>,
    Path(id): Path<i64>,
    Form(c): Form<Change>,
) -> Res<Response> {
    let Some(auth) = admin(auth) else { return Ok(login()) };
    let quota_mib = match c.quota_gb.as_deref().filter(|q| !q.trim().is_empty()) {
        None => None,
        Some(q) => Some(
            crate::units::parse_gigabytes(q)
                .ok_or_else(|| Problem::bad("space: a number of GB, 0 or more"))?,
        ),
    };
    change_user(&app, &auth, id, Edit { role: c.role, status: c.status, quota_mib }).await?;
    Ok(Redirect::to("/admin/personas#accounts").into_response())
}

#[derive(Deserialize)]
struct Door {
    access: String,
}

async fn access(State(app): St, auth: Option<Auth>, Form(d): Form<Door>) -> Res<Response> {
    let Some(_) = admin(auth) else { return Ok(login()) };
    let chosen = Access::parse(&d.access).ok_or_else(|| Problem::bad("access: closed, invite or open"))?;
    let fresh = app.set_access(chosen).await?;
    if let Err(e) = app.door.apply(&app.cfg, &fresh) {
        tracing::warn!("the new settings could not be applied: {e:#}");
    }
    Ok(Redirect::to(HERE).into_response())
}
