mod advanced;
mod devices;
mod phone;
mod security;
mod works;

use crate::St;
use crate::account::profile;
use crate::auth::Auth;
use crate::error::{Problem, Res};
use crate::frame::Frame;
use crate::lang::{t, tf, word};
use crate::me::listing;
use crate::sync::head;
use crate::ui::{Tone, empty, size, status_line};
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use maud::html;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Default)]
pub struct Ask {
    #[serde(default)]
    remove: String,
}

pub async fn account(State(app): St, auth: Option<Auth>, Query(ask): Query<Ask>) -> Res<Response> {
    let Some(auth) = auth else { return Ok(Redirect::to("/login?next=/account").into_response()) };
    let user = auth.user;
    let data = app.db.run(move |c| {
        let lists = ["versions", "uploads", "devices", "tokens", "keys"].map(|w| listing(c, user, w));
        Ok((profile(c, user)?, lists.into_iter().collect::<Res<Vec<_>>>()?, head(c, user).ok()))
    });
    let (me, lists, sync) = data.await?;
    let [versions, uploads, devices, tokens, keys] =
        <[Vec<Value>; 5]>::try_from(lists).map_err(|_| Problem::internal("account lists"))?;
    let fresh = versions.is_empty() && uploads.is_empty() && devices.is_empty();
    let asking = !ask.remove.is_empty() && devices.iter().any(|d| d["id"].as_str() == Some(&ask.remove));
    let (used, quota) =
        (me["used_bytes"].as_u64().unwrap_or_default(), me["quota_bytes"].as_u64().unwrap_or_default());
    let line = status_line(
        Tone::Good,
        &tf("You signed in as {} ({})", &[&auth.handle, &word(auth.role.name())]),
        Some(&tf("you use {} of {} for wallpapers", &[&size(used), &size(quota)])),
    );
    let phone = phone::phone(&app, fresh);
    let body = html! {
        @if fresh {
            (empty(t("Your account is ready."), html! { (t("To have the home wallpapers on your phone, read the \
                code below with the app. If you make wallpapers, upload them from here.")) }))
            (phone)
        }
        (works::works(&versions, &uploads, !fresh && !asking))
        (devices::devices(&app, &devices, if asking { &ask.remove } else { "" }, sync.as_ref()))
        @if !fresh { (phone) }
        (security::security(&app))
        (advanced::advanced(&app, auth.user, &tokens, &keys))
    };
    Ok(page_in_account(&app, &auth, Frame::new(t("Your account")).line(line), body).into_response())
}

fn page_in_account(app: &crate::App, auth: &Auth, frame: Frame, body: maud::Markup) -> maud::Markup {
    crate::layout::page_in(app, Some(auth), frame, "/account", body)
}
