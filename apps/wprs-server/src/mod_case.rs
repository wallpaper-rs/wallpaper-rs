use crate::auth::{Auth, Role};
use crate::cases::{case, history};
use crate::checks::checks;
use crate::error::{Input, Problem, Res};
use crate::frame::Frame;
use crate::lang::{count, t, tf, word};
use crate::layout::page_in;
use crate::mod_form::{Draft, decision_form};
use crate::mod_page::{Lead, ago, cap, frame_line, screen};
use crate::moderation::open_cases;
use crate::review::{Decision, decide};
use crate::ui::{
    Kind, Row, Tone, back_link, case_head, fold, link_button, notice, pairs, rows, status, subheading,
};
use crate::web::text;
use crate::{App, St};
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use maud::{Markup, html};
use serde_json::Value;

pub fn routes() -> crate::Router {
    crate::Router::new().route("/mod/cases/{id}", get(open).post(apply))
}

fn who(history: &Value, uploader: &str) -> Markup {
    if history["known"] != true {
        return html! { strong { (uploader) } };
    }
    let (earlier, strikes) =
        (history["earlier"].as_i64().unwrap_or(0), history["strikes"].as_i64().unwrap_or(0));
    let past = match earlier {
        0 => t("first upload").to_string(),
        n => tf("{} earlier: published {}, rejected {}", &[&n, &history["published"], &history["rejected"]]),
    };
    let age = tf("account opened {}", &[&ago(history["since"].as_i64().unwrap_or(0))]);
    let warned = match strikes {
        0 => t("no strikes").to_string(),
        n => count(n, "{} strike", "{} strikes"),
    };
    html! { strong { (uploader) } " · " (past) " · " (age) " · " (warned) }
}

fn details(case: &Value) -> Markup {
    let named = tf(
        "{} · reason: {} · state: {}",
        &[&case["id"], &word(text(case, "reason")), &word(text(case, "state"))],
    );
    let package = format!("{} {}", text(case, "package"), text(case, "version"));
    let items = [
        (t("Package"), html! { (package) }),
        (t("Case"), html! { (named) }),
        ("SHA-256", html! { (text(case, "sha256")) }),
    ];
    fold(t("Details"), html! { (pairs(&items)) }, false)
}

fn reported(case: &Value) -> Markup {
    let detail = text(case, "detail");
    html! { @if !detail.is_empty() {
        (subheading(t("Why it is here")))
        p.texto { strong { (cap(word(text(case, "reason")))) } ": " (detail) }
    } }
}

async fn card(app: &App, auth: &Auth, id: i64, draft: Draft) -> Res<Option<Markup>> {
    let (found, list) = app.db.run(move |c| Ok((case(c, id)?, open_cases(c)?))).await?;
    let Some(found) = found.filter(|c| c["state"] == "open") else { return Ok(None) };
    let past = found.clone();
    let history = app.db.run(move |c| history(c, &past)).await?;
    let position = list.iter().position(|c| c["id"] == found["id"]).map_or(1, |p| p + 1);
    let uploader = text(&found, "uploader");
    let own = found["uploader_id"].as_i64() == Some(auth.user);
    let foot = tf(
        "{} · version {} · uploaded {}",
        &[
            &cap(word(text(&found, "kind"))),
            &text(&found, "version"),
            &ago(found["uploaded_at"].as_i64().unwrap_or(0)),
        ],
    );
    let list_checks = checks(&found, &app.cfg.licenses)
        .into_iter()
        .map(|c| Row::new(html! { (c.label) }).value(status(c.tone, &c.text)));
    let title = text(&found, "title");
    let body = html! {
        (back_link("/mod", &tf("Review · case {} of {}", &[&position, &list.len()])))
        article.seccion {
            (case_head(&format!("/media/{}.jpg", text(&found, "sha256")), &tf("Preview of {}", &[&title]), title, &foot))
            (subheading(t("Who uploads it")))
            p.texto { (who(&history, uploader)) }
            (reported(&found))
            (subheading(t("What was already checked")))
            (rows(list_checks.collect(), true))
            (decision_form(&found, own, &draft))
            (details(&found))
        }
    };
    Ok(Some(page_in(app, Some(auth), Frame::new(t("Review")).line(frame_line(&list, false)), "/mod", body)))
}

fn moderator(auth: Option<Auth>) -> Option<Auth> {
    auth.filter(|a| a.role >= Role::Moderator)
}

fn sign_in() -> Response {
    Redirect::to("/login?next=/mod").into_response()
}

async fn open(State(app): St, auth: Option<Auth>, Path(id): Path<i64>) -> Res<Response> {
    let Some(auth) = moderator(auth) else { return Ok(sign_in()) };
    match card(&app, &auth, id, Draft { statement: String::new(), missing: None }).await? {
        Some(page) => Ok(page.into_response()),
        None => Ok(Redirect::to("/mod").into_response()),
    }
}

fn explain(problem: &Problem) -> String {
    match problem.code {
        "case_closed" => t("This case was already decided.").to_string(),
        _ => problem.detail.clone(),
    }
}

fn refused(title: &str, id: i64, problem: &Problem) -> Markup {
    let raw = &problem.detail;
    let said = format!(
        "{} {}: {}",
        problem.status.as_u16(),
        problem.status.canonical_reason().unwrap_or_default(),
        raw
    );
    let actions = link_button(Kind::Primary, &format!("/mod/cases/{id}"), t("Open the case again"));
    let body = html! {
        p.texto { (explain(problem)) " " (t("Look at the case again before deciding.")) }
        (fold(t("Details"), html! { code.dato { (said) } }, false))
    };
    notice(Tone::Bad, &tf("The decision on “{}” was not applied.", &[&title]), body, Some(actions))
}

async fn apply(
    State(app): St,
    auth: Option<Auth>,
    Path(id): Path<i64>,
    Input(d): Input<Decision>,
) -> Res<Response> {
    let Some(auth) = moderator(auth) else { return Ok(sign_in()) };
    let restrictive = matches!(d.decision.as_str(), "reject" | "remove" | "revoke");
    if restrictive && d.statement.trim().chars().count() < 10 {
        let missing = match d.decision.as_str() {
            "reject" => t("rejection"),
            "remove" => t("withdrawal"),
            _ => t("revocation"),
        };
        let draft = Draft { statement: d.statement, missing: Some(missing) };
        let page = card(&app, &auth, id, draft).await?.ok_or_else(|| Problem::missing("case"))?;
        return Ok((axum::http::StatusCode::UNPROCESSABLE_ENTITY, page).into_response());
    }
    match decide(&app, Some(auth.user), id, d).await {
        Ok(_) => Ok(Redirect::to(&format!("/mod?decided={id}")).into_response()),
        Err(problem) => {
            let (found, list) =
                app.db.run(move |c| Ok((case(c, id)?, crate::moderation::open_cases(c)?))).await?;
            let title = found.as_ref().map_or(String::new(), |c| text(c, "title").to_string());
            let lead = Lead::Failed(refused(&title, id, &problem));
            Ok((problem.status, screen(&app, &auth, lead, list).await?).into_response())
        }
    }
}
