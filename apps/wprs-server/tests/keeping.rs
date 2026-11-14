mod common;

use common::{Who, start};
use reqwest::{Method, StatusCode};

async fn page(server: &common::Server, path: &str, who: &Who, lang: Option<&str>) -> (StatusCode, String) {
    let mut request = server.call(Method::GET, path, who);
    if let Some(lang) = lang {
        request = request.header("accept-language", lang);
    }
    let response = request.send().await.unwrap();
    (response.status(), response.text().await.unwrap())
}

#[tokio::test]
async fn nobody_but_the_administrator_reaches_the_health_or_people_pages() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let trusted = server.user(&admin, "leo", "moderator").await;
    for path in ["/admin", "/admin/personas"] {
        for who in [&Who::Nobody, &trusted] {
            assert_eq!(page(&server, path, who, None).await.0, StatusCode::SEE_OTHER, "{path}");
        }
        let got = page(&server, path, &admin, None).await;
        assert_eq!(got.0, StatusCode::OK, "{path} {}", got.1);
    }
    for path in ["/admin/sign", "/admin/certificate", "/admin/personas/invites", "/admin/personas/access"] {
        let response = server
            .form(path, &trusted, &[("role", "user"), ("uses", "1"), ("days", "1"), ("access", "open")])
            .await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER, "{path}");
    }
    let (_, home) = page(&server, "/", &trusted, None).await;
    assert!(!home.contains("/admin/personas"));
    let (_, home) = page(&server, "/", &admin, None).await;
    assert!(home.contains("/admin/personas"));
}

#[tokio::test]
async fn the_health_page_lists_its_rows_in_both_languages() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let (_, english) = page(&server, "/admin", &admin, None).await;
    for word in ["Health", "Last backup", "Disk and quotas", "Mail", "Pending cases", "Live invitations"] {
        assert!(english.contains(word), "{word}");
    }
    let (_, spanish) = page(&server, "/admin", &admin, Some("es")).await;
    for word in [
        "Salud",
        "Última copia de seguridad",
        "Disco y cuotas",
        "Correo",
        "Casos pendientes",
        "Invitaciones vivas",
        "Cómo programarla",
        "Cómo arreglarlo",
        "Nunca",
    ] {
        assert!(spanish.contains(word), "{word}");
    }
    let (_, open) = page(&server, "/admin?advanced=true", &admin, Some("es")).await;
    assert!(open.contains("Informe de transparencia") && open.contains("Registro de auditoría"));
    assert!(wprs_server::missed_texts().is_empty(), "{:?}", wprs_server::missed_texts());
}

#[tokio::test]
async fn an_invitation_is_made_shown_once_and_revoked_after_confirming() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let fields = [("role", "trusted"), ("uses", "3"), ("days", "30")];
    let response = server.form("/admin/personas/invites", &admin, &fields).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    let made = response.text().await.unwrap();
    assert!(made.contains("Your new invitation") && made.contains("/register?invite="));
    let (_, again) = page(&server, "/admin/personas", &admin, None).await;
    assert!(!again.contains("Your new invitation") && !again.contains("/register?invite="));
    assert!(again.contains("3 uses left"));
    let (_, items) = server.json(Method::GET, "/api/v1/admin/invites", &admin, None).await;
    let id = items["items"][0]["id"].as_i64().unwrap();
    let (_, ask) = page(&server, &format!("/admin/personas?revoke={id}"), &admin, None).await;
    assert!(ask.contains("Revoke this invitation?") && ask.contains("name=\"confirm\""));
    let skip = server.form(&format!("/admin/personas/invites/{id}/revoke"), &admin, &[]).await;
    assert_eq!(skip.status(), StatusCode::SEE_OTHER);
    let (_, items) = server.json(Method::GET, "/api/v1/admin/invites", &admin, None).await;
    assert_eq!(items["items"].as_array().unwrap().len(), 1);
    let done =
        server.form(&format!("/admin/personas/invites/{id}/revoke"), &admin, &[("confirm", "true")]).await;
    assert_eq!(done.status(), StatusCode::SEE_OTHER);
    let (_, items) = server.json(Method::GET, "/api/v1/admin/invites", &admin, None).await;
    assert!(items["items"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn a_refused_invitation_keeps_what_was_typed_and_says_why() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let fields = [("role", "user"), ("uses", "1"), ("days", "120")];
    let response = server
        .call(Method::POST, "/admin/personas/invites", &admin)
        .header("accept-language", "es")
        .form(&fields)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let html = response.text().await.unwrap();
    assert!(html.contains("Una invitación sirve 90 días como mucho") && html.contains("value=\"120\""));
    assert!(html.contains("422 Unprocessable Entity: days must be between 1 and 90"));
    let (_, items) = server.json(Method::GET, "/api/v1/admin/invites", &admin, None).await;
    assert!(items["items"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn the_space_of_an_account_is_changed_in_gigabytes_and_who_gets_in_is_stored() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    server.user(&admin, "leo", "user").await;
    let (_, users) = server.json(Method::GET, "/api/v1/admin/users", &admin, None).await;
    let id = users["items"].as_array().unwrap().iter().find(|u| u["handle"] == "leo").unwrap()["id"]
        .as_i64()
        .unwrap();
    let path = format!("/admin/personas/accounts/{id}");
    let response =
        server.form(&path, &admin, &[("role", "trusted"), ("status", "active"), ("quota_gb", "1.5")]).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let (_, users) = server.json(Method::GET, "/api/v1/admin/users", &admin, None).await;
    let leo = users["items"].as_array().unwrap().iter().find(|u| u["id"] == id).unwrap().clone();
    assert_eq!((leo["role"].as_str(), leo["quota_bytes"].as_i64()), (Some("trusted"), Some(1_610_612_736)));
    let bad = server.form(&path, &admin, &[("quota_gb", "lots")]).await;
    assert_eq!(bad.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let own = server.form("/admin/personas/access", &admin, &[("access", "open")]).await;
    assert_eq!(own.status(), StatusCode::SEE_OTHER);
    let (_, html) = page(&server, "/admin/personas", &admin, None).await;
    assert!(html.contains("Anyone can sign up") && html.contains("1.5"));
}
