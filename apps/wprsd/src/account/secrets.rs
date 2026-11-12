use super::{block_on, vaultfile};
use crate::daemon::Daemon;
use crate::error::{Error, Result, failed};
use crate::keyring::{Keyring, Stored};
use std::time::Duration;
use wprs_core::dbus::error::KEYRING_UNAVAILABLE;
use wprs_sync::Api;
pub use wprs_sync::{Secrets, encode};
use zeroize::Zeroizing;

const ID: &str = "server";

fn broken() -> Error {
    failed("the stored access for this server is broken; remove the server and link again")
}

impl Daemon {
    fn server_keyring(&self) -> Result<&Keyring> {
        self.account.keyring.get().ok_or_else(|| failed("the keyring is not ready yet"))
    }

    pub async fn read_secrets(&self, prompt: bool) -> Result<Secrets> {
        if self.saved().file_secrets {
            let text = vaultfile::read(&self.secrets_file())?;
            let text = text.ok_or_else(|| {
                failed("the stored access for this server is missing; link this computer again")
            })?;
            return serde_json::from_str(&text).map_err(|_| broken());
        }
        match self.server_keyring()?.read(ID, prompt).await? {
            Stored::Secret(text) => serde_json::from_str(&text).map_err(|_| broken()),
            Stored::Locked => {
                Err(Error::Named(KEYRING_UNAVAILABLE, "unlock the keyring and try again".into()))
            }
            Stored::Missing => {
                Err(failed("the keyring holds no token for this server; link this computer again"))
            }
        }
    }

    pub fn secrets(&self, prompt: bool) -> Result<Secrets> {
        block_on(self.read_secrets(prompt))
    }

    pub fn store_secrets(&self, secrets: &Secrets) -> Result<()> {
        let text = Zeroizing::new(serde_json::to_string(secrets).map_err(|e| failed(e.to_string()))?);
        if self.saved().file_secrets {
            return vaultfile::write(&self.secrets_file(), &text);
        }
        block_on(self.server_keyring()?.store(ID, &text))
    }

    pub fn forget_secrets(&self) -> Result<()> {
        if self.saved().file_secrets {
            return vaultfile::remove(&self.secrets_file());
        }
        block_on(self.server_keyring()?.delete(ID))
    }

    pub fn keyring_ready(&self) -> Result<()> {
        if self.saved().file_secrets {
            return Ok(());
        }
        let keyring = self.server_keyring()?;
        let reached = block_on(tokio::time::timeout(Duration::from_secs(10), keyring.reachable(ID)));
        match reached {
            Ok(true) => Ok(()),
            _ => Err(Error::Named(
                KEYRING_UNAVAILABLE,
                "no keyring answers to store the access of this computer".into(),
            )),
        }
    }

    pub async fn events(&self) -> Result<reqwest::Response> {
        let secrets = self.read_secrets(false).await?;
        let (url, pin) = {
            let saved = self.saved();
            (saved.url.clone(), saved.pin.clone())
        };
        let api = Api::pinned(&url, pin.as_deref()).map_err(super::sync_error)?.with_token(&secrets.token);
        let opened = tokio::time::timeout(Duration::from_secs(30), api.events()).await;
        opened.map_err(|_| failed("the server event stream did not answer"))?.map_err(super::sync_error)
    }

    pub fn account_api(&self, prompt: bool) -> Result<(Api, Secrets)> {
        let secrets = self.secrets(prompt)?;
        let (url, pin) = {
            let saved = self.saved();
            (saved.url.clone(), saved.pin.clone())
        };
        let api = Api::pinned(&url, pin.as_deref()).map_err(super::sync_error)?.with_token(&secrets.token);
        Ok((api, secrets))
    }
}
