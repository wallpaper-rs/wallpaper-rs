use super::level::sync_state;
use super::secrets::{Secrets, encode};
use super::server::{exists, pause, same_origin};
use super::{block_on, hostname, sync_error};
use crate::daemon::Daemon;
use crate::error::{Error, Result, failed, invalid};
use crate::jobs::Ctl;
use reqwest::Method;
use serde_json::{Value, json};
use std::sync::Arc;
use std::time::{Duration, Instant};
use wprs_core::dbus::error::{EXISTS, KEYRING_UNAVAILABLE};
use wprs_core::model::JobKind;
use wprs_sync::{AGENT, Api, DeviceKey, Poll, poll};

pub const REFUSED: &str = "the link was refused";
pub const EXPIRED: &str = "the code expired before it was approved; link again";
pub const NEEDS_PLAIN: &str =
    "the account syncs without end-to-end encryption; link with plain text only if you accept that";

pub fn problem_of(error: &Error) -> Option<(&'static str, String)> {
    if matches!(error, Error::Cancelled) {
        return None;
    }
    let (name, message) = error.parts();
    let code = match message.as_str() {
        _ if name == KEYRING_UNAVAILABLE => "keyring",
        REFUSED => "refused",
        EXPIRED => "expired",
        NEEDS_PLAIN => "encryption",
        _ => "other",
    };
    Some((code, message))
}

impl Daemon {
    fn note_link(&self, done: &Result<Value>) {
        let (problem, error) = match done.as_ref().err().and_then(problem_of) {
            Some((code, message)) => (Some(code.to_owned()), Some(message)),
            None => (None, None),
        };
        if let Err(e) = self.update(|s| (s.link_problem, s.link_error) = (problem, error)) {
            tracing::warn!("the reason a link failed could not be kept: {e}");
        }
    }

    fn undo_link(&self, api: &Api, device: &str, token: &str) {
        let gone = format!("/api/v1/me/devices/{device}");
        let _ = block_on(api.clone().with_token(token).call(Method::DELETE, &gone, None));
        if let Err(e) = self.forget_secrets() {
            tracing::warn!("the access of an unfinished link could not be deleted: {e}");
        }
    }

    pub fn link_device(
        self: &Arc<Self>,
        name: &str,
        plaintext: bool,
        sync: bool,
    ) -> Result<(String, String, u64)> {
        let url = self.server_url()?;
        if self.saved().device.is_some() || !self.jobs.find(JobKind::Link, "link").is_empty() {
            return Err(exists("this computer is already linked or linking"));
        }
        self.keyring_ready()?;
        self.update(|s| (s.link_problem, s.link_error) = (None, None))?;
        let label = Some(name.trim()).filter(|n| !n.is_empty()).map_or_else(hostname, String::from);
        if label.chars().count() > 64 || label.chars().any(char::is_control) {
            return Err(invalid("a device name has 1 to 64 printable characters"));
        }
        let pin = self.saved().pin.clone();
        let (key, api) = (DeviceKey::generate(), Api::pinned(&url, pin.as_deref()).map_err(sync_error)?);
        let body = json!({"platform": "linux", "enc_key": encode(&key.public()), "client": AGENT});
        let start = block_on(api.post("/api/v1/auth/device/start", body)).map_err(sync_error)?;
        let text = |field: &str| start[field].as_str().unwrap_or_default().to_owned();
        let (code, interval, expires) =
            (text("device_code"), start["interval"].as_u64(), start["expires_in"].as_u64());
        let (interval, expires) = (interval.unwrap_or(5).clamp(1, 60), expires.unwrap_or(600).min(1800));
        let job = self.start_job(JobKind::Link, "link", move |d, ctl| {
            let done = d
                .enroll(&api, &code, interval, expires, ctl)
                .and_then(|token| d.finish_link(api, token, key, label, sync.then_some(plaintext)));
            d.note_link(&done);
            done
        });
        let shown = Some(text("verification_uri")).filter(|uri| same_origin(uri, &url));
        Ok((text("user_code"), shown.unwrap_or_else(|| format!("{url}/link")), job))
    }

