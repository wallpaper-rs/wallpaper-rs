mod common;

use common::{Server, Who, package, start, upload};
use reqwest::{Method, StatusCode};

async fn fetch(server: &Server, path: &str, who: &Who, language: &str) -> (StatusCode, String) {
    let reply = server.call(Method::GET, path, who).header("accept-language", language).send().await;
    let reply = reply.unwrap();
    (reply.status(), reply.text().await.unwrap())
}

async fn decide(
    server: &Server,
    id: i64,
    who: &Who,
    fields: &[(&str, &str)],
    language: &str,
) -> (StatusCode, String, String) {
    let call =
        server.call(Method::POST, &format!("/mod/cases/{id}"), who).header("accept-language", language);
    let reply = call.form(fields).send().await.unwrap();
    let place = reply.headers().get("location").map(|v| v.to_str().unwrap().to_string()).unwrap_or_default();
    (reply.status(), place, reply.text().await.unwrap())
}

async fn waiting() -> (Server, Who, Who, i64) {
    let server = start(&[("WPRS_AUTO_PUBLISH", "never")]).await;
    wprs_server::init(&server.cfg, "Example wallpapers", false, false).unwrap();
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    upload(&server, &ana, &package(server.dir.path(), "org.example.dawn", "1.0.0")).await;
    let (_, queue) = server.json(Method::GET, "/api/v1/mod/queue", &admin, None).await;
    let case = queue["items"][0]["id"].as_i64().unwrap();
    (server, admin, ana, case)
}

#[tokio::test(flavor = "multi_thread")]
async fn the_queue_lists_what_waits_in_both_languages_and_an_empty_one_says_so() {
    let (server, admin, _, case) = waiting().await;
    let (_, spanish) = fetch(&server, "/mod", &admin, "es").await;
    for word in ["Revisión", "1 caso por revisar", "Subidas que esperan", "Abrir", "Retirados", "Ninguno."] {
        assert!(spanish.contains(word), "{word}");
    }
    assert!(spanish.contains(&format!("/mod/cases/{case}")) && !spanish.contains("Moderación"));
    let (_, english) = fetch(&server, "/mod", &admin, "en").await;
    for word in ["Review", "1 case to review", "Uploads waiting", "Withdrawn", "Block a file"] {
        assert!(english.contains(word), "{word}");
    }
    let decision = [("decision", "dismiss")];
    assert_eq!(decide(&server, case, &admin, &decision, "es").await.0, StatusCode::SEE_OTHER);
    let (_, empty) = fetch(&server, "/mod", &admin, "es").await;
    assert!(empty.contains("Nada por revisar") && empty.contains("mientras el acceso es"));
    let (_, empty) = fetch(&server, "/mod", &admin, "en").await;
    assert!(empty.contains("Nothing to review"));
    assert!(wprs_server::missed_texts().is_empty(), "{:#?}", wprs_server::missed_texts());
}

