use crate::daemon::Daemon;
use crate::error::{Error, Result, failed};
use crate::store::private_dir;
use std::io::Write;
use std::path::{Path, PathBuf};
use zeroize::Zeroizing;

const NAME: &str = "server-secrets.json";

pub fn write(path: &Path, text: &str) -> Result<()> {
    let dir = path.parent().unwrap_or(Path::new("."));
    private_dir(dir)?;
    let mut file = tempfile::Builder::new().prefix(".tmp-").tempfile_in(dir)?;
    file.write_all(text.as_bytes())?;
    file.persist(path).map_err(|e| Error::from(e.error))?;
    Ok(())
}

pub fn read(path: &Path) -> Result<Option<Zeroizing<String>>> {
    match std::fs::read_to_string(path) {
        Ok(text) => Ok(Some(Zeroizing::new(text))),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn remove(path: &Path) -> Result<()> {
    match std::fs::remove_file(path) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e.into()),
        _ => Ok(()),
    }
}

impl Daemon {
    pub fn secrets_file(&self) -> PathBuf {
        self.store.data.join(NAME)
    }

    pub fn use_file_secrets(&self, on: bool) -> Result<()> {
        if self.saved().device.is_some() {
            return Err(failed(
                "remove the server first: where the access is kept can only change before linking",
            ));
        }
        self.update(|s| s.file_secrets = on)?;
        if !on {
            remove(&self.secrets_file())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn the_file_is_private_replaced_whole_and_removed() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("data").join(NAME);
        assert!(read(&path).unwrap().is_none());
        write(&path, "first").unwrap();
        write(&path, "second").unwrap();
        assert_eq!(read(&path).unwrap().as_deref().map(String::as_str), Some("second"));
        assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);
        remove(&path).unwrap();
        remove(&path).unwrap();
        assert!(read(&path).unwrap().is_none());
    }
}
