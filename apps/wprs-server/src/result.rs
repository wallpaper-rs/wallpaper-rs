use crate::App;
use crate::frame::Frame;
use crate::layout::bare_page;
use crate::pages::local;
use crate::ui::{Kind, Tone, link_button, notice, pairs, status_line};
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
    let title = if problem { parts.status.canonical_reason().unwrap_or("Error") } else { "Done" };
    let body = if problem { failure(&value, back) } else { success(&value, back) };
    let line = if problem {
        status_line(Tone::Bad, "It did not work", None)
    } else {
        status_line(Tone::Good, "It worked", None)
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

fn failure(value: &Value, back: &str) -> Markup {
    let detail = html! { p { (shown(&value["detail"])) } };
    html! {
        div.resultado {
            (notice(Tone::Bad, "The server refused it", detail, Some(continue_button(back))))
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
    link_button(Kind::Primary, back, "Continue")
}
