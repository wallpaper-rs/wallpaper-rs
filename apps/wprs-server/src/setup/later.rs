use super::steps::{inside, refused};
use super::{Stage, again, held_key, view};
use crate::creds::digest;
use crate::db::one;
use crate::error::{Problem, Res};
use crate::invite::new_invite;
use crate::settings::{self, Access, put};
use crate::{App, St};
use axum::extract::{Form, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use serde::Deserialize;
use std::sync::PoisonError;

#[derive(Deserialize)]
pub struct Repository {
    name: String,
    #[serde(default)]
    online_authors: Option<String>,
}

pub async fn repository(State(app): St, headers: HeaderMap, Form(f): Form<Repository>) -> Res<Response> {
    let visit = inside(&app, &headers, Stage::Repository).await?;
    let name = f.name.trim().to_owned();
    if name.is_empty() || name.chars().count() > 80 {
        return refused(view::repository(&app, Some(&name)), StatusCode::UNPROCESSABLE_ENTITY);
    }
    if app.repo.ready() {
        return Ok(again());
    }
    let (cfg, kept) = (app.cfg.clone(), name.clone());
    let made =
        tokio::task::spawn_blocking(move || crate::init(&cfg, &kept, false, f.online_authors.is_some()));
    let root = made
        .await
        .map_err(Problem::internal)?
        .map_err(|e| Problem::conflict("repository", format!("{e:#}")))?;
    let kept = name.clone();
    app.change(move |c| put(c, &[("root_key", "pending"), ("repo_name", &kept)])).await?;
    app.held.lock().unwrap_or_else(PoisonError::into_inner).key = Some((visit, name, root.secret));
    app.changed().await;
    Ok(again())
}

#[derive(Deserialize)]
pub struct Saved {
    #[serde(default)]
    saved: Option<String>,
}

pub async fn saved(State(app): St, headers: HeaderMap, Form(f): Form<Saved>) -> Res<Response> {
    let visit = inside(&app, &headers, Stage::Key).await?;
    if f.saved.is_none() {
        let held = held_key(&app, Some(&visit));
        return refused(view::key(&app, held.as_ref(), Some(())), StatusCode::UNPROCESSABLE_ENTITY);
    }
    app.change(|c| settings::drop_key(c, "root_key")).await?;
    app.held.lock().unwrap_or_else(PoisonError::into_inner).key = None;
    Ok(again())
}

pub async fn again_repo(app: &App) -> Res<()> {
    let data = app.cfg.data.clone();
    tokio::task::spawn_blocking(move || {
        drop(std::fs::remove_dir_all(data.join("repo")));
        drop(std::fs::remove_file(data.join("keys/online.key")));
    })
    .await
    .map_err(Problem::internal)?;
    app.change(|c| settings::drop_key(c, "root_key")).await?;
    Ok(())
}

pub async fn again_key(State(app): St, headers: HeaderMap) -> Res<Response> {
    let visit = inside(&app, &headers, Stage::Key).await?;
    if !app.settings().key_pending || held_key(&app, Some(&visit)).is_some() {
        return Ok(again());
    }
    again_repo(&app).await?;
    Ok(again())
}

pub async fn first_invite(app: &App, visit: Option<&[u8]>) -> Res<String> {
    let visit = visit.unwrap_or_default().to_vec();
    if let Some((v, code)) = &app.held.lock().unwrap_or_else(PoisonError::into_inner).invite
        && *v == visit
    {
        return Ok(code.clone());
    }
    let admin =
        app.db.run(|c| one::<String>(c, "SELECT value FROM settings WHERE key = 'admin'", [])).await?;
    let admin = admin.and_then(|a| a.parse().ok()).unwrap_or_default();
    let code = new_invite(app, admin, "user", 1, 7).await?.0;
    app.held.lock().unwrap_or_else(PoisonError::into_inner).invite = Some((visit, code.clone()));
    Ok(code)
}

#[derive(Deserialize)]
pub struct Entry {
    access: String,
}

pub async fn access(State(app): St, headers: HeaderMap, Form(f): Form<Entry>) -> Res<Response> {
    let visit = inside(&app, &headers, Stage::Access).await?;
    let chosen = Access::parse(&f.access).ok_or_else(|| Problem::bad("access: closed, invite or open"))?;
    let invite =
        app.held.lock().unwrap_or_else(PoisonError::into_inner).invite.take().filter(|(v, _)| *v == visit);
    let unused = invite.filter(|_| chosen == Access::Closed).map(|(_, code)| digest(&code));
    let fresh = app.change(move |c| {
        let tx = c.unchecked_transaction()?;
        put(&tx, &settings::access_pairs(chosen))?;
        put(&tx, &[("setup", "done")])?;
        tx.execute("DELETE FROM setup_visits", [])?;
        tx.execute("DELETE FROM setup_tokens", [])?;
        if let Some(hash) = unused {
            tx.execute("DELETE FROM invites WHERE code_hash = ?1", [hash])?;
        }
        Ok(tx.commit()?)
    });
    let fresh = fresh.await?;
    if let Err(e) = app.door.apply(&app.cfg, &fresh) {
        tracing::warn!("the new network settings could not be applied: {e:#}");
    }
    Ok(again())
}
