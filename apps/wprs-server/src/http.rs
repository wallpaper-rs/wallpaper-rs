use crate::error::Problem;
use crate::net::{API, client_ip, key};
use crate::{
    App, St, account, admin, catalog, device, featuring, files, hiding, legal, mailbox, me, moderation,
    pages, records, served, sync, upload, visibility, votes, web,
};
use axum::Router;
use axum::extract::{ConnectInfo, DefaultBodyLimit, MatchedPath, Request, State};
use axum::http::header::{self, HeaderName, HeaderValue};
use axum::http::{Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub fn router(app: Arc<App>) -> Router {
    let mut modules: Vec<fn() -> crate::Router> =
        vec![account::routes, me::routes, device::routes, catalog::routes];
    modules.extend([
        upload::routes,
        moderation::routes,
        hiding::routes,
        crate::mod_page::routes,
        crate::mod_case::routes,
        admin::routes,
        sync::routes,
        records::routes,
        mailbox::routes,
        files::routes,
        visibility::routes,
        votes::routes,
        featuring::routes,
    ]);
    modules.extend([web::routes, crate::assets::routes, pages::routes, legal::routes, crate::setup::routes]);
    #[cfg(feature = "oidc")]
    modules.push(crate::oidc::routes);
    let inner = modules
        .into_iter()
        .fold(Router::new(), |router, routes| router.merge(routes()))
        .route("/healthz", get(|| async { "ok" }))
        .route("/readyz", get(crate::discovery::ready))
        .route("/.well-known/wprs-server", get(crate::discovery::discovery))
        .route("/repo/{*path}", get(served::serve))
        .fallback(|| async { Problem::missing("page") })
        .layer(middleware::from_fn_with_state(app.clone(), guard))
        .layer(DefaultBodyLimit::max(1 << 20))
        .with_state(app);
    Router::new()
        .fallback_service(inner)
        .layer(middleware::from_fn(rewrite))
        .layer(middleware::from_fn(crate::lang::follow))
}

async fn rewrite(mut req: Request, next: Next) -> Response {
    let wanted =
        req.uri().query().into_iter().flat_map(|q| q.split('&')).find_map(|p| p.strip_prefix("_method="));
    let method = match wanted {
        Some("DELETE") => Some(Method::DELETE),
        Some("PATCH") => Some(Method::PATCH),
        _ => None,
    };
    if let Some(method) = method.filter(|_| req.method() == Method::POST) {
        *req.method_mut() = method;
    }
    next.run(req).await
}

fn csrf_ok(req: &Request) -> bool {
    let h = req.headers();
    if req.method().is_safe()
        || h.get(header::AUTHORIZATION).is_some_and(|v| v.as_bytes().starts_with(b"Bearer "))
    {
        return true;
    }
    if let Some(site) = h.get("sec-fetch-site") {
        return site == "same-origin" || site == "none";
    }
    let host = h.get(header::HOST).and_then(|v| v.to_str().ok());
    h.get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .is_none_or(|o| o.split_once("://").map(|(_, rest)| rest) == host)
}

async fn guard(State(app): St, req: Request, next: Next) -> Response {
    let started = Instant::now();
    let (method, path) = (req.method().clone(), req.uri().path().to_string());
    let route = req.extensions().get::<MatchedPath>().map_or("-", MatchedPath::as_str).to_string();
    let peer = req.extensions().get::<ConnectInfo<SocketAddr>>().map(|c| c.0);
    let ip = client_ip(&app.cfg, req.headers(), peer);
    let (agent, kind) = {
        let text = |name: HeaderName| req.headers().get(name).and_then(|v| v.to_str().ok()).map(String::from);
        (text(header::USER_AGENT), text(header::CONTENT_TYPE).unwrap_or_default())
    };
    let form =
        kind.starts_with("application/x-www-form-urlencoded") || kind.starts_with("multipart/form-data");
    let back = req
        .uri()
        .query()
        .into_iter()
        .flat_map(|q| q.split('&'))
        .find_map(|p| p.strip_prefix("back="))
        .map(String::from);
    let signed =
        req.headers().contains_key(header::AUTHORIZATION) || req.headers().contains_key(header::COOKIE);
    let long = path.starts_with("/api/v1/uploads/")
        || path.ends_with("/decision")
        || path.starts_with("/mod/cases/")
        || path == "/upload";
    let long = long || path == "/api/v1/sync/events" || path.starts_with("/api/v1/sync/blobs/");
    let probe = path == "/healthz" || path == "/readyz";
    let mut response = match app.limits.take(&API, &key(ip, 32, 64)) {
        Err(problem) if !probe => problem.into_response(),
        _ if !csrf_ok(&req) => Problem::forbidden("cross-site request refused").into_response(),
        _ if long => next.run(req).await,
        _ => tokio::time::timeout(Duration::from_secs(30), next.run(req)).await.unwrap_or_else(|_| {
            Problem::new(StatusCode::SERVICE_UNAVAILABLE, "timeout", "the request took too long")
                .into_response()
        }),
    };
    let api = ["/api/", "/repo/", "/.well-known/"].iter().any(|p| path.starts_with(p));
    if form || !api {
        response = crate::result::screen(&app, response, form, back.as_deref()).await;
    }
    secure(&app, &method, &path, signed, &mut response);
    let status = response.status().as_u16();
    let ms = started.elapsed().as_millis() as u64;
    match probe {
        true => tracing::debug!(%method, route, status, ms, "request"),
        false => tracing::info!(%method, route, status, ms, "request"),
    }
    app.clients.write(ip, agent.as_deref(), method.as_str(), &route, status);
    response
}

fn secure(app: &App, method: &Method, path: &str, signed: bool, response: &mut Response) {
    let open = method == Method::GET
        && ["/repo/", "/media/", "/api/v1/catalog", "/api/v1/packages/"].iter().any(|p| path.starts_with(p))
        && !path.ends_with("/reports");
    let page =
        response.headers().get(header::CONTENT_TYPE).is_some_and(|v| v.as_bytes().starts_with(b"text/html"));
    let h = response.headers_mut();
    let mut set =
        |name: &'static str, value: &'static str| drop(h.insert(name, HeaderValue::from_static(value)));
    set("x-content-type-options", "nosniff");
    set("referrer-policy", "no-referrer");
    set("cross-origin-opener-policy", "same-origin");
    set("cross-origin-resource-policy", if open { "cross-origin" } else { "same-site" });
    set("permissions-policy", "camera=(), microphone=(), geolocation=()");
    if page {
        set("content-security-policy", crate::assets::CSP);
    }
    if open {
        set("access-control-allow-origin", "*");
    }
    if app.cfg.secure() {
        set("strict-transport-security", "max-age=63072000; includeSubDomains");
    }
    if signed && !h.contains_key(header::CACHE_CONTROL) {
        h.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    }
}
