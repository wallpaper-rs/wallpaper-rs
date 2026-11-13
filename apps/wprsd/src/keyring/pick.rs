use super::{Keyring, Result, Stored, bounded};
use std::collections::HashMap;

impl Keyring {
    fn part(&self) -> &'static str {
        self.kind.1
    }

    pub async fn has(&self, id: &str) -> Result<bool> {
        if !self.home.in_file() {
            return self.ring_has(id).await;
        }
        let stored = self.home.get(self.part(), id)?.is_some();
        self.remember(id, stored);
        Ok(stored)
    }

    pub async fn read(&self, id: &str, prompt: bool) -> Result<Stored> {
        if !self.home.in_file() {
            return self.ring_read(id, prompt).await;
        }
        Ok(self.home.get(self.part(), id)?.map_or(Stored::Missing, Stored::Secret))
    }

    pub async fn store(&self, id: &str, secret: &str) -> Result<()> {
        if !self.home.in_file() {
            return self.ring_store(id, secret).await;
        }
        self.home.put(self.part(), id, secret)?;
        self.remember(id, true);
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        if !self.home.in_file() {
            return self.ring_delete(id).await;
        }
        self.home.remove(self.part(), id)?;
        self.remember(id, false);
        Ok(())
    }

    pub async fn reachable(&self, id: &str) -> bool {
        self.home.in_file() || self.ring_reachable(id).await
    }

    pub async fn discard(&self, id: &str) -> Result<bool> {
        if !self.home.in_file() {
            return self.ring_discard(id).await;
        }
        self.home.remove(self.part(), id)?;
        Ok(true)
    }

    pub(crate) async fn ring_ids(&self) -> Result<Vec<String>> {
        bounded(self.write, async {
            let attributes = HashMap::from([("xdg:schema", self.kind.0)]);
            let Ok(service) = self.service().await else { return Ok(Vec::new()) };
            let Ok((open, locked)) = service.search_items(attributes).await else { return Ok(Vec::new()) };
            if !locked.is_empty() {
                self.unlock(&locked).await?;
            }
            let mut ids = Vec::new();
            for path in open.into_iter().chain(locked) {
                let found = self.item(path).await?.attributes().await;
                ids.extend(found.ok().and_then(|mut a| a.remove(self.kind.1)));
            }
            Ok(ids)
        })
        .await
    }
}
