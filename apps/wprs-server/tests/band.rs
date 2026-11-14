mod common;

use common::{Server, Who, package_v2, start, upload};
use reqwest::header::{CACHE_CONTROL, CONTENT_TYPE};
use reqwest::{Method, StatusCode};
use serde_json::json;
use std::fs;
use wprs_format::Rating;

async fn login(server: &Server, language: &str) -> (String, String) {
    let call = server.call(Method::GET, "/login", &Who::Nobody).header("accept-language", language);
    let response = call.send().await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let csp = response.headers()["content-security-policy"].to_str().unwrap().to_string();
    (csp, response.text().await.unwrap())
}

async fn cover(server: &Server) -> Option<String> {
    let (_, html) = login(server, "en").await;
    let sha = html.split("href=\"/band/").nth(1).map(|rest| rest[..64].to_string());
    match &sha {
        Some(sha) => assert!(
            html.contains(&format!("src=\"/media/{sha}.jpg\""))
                && html.contains("class=\"banda\"")
                && html.contains("class=\"portada-de\"")
                && !html.contains("f-ninguno"),
            "{html}"
        ),
        None => assert!(
            html.contains("class=\"f-ninguno\"")
                && html.contains("class=\"banda sin\"")
                && !html.contains("portada"),
            "{html}"
        ),
    }
    sha
}

async fn sha_of(server: &Server, admin: &Who, id: &str) -> String {
    let (_, item) = server.json(Method::GET, &format!("/api/v1/packages/{id}"), admin, None).await;
    let versions = item["versions"].as_array().cloned().unwrap_or_default();
    versions.iter().chain([&item]).find_map(|v| v["sha256"].as_str()).unwrap().to_string()
}

async fn decide(server: &Server, admin: &Who, id: &str, reason: &str, decision: &str) {
    let (_, queue) = server.json(Method::GET, "/api/v1/mod/queue", admin, None).await;
    let items = queue["items"].as_array().unwrap().clone();
    let case = items.iter().find(|c| c["package"] == id && c["reason"] == reason).unwrap()["id"].as_i64();
    let body = json!({"decision": decision, "statement": "Decided while testing the band."});
    let path = format!("/api/v1/mod/cases/{}/decision", case.unwrap());
    assert_eq!(server.json(Method::POST, &path, admin, Some(body)).await.0, StatusCode::OK);
}

async fn send(server: &Server, who: &Who, id: &str, rating: Rating) {
    let sent = upload(server, who, &package_v2(server.dir.path(), id, "1.0.0", rating)).await;
    assert!(sent["state"] == "pending_review" || sent["state"] == "published", "{sent}");
}

