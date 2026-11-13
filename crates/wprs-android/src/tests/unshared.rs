use crate::unshared;
use serde_json::json;
use wprs_sync::{Doc, Hlc, Replica, View};

fn doc() -> Doc {
    let view: View = serde_json::from_value(json!({"secret": "swordfish"})).unwrap();
    Doc::stamp(None, Some(&view), |_| false, &Hlc(1, 0, "a".into())).unwrap()
}

#[test]
fn provider_keys_never_stay_on_the_phone() {
    let mut replica = Replica::default();
    for key in ["secrets/one", "library/two"] {
        replica.bases.insert(key.into(), (1, doc()));
        replica.held.insert(key.into(), (1, doc()));
    }
    unshared::strip(&mut replica);
    assert_eq!(replica.bases.keys().collect::<Vec<_>>(), ["library/two"]);
    assert_eq!(replica.held.keys().collect::<Vec<_>>(), ["library/two"]);

    let mut arrived = vec![("secrets/one".to_owned(), 1), ("library/two".to_owned(), 2)];
    unshared::keep(&mut arrived);
    assert_eq!(arrived, [("library/two".to_owned(), 2)]);

    let mut stored = json!({"token": "t", "records": {"secrets/one": [1, "x"]}, "held": {}});
    unshared::forget(&mut stored);
    assert_eq!(stored, json!({"token": "t"}));
}
