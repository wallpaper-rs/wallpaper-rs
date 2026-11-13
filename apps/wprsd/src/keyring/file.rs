use crate::compositor::lock;
use crate::error::{Error, Result, failed};
use crate::store::private_dir;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use zeroize::{Zeroize, Zeroizing};

pub const NAME: &str = "secrets.json";

#[derive(Default)]
struct Held(BTreeMap<String, String>);

impl Drop for Held {
    fn drop(&mut self) {
        self.0.values_mut().for_each(Zeroize::zeroize);
    }
}

#[derive(Default)]
pub struct Home {
    path: Option<PathBuf>,
    io: Mutex<()>,
}

fn key(kind: &str, id: &str) -> String {
    format!("{kind}/{id}")
}

impl Home {
    pub fn at(path: PathBuf) -> Home {
        Home { path: Some(path), io: Mutex::default() }
    }

    pub fn in_file(&self) -> bool {
        self.path.as_ref().is_some_and(|path| path.exists())
    }

    fn load(&self) -> Result<Held> {
        let Some(path) = &self.path else { return Ok(Held::default()) };
        let text = match std::fs::read_to_string(path) {
            Ok(text) => Zeroizing::new(text),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Held::default()),
            Err(e) => return Err(e.into()),
        };
        serde_json::from_str(&text)
            .map(Held)
            .map_err(|_| failed("the private file with the secrets cannot be read"))
    }

    fn save(&self, held: &Held) -> Result<()> {
        let path = self.path.as_ref().ok_or_else(|| failed("no private file is set for the secrets"))?;
        let dir = path.parent().unwrap_or(std::path::Path::new("."));
        private_dir(dir)?;
        let text = Zeroizing::new(serde_json::to_string(&held.0).map_err(|e| failed(e.to_string()))?);
        let mut file = tempfile::Builder::new().prefix(".tmp-").tempfile_in(dir)?;
        file.write_all(text.as_bytes())?;
        file.persist(path).map_err(|e| Error::from(e.error))?;
        Ok(())
    }

    pub fn get(&self, kind: &str, id: &str) -> Result<Option<Zeroizing<String>>> {
        let _io = lock(&self.io);
        Ok(self.load()?.0.get(&key(kind, id)).map(|text| Zeroizing::new(text.clone())))
    }

    pub fn put(&self, kind: &str, id: &str, secret: &str) -> Result<()> {
        let _io = lock(&self.io);
        let mut held = self.load()?;
        held.0.insert(key(kind, id), secret.to_owned());
        self.save(&held)
    }

    pub fn remove(&self, kind: &str, id: &str) -> Result<()> {
        let _io = lock(&self.io);
        let mut held = self.load()?;
        if held.0.remove(&key(kind, id)).is_some() {
            self.save(&held)?;
        }
        Ok(())
    }

    pub fn everything(&self) -> Result<Vec<(String, String, Zeroizing<String>)>> {
        let _io = lock(&self.io);
        let held = self.load()?;
        let split = |(k, v): (&String, &String)| {
            let (kind, id) = k.split_once('/')?;
            Some((kind.to_owned(), id.to_owned(), Zeroizing::new(v.clone())))
        };
        Ok(held.0.iter().filter_map(split).collect())
    }

    pub fn begin(&self, entries: &[(String, String, Zeroizing<String>)]) -> Result<()> {
        let _io = lock(&self.io);
        let mut held = Held::default();
        for (kind, id, secret) in entries {
            held.0.insert(key(kind, id), secret.to_string());
        }
        self.save(&held)
    }

    pub fn end(&self) -> Result<()> {
        let _io = lock(&self.io);
        match self.path.as_ref().map(std::fs::remove_file) {
            Some(Err(e)) if e.kind() != std::io::ErrorKind::NotFound => Err(e.into()),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn the_file_exists_only_while_chosen_is_private_and_replaced_whole() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("data").join(NAME);
        let home = Home::at(path.clone());
        assert!(!home.in_file() && home.get("provider", "a").unwrap().is_none());
        home.begin(&[]).unwrap();
        assert!(home.in_file());
        home.put("provider", "photos", "first").unwrap();
        home.put("provider", "photos", "second").unwrap();
        home.put("property", "pkg/key", "other").unwrap();
        assert_eq!(home.get("provider", "photos").unwrap().as_deref().map(String::as_str), Some("second"));
        assert_eq!(home.everything().unwrap().len(), 2);
        assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);
        home.remove("provider", "photos").unwrap();
        home.remove("provider", "photos").unwrap();
        assert!(home.get("provider", "photos").unwrap().is_none());
        home.end().unwrap();
        home.end().unwrap();
        assert!(!home.in_file());
    }

    #[test]
    fn a_home_without_a_path_is_never_a_file() {
        let home = Home::default();
        assert!(!home.in_file() && home.everything().unwrap().is_empty());
    }
}
