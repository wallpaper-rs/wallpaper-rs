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
