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
) -> (StatusCode, String) {
    let call =
        server.call(Method::POST, &format!("/mod/cases/{id}"), who).header("accept-language", language);
    let reply = call.form(fields).send().await.unwrap();
    (reply.status(), reply.text().await.unwrap())
}

#[tokio::test(flavor = "multi_thread")]
async fn a_decision_on_a_version_that_has_a_newer_one_changes_nothing_and_points_to_the_new_one() {
    let server = start(&[("WPRS_AUTO_PUBLISH", "never")]).await;
    wprs_server::init(&server.cfg, "Example wallpapers", false, false).unwrap();
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    upload(&server, &ana, &package(server.dir.path(), "org.example.dawn", "1.0.0")).await;
    let (_, first) = fetch(&server, "/mod", &admin, "es").await;
    assert!(first.contains("primera subida"), "{first}");
    assert!(fetch(&server, "/mod", &admin, "en").await.1.contains("first upload"));
    let (_, queue) = server.json(Method::GET, "/api/v1/mod/queue", &admin, None).await;
    let old = queue["items"][0]["id"].as_i64().unwrap();
    upload(&server, &ana, &package(server.dir.path(), "org.example.dawn", "1.1.0")).await;
    let (_, queue) = server.json(Method::GET, "/api/v1/mod/queue", &admin, None).await;
    let items = queue["items"].as_array().unwrap();
    let new = items.iter().find(|c| c["version"] == "1.1.0").unwrap()["id"].as_i64().unwrap();
    let (_, spanish) = fetch(&server, "/mod", &admin, "es").await;
    assert!(spanish.contains("Hay una versión nueva") && !spanish.contains("primera subida"));
    assert!(fetch(&server, "/mod", &admin, "en").await.1.contains("There is a newer version"));
    let (_, card) = fetch(&server, &format!("/mod/cases/{old}"), &admin, "es").await;
    assert!(card.contains("Hay una versión nueva: 1.1.0") && card.contains("Abrir la versión nueva"));
    for (language, words) in [
        ("es", ["Abrir la versión nueva", "ana subió otra versión mientras la mirabas"]),
        ("en", ["Open the new version", "ana uploaded another version while you were looking at this one"]),
    ] {
        for fields in [
            vec![("decision", "approve")],
            vec![("decision", "reject"), ("statement", "It does not run well.")],
        ] {
            let (status, page) = decide(&server, old, &admin, &fields, language).await;
            assert_eq!(status, StatusCode::CONFLICT);
            assert!(words.iter().all(|w| page.contains(w)), "{language}");
            assert!(page.contains(&format!("/mod/cases/{new}")));
        }
    }
    let (_, queue) = server.json(Method::GET, "/api/v1/mod/queue", &admin, None).await;
    let states: Vec<&str> =
        queue["items"].as_array().unwrap().iter().map(|c| c["state"].as_str().unwrap()).collect();
    assert_eq!(states, ["pending", "pending"]);
    let (status, _) = decide(&server, new, &admin, &[("decision", "approve")], "es").await;
    assert_eq!(status, StatusCode::SEE_OTHER);
    assert!(wprs_server::missed_texts().is_empty(), "{:#?}", wprs_server::missed_texts());
}
