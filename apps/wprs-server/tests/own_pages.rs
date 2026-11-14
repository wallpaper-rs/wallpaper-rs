mod common;

use common::{Who, start};
use reqwest::{Method, StatusCode};
use serde_json::json;

const ES: &str = "es";

async fn get(server: &common::Server, path: &str, who: &Who, lang: &str) -> (StatusCode, String) {
    let response = server.call(Method::GET, path, who).header("accept-language", lang).send().await.unwrap();
    (response.status(), response.text().await.unwrap())
}

#[tokio::test]
async fn a_wrong_password_comes_back_on_the_sign_in_page() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    server.user(&admin, "ana", "user").await;
    for (lang, said, fold) in [("en", "wrong handle or password", false), (ES, "no coinciden", true)] {
        let response = server
            .call(Method::POST, "/login", &Who::Nobody)
            .header("accept-language", lang)
            .form(&[("handle", "ana"), ("password", "not the password"), ("next", "/link")])
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let page = response.text().await.unwrap();
        assert!(page.contains(said) && page.contains("aria-invalid=\"true\""), "{lang}");
        assert!(page.contains("value=\"ana\"") && page.contains("value=\"/link\""), "{lang}");
        assert_eq!(page.contains("401 Unauthorized"), fold, "{lang}");
    }
    assert!(wprs_server::missed_texts().is_empty());
}

#[tokio::test]
async fn registering_and_linking_read_in_both_languages() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    let (_, es) = get(&server, "/register?invite=K4TQ", &Who::Nobody, ES).await;
    for word in
        ["Crear una cuenta", "10 caracteres o más.", "value=\"K4TQ\"", "Lee también el", "Crear cuenta"]
    {
        assert!(es.contains(word), "{word}");
    }
    let (_, en) = get(&server, "/register", &Who::Nobody, "en").await;
    assert!(en.contains("Create an account") && en.contains("Read the"));
    let (_, search) = get(&server, "/link", &ana, ES).await;
    for word in ["Vincular un dispositivo", "Aprueba solo un código", "Buscar", "Código que muestra"] {
        assert!(search.contains(word), "{word}");
    }
    let (_, missing) = get(&server, "/link?code=ZZZZ-ZZZZ", &ana, ES).await;
    assert!(
        missing.contains("Ningún dispositivo espera este código")
            && missing.contains("aria-invalid") == false
    );
    let key = "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=";
    let (_, started) = server
        .json(
            Method::POST,
            "/api/v1/auth/device/start",
            &Who::Nobody,
            Some(json!({"platform": "android", "enc_key": key})),
        )
        .await;
    let code = started["user_code"].as_str().unwrap();
    let (_, asking) = get(&server, &format!("/link?code={code}"), &ana, ES).await;
    for word in
        ["Un dispositivo pide entrar", "Plataforma", "Es mi dispositivo: aprobar", "Denegar", "Pedido"]
    {
        assert!(asking.contains(word), "{word}");
    }
    assert!(wprs_server::missed_texts().is_empty());
}

#[tokio::test]
async fn a_new_account_starts_with_the_phone_code_and_no_other_person_shows() {
    let server = start(&[("WPRS_PUBLIC_URL", "https://wall.example")]).await;
    let admin = server.admin().await;
    let mar = server.user(&admin, "mar", "user").await;
    let (_, page) = get(&server, "/account", &mar, ES).await;
    for word in [
        "Tu cuenta está lista.",
        "En tu teléfono",
        "<svg",
        "https://wall.example/repo",
        "Copiar la dirección",
        "Todavía no subiste ninguno.",
        "Ninguno todavía.",
        "Avanzado",
        "Entraste como mar",
    ] {
        assert!(page.contains(word), "{word}");
    }
    assert!(page.find("En tu teléfono") < page.find("Tus fondos"));
    assert!(!page.contains("href=\"/admin\"") && !page.contains("href=\"/mod\""));
    let (_, english) = get(&server, "/account", &mar, "en").await;
    assert!(english.contains("On your phone") && english.contains("Your account is ready."));
    assert!(wprs_server::missed_texts().is_empty());
}

#[tokio::test]
async fn removing_a_device_asks_inside_its_row_and_only_for_your_own() {
    let server = start(&[("WPRS_PUBLIC_URL", "https://wall.example")]).await;
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    let leo = server.user(&admin, "leo", "user").await;
    let (_, mine) = server.device("ana").await;
    let (_, other) = server.device("leo").await;
    let (_, plain) = get(&server, "/account", &ana, ES).await;
    assert!(plain.contains(&format!("href=\"/account?remove={mine}#dispositivos\"")));
    assert!(!plain.contains("class=\"confirmar\"") && plain.contains("Tus fondos"));
    assert!(plain.find("Dispositivos") < plain.find("En tu teléfono"));
    let (_, asking) = get(&server, &format!("/account?remove={mine}"), &ana, ES).await;
    assert!(asking.contains("class=\"confirmar\"") && asking.contains("¿Quitar «Dispositivo 1»?"));
    assert!(asking.contains("class=\"fila foco\"") && asking.contains("Cancelar"));
    assert!(asking.contains(&format!("/api/v1/me/devices/{mine}?_method=DELETE")));
    let (_, foreign) = get(&server, &format!("/account?remove={other}"), &ana, ES).await;
    assert!(!foreign.contains("class=\"confirmar\"") && !foreign.contains(&other));
    let gone =
        server.form(&format!("/api/v1/me/devices/{other}?_method=DELETE&back=/account"), &ana, &[]).await;
    assert_ne!(gone.status(), StatusCode::OK);
    let done =
        server.form(&format!("/api/v1/me/devices/{mine}?_method=DELETE&back=/account"), &ana, &[]).await;
    assert_eq!(done.status(), StatusCode::OK);
    let (_, after) = get(&server, "/account", &ana, ES).await;
    assert!(!after.contains(&mine));
    let (_, theirs) = get(&server, "/account", &leo, ES).await;
    assert!(theirs.contains(&format!("remove={other}")));
    assert!(wprs_server::missed_texts().is_empty());
}

#[tokio::test]
async fn a_new_token_is_shown_once_with_its_name_in_the_result_screen() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    let done = server
        .call(Method::POST, "/api/v1/me/tokens?back=/account", &ana)
        .header("accept-language", ES)
        .form(&[("name", "copia-noche"), ("scopes", "catalog:read sync"), ("days", "30")])
        .send()
        .await
        .unwrap();
    assert_eq!(done.status(), StatusCode::OK);
    let page = done.text().await.unwrap();
    for word in [
        "Token creado.",
        "no se vuelve a mostrar",
        "copia-noche",
        "catalog:read sync",
        "en 30 días",
        "Continuar",
    ] {
        assert!(page.contains(word), "{word}");
    }
    assert!(wprs_server::missed_texts().is_empty());
}
