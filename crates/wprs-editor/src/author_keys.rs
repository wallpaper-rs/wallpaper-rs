use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use wprs_core::message::said;
use wprs_format::{Signer, fingerprint, key_text, public_key, sign_rotation};

pub fn folder() -> PathBuf {
    dirs::data_dir().unwrap_or_default().join("wallpaper-rs").join("keys")
}

pub fn repositories(dir: &Path) -> PathBuf {
    dir.join("repositories")
}

fn named(name: &str) -> Result<()> {
    let fine = !name.is_empty()
        && name.len() <= 64
        && !name.starts_with('.')
        && name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
    ensure!(
        fine,
        said("editor.key_name", "a key name has 1 to 64 letters, digits, dots, dashes or underscores", &[])
    );
    Ok(())
}

fn write(path: &Path, text: &str, mode: u32) -> Result<()> {
    let temp = path.with_extension("tmp");
    drop(fs::remove_file(&temp));
    let mut file = OpenOptions::new().write(true).create_new(true).mode(mode).open(&temp)?;
    file.write_all(text.as_bytes())?;
    drop(file);
    Ok(fs::rename(&temp, path)?)
}

pub fn create(dir: &Path, name: &str, password: Option<String>, replace: bool) -> Result<Value> {
    named(name)?;
    ensure!(
        password.as_ref().is_none_or(|p| !p.is_empty()),
        said("editor.key_password", "the password is empty; leave it out for a key without one", &[])
    );
    let (secret, public) = (dir.join(format!("{name}.key")), dir.join(format!("{name}.pub")));
    let taken = secret.exists() || public.exists();
    ensure!(
        replace || !taken,
        said("editor.key_exists", "%1 or %2 already exists", &[&secret.display(), &public.display()])
    );
    if !dir.exists() {
        fs::create_dir_all(dir)?;
        drop(fs::set_permissions(dir, fs::Permissions::from_mode(0o700)));
    }
    let pair = wprs_format::keygen(password)?;
    write(&secret, &pair.secret, 0o600)?;
    write(&public, &pair.public, 0o644)?;
    Ok(
        json!({ "name": name, "fingerprint": pair.fingerprint, "secret": secret, "public": public, "replaced": taken }),
    )
}

pub fn list(dir: &Path) -> Value {
    let mut found: Vec<Value> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name()?.to_str()?.strip_suffix(".key")?.to_owned();
            let public = path.with_extension("pub");
            let print = fs::read_to_string(&public).ok().and_then(|t| fingerprint(&t).ok());
            let secret = fs::read_to_string(&path).ok()?;
            let encrypted = Signer::new(&secret, None).is_err();
            Some(json!({ "name": name, "secret": path, "public": public.exists().then_some(public), "fingerprint": print, "encrypted": encrypted }))
        })
        .collect();
    found.sort_by_key(|k| k["name"].as_str().unwrap_or_default().to_owned());
    json!(found)
}

pub fn rotate(old: &Path, password: Option<String>, new: &str, author: &str) -> Result<Value> {
    ensure!(
        !author.trim().is_empty(),
        said("editor.key_author", "name the author id that the repository shows", &[])
    );
    let secret = fs::read_to_string(old)
        .map_err(|e| said("editor.cannot_read", "cannot read %1: %2", &[&old.display(), &e]))?;
    let signer = Signer::new(&secret, password)?;
    let target = public_key(&key_text(new)?)?;
    let signature = sign_rotation(&signer, author.trim(), &target)?;
    Ok(
        json!({ "from": signer.fingerprint, "to": fingerprint(&target)?, "author": author.trim(), "signature": signature }),
    )
}
