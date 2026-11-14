mod facts;
mod mail;
mod mail_routes;
mod mail_row;
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
use crate::{App, Router, St, lan, ui};
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
        .route("/admin/mail", post(mail_routes::save))
        .route("/admin/mail/test", post(mail_routes::test))
}

#[derive(Deserialize)]
struct Open {
    #[serde(default)]
    advanced: bool,
    #[serde(default)]
    mail: String,
}

fn admin(auth: Option<Auth>) -> Option<Auth> {
    auth.filter(|a| a.role >= Role::Admin)
}

#[derive(Default)]
struct View {
    failed: Option<String>,
    advanced: bool,
    asked: mail_row::Asked,
    tested: Option<String>,
}

async fn render(app: &Arc<App>, auth: &Auth, view: View) -> Res<Response> {
    let facts = collect(app).await?;
    let (info, settings) = (app.repo.info(&app.cfg), app.settings());
    let featured =
        app.db.run(|c| one::<i64>(c, "SELECT COUNT(*) FROM featured", [])).await?.unwrap_or_default();
    let failed = view.failed.or_else(|| facts.sign_failed.as_ref().map(|f| f.1.clone()));
    let extras = Extras {
        phone_svg: crate::qr::own_connect_svg(app, None).unwrap_or_default(),
        fingerprint: settings.network == Network::Home,
        repo_url: info.and_then(|i| i["url"].as_str().map(str::to_owned)),
        featured,
        failed,
        advanced: view.advanced,
        asked: view.asked,
        tested: view.tested,
    };
    let (line, body) = page::render(&facts, &mail::status(app).await?, &extras);
    let name = settings.repo_name.clone().unwrap_or_else(|| t("Your wallpaper server").into());
    Ok(page_in(app, Some(auth), Frame::new(&name).line(line), "/admin", body).into_response())
}

async fn show(State(app): St, auth: Option<Auth>, Query(open): Query<Open>) -> Res<Response> {
    let Some(auth) = admin(auth) else {
        return Ok(Redirect::to("/login?next=/admin").into_response());
    };
    let asked = mail_row::Asked { off: open.mail == "off", ..Default::default() };
    render(&app, &auth, View { advanced: open.advanced, asked, ..View::default() }).await
}

pub use facts::Facts;

pub fn watched(facts: &Facts) -> Vec<(&'static str, &'static str, Option<ui::Tone>)> {
    let mut out = Vec::new();
    out.extend(facts.cert.as_ref().map(|c| ("certificate", "Certificate", rows::certificate(c).tone)));
    let failed = facts.sign_failed.is_some();
    out.extend(
        facts.sign.as_ref().map(|s| ("signature", "Index signature", rows::signature(s, failed).tone)),
    );
    out.push(("backup", "Last backup", rows::backup(facts.backup).tone));
    out.extend(facts.disk.as_ref().map(|d| ("disk", "Disk and quotas", rows::disk(d).tone)));
    out
}

async fn sign(State(app): St, auth: Option<Auth>) -> Res<Response> {
    let Some(auth) = admin(auth) else {
        return Ok(Redirect::to("/login?next=/admin").into_response());
    };
    match republish(&app, true).await {
        Ok(_) => Ok(Redirect::to("/admin").into_response()),
        Err(problem) => {
            render(&app, &auth, View { failed: Some(problem.detail.to_string()), ..View::default() }).await
        }
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
