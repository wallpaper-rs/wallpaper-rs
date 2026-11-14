use crate::auth::{Auth, Role};
use crate::cases::{case, hidden};
use crate::checks::{checks, verdict};
use crate::db::now;
use crate::error::Res;
use crate::frame::Frame;
use crate::lang::{count, since, t, tf, word};
use crate::layout::page_in;
use crate::mod_more::{more, withdrawn};
use crate::moderation::open_cases;
use crate::ui::{Kind, Row, Tone, empty, link_button, notice, rows, status, status_line};
use crate::web::text;
use crate::{App, St};
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use maud::{Markup, html};
use serde::Deserialize;
use serde_json::Value;

pub fn routes() -> crate::Router {
    crate::Router::new().route("/mod", get(review))
}

#[derive(Deserialize)]
struct Seen {
    decided: Option<i64>,
}

pub fn cap(word: &str) -> String {
    let mut letters = word.chars();
    letters.next().map(|f| f.to_uppercase().chain(letters).collect()).unwrap_or_default()
}

pub fn ago(at: i64) -> String {
    since((now() - at) / 1000)
}

pub fn status_text(cases: &[Value]) -> (Tone, String) {
    let reports = cases.iter().filter(|c| is_report(c)).count() as i64;
    let all = count(cases.len() as i64, "{} case to review", "{} cases to review");
    match (cases.len(), reports) {
        (0, _) => (Tone::Good, t("Nothing to review").to_string()),
        (_, 0) => (Tone::Warn, all),
        (_, 1) => (Tone::Warn, tf("{}, one is a report", &[&all])),
        (_, n) => (Tone::Warn, tf("{}, {} are reports", &[&all, &n])),
    }
}

fn is_report(case: &Value) -> bool {
    case["reason"] != "upload" || case["priority"].as_i64().unwrap_or(0) > 0
}

pub fn frame_line(cases: &[Value], failed: bool) -> Markup {
    let (tone, said) = status_text(cases);
    match failed {
        true => status_line(Tone::Bad, t("A decision was not applied"), Some(&said)),
        false => status_line(tone, &said, None),
    }
}

fn report_kind(case: &Value) -> String {
    match text(case, "reason") {
        "report" => {
            let category = text(case, "detail").split(':').next().unwrap_or_default();
            tf("Report: {}", &[&word(category)])
        }
        other => cap(word(other)),
    }
}

fn first(case: &Value) -> bool {
    case["first_upload"].as_i64() == Some(1) || case["first_upload"].as_bool() == Some(true)
}

fn newest(case: &Value) -> String {
    text(case, if case["newer"].is_null() { "version" } else { "newer" }).to_string()
}

fn case_row(case: &Value, allowed: &[String]) -> Row {
    let (tone, said) = match (is_report(case), case["newer"].as_str()) {
        (true, _) => (Tone::Warn, report_kind(case)),
        (false, Some(_)) => (Tone::Warn, t("There is a newer version").to_string()),
        (false, None) => verdict(&checks(case, allowed)),
    };
    let (title, made) = (text(case, "title"), ago(case["created_at"].as_i64().unwrap_or(0)));
    let who = text(case, "uploader");
    let line = match is_report(case) {
        true => tf("{} · by {}", &[&made, &who]),
        false if case["newer"].is_null() && first(case) => tf("{} · first upload · {}", &[&who, &made]),
        false => tf("{} · version {} · {}", &[&who, &newest(case), &made]),
    };
    Row::new(html! { (title) })
        .thumb(format!("/media/{}.jpg", text(case, "sha256")))
        .value(html! { (status(tone, &said)) span { (line) } })
        .action(link_button(Kind::Plain, &format!("/mod/cases/{}", case["id"]), t("Open")))
}

fn queue(cases: &[Value], allowed: &[String]) -> Markup {
    let (reports, uploads): (Vec<&Value>, Vec<&Value>) = cases.iter().partition(|c| is_report(c));
    let list = |title: &str, items: Vec<&Value>| {
        html! { @if !items.is_empty() { section.seccion {
            h2 { (title) }
            (rows(items.into_iter().map(|c| case_row(c, allowed)).collect(), false))
        } } }
    };
    html! {
        (list(t("Reports"), reports))
        (list(t("Uploads waiting"), uploads))
    }
}

pub enum Lead {
    Nothing,
    Failed(Markup),
    Decided(Markup),
}

pub async fn screen(app: &App, auth: &Auth, lead: Lead, cases: Vec<Value>) -> Res<Markup> {
    let items = app.db.run(|c| hidden(c)).await?;
    let allowed = &app.cfg.licenses;
    let line = frame_line(&cases, matches!(lead, Lead::Failed(_)));
    let body = match lead {
        Lead::Decided(note) => note,
        lead => html! {
            @if let Lead::Failed(note) = &lead { (note) }
            @if cases.is_empty() && matches!(lead, Lead::Nothing) {
                (empty(t("Nothing to review."), html! { (t("Reports of content and what new accounts upload arrive here while access is “Open”.")) }))
            }
            (queue(&cases, allowed))
            (withdrawn(&items))
            (more())
        },
    };
    Ok(page_in(app, Some(auth), Frame::new(t("Review")).line(line), "/mod", body))
}

pub fn done(case: &Value, next: Option<i64>) -> Markup {
    let (title, who) = (text(case, "title"), text(case, "uploader"));
    let (said, more) = match text(case, "decision") {
        "approve" => (
            tf("You published “{}”.", &[&title]),
            tf("It is in the catalog now. {} sees the notice in their account.", &[&who]),
        ),
        "reject" => {
            (tf("You rejected “{}”.", &[&title]), tf("{} receives your reason in their account.", &[&who]))
        }
        "remove" => {
            (tf("You withdrew “{}”.", &[&title]), tf("{} receives your reason in their account.", &[&who]))
        }
        "revoke" => {
            (tf("You revoked “{}”.", &[&title]), tf("{} receives your reason in their account.", &[&who]))
        }
        "restore" => (tf("You restored “{}”.", &[&title]), t("It is in the catalog again.").to_string()),
        _ => (
            tf("You dismissed the case of “{}”.", &[&title]),
            t("The wallpaper was not touched.").to_string(),
        ),
    };
    let actions = html! {
        @if let Some(next) = next { (link_button(Kind::Primary, &format!("/mod/cases/{next}"), t("Next case"))) }
        @let kind = if next.is_some() { Kind::Plain } else { Kind::Primary };
        (link_button(kind, "/mod", t("Back to the queue")))
    };
    notice(Tone::Good, &said, html! { p.texto { (more) } }, Some(actions))
}

async fn review(State(app): St, auth: Option<Auth>, Query(seen): Query<Seen>) -> Res<Response> {
    let Some(auth) = auth.filter(|a| a.role >= Role::Moderator) else {
        return Ok(Redirect::to("/login?next=/mod").into_response());
    };
    let wanted = seen.decided;
    let (cases, closed) = app
        .db
        .run(move |c| Ok((open_cases(c)?, wanted.map(|id| case(c, id)).transpose()?.flatten())))
        .await?;
    let closed = closed.filter(|c| c["state"] != "open");
    let next = cases.first().and_then(|n| n["id"].as_i64());
    let lead = closed.map_or(Lead::Nothing, |c| Lead::Decided(done(&c, next)));
    Ok(screen(&app, &auth, lead, cases).await?.into_response())
}
