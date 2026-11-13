use super::link::NEEDS_PLAIN;
use super::secrets::Secrets;
use super::server::exists;
use super::{block_on, sync_error};
use crate::daemon::Daemon;
use crate::error::{Result, failed};
use serde_json::{Value, json};
use std::sync::Arc;
use wprs_core::model::JobKind;
use wprs_sync::{Api, SyncKey};

pub const OFF: &str = "syncing is off on this computer; turn it on first";

pub fn sync_state(api: &Api, plaintext: bool) -> Result<Option<Value>> {
    let existing = match block_on(api.get("/api/v1/sync/state")) {
        Err(e) if e.code() == "sync_not_set_up" => None,
        state => Some(state.map_err(sync_error)?),
    };
    if existing.as_ref().is_some_and(|state| state["encrypted"] != true) && !plaintext {
        return Err(failed(NEEDS_PLAIN));
    }
    Ok(existing)
}

impl Daemon {
    pub fn syncing(&self) -> Result<()> {
        self.linked()?;
        if self.saved().sync_off { Err(failed(OFF)) } else { Ok(()) }
    }

    pub fn set_syncing(self: &Arc<Self>, on: bool, plaintext: bool) -> Result<u64> {
        self.linked()?;
        if !self.jobs_of(JobKind::Link).is_empty() {
            return Err(exists("this computer is linking, joining or sharing the key; wait for it to end"));
        }
        if !on {
            self.update(|s| (s.sync_off, s.last_error, s.pairing) = (true, None, None))?;
            for job in
                self.jobs_of(JobKind::AccountSync).into_iter().chain(self.jobs.find(JobKind::Upload, "files"))
            {
                self.cancel_job(job)?;
            }
            self.account.restart.notify_one();
            tracing::info!("syncing turned off");
        }
        Ok(self.start_job(JobKind::Link, "syncing", move |d, _| {
            if !on {
                return Ok(json!({"syncing": false}));
            }
            let (api, mut secrets) = d.account_api(true)?;
            let existing = sync_state(&api, plaintext)?;
            let state = d.setup_sync(&api, &mut secrets, existing, plaintext)?;
            let _ = d.refresh_peers(&api);
            d.account.wake.notify_one();
            d.account.restart.notify_one();
            tracing::info!("syncing turned on");
            Ok(json!({"syncing": true, "encrypted": state["encrypted"], "needs_key": secrets.key.is_empty()}))
        }))
    }

    pub(super) fn setup_sync(
        &self,
        api: &Api,
        secrets: &mut Secrets,
        existing: Option<Value>,
        plaintext: bool,
    ) -> Result<Value> {
        let state = match existing {
            None => {
                let sync = SyncKey::generate();
                secrets.set_key(&sync, 0);
                self.store_secrets(secrets)?;
                let body =
                    json!({"encrypted": !plaintext, "confirm_plaintext": plaintext, "key_id": sync.id()});
                let state = block_on(api.post("/api/v1/sync/setup", body)).map_err(sync_error)?;
                if state["encrypted"].as_bool() != Some(!plaintext) {
                    return Err(failed("the server set up sync with another encryption than asked"));
                }
                secrets.set_key(&sync, state["epoch"].as_i64().unwrap_or(0));
                self.store_secrets(secrets)?;
                self.update(|s| (s.ready, s.stale) = (false, false))?;
                state
            }
            Some(state) => state,
        };
        let fresh = !self.saved().ready && !self.saved().stale;
        self.update(|s| {
            if fresh {
                (s.epoch, s.ready) = (state["epoch"].as_i64().unwrap_or(0), !secrets.key.is_empty());
            }
            (s.encrypted, s.warning) =
                (state["encrypted"].as_bool(), state["warning"].as_str().map(String::from));
            (s.plaintext, s.sync_off, s.last_error) = (plaintext, false, None);
        })?;
        Ok(state)
    }
}
