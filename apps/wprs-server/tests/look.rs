mod common;

use common::{Who, content_type, start};
use reqwest::{Method, StatusCode};

async fn get(
    server: &common::Server,
    path: &str,
    who: &Who,
) -> (StatusCode, reqwest::header::HeaderMap, String) {
    let response = server.call(Method::GET, path, who).send().await.unwrap();
    (response.status(), response.headers().clone(), response.text().await.unwrap())
}

#[tokio::test]
async fn sheet_and_fonts_are_served_by_the_server_itself() {
    let server = start(&[]).await;
    let (status, headers, css) = get(&server, "/static/style.css", &Who::Nobody).await;
    assert_eq!(status, StatusCode::OK);
    assert!(headers["content-type"].to_str().unwrap().starts_with("text/css"));
    assert!(!css.contains("@import") && !css.contains("http://") && !css.contains("https://"));
    assert!(css.contains(".f-ninguno") && css.contains("--noche"));
    assert!(css.contains("/static/fonts/onest.ttf") && css.contains("/static/fonts/red-hat-display.ttf"));
    assert!(!css.contains("/*"));
    for font in ["onest", "red-hat-display"] {
        let response = server
            .call(Method::GET, &format!("/static/fonts/{font}.ttf"), &Who::Nobody)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(content_type(&response), "font/ttf");
        assert!(response.headers()["cache-control"].to_str().unwrap().contains("max-age=604800"));
        let bytes = response.bytes().await.unwrap();
        assert!(bytes.len() > 50_000 && bytes[..4] == [0, 1, 0, 0]);
    }
}

#[tokio::test]
async fn pages_use_no_other_host_and_no_inline_code() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let paths =
        ["/", "/login", "/register", "/setup", "/legal", "/account", "/admin", "/mod", "/nothing-here"];
    for path in paths {
        let (_, headers, html) = get(&server, path, &admin).await;
        if !content_type_is_html(&headers) {
            continue;
        }
        let csp = headers["content-security-policy"].to_str().unwrap();
        assert!(csp.contains("script-src 'self'") && csp.contains("font-src 'self'"), "{path}: {csp}");
        assert!(
            !csp.contains("unsafe-inline") && !csp.contains("unsafe-eval") && !csp.contains("http"),
            "{path}"
        );
        for attribute in ["src=\"", "href=\"", "action=\""] {
            for piece in html.split(attribute).skip(1) {
                let value = piece.split('"').next().unwrap();
                assert!(!value.contains("//") || value.starts_with("/legal"), "{path}: {value}");
                assert!(!value.starts_with("http"), "{path}: {value}");
            }
        }
        assert!(
            !html.contains("style=\"") && !html.contains("onclick") && !html.contains("<script>"),
            "{path}"
        );
        assert!(html.contains("<script src=\"/static/copy.js\" defer></script>"), "{path}");
    }
}

fn content_type_is_html(headers: &reqwest::header::HeaderMap) -> bool {
    headers.get("content-type").is_some_and(|v| v.as_bytes().starts_with(b"text/html"))
}

#[tokio::test]
async fn copy_script_is_plain_and_the_frame_is_complete() {
    let server = start(&[]).await;
    let (status, headers, js) = get(&server, "/static/copy.js", &Who::Nobody).await;
    assert_eq!(status, StatusCode::OK);
    assert!(headers["content-type"].to_str().unwrap().starts_with("text/javascript"));
    assert!(js.contains("navigator.clipboard") && !js.contains("eval(") && !js.contains("fetch("));
    let (_, _, login) = get(&server, "/login", &Who::Nobody).await;
    for piece in [
        "class=\"saltar\"",
        "banda sin",
        "class=\"cabeza\"",
        "id=\"contenido\"",
        "class=\"pie\"",
        "href=\"/legal/notice\"",
    ] {
        assert!(login.contains(piece), "{piece}");
    }
    let (_, _, setup) = get(&server, "/setup", &Who::Nobody).await;
    assert!(!setup.contains("href=\"/login\""));
    let (status, _, missing) = get(&server, "/packages/none-here", &Who::Nobody).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(missing.contains("nota mal") && missing.contains("Continue") && missing.contains("linea-estado"));
}
