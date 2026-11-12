use super::keys::find;
use super::{block_on, sync_error};
use crate::daemon::Daemon;
use crate::error::{Result, invalid};
use reqwest::Method;
use serde_json::json;
use std::sync::Arc;
use wprs_core::model::JobKind;
use zeroize::Zeroizing;

impl Daemon {
    pub fn revoke_device(self: &Arc<Self>, device: &str, password: Zeroizing<String>) -> Result<u64> {
        self.linked()?;
        let (api, _) = self.account_api(true)?;
        let peers = self.refresh_peers(&api)?;
        let peer = find(&peers, device, |id| self.saved().label_of(id))?;
        if self.saved().device.as_ref() == Some(&peer.id) {
            return Err(invalid("this computer cannot revoke itself; remove the server instead"));
        }
        Ok(self.start_job(JobKind::Account, &format!("revoke {}", peer.name), move |d, _| {
            let gone = format!("/api/v1/me/devices/{}/revoke", peer.id);
            let body = json!({"password": password.as_str()});
            block_on(api.call(Method::POST, &gone, Some(body))).map_err(sync_error)?;
            d.refresh_peers(&api)?;
            d.account.wake.notify_one();
            Ok(json!({"revoked": peer.name}))
        }))
    }
}
