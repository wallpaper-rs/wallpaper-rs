mod common;

use common::{Server, Who, start};
use reqwest::StatusCode;
use std::path::Path;

fn pair(dir: &Path, name: &str, host: &str, expired: bool) -> (String, String) {
    let mut params = rcgen::CertificateParams::new(vec![host.to_owned()]).unwrap();
    if expired {
        params.not_before = rcgen::date_time_ymd(2020, 1, 1);
        params.not_after = rcgen::date_time_ymd(2021, 11, 2);
    }
    let key = rcgen::KeyPair::generate().unwrap();
    let cert = params.self_signed(&key).unwrap();
    let (c, k) = (dir.join(format!("{name}.pem")), dir.join(format!("{name}.key")));
    std::fs::write(&c, cert.pem()).unwrap();
    std::fs::write(&k, key.serialize_pem()).unwrap();
    (c.display().to_string(), k.display().to_string())
}

async fn send(server: &Server, visit: &Who, cert: &str, key: &str, spanish: bool) -> (StatusCode, String) {
    let mut call = server.call(reqwest::Method::POST, "/setup/network", visit);
    if spanish {
        call = call.header("accept-language", "es");
    }
    let response = call.form(&[("network", "own"), ("cert", cert), ("key", key)]).send().await.unwrap();
    (response.status(), response.text().await.unwrap())
}

#[tokio::test]
async fn a_certificate_that_would_not_work_is_refused_with_the_reason() {
    let server = start(&[("WPRS_PUBLIC_URL", "https://127.0.0.1:8443")]).await;
    let visit = server.visit().await;
    let (old, _) = pair(server.dir.path(), "old", "127.0.0.1", true);
    let (_, other) = pair(server.dir.path(), "other", "127.0.0.1", false);
    let (status, page) = send(&server, &visit, &old, &other, false).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    for piece in ["expired on November 2, 2021", "not the one of this certificate", "Check again", "notAfter"]
    {
        assert!(page.contains(piece), "{piece}");
    }
    let (_, page) = send(&server, &visit, &old, &other, true).await;
    assert!(page.contains("venció el 2 de noviembre de 2021") && page.contains("Comprobar otra vez"));
    let (wrong, wrong_key) = pair(server.dir.path(), "wrong", "fondos.example", false);
    let (_, page) = send(&server, &visit, &wrong, &wrong_key, false).await;
    assert!(page.contains("not for 127.0.0.1"), "{page}");
    let (_, page) = send(&server, &visit, "/nowhere/cert.pem", "", false).await;
    assert!(page.contains("finds no file") && page.contains("Write the path of this file too"), "{page}");
    let (good, good_key) = pair(server.dir.path(), "good", "127.0.0.1", false);
    assert_eq!(send(&server, &visit, &good, &good_key, false).await.0, StatusCode::SEE_OTHER);
    let chosen = server.app.settings().network;
    assert_eq!(chosen, wprs_server::settings::Network::Own { cert: good, key: good_key });
}

#[tokio::test]
async fn a_server_on_plain_http_cannot_choose_to_serve_https() {
    let server = start(&[]).await;
    let visit = server.visit().await;
    let response = server.form("/setup/network", &visit, &[("network", "home")]).await;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert!(response.text().await.unwrap().contains("starts with http://"));
    assert_eq!(
        server.form("/setup/network", &visit, &[("network", "own")]).await.status(),
        StatusCode::SEE_OTHER
    );
}
