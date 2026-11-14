mod facts;
mod mail;
mod page;
mod rows;
#[cfg(test)]
mod tests;

use crate::auth::{Auth, Role};
use crate::db::one;
use crate::error::{Problem, Res};
use crate::frame::Frame;
use crate::jobs::republish;
use crate::lang::t;
use crate::layout::page_in;
use crate::settings::Network;
use crate::{App, Router, St, lan};
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use page::Extras;
use serde::Deserialize;
use std::sync::Arc;

pub use facts::collect;

pub fn routes() -> Router {
    Router::new()
        .route("/admin", get(show))
        .route("/admin/sign", post(sign))
        .route("/admin/certificate", post(renew))
}

#[derive(Deserialize)]
struct Open {
    #[serde(default)]
    advanced: bool,
}

fn admin(auth: Option<Auth>) -> Option<Auth> {
    auth.filter(|a| a.role >= Role::Admin)
}

async fn render(app: &Arc<App>, auth: &Auth, failed: Option<String>, advanced: bool) -> Res<Response> {
    let facts = collect(app).await?;
    let (info, settings) = (app.repo.info(&app.cfg), app.settings());
    let featured =
        app.db.run(|c| one::<i64>(c, "SELECT COUNT(*) FROM featured", [])).await?.unwrap_or_default();
    let extras = Extras {
        phone_svg: crate::qr::own_connect_svg(app, None).unwrap_or_default(),
        fingerprint: settings.network == Network::Home,
        repo_url: info.and_then(|i| i["url"].as_str().map(str::to_owned)),
        featured,
        failed,
        advanced,
    };
    let (line, body) = page::render(&facts, &mail::status(app), &extras);
    let name = settings.repo_name.clone().unwrap_or_else(|| t("Your wallpaper server").into());
    Ok(page_in(app, Some(auth), Frame::new(&name).line(line), "/admin", body).into_response())
}

async fn show(State(app): St, auth: Option<Auth>, Query(open): Query<Open>) -> Res<Response> {
    let Some(auth) = admin(auth) else {
        return Ok(Redirect::to("/login?next=/admin").into_response());
    };
    render(&app, &auth, None, open.advanced).await
}

async fn sign(State(app): St, auth: Option<Auth>) -> Res<Response> {
    let Some(auth) = admin(auth) else {
        return Ok(Redirect::to("/login?next=/admin").into_response());
    };
    match republish(&app, true).await {
        Ok(_) => Ok(Redirect::to("/admin").into_response()),
        Err(problem) => render(&app, &auth, Some(problem.detail.to_string()), false).await,
    }
}

async fn renew(State(app): St, auth: Option<Auth>) -> Res<Response> {
    let Some(_) = admin(auth) else {
        return Ok(Redirect::to("/login?next=/admin").into_response());
    };
    if app.settings().network != Network::Home {
        return Err(Problem::conflict(
            "own_certificate",
            "the certificate is yours: replace its files and restart",
        ));
    }
    let dir = app.cfg.data.join("tls");
    for file in ["cert.der", "key.der"] {
        let _ = std::fs::remove_file(dir.join(file));
    }
    lan::certificate(&app.cfg.data, &lan::host(&app.cfg.public_url))?;
    if let Err(e) = app.door.apply(&app.cfg, &app.settings()) {
        tracing::warn!("the renewed certificate could not be applied: {e:#}");
    }
    Ok(Redirect::to("/admin").into_response())
}
