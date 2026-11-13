use serde_json::Value;
use wprs_sync::Replica;

const PREFIX: &str = "secrets/";

pub(crate) fn strip(replica: &mut Replica) {
    replica.bases.retain(|key, _| !key.starts_with(PREFIX));
    replica.held.retain(|key, _| !key.starts_with(PREFIX));
}

pub(crate) fn keep<T>(items: &mut Vec<(String, T)>) {
    items.retain(|(key, _)| !key.starts_with(PREFIX));
}

pub(crate) fn forget(stored: &mut Value) {
    if let Some(map) = stored.as_object_mut() {
        map.remove("records");
        map.remove("held");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wprs_sync::{Doc, Hlc, View};

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
        strip(&mut replica);
        assert_eq!(replica.bases.keys().collect::<Vec<_>>(), ["library/two"]);
        assert_eq!(replica.held.keys().collect::<Vec<_>>(), ["library/two"]);

        let mut arrived = vec![("secrets/one".to_owned(), 1), ("library/two".to_owned(), 2)];
        keep(&mut arrived);
        assert_eq!(arrived, [("library/two".to_owned(), 2)]);

        let mut stored = json!({"token": "t", "records": {"secrets/one": [1, "x"]}, "held": {}});
        forget(&mut stored);
        assert_eq!(stored, json!({"token": "t"}));
    }
}
