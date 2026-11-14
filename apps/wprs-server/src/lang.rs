mod check;
mod es;
mod when;

use axum::extract::Request;
use axum::http::header::{self, HeaderValue};
use axum::middleware::Next;
use axum::response::Response;
use maud::{Markup, PreEscaped, html};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Mutex, OnceLock};

pub use when::{count, since};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    En,
    Es,
}

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Es => "es",
        }
    }

    pub fn from_header(value: Option<&str>) -> Lang {
        let best = value.into_iter().flat_map(|v| v.split(',')).filter_map(|item| {
            let mut parts = item.split(';');
            let tag = parts.next()?.trim();
            let weight = parts
                .find_map(|p| p.trim().strip_prefix("q=").map(|q| q.trim().parse::<f32>().unwrap_or(0.0)))
                .unwrap_or(1.0);
            (weight > 0.0 && tag != "*").then_some((tag, weight))
        });
        let first = best.fold(None::<(&str, f32)>, |top, item| match top {
            Some(t) if t.1 >= item.1 => Some(t),
            _ => Some(item),
        });
        match first.map(|(tag, _)| tag.split('-').next().unwrap_or_default().to_ascii_lowercase()) {
            Some(primary) if primary == "es" => Lang::Es,
            _ => Lang::En,
        }
    }
}

tokio::task_local! {
    static CURRENT: Lang;
}

static MISSED: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

pub fn current() -> Lang {
    CURRENT.try_with(|l| *l).unwrap_or(Lang::En)
}

pub fn within<R>(lang: Lang, f: impl FnOnce() -> R) -> R {
    CURRENT.sync_scope(lang, f)
}

pub fn missed_texts() -> Vec<&'static str> {
    MISSED.lock().map(|m| m.clone()).unwrap_or_default()
}

pub async fn follow(req: Request, next: Next) -> Response {
    let wanted = req.headers().get(header::ACCEPT_LANGUAGE).and_then(|v| v.to_str().ok());
    let lang = Lang::from_header(wanted);
    let mut response = CURRENT.scope(lang, next.run(req)).await;
    let page =
        response.headers().get(header::CONTENT_TYPE).is_some_and(|v| v.as_bytes().starts_with(b"text/html"));
    if page {
        response.headers_mut().append(header::VARY, HeaderValue::from_static("accept-language"));
    }
    response
}

fn catalogue() -> &'static HashMap<&'static str, &'static str> {
    static MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| es::TEXTS.iter().flat_map(|part| part.iter().copied()).collect())
}

fn vocabulary() -> &'static HashMap<&'static str, &'static str> {
    static MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| es::WORDS.iter().copied().collect())
}

pub fn t(en: &'static str) -> &'static str {
    if current() == Lang::En {
        return en;
    }
    catalogue().get(en).copied().unwrap_or_else(|| {
        if let Ok(mut missed) = MISSED.lock()
            && !missed.contains(&en)
        {
            tracing::warn!(text = en, "no Spanish text");
            missed.push(en);
        }
        en
    })
}

pub fn word(en: &str) -> &str {
    if current() == Lang::En { en } else { vocabulary().get(en).copied().unwrap_or(en) }
}

enum Piece<'a> {
    Text(&'a str),
    Arg(usize),
}

fn expand(template: &str, mut put: impl FnMut(Piece)) {
    let (mut rest, mut next) = (template, 0);
    while let Some(open) = rest.find('{') {
        let Some(close) = rest[open..].find('}') else { break };
        let name = &rest[open + 1..open + close];
        put(Piece::Text(&rest[..open]));
        match name.parse::<usize>() {
            Ok(n) if n >= 1 => put(Piece::Arg(n - 1)),
            _ => {
                put(Piece::Arg(next));
                next += 1;
            }
        }
        rest = &rest[open + close + 1..];
    }
    put(Piece::Text(rest));
}

pub fn tf(en: &'static str, args: &[&dyn Display]) -> String {
    let mut out = String::new();
    expand(t(en), |piece| match piece {
        Piece::Text(s) => out.push_str(s),
        Piece::Arg(i) => {
            if let Some(a) = args.get(i) {
                out.push_str(&a.to_string());
            }
        }
    });
    out
}

pub fn tm(en: &'static str, args: &[Markup]) -> Markup {
    let mut out = String::new();
    expand(t(en), |piece| match piece {
        Piece::Text(s) => out.push_str(&html! { (s) }.into_string()),
        Piece::Arg(i) => {
            if let Some(a) = args.get(i) {
                out.push_str(&a.0);
            }
        }
    });
    html! { (PreEscaped(out)) }
}
