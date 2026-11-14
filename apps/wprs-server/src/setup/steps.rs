use super::{Stage, again, finished, stage, view, visit};
use crate::account::insert_user;
use crate::creds::{self, check_handle, check_password, digest};
use crate::db::{now, one};
use crate::error::{Problem, Res};
use crate::lang::word;
use crate::net::{Client, LOGIN};
use crate::settings::{Network, put};
use crate::{App, St, certs, lan};
use axum::extract::{Form, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use rusqlite::params;
use serde::Deserialize;

pub fn refused(page: maud::Markup, status: StatusCode) -> Res<Response> {
    Ok((status, [(header::CACHE_CONTROL, "no-store")], page).into_response())
}

pub async fn inside(app: &App, headers: &HeaderMap, at_least: Stage) -> Res<Vec<u8>> {
    let visit = visit(app, headers).await?;
    match stage(app, visit.is_some()).await? {
        Stage::Done => Err(finished()),
        Stage::Code => Err(Problem::forbidden("enter the setup token first")),
        s if s.number() < at_least.number() => {
            Err(Problem::conflict("step", "finish the earlier step first"))
        }
        _ => Ok(visit.unwrap_or_default()),
    }
}

#[derive(Deserialize)]
pub struct Code {
    token: String,
}

pub async fn code(State(app): St, client: Client, Form(f): Form<Code>) -> Res<Response> {
    app.limits.take(&LOGIN, &client.key())?;
    if app.settings().done {
        return Err(finished());
    }
    let (token, visit) = (digest(f.token.trim()), creds::token('v'));
    let hash = digest(&visit);
    let used = app.db.run(move |c| {
        let tx = c.transaction()?;
        let sql = "DELETE FROM setup_tokens WHERE hash = ?1 AND expires_at > ?2";
        let used = tx.execute(sql, params![token, now()])?;
        if used > 0 {
            let sql = "INSERT INTO setup_visits (hash, expires_at) VALUES (?1, ?2)";
            tx.execute(sql, params![hash, now() + crate::db::DAY])?;
        }
        tx.commit()?;
        Ok(used > 0)
    });
    if !used.await? {
        return refused(view::code(&app, Some(f.token.trim())), StatusCode::FORBIDDEN);
    }
    let cookie =
        creds::cookie(&app.cfg, &visit, 86400).replacen(app.cfg.cookie(), &super::cookie_name(&app), 1);
    Ok(([(header::SET_COOKIE, cookie)], again()).into_response())
}

#[derive(Deserialize)]
pub struct Net {
    network: String,
    #[serde(default)]
    cert: String,
    #[serde(default)]
    key: String,
}

pub async fn network(State(app): St, headers: HeaderMap, Form(f): Form<Net>) -> Res<Response> {
    inside(&app, &headers, Stage::Network).await?;
    let (cert, key) = (f.cert.trim().to_owned(), f.key.trim().to_owned());
    let chosen = match f.network.as_str() {
        "home" => Network::Home,
        _ => Network::Own { cert: cert.clone(), key: key.clone() },
    };
    let mut checked = certs::Checked::default();
    if chosen.served() || cert != key && (cert.is_empty() || key.is_empty()) {
        if !app.cfg.secure() {
            return refused(
                view::network_failed(&app, &chosen, view::NetFault::Plain),
                StatusCode::UNPROCESSABLE_ENTITY,
            );
        }
        match &chosen {
            Network::Home => drop(
                lan::certificate(&app.cfg.data, &lan::host(&app.cfg.public_url))
                    .map_err(Problem::internal)?,
            ),
            Network::Own { .. } => {
                if let Err(found) = certs::check(&cert, &key, &lan::host(&app.cfg.public_url), now() / 1000) {
                    checked = found;
                }
                for (path, slot) in [(&cert, &mut checked.cert), (&key, &mut checked.key)] {
                    if path.is_empty() {
                        *slot = Some(certs::Fault::Missing);
                    }
                }
            }
        }
    }
    if checked.cert.is_some() || checked.key.is_some() {
        let page = view::network_failed(&app, &chosen, view::NetFault::Cert(checked));
        return refused(page, StatusCode::UNPROCESSABLE_ENTITY);
    }
    let kind = if chosen == Network::Home { "home" } else { "own" };
    app.change(move |c| put(c, &[("network", kind), ("cert_path", &cert), ("key_path", &key)])).await?;
    Ok(again())
}

#[derive(Deserialize)]
pub struct Account {
    handle: String,
    password: String,
}

pub async fn account(State(app): St, headers: HeaderMap, Form(f): Form<Account>) -> Res<Response> {
    inside(&app, &headers, Stage::Account).await?;
    let handle = f.handle.trim().to_owned();
    if let Err(problem) = check_handle(&handle).and_then(|()| check_password(&handle, &f.password)) {
        let page = view::account(&app, Some(&handle), Some(word(&problem.detail)));
        return refused(page, StatusCode::UNPROCESSABLE_ENTITY);
    }
    let (hash, quota) = (creds::hash(&app, f.password).await?, app.cfg.user_quota);
    let kept = handle.clone();
    let made = app.db.run(move |c| {
        let tx = c.transaction()?;
        if one::<String>(&tx, "SELECT value FROM settings WHERE key = 'setup'", [])?.is_some() {
            return Err(finished());
        }
        let sql = "SELECT id FROM users WHERE id = (SELECT CAST(value AS INTEGER) FROM settings WHERE key = 'admin')";
        let id = match one::<i64>(&tx, sql, [])? {
            Some(id) => {
                let sql = "UPDATE users SET handle = ?1, password_hash = ?2 WHERE id = ?3";
                tx.execute(sql, params![kept, hash, id]).map_err(|_| Problem::conflict("handle", "this handle is taken"))?;
                id
            }
            None => insert_user(&tx, &kept, Some(&hash), "admin", quota)?,
        };
        put(&tx, &[("admin", &id.to_string())])?;
        tx.commit()?;
        Ok(id)
    });
    let id = match made.await {
        Ok(id) => id,
        Err(problem) if problem.status == StatusCode::CONFLICT => {
            let page = view::account(&app, Some(&handle), Some(word(&problem.detail)));
            return refused(page, StatusCode::UNPROCESSABLE_ENTITY);
        }
        Err(problem) => return Err(problem),
    };
    app.change(|_| Ok(())).await?;
    let value = app.db.run(move |c| creds::session(c, id)).await?;
    Ok(([(header::SET_COOKIE, creds::cookie(&app.cfg, &value, 30 * 86400))], again()).into_response())
}
