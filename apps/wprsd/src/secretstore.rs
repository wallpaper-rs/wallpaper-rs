use crate::daemon::Daemon;
use crate::error::{Error, Result};
use crate::keyring::relocate;
use std::sync::Arc;
use std::time::Duration;
use wprs_core::dbus::error::KEYRING_UNAVAILABLE;

impl Daemon {
    pub fn secrets_in_file(&self) -> bool {
        self.secret_home().in_file()
    }

    pub async fn keyring_found(&self) -> bool {
        let Ok(keyring) = self.keyring() else { return false };
        tokio::time::timeout(Duration::from_millis(300), keyring.ring_reachable("")).await.unwrap_or(true)
    }

    pub async fn set_secret_store(self: &Arc<Self>, file: bool) -> Result<()> {
        let providers = self.keyring()?;
        let server = self
            .account
            .server_ring()
            .ok_or_else(|| Error::Named(KEYRING_UNAVAILABLE, "the keyring is not connected yet".into()))?;
        let properties = providers.properties();
        let left = relocate(&[providers, server, &properties], file).await?;
        if left > 0 {
            tracing::warn!(left, "some secrets could not be deleted from the keyring after moving them");
        }
        tracing::info!(file, "the place where secrets are kept changed");
        self.changes.changed("providers", Vec::new());
        self.changes.changed("account", Vec::new());
        Ok(())
    }
}
