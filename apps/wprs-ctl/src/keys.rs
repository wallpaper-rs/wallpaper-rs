use crate::input::password;
use crate::told::{fill, told};
use crate::usage;
use anyhow::Result;
use clap::Args;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use wprs_format::{Charter, Signer};

#[derive(Args, Debug)]
pub struct Policy {
    #[arg(long, help = "Number of root keys that must sign a root")]
    threshold: Option<usize>,
    #[arg(
        long = "author",
        value_name = "ID=PUB",
        help = "Author key; repeat for several keys of an author. The keys given replace the keys the author had"
    )]
    authors: Vec<String>,
    #[arg(
        long, num_args = 0..=1, default_missing_value = "true", value_name = "BOOL",
        help = "Refuse packages that no author of the root signed"
    )]
    require_author_signature: Option<bool>,
    #[arg(
        long,
        num_args = 0..=1,
        default_missing_value = "true",
        value_name = "BOOL",
        help = "Let authors register online: clients trust per package the key that signed its first version"
    )]
    online_authors: Option<bool>,
    #[arg(
        long,
        value_name = "DAYS",
        help = "Lifetime of the new root; when rotating, the lifetime of the current root, 365 in a new repository"
    )]
    root_days: Option<u32>,
    #[arg(long, value_name = "DAYS", help = "Longest lifetime of a signed index; 14 in a new repository")]
    entry_max_days: Option<u32>,
    #[arg(long, value_name = "BYTES", help = "Largest package the repository takes, in bytes")]
    max_package_size: Option<u64>,
    #[arg(long = "mirror", value_name = "URL", help = "HTTPS mirror; replaces the mirrors")]
    mirrors: Vec<String>,
    #[arg(long, conflicts_with = "mirrors", help = "Drop every mirror of the root")]
    no_mirrors: bool,
    #[arg(long, help = "Bits of the index shards; 4 in a new repository")]
    shard_bits: Option<u8>,
}

impl Policy {
    pub fn charter(self) -> Result<Charter> {
        let mut authors = BTreeMap::<String, Vec<String>>::new();
        for author in &self.authors {
            let (id, key) = author.split_once('=').ok_or_else(|| usage(format!("{author} is not ID=PUB")))?;
            authors.entry(id.into()).or_default().push(key.into());
        }
        Ok(Charter {
            authors,
            threshold: self.threshold,
            require_author_signature: self.require_author_signature,
            online_authors: self.online_authors,
            root_days: self.root_days,
            entry_max_days: self.entry_max_days,
            max_package_size: self.max_package_size,
            mirrors: (self.no_mirrors || !self.mirrors.is_empty()).then_some(self.mirrors),
            shard_bits: self.shard_bits,
            ..Charter::default()
        })
    }
}

pub fn keygen(name: &str, no_password: bool) -> Result<String> {
    let password = (!no_password).then(|| password(true)).transpose()?;
    let made = wprs_editor::author_keys::create(Path::new("."), name, password, false)?;
    let file =
        |key: &str| made[key].as_str().and_then(|p| Path::new(p).file_name()?.to_str()).unwrap_or(name);
    eprintln!(
        "{}",
        fill(
            told(
                "%1 is your SECRET key: never share it, and keep a copy somewhere safe. %2 is the public key: give it to whoever must check your signatures."
            ),
            &[&file("secret"), &file("public")]
        )
    );
    Ok(made["fingerprint"].as_str().unwrap_or_default().to_owned())
}

pub fn key_text(key: &str) -> Result<String> {
    Ok(wprs_format::key_text(key)?)
}

pub fn keys(keys: &[String]) -> Result<Vec<String>> {
    keys.iter().map(|key| key_text(key)).collect()
}

pub fn signer(key: &Path) -> Result<Signer> {
    let secret = fs::read_to_string(key)?;
    Signer::new(&secret, None).or_else(|_| Ok(Signer::new(&secret, Some(password(false)?))?))
}
