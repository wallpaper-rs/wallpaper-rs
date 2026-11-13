use super::{Keyring, Result, Stored, unavailable};
use std::time::Duration;
use zeroize::Zeroizing;

type Entry = (String, String, Zeroizing<String>);

async fn gather(rings: &[&Keyring]) -> Result<Vec<Entry>> {
    let mut found = Vec::new();
    for ring in rings {
        for id in ring.ring_ids().await? {
            match ring.ring_read(&id, true).await? {
                Stored::Secret(text) => found.push((ring.kind.1.to_owned(), id, text)),
                _ => return Err(unavailable("unlock the keyring and try again")),
            }
        }
    }
    Ok(found)
}

fn ring_of<'a>(rings: &[&'a Keyring], part: &str) -> Option<&'a Keyring> {
    rings.iter().copied().find(|ring| ring.kind.1 == part)
}

async fn to_file(rings: &[&Keyring]) -> Result<usize> {
    let entries = gather(rings).await?;
    rings[0].home.begin(&entries)?;
    let mut left = 0;
    for (part, id, _) in &entries {
        let gone = match ring_of(rings, part) {
            Some(ring) => ring.ring_delete(id).await.is_ok(),
            None => true,
        };
        left += usize::from(!gone);
    }
    Ok(left)
}

async fn to_keyring(rings: &[&Keyring]) -> Result<usize> {
    let reached = tokio::time::timeout(Duration::from_secs(10), rings[0].ring_reachable("")).await;
    if !reached.unwrap_or(false) {
        return Err(unavailable("no keyring answers to store the secrets"));
    }
    let entries = rings[0].home.everything()?;
    let mut stored: Vec<(&Keyring, String)> = Vec::new();
    for (part, id, secret) in &entries {
        let Some(ring) = ring_of(rings, part) else { continue };
        if let Err(e) = ring.ring_store(id, secret).await {
            for (ring, id) in &stored {
                ring.ring_delete(id).await.ok();
            }
            return Err(e);
        }
        stored.push((ring, id.clone()));
    }
    rings[0].home.end()?;
    Ok(0)
}

pub async fn relocate(rings: &[&Keyring], file: bool) -> Result<usize> {
    match (file, rings[0].home.in_file()) {
        (true, false) => to_file(rings).await,
        (false, true) => to_keyring(rings).await,
        _ => Ok(0),
    }
}
