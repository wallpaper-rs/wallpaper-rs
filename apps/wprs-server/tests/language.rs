mod common;

use common::{Who, start};
use reqwest::Method;

async fn page(
    server: &common::Server,
    path: &str,
    who: &Who,
    language: Option<&str>,
) -> (u16, String, String) {
    let mut call = server.call(Method::GET, path, who);
    if let Some(language) = language {
        call = call.header("accept-language", language);
    }
    let response = call.send().await.unwrap();
    let vary = response.headers().get("vary").map(|v| v.to_str().unwrap().to_string()).unwrap_or_default();
    (response.status().as_u16(), vary, response.text().await.unwrap())
}

#[tokio::test]
async fn the_sign_in_page_follows_the_browser_language() {
    let server = start(&[]).await;
    let (_, vary, english) = page(&server, "/login", &Who::Nobody, None).await;
    assert!(
        english.contains("<html lang=\"en\"") && english.contains("Sign in") && english.contains("Password")
    );
    assert!(vary.contains("accept-language"));
    let (_, _, spanish) = page(&server, "/login", &Who::Nobody, Some("es-AR,es;q=0.9,en;q=0.5")).await;
    for word in ["<html lang=\"es\"", "Entrar", "Contraseña", "Usuario", "Registrarse", "Saltar al contenido"]
    {
        assert!(spanish.contains(word), "{word}");
    }
    assert!(!spanish.contains("Skip to content") && !spanish.contains("No account yet"));
    let (_, _, other) = page(&server, "/login", &Who::Nobody, Some("fr-FR,fr;q=0.9,es;q=0.3")).await;
    assert!(other.contains("<html lang=\"en\""));
}

#[tokio::test]
async fn the_account_and_administration_pages_speak_spanish() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let (_, _, account) = page(&server, "/account", &admin, Some("es")).await;
    for word in
        ["Tu cuenta", "Subir un fondo", "Dispositivos", "Claves de autor", "Borrar mi cuenta", "Salir"]
    {
        assert!(account.contains(word), "{word}");
    }
    assert!(account.contains("aria-current=\"page\"") && !account.contains("Upload a wallpaper"));
    let (_, _, english) = page(&server, "/account", &admin, None).await;
    assert!(english.contains("Upload a wallpaper package") && english.contains("aria-current=\"page\""));
    let (_, _, admin_es) = page(&server, "/admin", &admin, Some("es")).await;
    for word in ["Administración", "Invitaciones", "Cuentas", "Crear invitación", "Fondos destacados"] {
        assert!(admin_es.contains(word), "{word}");
    }
    assert!(admin_es.contains("aria-current=\"page\""));
    let (_, _, moderation) = page(&server, "/mod", &admin, Some("es")).await;
    assert!(moderation.contains("Bloquear un archivo") && moderation.contains("aria-current=\"page\""));
}

#[tokio::test]
async fn an_error_page_says_what_happened_in_the_visitor_language() {
    let server = start(&[]).await;
    let (status, _, missing) = page(&server, "/nothing-here", &Who::Nobody, Some("es")).await;
    assert_eq!(status, 404);
    assert!(
        missing.contains("Eso no existe")
            && missing.contains("No existe esa página.")
            && missing.contains("Continuar")
    );
    let (_, _, english) = page(&server, "/nothing-here", &Who::Nobody, None).await;
    assert!(english.contains("That does not exist") && english.contains("page not found"));
    let wrong = |language: &'static str| {
        let call = server.http.post(format!("{}/login", server.url)).header("accept-language", language);
        call.form(&[("handle", "nobody"), ("password", "wrong-password")]).send()
    };
    let reply = wrong("es").await.unwrap().text().await.unwrap();
    assert!(reply.contains("El usuario o la contraseña no coinciden") && reply.contains("<html lang=\"es\""));
    let reply = wrong("en").await.unwrap().text().await.unwrap();
    assert!(reply.contains("wrong handle or password"));
}

#[tokio::test]
async fn no_page_shows_english_when_spanish_was_asked() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let paths = ["/", "/login", "/register", "/setup", "/legal", "/legal/terms", "/legal/privacy"];
    let more = ["/legal/content", "/legal/notice", "/legal/counter-notice", "/account", "/admin", "/mod"];
    for path in paths.iter().chain(more.iter()).chain(["/admin/featured", "/link"].iter()) {
        let (_, _, html) = page(&server, path, &admin, Some("es")).await;
        assert!(html.contains("<html lang=\"es\"") || html.is_empty(), "{path}");
    }
    assert!(wprs_server::missed_texts().is_empty(), "{:#?}", wprs_server::missed_texts());
}
