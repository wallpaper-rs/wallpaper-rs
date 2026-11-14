mod common;

use common::{Server, Who, cookie, start};
use reqwest::{Method, StatusCode};
use serde_json::json;

async fn show(server: &Server, who: &Who, spanish: bool) -> (StatusCode, String) {
    let mut call = server.call(Method::GET, "/setup", who);
    if spanish {
        call = call.header("accept-language", "es");
    }
    let response = call.send().await.unwrap();
    (response.status(), response.text().await.unwrap())
}

async fn both(server: &Server, who: &Who, english: &[&str], spanish: &[&str]) -> String {
    let (status, page) = show(server, who, false).await;
    assert_eq!(status, StatusCode::OK);
    for piece in english {
        assert!(page.contains(piece), "{piece}");
    }
    let (_, page) = show(server, who, true).await;
    for piece in spanish {
        assert!(page.contains(piece), "{piece}");
    }
    page
}

fn joined(visit: &Who, session: &str) -> Who {
    let Who::Raw(visit) = visit else { unreachable!() };
    Who::Raw(format!("{visit}; wprs={session}"))
}

async fn post(server: &Server, path: &str, who: &Who, fields: &[(&str, &str)]) -> StatusCode {
    server.form(path, who, fields).await.status()
}

#[tokio::test]
async fn the_assistant_walks_every_step_in_both_languages() {
    let server = start(&[]).await;
    both(&server, &Who::Nobody, &["Setup token", "Where the code is"], &["Código de arranque", "Empezar"])
        .await;
    let wrong = server.form("/setup", &Who::Nobody, &[("token", "wprs_s_bad")]).await;
    assert_eq!(wrong.status(), StatusCode::FORBIDDEN);
    assert!(wrong.text().await.unwrap().contains("This code does not work"));
    let visit = server.visit().await;
    let network = ["How do the others reach this server?", "Only at home", "step 1 of 4"];
    both(&server, &visit, &network, &["¿Cómo llegan los demás a este servidor?", "Solo en casa"]).await;
    assert_eq!(
        post(&server, "/setup/account", &visit, &[("handle", "ana"), ("password", "x")]).await,
        StatusCode::CONFLICT
    );
    assert_eq!(post(&server, "/setup/network", &visit, &[("network", "own")]).await, StatusCode::SEE_OTHER);
    both(
        &server,
        &visit,
        &["Administrator account", "step 2 of 4"],
        &["Cuenta de administrador", "Contraseña"],
    )
    .await;
    let short = server.form("/setup/account", &visit, &[("handle", "ana"), ("password", "short")]).await;
    assert_eq!(short.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let made = server
        .form("/setup/account", &visit, &[("handle", "ana"), ("password", "a long enough password")])
        .await;
    assert_eq!(made.status(), StatusCode::SEE_OTHER);
    let visit = joined(&visit, &cookie(&made));
    both(&server, &visit, &["Create the repository", "ana"], &["Crear el repositorio", "Autores en línea"])
        .await;
    let fields = [("name", "Home wallpapers"), ("online_authors", "on")];
    assert_eq!(post(&server, "/setup/repository", &visit, &fields).await, StatusCode::SEE_OTHER);
    let key = both(
        &server,
        &visit,
        &["Save the root key", "I saved it"],
        &["Guarda la clave raíz", "Ya la guardé"],
    )
    .await;
    let secret = key.split("<p class=\"dato\">").nth(1).unwrap().split('<').next().unwrap().to_owned();
    assert!(secret.len() > 20, "{secret}");
    assert_eq!(post(&server, "/setup/saved", &visit, &[]).await, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(post(&server, "/setup/saved", &visit, &[("saved", "on")]).await, StatusCode::SEE_OTHER);
    let (_, access) = show(&server, &visit, false).await;
    assert!(!access.contains(&secret), "the root key is shown only once");
    let invite =
        both(&server, &visit, &["Who gets in", "/register?invite=", "<svg"], &["Por invitación", "Terminar"])
            .await;
    let code = invite.split("/register?invite=").nth(1).unwrap().split('<').next().unwrap().to_owned();
    assert_eq!(post(&server, "/setup/access", &visit, &[("access", "invite")]).await, StatusCode::SEE_OTHER);
    let listo =
        ["Go to Administration", "Repository address, with its fingerprint", "/repo/?fingerprint=", "as ana"];
    both(&server, &visit, &listo, &["Listo: tu servidor ya reparte fondos.", "Ir a Administración"]).await;
    let body = json!({"handle": "bea", "password": "a long enough password", "invite": code});
    assert_eq!(
        server.json(Method::POST, "/api/v1/auth/register", &Who::Nobody, Some(body)).await.0,
        StatusCode::CREATED
    );
    assert!(wprs_server::missed_texts().is_empty(), "{:#?}", wprs_server::missed_texts());
}

#[tokio::test]
async fn a_finished_setup_can_never_make_another_administrator_or_repository() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let visit = server.visit().await;
    let fields = [("name", "Home"), ("online_authors", "on")];
    assert_eq!(post(&server, "/setup/repository", &visit, &fields).await, StatusCode::SEE_OTHER);
    assert_eq!(post(&server, "/setup/saved", &visit, &[("saved", "on")]).await, StatusCode::SEE_OTHER);
    assert_eq!(post(&server, "/setup/access", &visit, &[("access", "closed")]).await, StatusCode::SEE_OTHER);
    let fresh = server.app.setup_token().await.unwrap();
    assert_eq!(post(&server, "/setup", &Who::Nobody, &[("token", &fresh)]).await, StatusCode::FORBIDDEN);
    let again = [("handle", "eve"), ("password", "a long enough password")];
    assert_eq!(post(&server, "/setup/account", &visit, &again).await, StatusCode::FORBIDDEN);
    assert_eq!(post(&server, "/setup/repository", &visit, &fields).await, StatusCode::FORBIDDEN);
    assert_eq!(post(&server, "/setup/again", &visit, &[]).await, StatusCode::FORBIDDEN);
    assert_eq!(post(&server, "/setup/network", &visit, &[("network", "home")]).await, StatusCode::FORBIDDEN);
    let (status, _) = show(&server, &Who::Nobody, false).await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    let (status, page) = show(&server, &admin, false).await;
    assert!(status == StatusCode::OK && page.contains("Go to Administration"));
}

#[tokio::test]
async fn the_stored_choice_wins_over_the_starting_environment() {
    let server = start(&[("WPRS_REGISTRATION", "open")]).await;
    assert_eq!(server.app.settings().access(), wprs_server::settings::Access::Open);
    let admin = server.admin().await;
    let visit = server.visit().await;
    let fields = [("name", "Home"), ("online_authors", "on")];
    post(&server, "/setup/repository", &visit, &fields).await;
    post(&server, "/setup/saved", &visit, &[("saved", "on")]).await;
    assert_eq!(post(&server, "/setup/access", &visit, &[("access", "closed")]).await, StatusCode::SEE_OTHER);
    let body = json!({"handle": "eve", "password": "a long enough password"});
    let (status, _) = server.json(Method::POST, "/api/v1/auth/register", &Who::Nobody, Some(body)).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let (_, found) = server.json(Method::GET, "/.well-known/wprs-server", &Who::Nobody, None).await;
    assert_eq!(found["registration"], "closed", "{found}");
    let reopened = wprs_server::App::open(server.cfg.clone()).unwrap();
    assert_eq!(reopened.settings().access(), wprs_server::settings::Access::Closed);
    assert!(reopened.settings().done);
    let path = "/api/v1/admin/settings";
    let (status, stored) =
        server.json(Method::PATCH, path, &admin, Some(json!({"auto_publish": "never"}))).await;
    assert_eq!((status, stored["auto_publish"].as_str()), (StatusCode::OK, Some("never")), "{stored}");
    assert_eq!(stored["registration"], "closed");
    let bad = json!({"registration": "everyone"});
    assert_eq!(server.json(Method::PATCH, path, &admin, Some(bad)).await.0, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(server.json(Method::GET, path, &Who::Nobody, None).await.0, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn a_root_key_lost_to_a_restart_offers_to_make_the_repository_again() {
    let server = start(&[]).await;
    server.admin().await;
    let visit = server.visit().await;
    let fields = [("name", "Home"), ("online_authors", "on")];
    assert_eq!(post(&server, "/setup/repository", &visit, &fields).await, StatusCode::SEE_OTHER);
    let other = server.visit().await;
    let (_, page) = show(&server, &other, false).await;
    assert!(page.contains("The root key is no longer shown.") && !page.contains("class=\"clave\""), "{page}");
    assert_eq!(post(&server, "/setup/saved", &other, &[]).await, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(post(&server, "/setup/again", &other, &[]).await, StatusCode::SEE_OTHER);
    let (_, page) = show(&server, &other, false).await;
    assert!(page.contains("Create the repository"), "{page}");
    assert_eq!(post(&server, "/setup/repository", &other, &fields).await, StatusCode::SEE_OTHER);
    let (_, page) = show(&server, &other, false).await;
    assert!(page.contains("Save the root key") && page.contains("class=\"clave\""));
}
