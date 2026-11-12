mod common;

use common::start;
use reqwest::{Method, StatusCode};
use serde_json::{Value, json};

const PASSWORD: &str = "a long enough password";

#[tokio::test]
async fn a_device_removes_another_only_with_the_account_password() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    server.user(&admin, "ana", "user").await;
    let ((desk, _), (phone, phone_id)) = (server.device("ana").await, server.device("ana").await);
    let path = format!("/api/v1/me/devices/{phone_id}/revoke");
    let ask = |body: Option<Value>| server.json(Method::POST, &path, &desk, body);
    assert_eq!(ask(None).await.0, StatusCode::BAD_REQUEST);
    assert_eq!(ask(Some(json!({"password": "not the password at all"}))).await.0, StatusCode::UNAUTHORIZED);
    let (_, list) = server.json(Method::GET, "/api/v1/me/devices", &desk, None).await;
    assert_eq!(list["items"].as_array().unwrap().len(), 2);
    assert_eq!(ask(Some(json!({"password": PASSWORD}))).await.0, StatusCode::NO_CONTENT);
    let (_, list) = server.json(Method::GET, "/api/v1/me/devices", &desk, None).await;
    assert_eq!(list["items"].as_array().unwrap().len(), 1);
    assert_eq!(
        server.json(Method::GET, "/api/v1/me/devices", &phone, None).await.0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(ask(Some(json!({"password": PASSWORD}))).await.0, StatusCode::NOT_FOUND);
}
