use crate::App;
use crate::frame::Frame;
use crate::lang::{Lang, current, t, word};
use crate::layout::bare_page;
use crate::pages::local;
use crate::ui::{Kind, Tone, fold, link_button, notice, pairs, status_line};
use axum::body::{Body, to_bytes};
use axum::http::StatusCode;
use axum::http::header::{self, HeaderValue};
use axum::response::Response;
use maud::{Markup, html};
use serde_json::Value;

pub async fn screen(app: &App, response: Response, form: bool, back: Option<&str>) -> Response {
    let kind = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string();
    let problem = kind.starts_with("application/problem+json");
    let done = form
        && response.status().is_success()
        && (kind.starts_with("application/json") || response.status() == StatusCode::NO_CONTENT);
    if !problem && !done {
        return response;
    }
    let (mut parts, body) = response.into_parts();
    let value: Value =
        serde_json::from_slice(&to_bytes(body, 1 << 20).await.unwrap_or_default()).unwrap_or_default();
    let back = local(back.unwrap_or("/"));
    let title = if problem { t("It did not work") } else { t("Done") };
    let body = if problem { failure(&value, back, parts.status) } else { success(&value, back) };
    let line = if problem {
        status_line(Tone::Bad, headline(parts.status), None)
    } else {
        status_line(Tone::Good, t("It worked"), None)
    };
    parts.status = if problem { parts.status } else { StatusCode::OK };
    parts.headers.remove(header::CONTENT_LENGTH);
    parts.headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html; charset=utf-8"));
    let page = bare_page(app, Frame::new(title).line(line), body);
    Response::from_parts(parts, Body::from(page.into_string()))
}

fn shown(v: &Value) -> String {
    v.as_str().map_or_else(|| v.to_string(), String::from)
}

fn headline(status: StatusCode) -> &'static str {
    match status.as_u16() {
        401 => t("Sign in first"),
        403 => t("You are not allowed to do that"),
        404 => t("That does not exist"),
        409 => t("That clashes with what is already there"),
        429 => t("Too many tries: wait a moment"),
        500..=599 => t("The server failed"),
        _ => t("The server refused it"),
    }
}

fn failure(value: &Value, back: &str, status: StatusCode) -> Markup {
    let raw = shown(&value["detail"]);
    let said = word(&raw);
    let plain = current() == Lang::En || said != raw;
    let detail = if plain {
        html! { p { (said) } }
    } else {
        let reason = status.canonical_reason().unwrap_or_default();
        fold(t("Details"), html! { p { code.dato { (status.as_u16()) " " (reason) ": " (raw) } } }, false)
    };
    html! {
        div.resultado {
            (notice(Tone::Bad, t("The server refused it"), detail, Some(continue_button(back))))
        }
    }
}

fn success(value: &Value, back: &str) -> Markup {
    let items: Vec<(String, Markup)> = value
        .as_object()
        .into_iter()
        .flatten()
        .map(|(k, v)| (k.replace('_', " "), html! { code.dato { (shown(v)) } }))
        .collect();
    let refs: Vec<(&str, Markup)> = items.iter().map(|(k, v)| (k.as_str(), v.clone())).collect();
    html! {
        div.resultado {
            @if !refs.is_empty() { (pairs(&refs)) }
            div.acciones { (continue_button(back)) }
        }
    }
}

fn continue_button(back: &str) -> Markup {
    link_button(Kind::Primary, back, t("Continue"))
}
