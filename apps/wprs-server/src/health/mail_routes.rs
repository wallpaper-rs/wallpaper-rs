use super::mail_row::{Asked, Draft};
use super::{View, admin, render};
use crate::St;
use crate::db::now;
use crate::error::Res;
use crate::lang::{current, t, tf};
use crate::mail::{
    KEYS, RESULT, Security, Setup, deliver, enqueue, forget, keep_secret, setup, valid_from, valid_to,
};
use crate::settings::{drop_key, put};
use axum::Form;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect, Response};

fn checked(d: &Draft) -> Result<Setup, (&'static str, &'static str)> {
    let host = d.host.trim();
    if host.is_empty() || host.contains(char::is_whitespace) || host.contains('/') {
        return Err(("host", t("Write the name of the mail server, like smtp.example.org.")));
    }
    let port = d.port.trim().parse::<u16>().ok().filter(|p| *p > 0);
    let port = port.ok_or(("port", t("The port is a number from 1 to 65535.")))?;
    if !valid_from(d.from.trim()) {
        return Err(("from", t("Write an address like server@example.org, or Name <server@example.org>.")));
    }
    if !valid_to(d.to.trim()) {
        return Err(("to", t("Write one address, like you@example.org.")));
    }
    Ok(Setup {
        host: host.to_owned(),
        port,
        security: Security::parse(&d.security).unwrap_or(Security::StartTls),
        user: d.user.trim().to_owned(),
        from: d.from.trim().to_owned(),
        to: d.to.trim().to_owned(),
        lang: current(),
    })
}

pub async fn save(
    State(app): St,
    auth: Option<crate::auth::Auth>,
    Form(draft): Form<Draft>,
) -> Res<Response> {
    let Some(auth) = admin(auth) else {
        return Ok(Redirect::to("/login?next=/admin").into_response());
    };
    let data = app.cfg.data.clone();
    if draft.action == "off" {
        app.db.run(move |c| forget(c, &data)).await?;
        return Ok(Redirect::to("/admin").into_response());
    }
    let setup = match checked(&draft) {
        Ok(setup) => setup,
        Err(error) => {
            let asked = Asked { draft: Some(Draft { password: String::new(), ..draft }), error: Some(error) };
            return render(&app, &auth, View { asked, ..View::default() }).await;
        }
    };
    if !draft.password.is_empty() {
        keep_secret(&data, &draft.password)?;
    }
    let (port, lang) = (setup.port.to_string(), setup.lang.code());
    app.db
        .run(move |c| {
            let values =
                [&setup.host, &port, setup.security.value(), &setup.user, &setup.from, &setup.to, lang];
            let pairs: Vec<(&str, &str)> = KEYS.iter().copied().zip(values).collect();
            put(c, &pairs)?;
            RESULT.iter().try_for_each(|key| drop_key(c, key))
        })
        .await?;
    Ok(Redirect::to("/admin").into_response())
}

pub async fn test(State(app): St, auth: Option<crate::auth::Auth>) -> Res<Response> {
    let Some(auth) = admin(auth) else {
        return Ok(Redirect::to("/login?next=/admin").into_response());
    };
    let name = app.cfg.name.clone();
    let queued = enqueue(&app, "test", None, move |_| {
        let body =
            t("This is the test mail of your wallpaper server. If you read it, the notices reach you.");
        (tf("{}: test mail", &[&name]), body.to_string())
    })
    .await?;
    if !queued {
        return Ok(Redirect::to("/admin").into_response());
    }
    deliver(&app, now()).await?;
    let to = app.db.run(|c| Ok(setup(c)?.map(|s| s.to))).await?;
    let sent = super::mail::status(&app).await?;
    let went = matches!(sent.mail, super::mail::Mail::Works { .. });
    render(&app, &auth, View { tested: went.then_some(to).flatten(), ..View::default() }).await
}