#[tokio::test(flavor = "multi_thread")]
async fn the_band_shows_the_newest_wallpaper_a_visitor_sees() {
    let server = start(&[]).await;
    wprs_server::init(&server.cfg, "Example wallpapers", false, false).unwrap();
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    let (plain_csp, _) = login(&server, "en").await;
    assert_eq!(cover(&server).await, None);
    send(&server, &ana, "org.example.dawn", Rating::General).await;
    assert_eq!(cover(&server).await, None, "a wallpaper waiting for review");
    decide(&server, &admin, "org.example.dawn", "upload", "approve").await;
    let dawn = sha_of(&server, &admin, "org.example.dawn").await;
    assert_eq!(cover(&server).await.as_deref(), Some(dawn.as_str()));
    let (csp, html) = login(&server, "es").await;
    assert_eq!(csp, plain_csp);
    assert!(!html.contains("style=\""), "{html}");
    assert!(html.contains("Portada: «Dawn 1.0.0», de ana, lo último publicado"), "{html}");
    let (_, html) = login(&server, "en").await;
    assert!(html.contains("Cover: “Dawn 1.0.0” by ana, the latest published"), "{html}");
    let sheet = server.call(Method::GET, &format!("/band/{dawn}.css"), &Who::Nobody).send().await.unwrap();
    assert_eq!(sheet.status(), StatusCode::OK);
    assert_eq!(sheet.headers()[CONTENT_TYPE], "text/css");
    assert!(sheet.headers()[CACHE_CONTROL].to_str().unwrap().contains("immutable"));
    let sheet = sheet.text().await.unwrap();
    assert!(sheet.starts_with(":root{--noche:#") && sheet.contains("--velo:"), "{sheet}");
    let image = server.call(Method::GET, &format!("/media/{dawn}.jpg"), &Who::Nobody).send().await.unwrap();
    assert_eq!(image.headers()[CONTENT_TYPE], "image/jpeg");
    assert!(image.headers()[CACHE_CONTROL].to_str().unwrap().contains("immutable"));
    let account = server.call(Method::GET, "/admin", &admin).send().await.unwrap().text().await.unwrap();
    assert!(account.contains(&format!("href=\"/band/{dawn}.css\"")), "{account}");

    send(&server, &ana, "org.example.night", Rating::Mature).await;
    decide(&server, &admin, "org.example.night", "upload", "approve").await;
    send(&server, &ana, "org.example.dusk", Rating::General).await;
    decide(&server, &admin, "org.example.dusk", "upload", "reject").await;
    assert_eq!(cover(&server).await.as_deref(), Some(dawn.as_str()), "mature or rejected");
    let night = sha_of(&server, &admin, "org.example.night").await;
    let asked = server.call(Method::GET, &format!("/band/{night}.css"), &Who::Nobody).send().await.unwrap();
    assert_eq!(asked.status(), StatusCode::NOT_FOUND);

    send(&server, &ana, "org.example.noon", Rating::General).await;
    decide(&server, &admin, "org.example.noon", "upload", "approve").await;
    let noon = sha_of(&server, &admin, "org.example.noon").await;
    assert_eq!(cover(&server).await.as_deref(), Some(noon.as_str()), "a newer wallpaper");
    let path = "/api/v1/me/packages/org.example.noon/visibility";
    let unlisted = json!({"visibility": "unlisted"});
    assert_eq!(server.json(Method::POST, path, &ana, Some(unlisted)).await.0, StatusCode::OK);
    assert_eq!(cover(&server).await.as_deref(), Some(dawn.as_str()), "unlisted");

    send(&server, &ana, "org.example.pond", Rating::General).await;
    decide(&server, &admin, "org.example.pond", "upload", "approve").await;
    assert_ne!(cover(&server).await.as_deref(), Some(dawn.as_str()));
    let report = json!({"category": "spam", "details": "Not a wallpaper."});
    let path = "/api/v1/packages/org.example.pond/reports";
    server.json(Method::POST, path, &Who::Nobody, Some(report)).await;
    decide(&server, &admin, "org.example.pond", "report", "remove").await;
    assert_eq!(cover(&server).await.as_deref(), Some(dawn.as_str()), "removed");

    let hide = json!({"id": "org.example.dawn", "reason": "Shown by mistake in the catalog."});
    assert_eq!(server.json(Method::POST, "/api/v1/mod/hidden", &admin, Some(hide)).await.0, StatusCode::OK);
    assert_eq!(cover(&server).await, None, "hidden");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_cover_that_cannot_be_read_leaves_the_plain_band() {
    let server = start(&[("WPRS_AUTO_PUBLISH", "all")]).await;
    wprs_server::init(&server.cfg, "Example wallpapers", false, false).unwrap();
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    send(&server, &ana, "org.example.dawn", Rating::General).await;
    let dawn = sha_of(&server, &admin, "org.example.dawn").await;
    assert_eq!(cover(&server).await.as_deref(), Some(dawn.as_str()));
    let data = server.dir.path().join("data");
    fs::write(data.join(format!("thumbs/{dawn}.jpg")), b"\xff\xd8\xff broken").unwrap();
    let db = rusqlite::Connection::open(data.join("db/wprs.sqlite")).unwrap();
    db.execute("UPDATE versions SET band_sheet = NULL", []).unwrap();
    server.app.changed().await;
    assert_eq!(cover(&server).await, None);
    let asked = server.call(Method::GET, &format!("/band/{dawn}.css"), &Who::Nobody).send().await.unwrap();
    assert_eq!(asked.status(), StatusCode::NOT_FOUND);
}
