use crate::account::{Register, authenticate, create_user};
use crate::approval::{pending_device, user_code};
use crate::auth::Auth;
use crate::db::now;
use crate::error::Res;
use crate::frame::Frame;
use crate::lang::{Lang, current, since, t, tm, word};
use crate::net::Client;
use crate::pages::{Next, local, start_session};
use crate::ui::{
    Kind, Row, Tone, button, button_named, field, fold, form_actions, link_button, notice, quiet_line, rows,
};
use crate::web::text;
use crate::{App, St};
use axum::extract::{Form, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use maud::{Markup, html};
use serde::Deserialize;

fn sign_in_form(app: &App, next: &str, handle: &str, wrong: Option<&str>) -> Markup {
    let raw = "wrong handle or password";
    let body = html! {
        form.formulario method="post" action="/login" {
            input type="hidden" name="next" value=(next);
            (field(t("Handle"), html! { input name="handle" autocomplete="username" required value=(handle); }, None, None))
            (field(t("Password"), html! {
                input type="password" name="password" autocomplete="current-password" required aria-invalid=[wrong.map(|_| "true")]; },
                None, wrong.map(|_| word(raw))))
            (form_actions(html! {
                (button(Kind::Primary, t("Sign in")))
                @if app.cfg.oidc.is_some() {
                    (link_button(Kind::Plain, &format!("/api/v1/auth/oidc/start?next={next}"), t("Sign in with single sign-on")))
                }
            }))
            p.texto { (t("No account yet?")) " " a href="/register" { (t("Register")) } }
            @if wrong.is_some() && current() != Lang::En {
                (fold(t("Details"), html! { p { code.dato { "401 Unauthorized: " (raw) } } }, false))
            }
        }
    };
    page_in_entry(app, "/login", t("Sign in"), body)
}

fn page_in_entry(app: &App, current: &str, title: &str, body: Markup) -> Markup {
    let frame = Frame::new(title).line(quiet_line(&app.cfg.name));
    crate::layout::page_in(app, None, frame, current, body)
}

pub async fn login_page(State(app): St, Query(n): Query<Next>) -> Markup {
    sign_in_form(&app, local(&n.next), "", None)
}

#[derive(Deserialize)]
pub struct Credentials {
    handle: String,
    password: String,
    #[serde(default)]
    next: String,
}

pub async fn login(State(app): St, client: Client, Form(f): Form<Credentials>) -> Res<Response> {
    match authenticate(&app, &client, &f.handle, f.password).await {
        Ok(user) => start_session(&app, user, &f.next).await,
        Err(problem) if problem.status == StatusCode::UNAUTHORIZED => {
            let form = sign_in_form(&app, local(&f.next), f.handle.trim(), Some("wrong"));
            Ok((StatusCode::UNAUTHORIZED, form).into_response())
        }
        Err(problem) => Err(problem),
    }
}

pub async fn register_page(State(app): St, Query(n): Query<Next>) -> Markup {
    let body = html! {
        form.formulario method="post" action="/register" {
            (field(t("Handle"), html! {
                input name="handle" autocomplete="username" required minlength="3" maxlength="32"; },
                Some(t("3 to 32 characters.")), None))
            (field(t("Password"), html! {
                input type="password" name="password" autocomplete="new-password" required minlength="10"; },
                Some(t("10 characters or more.")), None))
            (field(t("Invitation code"), html! { input name="invite" value=(n.invite) autocomplete="off"; },
                Some(t("It comes filled in if you opened the invitation link.")), None))
            p.texto { (tm("By creating the account you accept the {}. Read the {} too.", &[
                html! { a href="/legal/terms" { (t("terms")) } },
                html! { a href="/legal/privacy" { (t("privacy notice")) } },
            ])) }
            (form_actions(button(Kind::Primary, t("Create account"))))
        }
    };
    page_in_entry(&app, "", t("Create an account"), body)
}

pub async fn register(State(app): St, client: Client, Form(r): Form<Register>) -> Res<Response> {
    let user = create_user(&app, &client, r).await?;
    start_session(&app, user, "/account").await
}

pub async fn link(State(app): St, auth: Option<Auth>, Query(n): Query<Next>) -> Response {
    let code: String = n.code.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '-').collect();
    let Some(auth) = auth else {
        return Redirect::to(&format!("/login?next=/link%3Fcode%3D{code}")).into_response();
    };
    let pending = match user_code(&code) {
        Ok(key) => app.db.run(move |c| pending_device(c, &key)).await.ok(),
        Err(_) => None,
    };
    let warning = notice(
        Tone::Warn,
        t("Approve only a code that your own new device shows on its screen."),
        html! { p.texto { (t("Someone who sends you a code or a link is trying to get into your account.")) } },
        None,
    );
    let body = html! {
        (warning)
        @if let Some(d) = &pending {
            section.seccion aria-labelledby="pide" {
                h2 #pide { (t("A device asks to sign in")) }
                (rows(vec![
                    Row::new(html! { (t("Code")) }).value(html! { code.dato { (code) } }),
                    Row::new(html! { (t("Platform")) }).value(html! { (text(d, "platform")) }),
                    Row::new(html! { (t("Software")) }).value(html! {
                        (d["client"].as_str().filter(|c| !c.is_empty()).unwrap_or(t("not given"))) }),
                    Row::new(html! { (t("Requested")) }).value(html! {
                        (since((now() - d["requested_at"].as_i64().unwrap_or_default()) / 1000)) }),
                ], true))
                form.acciones method="post" action="/api/v1/auth/device/approve?back=/account" {
                    input type="hidden" name="user_code" value=(code);
                    (button_named(Kind::Primary, "approve", "true", t("This is my device: approve")))
                    (button_named(Kind::Plain, "approve", "false", t("Deny")))
                }
            }
        } @else {
            form.formulario method="get" action="/link" {
                (field(t("Code shown on the new device"), html! {
                    input name="code" value=(code) autocomplete="off" required; }, None,
                    (!code.is_empty()).then(|| t("No device waits for this code: it is wrong, used or expired."))))
                (form_actions(button(Kind::Primary, t("Look up"))))
            }
        }
    };
    let frame = Frame::new(t("Link a device"))
        .line(quiet_line(t("For the app or phone you want to add to your account")));
    crate::layout::page_in(&app, Some(&auth), frame, "/account", body).into_response()
}
