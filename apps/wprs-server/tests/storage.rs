mod common;

use base64::Engine;
use base64::prelude::BASE64_STANDARD as B64;
use common::start;
use reqwest::Method;
use serde_json::json;

#[tokio::test]
async fn the_account_page_tells_packages_apart_from_sync_and_private_files() {
    let server = start(&[]).await;
    let admin = server.admin().await;
    let ana = server.user(&admin, "ana", "user").await;
    let shown = async |who: &common::Who| {
        server.call(Method::GET, "/account", who).send().await.unwrap().text().await.unwrap()
    };
    let before = shown(&ana).await;
    assert!(before.contains("Package storage used: 0 of"), "{before}");
    assert!(!before.contains("Sync data:"));
    let (desk, _) = server.device("ana").await;
    server.json(Method::POST, "/api/v1/sync/setup", &desk, Some(json!({}))).await;
    let mut record = vec![1u8];
    record.extend([7u8; 24 + 512 + 16]);
    let change =
        json!([{"collection": "library", "item": "01".repeat(32), "base": 0, "payload": B64.encode(record)}]);
    server
        .json(Method::POST, "/api/v1/sync/changes", &desk, Some(json!({"epoch": 0, "changes": change})))
        .await;
    let after = shown(&ana).await;
    assert!(after.contains("Package storage used: 0 of"), "{after}");
    assert!(after.contains("Sync data: 553 of"), "{after}");
}
