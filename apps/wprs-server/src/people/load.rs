use super::view::{Account, Created, Draft, Invite, People};
use super::{HERE, view};
use crate::App;
use crate::admin::{INVITES, USERS};
use crate::auth::Auth;
use crate::db::{now, rows};
use crate::error::Res;
use crate::frame::Frame;
use crate::lang::t;
use crate::layout::page_in;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::Value;

pub fn text(v: &Value, key: &str) -> String {
    v[key].as_str().unwrap_or_default().to_owned()
}

pub async fn load(
    app: &App,
    auth: &Auth,
    created: Option<Created>,
    draft: Draft,
    revoke: Option<i64>,
) -> Res<People> {
    let at = now();
    let (invites, users) = app
        .db
        .run(move |c| {
            Ok((rows(c, &format!("{INVITES} ORDER BY expires_at"), [at])?, rows(c, &users_sql(), [0])?))
        })
        .await?;
    Ok(People {
        me: auth.user,
        invites: invites
            .iter()
            .map(|i| Invite {
                id: i["id"].as_i64().unwrap_or(0),
                role: text(i, "role"),
                uses: i["uses_left"].as_i64().unwrap_or(0),
                expires: i["expires_at"].as_i64().unwrap_or(0),
            })
            .collect(),
        accounts: users
            .iter()
            .map(|u| Account {
                id: u["id"].as_i64().unwrap_or(0),
                handle: text(u, "handle"),
                role: text(u, "role"),
                status: text(u, "status"),
                quota: u["quota_bytes"].as_u64().unwrap_or(0),
                used: u["used"].as_u64().unwrap_or(0),
                created: u["created_at"].as_i64().unwrap_or(0),
            })
            .collect(),
        access: app.settings().access(),
        created,
        draft,
        revoke,
        now: at,
    })
}

pub fn users_sql() -> String {
    format!(
        "SELECT id, handle, role, status, quota_bytes, created_at, (SELECT COALESCE(SUM(size), 0) FROM versions \
         WHERE uploaded_by = users.id AND state != 'rejected') AS used FROM ({USERS}) AS users ORDER BY id"
    )
}

pub async fn draw(app: &App, auth: &Auth, status: StatusCode, people: People) -> Res<Response> {
    let frame = Frame::new(t("People")).line(view::headline(&people));
    Ok((status, page_in(app, Some(auth), frame, HERE, view::body(&people))).into_response())
}
