use super::ask;
use crate::page::{self, script};
use serde_json::json;
use std::path::Path;

pub(super) fn a_page_error_marks_its_library_item(handle: i64, uid: &str, path: &Path) {
    let error = |uid: &str| ask(json!({ "op": "item", "uid": uid }))["error"].clone();
    page::spec(handle, uid, path, false, None).unwrap();
    assert!(error(uid).is_null());
    script::heard(handle, r#"{"error":"TypeError: boom"}"#);
    assert_eq!(error(uid), json!("TypeError: boom"));
    let found = ask(json!({ "op": "items", "query": "kind:web" }));
    assert!(
        found
            .as_array()
            .unwrap()
            .iter()
            .any(|i| i["uid"] == json!(uid) && i["error"] == json!("TypeError: boom"))
    );
    page::close(handle);
    page::spec(handle, uid, path, false, None).unwrap();
    assert!(error(uid).is_null(), "a page that loads again starts clean");
}