    fn enroll(&self, api: &Api, code: &str, mut interval: u64, expires: u64, ctl: &Ctl) -> Result<Value> {
        let deadline = Instant::now() + Duration::from_secs(expires);
        while Instant::now() < deadline {
            pause(ctl, Duration::from_secs(interval))?;
            let answer = block_on(api.post("/api/v1/auth/device/token", json!({"device_code": code})));
            match poll(&mut interval, answer) {
                Poll::Done(token) => return Ok(token),
                Poll::Wait(_) => {}
                Poll::Stop(e) if e.code() == "access_denied" => return Err(failed(REFUSED)),
                Poll::Stop(e) => return Err(sync_error(e)),
            }
        }
        Err(failed(EXPIRED))
    }

    fn finish_link(
        self: &Arc<Self>,
        api: Api,
        token: Value,
        key: DeviceKey,
        label: String,
        sync: Option<bool>,
    ) -> Result<Value> {
        let text = |field: &str| token[field].as_str().unwrap_or_default().to_owned();
        let (device, secret) = (text("device_id"), text("token"));
        let done = self.complete_link(api.clone(), token, key, label, sync);
        if done.is_err() && self.saved().device.is_none() {
            self.undo_link(&api, &device, &secret);
        }
        done
    }

    fn complete_link(
        self: &Arc<Self>,
        api: Api,
        token: Value,
        key: DeviceKey,
        label: String,
        sync: Option<bool>,
    ) -> Result<Value> {
        let text = |field: &str| token[field].as_str().unwrap_or_default().to_owned();
        let mut secrets = Secrets::default();
        (secrets.token, secrets.device) = (text("token"), encode(&key.bytes()));
        self.store_secrets(&secrets)?;
        let api = api.with_token(&secrets.token);
        let me = block_on(api.get("/api/v1/me")).map_err(sync_error)?;
        let existing = sync.map(|plaintext| sync_state(&api, plaintext)).transpose()?;
        let owner = (self.server_url()?, me["id"].as_i64().unwrap_or_default());
        self.update(|s| {
            if s.owner.as_ref() != Some(&owner) {
                (s.replica, s.links, s.clock, s.present) = Default::default();
                (s.kept, s.declined, s.unapplied, s.offers) = Default::default();
                (s.files.stored, s.files.dropped, s.files.uploaded) = Default::default();
                s.owner = Some(owner.clone());
            }
            (s.device, s.device_name, s.label) = (Some(text("device_id")), Some(text("device_name")), label);
            (s.plaintext, s.pinned, s.members) = (false, Some(Default::default()), Default::default());
            (s.ready, s.stale, s.encrypted, s.warning, s.sync_off) = (false, false, None, None, true);
            s.handle = me["handle"].as_str().map(String::from);
        })?;
        let state = match (existing, sync) {
            (Some(existing), Some(plaintext)) => {
                Some(self.setup_sync(&api, &mut secrets, existing, plaintext)?)
            }
            _ => None,
        };
        let repo = self.saved().repo_url.clone();
        if let Some(Err(e)) =
            repo.map(|url| self.add_repo(&url)).filter(|r| r.as_ref().is_err_and(|e| e.parts().0 != EXISTS))
        {
            tracing::warn!("the repository of the server could not be added: {e}");
        }
        let _ = self.refresh_peers(&api);
        self.account.wake.notify_one();
        self.account.restart.notify_one();
        let encrypted = state.as_ref().map_or(Value::Null, |state| state["encrypted"].clone());
        let needs_key = state.is_some() && secrets.key.is_empty();
        Ok(
            json!({"device": text("device_name"), "syncing": state.is_some(), "encrypted": encrypted, "needs_key": needs_key}),
        )
    }
}
