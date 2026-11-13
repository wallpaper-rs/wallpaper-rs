use crate::Router;
use axum::http::header;
use axum::routing::get;

pub const CSP: &str = "default-src 'none'; script-src 'self'; style-src 'self'; font-src 'self'; \
                       img-src 'self'; form-action 'self'; frame-ancestors 'none'; base-uri 'none'";

const STYLE: &str = include_str!("../assets/style.css");
const COPY: &str = include_str!("../assets/copy.js");
const ONEST: &[u8] = include_bytes!("../assets/fonts/Onest.ttf");
const RED_HAT_DISPLAY: &[u8] = include_bytes!("../assets/fonts/RedHatDisplay.ttf");

const SHORT: &str = "public, max-age=3600";
const LONG: &str = "public, max-age=604800";

pub fn routes() -> Router {
    Router::new()
        .route(
            "/static/style.css",
            get(|| async { ([(header::CONTENT_TYPE, "text/css"), (header::CACHE_CONTROL, SHORT)], STYLE) }),
        )
        .route(
            "/static/copy.js",
            get(|| async {
                ([(header::CONTENT_TYPE, "text/javascript"), (header::CACHE_CONTROL, SHORT)], COPY)
            }),
        )
        .route(
            "/static/fonts/onest.ttf",
            get(|| async { ([(header::CONTENT_TYPE, "font/ttf"), (header::CACHE_CONTROL, LONG)], ONEST) }),
        )
        .route(
            "/static/fonts/red-hat-display.ttf",
            get(|| async {
                ([(header::CONTENT_TYPE, "font/ttf"), (header::CACHE_CONTROL, LONG)], RED_HAT_DISPLAY)
            }),
        )
}