#[tokio::test(flavor = "multi_thread")]
async fn the_case_card_shows_the_checks_the_history_and_the_decision_in_both_languages() {
    let (server, admin, _, case) = waiting().await;
    let path = format!("/mod/cases/{case}");
    let (status, card) = fetch(&server, &path, &admin, "es").await;
    assert_eq!(status, StatusCode::OK);
    for word in [
        "Revisión · caso 1 de 1",
        "Quién lo sube",
        "primera subida",
        "sin avisos",
        "Lo que ya se comprobó",
        "Licencia",
        "Firma",
        "Sin firma",
        "Permisos",
        "Destellos",
        "Clasificación",
        "Publicar",
        "Si no lo publicas",
        "Motivo que recibe ana",
        "Rechazar",
        "Más",
        "Descartar el caso",
        "Darle un aviso a ana",
        "Detalles",
        "class=\"caso-portada\"",
    ] {
        assert!(card.contains(word), "{word}");
    }
    let (_, english) = fetch(&server, &path, &admin, "en").await;
    for word in [
        "first upload",
        "What was already checked",
        "Publish",
        "If you do not publish it",
        "Dismiss the case",
    ] {
        assert!(english.contains(word), "{word}");
    }
    let mia = server.user(&admin, "mia", "moderator").await;
    upload(&server, &mia, &package(server.dir.path(), "org.example.dusk", "1.0.0")).await;
    let (_, queue) = server.json(Method::GET, "/api/v1/mod/queue", &mia, None).await;
    let own = queue["items"].as_array().unwrap().iter().find(|c| c["uploader"] == "mia").unwrap();
    let (_, own) = fetch(&server, &format!("/mod/cases/{}", own["id"]), &mia, "en").await;
    assert!(own.contains("disabled") && own.contains("You cannot publish your own upload."));
    assert!(wprs_server::missed_texts().is_empty(), "{:#?}", wprs_server::missed_texts());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_moderator_publishes_another_account_upload_and_goes_to_the_next_case() {
    let (server, admin, ana, case) = waiting().await;
    let mia = server.user(&admin, "mia", "moderator").await;
    upload(&server, &ana, &package(server.dir.path(), "org.example.dusk", "1.0.0")).await;
    let (_, card) = fetch(&server, &format!("/mod/cases/{case}"), &mia, "es").await;
    assert!(card.contains("name=\"decision\" value=\"approve\"") && !card.contains("disabled"));
    let (status, place, _) = decide(&server, case, &mia, &[("decision", "approve")], "es").await;
    assert_eq!((status, place.as_str()), (StatusCode::SEE_OTHER, format!("/mod?decided={case}").as_str()));
    let (_, done) = fetch(&server, &place, &mia, "es").await;
    for word in [
        "Publicaste «Dawn 1.0.0»",
        "Ya está en el catálogo",
        "Siguiente caso",
        "Volver a la cola",
        "1 caso por revisar",
    ] {
        assert!(done.contains(word), "{word}");
    }
    assert!(!done.contains("Subidas que esperan"));
    let (_, shown) = fetch(&server, &format!("/mod/cases/{case}"), &mia, "es").await;
    assert!(shown.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn rejecting_needs_the_reason_in_view_and_a_second_decision_is_refused_in_the_page() {
    let (server, admin, _, case) = waiting().await;
    let mia = server.user(&admin, "mia", "moderator").await;
    let (status, _, page) =
        decide(&server, case, &mia, &[("decision", "reject"), ("statement", "short")], "es").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(
        page.contains("Escribe el motivo: ana lo recibe junto con el rechazo.") && page.contains("short")
    );
    let (_, _, page) =
        decide(&server, case, &mia, &[("decision", "reject"), ("statement", "short")], "en").await;
    assert!(page.contains("Write the reason: ana receives it with the rejection."));
    let reason = "The preview shows another work.";
    let fields = [("decision", "reject"), ("statement", reason), ("strike", "true")];
    assert_eq!(decide(&server, case, &mia, &fields, "es").await.0, StatusCode::SEE_OTHER);
    let (status, _, again) = decide(&server, case, &mia, &fields, "es").await;
    assert_eq!(status, StatusCode::CONFLICT);
    for word in [
        "No se aplicó la decisión sobre «Dawn 1.0.0»",
        "Una decisión no se aplicó",
        "Abrir el caso otra vez",
        "Detalles",
    ] {
        assert!(again.contains(word), "{word}");
    }
    let (_, done) = fetch(&server, &format!("/mod?decided={case}"), &mia, "es").await;
    assert!(done.contains("Rechazaste «Dawn 1.0.0»") && done.contains("ana recibe tu motivo"));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_person_who_is_not_a_moderator_never_reaches_a_case() {
    let (server, _, ana, case) = waiting().await;
    let path = format!("/mod/cases/{case}");
    for who in [&ana, &Who::Nobody] {
        let reply = server.call(Method::GET, &path, who).send().await.unwrap();
        assert_eq!(reply.status(), StatusCode::SEE_OTHER);
        assert!(reply.headers()["location"].to_str().unwrap().starts_with("/login"));
        let (status, place, _) = decide(&server, case, who, &[("decision", "approve")], "en").await;
        assert_eq!((status, place.starts_with("/login")), (StatusCode::SEE_OTHER, true));
    }
    let (_, queue) = server.json(Method::GET, "/api/v1/mod/queue", &Who::Nobody, None).await;
    assert!(queue["items"].is_null());
}
