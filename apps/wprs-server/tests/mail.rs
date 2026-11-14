mod common;

use common::smtp::{Inbox, listen};
use common::{Server, Who, package, start, upload};
use reqwest::{Method, StatusCode};
use serde_json::json;
use std::time::Duration;

const DAWN: &str = "org.example.dawn";

async fn page(server: &Server, path: &str, who: &Who, lang: Option<&str>) -> String {
    let mut request = server.call(Method::GET, path, who);
    if let Some(lang) = lang {
        request = request.header("accept-language", lang);
    }
    request.send().await.unwrap().text().await.unwrap()
}

fn settings(port: &str) -> Vec<(&'static str, String)> {
    let fields = [
        ("host", "127.0.0.1"),
        ("port", port),
        ("security", "none"),
        ("user", "ana"),
        ("password", "a mail secret"),
        ("from", "Example <server@example.org>"),
        ("to", "ana@example.org"),
        ("action", "save"),
    ];
    fields.into_iter().map(|(k, v)| (k, v.to_string())).collect()
}

async fn save(server: &Server, who: &Who, fields: &[(&'static str, String)]) -> reqwest::Response {
    let fields: Vec<(&str, &str)> = fields.iter().map(|(k, v)| (*k, v.as_str())).collect();
    server.form("/admin/mail", who, &fields).await
}

async fn arrived(inbox: &Inbox, count: usize) -> Vec<String> {
    for _ in 0..100 {
        if inbox.mails().len() >= count {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    inbox.mails()
}

#[tokio::test(flavor = "multi_thread")]
async fn only_the_administrator_reaches_the_mail_settings() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let moderator = server.user(&admin, "leo", "moderator").await;
    let user = server.user(&admin, "eva", "user").await;
    for who in [&Who::Nobody, &moderator, &user] {
        let response = save(&server, who, &settings("2525")).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        let tested = server.form("/admin/mail/test", who, &[]).await;
        assert_eq!(tested.status(), StatusCode::SEE_OTHER);
        assert!(!page(&server, "/admin", who, None).await.contains("Mail settings"));
    }
    let off = page(&server, "/admin", &admin, None).await;
    assert!(
        off.contains("Not set up: the server sends no notices by mail") && !off.contains("ana@example.org")
    );
    assert!(!server.dir.path().join("data/keys/mail.secret").exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn the_administrator_sets_the_mail_up_and_a_test_mail_arrives() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let mut wrong = settings("not a port");
    wrong[0].1 = "smtp.example.org".into();
    let shown = save(&server, &admin, &wrong).await.text().await.unwrap();
    assert!(shown.contains("The port is a number from 1 to 65535.") && shown.contains("smtp.example.org"));
    assert!(!shown.contains("a mail secret"));
    let inbox = listen().await;
    let response = save(&server, &admin, &settings(&inbox.port.to_string())).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let ready = page(&server, "/admin", &admin, None).await;
    assert!(ready.contains("Ready to write to ana@example.org") && !ready.contains("a mail secret"));
    assert!(ready.contains("If a row turns red or has little time left, we write to ana@example.org."));
    let tested = server.form("/admin/mail/test", &admin, &[]).await.text().await.unwrap();
    assert!(tested.contains("The test mail went out to ana@example.org."));
    let mails = arrived(&inbox, 1).await;
    assert!(mails.iter().any(|m| m.contains("This is the test mail of your wallpaper server.")));
    let english = page(&server, "/admin", &admin, None).await;
    assert!(english.contains("Reaches ana@example.org") && english.contains("Mail settings"));
    let spanish = page(&server, "/admin", &admin, Some("es")).await;
    for word in
        ["Llega a ana@example.org", "Enviar un correo de prueba", "Ajustes del correo", "te escribimos a"]
    {
        assert!(spanish.contains(word), "{word}");
    }
    assert!(!spanish.contains("a mail secret"));
    assert!(spanish.contains("href=\"/admin?mail=off#salud\"") && !spanish.contains("¿Apagar el correo?"));
    let asking = page(&server, "/admin?mail=off", &admin, Some("es")).await;
    assert!(
        asking.contains("¿Apagar el correo? El servidor olvida")
            && asking.contains("form class=\"confirmar\"")
    );
    assert!(
        asking.contains("value=\"off\"")
            && asking.contains("href=\"/admin#salud\"")
            && asking.contains("Cancelar")
    );
    let kept = page(&server, "/admin", &admin, None).await;
    assert!(kept.contains("Reaches ana@example.org") && !kept.contains("Turn the mail off?"));
    assert!(server.dir.path().join("data/keys/mail.secret").exists());
    let response = server.form("/admin/mail", &admin, &[("action", "off")]).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert!(page(&server, "/admin", &admin, None).await.contains("Not set up"));
    assert!(!server.dir.path().join("data/keys/mail.secret").exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_report_mails_who_moderates_without_the_reporter_and_the_queue_says_so() {
    let server = start(&[]).await;
    wprs_server::init(&server.cfg, "Example wallpapers", false, false).unwrap();
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    upload(&server, &ana, &package(server.dir.path(), DAWN, "1.0.0")).await;
    let (_, queue) = server.json(Method::GET, "/api/v1/mod/queue", &admin, None).await;
    let case = queue["items"][0]["id"].as_i64().unwrap();
    let decision = json!({"decision": "approve"});
    server.json(Method::POST, &format!("/api/v1/mod/cases/{case}/decision"), &admin, Some(decision)).await;
    let inbox = listen().await;
    save(&server, &admin, &settings(&inbox.port.to_string())).await;
    let report = json!({"category": "malware", "details": "sent by bob@example.org"});
    let path = format!("/api/v1/packages/{DAWN}/reports");
    assert_eq!(server.json(Method::POST, &path, &Who::Nobody, Some(report)).await.0, StatusCode::CREATED);
    let mails = arrived(&inbox, 1).await;
    let mail = mails.iter().find(|m| m.contains("a new report to review")).expect("the case mail");
    assert!(mail.contains("/mod/cases/") && !mail.contains("bob@example.org"));
    let subject = mail.lines().find(|l| l.starts_with("Subject:")).unwrap();
    assert!(!subject.contains("Dawn"));
    for _ in 0..50 {
        if page(&server, "/mod", &admin, None).await.contains("notice sent by mail") {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("the queue does not say the notice went out");
}
