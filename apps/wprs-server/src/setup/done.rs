use crate::App;
use crate::auth::{Auth, Role};
use crate::db::one;
use crate::error::Res;
use crate::frame::Frame;
use crate::lang::{t, tf, tm};
use crate::layout::setup_screen;
use crate::settings::Network;
use crate::ui::{Kind, Row, Tone, copyable, link_button, qr, rows, status};
use axum::http::header;
use axum::response::{IntoResponse, Redirect, Response};
use maud::html;

pub async fn screen(app: &App, auth: Option<Auth>) -> Res<Response> {
    let Some(auth) = auth.filter(|a| a.at_least(Role::Admin).is_ok()) else {
        return Ok(Redirect::to("/").into_response());
    };
    let count = app.db.run(|c| one::<i64>(c, "SELECT COUNT(*) FROM packages", [])).await?.unwrap_or_default();
    let settings = app.settings();
    let home = settings.network == Network::Home;
    let svg = crate::qr::own_connect_svg(app, None).unwrap_or_default();
    let repo = app.repo.info(&app.cfg).and_then(|v| v["url"].as_str().map(str::to_owned)).unwrap_or_default();
    let name = settings.repo_name.clone().unwrap_or_else(|| t("Your wallpaper server").into());
    let line = html! {
        p.linea-estado {
            (status(Tone::Good, t("Set up")))
            @if count == 0 { span.tenue { (t("No wallpapers yet")) } }
        }
    };
    let (read, alt) = match home {
        true => (
            t("The phone reads this code: it brings the address and the fingerprint."),
            t("QR code with the address and the fingerprint of the server"),
        ),
        false => (
            t("The phone reads this code: it brings the address."),
            t("QR code with the address of the server"),
        ),
    };
    let web = Row::new(html! { (t("The web")) }).value(html! { span.dato { (app.cfg.public_url) } });
    let body = html! {
        div.vacio {
            b { (t("Done: your server already hands out wallpapers.")) }
            p { (t("What you publish appears in the app and on the phone of whoever connects with these details.")) }
        }
        section.seccion aria-labelledby="tel" {
            h2 #tel { (t("For the app and the phone")) }
            (qr(&svg, alt, html! {
                b { (read) }
                span.texto { (t("In the desktop app, paste the repository address below.")) }
            }))
            (rows(vec![web], true))
            h3.subtitulo { (t("Repository address, with its fingerprint")) }
            (copyable(&repo, t("Copy"), t("Copied")))
        }
        section.seccion aria-labelledby="sub" {
            h2 #sub { (t("To upload wallpapers")) }
            ol.texto.lista-pasos {
                li { (t("In the desktop app, open Account → Add server.")) }
                li { (tf("Paste the repository address above and sign in as {}.", &[&auth.handle])) }
                li { (tm("In the editor of your wallpaper, press {} and choose this server.", &[html! { "«" (t("Publish…")) "»" }])) }
            }
            p.texto { (t("Without the app, you can also upload a .wpak file from your Account page in this browser.")) }
        }
        div.acciones.final {
            (link_button(Kind::Primary, "/admin", t("Go to Administration")))
            span.porque { (t("There you see the health of the server; the invitations are in People.")) }
        }
    };
    let page = setup_screen(app, Frame::new(&name).line(line), body);
    Ok(([(header::CACHE_CONTROL, "no-store")], page).into_response())
}
