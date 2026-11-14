use crate::account::{LISTEN, NETWORK};
use crate::state::{api, linked, read, update};
use crate::{Error, keys, link, pairing, said, settings, vault};
use serde_json::{Value, json};
use wprs_sync::{Api, Peer, Secrets, SyncKey};

pub(crate) fn status() -> Value {
    read(|s| {
        let device = |p: &Peer| {
            let current = s.device.as_ref() == Some(&p.id);
            json!({"id": p.id, "name": p.name, "label": s.label_of(&p.id), "platform": p.platform,
                "seen_at": p.seen_at, "current": current})
        };
        json!({
            "url": s.url, "name": s.name, "handle": s.handle, "device": s.device_name, "label": s.label,
            "state": s.state(), "encrypted": s.encrypted, "paused": s.paused, "last_sync": s.last_sync,
            "last_error": s.last_error, "last_reply": s.last_reply, "warning": s.warning.as_ref().filter(|_| !s.sync_off),
            "records": s.replica.bases.len(), "quarantined": s.quarantine.len(), "unapplied": s.unapplied.len(),
            "devices": s.peers.iter().map(device).collect::<Vec<_>>(), "link": link::shown(),
            "pairing": pairing::shown(), "network": settings::chosen(NETWORK),
            "listen": settings::chosen(LISTEN), "active": s.active(),
            "outdated": s.outdated, "signed_out": s.signed_out, "recovery_stale": s.recovery_stale, "pin": s.pin,
            "recovery": match (s.recovery, s.recovery_stale) { (None, _) => "off", (_, true) => "stale", _ => "current" },
            "offers": s.offers.len(), "files": s.files.items.len(), "repository_failed": s.repo_failed,
        })
    })
}

pub(crate) fn syncing() -> Result<(), Error> {
    linked()?;
    match read(|s| s.sync_off) {
        true => Err(said("android.syncing_off", "syncing is off on this phone; turn it on first", &[])),
        false => Ok(()),
    }
}

pub(crate) async fn sync_state(api: &Api, plaintext: bool) -> Result<Option<Value>, Error> {
    let existing = match api.get("/api/v1/sync/state").await {
        Err(e) if e.code() == "sync_not_set_up" => None,
        state => Some(state?),
    };
    if existing.as_ref().is_some_and(|state| state["encrypted"] != true) && !plaintext {
        return Err(said(
            "android.account_syncs_without",
            "the account syncs without end-to-end encryption; link with plain text only if you accept that",
            &[],
        ));
    }
    Ok(existing)
}

pub(crate) async fn setup(
    api: &Api,
    secrets: &mut Secrets,
    existing: Option<Value>,
    plaintext: bool,
) -> Result<Value, Error> {
    let state = match existing {
        Some(state) => state,
        None => {
            let sync = SyncKey::generate();
            secrets.set_key(&sync, 0);
            vault::store(secrets)?;
            let (encrypted, plaintext) = (!plaintext, plaintext);
            let body = json!({"encrypted": encrypted, "confirm_plaintext": plaintext, "key_id": sync.id()});
            let state = api.post("/api/v1/sync/setup", body).await?;
            if state["encrypted"].as_bool() != Some(encrypted) {
                return Err(said(
                    "android.server_set_up",
                    "the server set up sync with another encryption than asked",
                    &[],
                ));
            }
            secrets.set_key(&sync, state["epoch"].as_i64().unwrap_or(0));
            vault::store(secrets)?;
            update(|s| (s.ready, s.stale) = (false, false))?;
            state
        }
    };
    let fresh = read(|s| !s.ready && !s.stale);
    update(|s| {
        if fresh {
            (s.epoch, s.ready) = (state["epoch"].as_i64().unwrap_or(0), !secrets.key.is_empty());
        }
        (s.encrypted, s.warning) =
            (state["encrypted"].as_bool(), state["warning"].as_str().map(String::from));
        (s.plaintext, s.sync_off, s.last_error, s.last_reply) = (plaintext, false, None, None);
    })?;
    drop(keys::refresh(api).await);
    Ok(state)
}

pub(crate) async fn set(on: bool, plaintext: bool) -> Result<Value, Error> {
    linked()?;
    if !on {
        pairing::cancel();
        update(|s| (s.sync_off, s.last_error, s.last_reply) = (true, None, None))?;
        return Ok(json!({"syncing": false, "active": false}));
    }
    let (api, mut secrets) = api()?;
    let existing = sync_state(&api, plaintext).await?;
    let state = setup(&api, &mut secrets, existing, plaintext).await?;
    let needs_key = secrets.key.is_empty();
    Ok(
        json!({"syncing": true, "encrypted": state["encrypted"], "needs_key": needs_key, "active": crate::state::active()}),
    )
}
