use crate::auth::{Auth, Role};
use crate::error::{Input, Problem, Res};
use crate::settings::{Access, Settings, put};
use crate::{App, St};
use axum::Json;
use axum::extract::State;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct Change {
    registration: Option<String>,
    auto_publish: Option<String>,
}

fn shown(app: &App, s: &Settings) -> Value {
    json!({
        "registration": s.access().value(),
        "auto_publish": crate::settings::publish_value(s.auto_publish),
        "network": if s.network == crate::settings::Network::Home { "home" } else { "own" },
        "advertise": s.advertise,
        "setup_done": s.done,
        "public_url": app.cfg.public_url,
    })
}

pub async fn read(State(app): St, auth: Auth) -> Res<Json<Value>> {
    auth.at_least(Role::Admin)?;
    Ok(Json(shown(&app, &app.settings())))
}

pub async fn change(State(app): St, auth: Auth, Input(c): Input<Change>) -> Res<Json<Value>> {
    auth.at_least(Role::Admin)?;
    if let Some(r) = &c.registration {
        Access::parse(r).ok_or_else(|| Problem::bad("registration: closed, invite or open"))?;
    }
    if let Some(p) = &c.auto_publish {
        crate::settings::parse_publish(p)
            .ok_or_else(|| Problem::bad("auto_publish: never, trusted or all"))?;
    }
    let fresh = app
        .change(move |conn| {
            let pairs: Vec<(&str, &str)> =
                [("registration", &c.registration), ("auto_publish", &c.auto_publish)]
                    .into_iter()
                    .filter_map(|(k, v)| v.as_deref().map(|v| (k, v)))
                    .collect();
            put(conn, &pairs)
        })
        .await?;
    Ok(Json(shown(&app, &fresh)))
}
