use crate::compositor::lock;
use crate::daemon::Daemon;
use crate::error::{Error, Result, not_found};
use crate::kinds::{Options, effective, kind_of, table};
use std::collections::BTreeMap;
use std::sync::Arc;
use wprs_core::model::JobKind;

impl Daemon {
    pub async fn set_provider(self: &Arc<Self>, id: String, key: String, value: String) -> Result<()> {
        self.update_provider(id, Options::from([(key, value)])).await
    }

    pub async fn update_provider(self: &Arc<Self>, id: String, changes: Options) -> Result<()> {
        let (known, mut given, _) = self.provider(&id)?;
        let keys: Vec<String> = changes.keys().cloned().collect();
        given.extend(changes.into_iter().map(|(key, value)| (key, value.trim().to_owned())));
        effective(&known, &given)?;
        if known.secret.is_some() && keys.iter().any(|key| ["url", "server", "user"].contains(&key.as_str()))
        {
            self.keyring()?.delete(&id).await?;
        }
        let entry = table(&known, &given);
        self.run(move |d| {
            d.update_config(|c| {
                c.providers.insert(id.clone(), entry);
                Ok(())
            })?;
            tracing::info!(provider = id, keys = keys.join(","), "provider options changed");
            d.changes.changed("providers", vec![id]);
            d.remote.poke.notify_one();
            Ok::<_, Error>(())
        })
        .await
    }

    pub async fn remove_provider(self: &Arc<Self>, id: String, purge: bool) -> Result<()> {
        let value = self.state().config.providers.get(&id).cloned();
        let value = value.ok_or_else(|| not_found(format!("provider {id:?} not found")))?;
        self.library_ready()?;
        let kind = value.get("kind").and_then(toml::Value::as_str);
        if self.has_secret(&id, kind).await? {
            self.keyring()?.delete(&id).await?;
        } else if kind.and_then(|k| kind_of(k).ok()?.secret).is_some()
            && !self.keyring()?.reachable(&id).await
        {
            self.orphan(&id, true);
        }
        self.jobs.find(JobKind::ProviderSync, &id).into_iter().try_for_each(|job| self.cancel_job(job))?;
        self.run(move |d| d.drop_provider(&id, purge)).await
    }

    pub async fn clear_provider_cache(self: &Arc<Self>, id: String) -> Result<()> {
        self.provider(&id)?;
        self.library_ready()?;
        self.jobs.find(JobKind::ProviderSync, &id).into_iter().try_for_each(|job| self.cancel_job(job))?;
        self.run(move |d| d.empty_provider(&id)).await
    }

    fn empty_provider(&self, id: &str) -> Result<()> {
        self.drop_photos(id, false)?;
        let entries = std::fs::read_dir(self.provider_dir(id)).into_iter().flatten().flatten();
        for entry in entries.filter(|e| e.file_name() != crate::remote::CREDITS) {
            let path = entry.path();
            let gone =
                if path.is_dir() { std::fs::remove_dir_all(&path) } else { std::fs::remove_file(&path) };
            gone.unwrap_or_else(|e| tracing::warn!(provider = id, "cannot delete a stored photo: {e}"));
        }
        tracing::info!(provider = id, "stored photos deleted");
        self.library_changed(Vec::new());
        self.changes.changed("providers", vec![id.to_owned()]);
        Ok(())
    }

    fn drop_photos(&self, id: &str, source_too: bool) -> Result<()> {
        if let Some(source) = self.provider_source(id) {
            let removed = self.store.write(|lib| match source_too {
                true => lib.remove_source(&source.uid),
                false => lib.empty_source(&source.uid),
            })?;
            self.forget_files(&removed);
            self.remember_credits(&source.path, &BTreeMap::new());
        }
        Ok(())
    }

    fn drop_provider(&self, id: &str, purge: bool) -> Result<()> {
        self.drop_photos(id, true)?;
        self.update_config(|c| {
            c.providers.remove(id);
            Ok(())
        })?;
        match purge {
            true => std::fs::remove_dir_all(self.provider_dir(id)).ok(),
            false => std::fs::remove_file(self.provider_dir(id).join(crate::remote::CREDITS)).ok(),
        };
        lock(&self.remote.health).remove(id);
        tracing::info!(provider = id, "provider removed");
        self.library_changed(Vec::new());
        self.changes.changed("providers", vec![id.to_owned()]);
        Ok(())
    }
}
