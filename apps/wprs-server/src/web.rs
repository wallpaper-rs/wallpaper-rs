use crate::auth::Auth;
use crate::catalog::{Search, details, search};
use crate::db::one;
use crate::error::{Problem, Res};
use crate::lang::{t, tf, tm, word};
pub use crate::layout::page;
use crate::layout::preview;
use crate::ui::{Kind, Tone, button, checkbox, empty, notice};
use crate::{Router, St};
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use maud::{Markup, PreEscaped, html};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use serde_json::Value;
use wprs_format::markup;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/packages/{id}", get(package))
        .route("/media/{file}", get(media))
}

pub fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_default()
}

fn joined(value: &Value) -> String {
    value.as_array().into_iter().flatten().filter_map(Value::as_str).collect::<Vec<_>>().join(", ")
}

const FACETS: [&str; 8] =
    ["audio", "pointer", "sound", "customizable", "localized", "photosensitive", "mature", "4k"];

async fn home(State(app): St, auth: Option<Auth>, Query(s): Query<Search>) -> Res<Markup> {
    let (q, kind, rating, facet) = (s.q.clone(), s.kind.clone(), s.content.clone(), s.facet.clone());
    let picked = !s.featured.is_empty();
    let (items, next) = app.db.run(move |c| search(c, &s)).await?;
    let encode = |text: &str| utf8_percent_encode(text, NON_ALPHANUMERIC).to_string();
    let body = html! {
        form role="search" method="get" action="/" {
            label { (t("Search")) input type="search" name="q" value=(q); }
            label { (t("Type")) select name="kind" {
                option value="" { (t("Any")) }
                @for k in ["image", "video", "scene", "web"] { option value=(k) selected[kind == k] { (word(k)) } }
            } }
            label { (t("Rating")) select name="content" {
                option value="" { (t("Any")) }
                @for r in ["general", "mature", "unrated"] { option value=(r) selected[rating == r] { (word(r)) } }
            } }
            label { (t("With")) select name="facet" {
                option value="" { (t("Anything")) }
                @for f in FACETS { option value=(f) selected[facet == f] { (word(f)) } }
            } }
            (checkbox(html! { input type="checkbox" name="featured" value="true" checked[picked]; }, t("Featured only"), None))
            (button(Kind::Primary, t("Search")))
        }
        @if items.is_empty() { (empty(t("Nothing found"), html! { (t("Try other words or clear the filters.")) })) }
        ul.grid { @for i in &items { li {
            (preview(text(i, "thumbnail"), "", 320, i["content"] == "mature"))
            a href={ "/packages/" (text(i, "id")) } { (text(i, "title")) }
            br;
            small {
                @if i["featured"] == true { (t("Featured")) " · " }
                (word(text(i, "kind"))) " · " (text(i, "license")) " · ▲ " (i["votes"]["up"]) " ▼ " (i["votes"]["down"])
                @if let Some(o) = i["owner"].as_str() { " · " (o) }
            }
        } } }
        @if let Some(next) = next {
            p { a rel="next" href={ "/?cursor=" (next) "&q=" (encode(&q)) "&kind=" (encode(&kind))
                "&content=" (encode(&rating)) "&facet=" (encode(&facet)) } {
                (t("More wallpapers"))
            } }
        }
    };
    Ok(page(&app, auth.as_ref(), "/", t("Wallpapers"), body))
}

async fn package(State(app): St, auth: Option<Auth>, Path(id): Path<String>) -> Res<Markup> {
    let key = id.clone();
    let (latest, versions) = app.db.run(move |c| details(c, &key)).await?;
    let (title, owner) = (text(&latest, "title"), latest["owner"].as_str().unwrap_or(t("a deleted account")));
    let render = if latest["content"].is_null() { markup::verbatim } else { markup::html };
    let description = render(text(&latest, "description"));
    let body = html! {
        (preview(text(&latest, "thumbnail"), &tf("Preview of {}", &[&title]), 640, latest["content"] == "mature"))
        @if let Some(reason) = latest["hidden"].as_str() {
            (notice(Tone::Warn, t("Out of the catalog"), html! { p { (tm("The moderators took this wallpaper out of the catalog: {}", &[html! { (reason) }])) } }, None))
        }
        @if versions.iter().any(|v| v["listed"] == false) {
            p.texto { (t("Unlisted: this wallpaper, or some of its versions, stays out of the catalog and search; \
                whoever has this link can see and install it. It is not private: it stays in the signed \
                repository files, which anyone can read.")) }
        }
        @if latest["photosensitive"] == true {
            (notice(Tone::Warn, t("Flashing lights"), html! { p { (t("This wallpaper has flashing lights or patterns that can affect people with \
                photosensitive epilepsy.")) } }, None))
        }
        (PreEscaped(description))
        (crate::votes::ballot(&id, &latest, auth.is_some()))
        dl {
            dt { (t("Package")) } dd { code { (id) } }
            dt { (t("Type")) } dd { (word(text(&latest, "kind"))) }
            dt { (t("License")) } dd { (text(&latest, "license")) }
            dt { (t("Authors")) } dd { (joined(&latest["authors"])) }
            dt { (t("Tags")) } dd { (joined(&latest["tags"])) }
            dt { (t("Rating")) } dd { (word(latest["content"].as_str().unwrap_or("unrated"))) }
            dt { (t("Facets")) } dd { (joined(&latest["facets"])) }
            dt { (t("Uploaded by")) } dd { (owner) }
            dt { "SHA-256" } dd { code { (text(&latest, "sha256")) } }
        }
        p {
            a href=(text(&latest, "download")) { (tf("Download version {}", &[&text(&latest, "version")])) }
            " · " (t("or install it with a client that verifies the signed repository."))
        }
        h2 { (t("Versions")) }
        ul { @for v in &versions { li { a href=(text(v, "download")) { (text(v, "version")) } } } }
        h2 { (t("Report this wallpaper")) }
        form.formulario method="post" action={ "/api/v1/packages/" (id) "/reports?back=/packages/" (id) } {
            label { (t("Reason")) select name="category" {
                @for c in ["copyright", "illegal", "malware", "mature", "spam", "other"] {
                    option value=(c) { (word(c)) }
                }
            } }
            label { (t("Details")) textarea name="details" rows="3" maxlength="4000" {} }
            button { (t("Send report")) }
        }
        p.texto {
            (tm("For legal notices with your contact details use the {}.", &[html! { a href="/legal/notice" { (t("notice form")) } }]))
        }
    };
    Ok(page(&app, auth.as_ref(), "/", title, body))
}

async fn media(State(app): St, Path(file): Path<String>) -> Res<Response> {
    let lower =
        |s: &&str| s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    let sha = file.strip_suffix(".jpg").filter(lower).ok_or_else(|| Problem::missing("image"))?.to_string();
    let (key, sql) =
        (sha.clone(), "SELECT 1 FROM versions WHERE sha256 = ?1 AND state IN ('pending', 'published')");
    app.db.run(move |c| one::<i64>(c, sql, [key])).await?.ok_or_else(|| Problem::missing("image"))?;
    let path = app.cfg.data.join("thumbs").join(format!("{sha}.jpg"));
    let file = tokio::fs::File::open(path).await.map_err(|_| Problem::missing("image"))?;
    let headers = [
        (header::CONTENT_TYPE, "image/jpeg"),
        (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
    ];
    Ok((headers, Body::from_stream(tokio_util::io::ReaderStream::new(file))).into_response())
}
