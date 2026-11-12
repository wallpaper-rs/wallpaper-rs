use crate::cli::JSON;
use crate::keys::{key_text, signer};
use crate::providers::patient;
use anyhow::{Result, ensure};
use clap::Subcommand;
use serde_json::Value;
use std::path::PathBuf;
use wprs_client::Client;

#[derive(Subcommand, Debug)]
pub enum AuthorKey {
    #[command(about = "List the keys registered for your online author id")]
    List {
        #[arg(long, help = JSON)]
        json: bool,
    },
    #[command(about = "Register a public key with which you sign your packages online")]
    Add {
        #[arg(help = "Public key file or text of the key")]
        key: String,
    },
    #[command(about = "Hand your online author over from an old key to a new one, signing with the old key")]
    Rotate {
        #[arg(help = "Secret key file that signed the earlier versions")]
        old: PathBuf,
        #[arg(help = "Public key file or text of the new key")]
        new: String,
        #[arg(long, help = "Author id that the repository shows for the account")]
        author: String,
    },
    #[command(about = "Remove a registered key")]
    Remove {
        #[arg(help = "Fingerprint of the key, as list prints it")]
        fingerprint: String,
    },
}

fn items(found: &str) -> Result<Vec<Value>> {
    let found: Value = serde_json::from_str(found)?;
    Ok(found["items"].as_array().cloned().unwrap_or_default())
}

pub async fn run(client: &Client, command: AuthorKey) -> Result<()> {
    let sync = &client.sync;
    match command {
        AuthorKey::List { json } => {
            let found = patient(sync.list_author_keys()).await?;
            match json {
                true => println!("{found}"),
                false => {
                    for key in items(&found)? {
                        println!("{} {}", key["fingerprint"].as_str().unwrap_or("-"), key["created_at"]);
                    }
                }
            }
        }
        AuthorKey::Add { key } => println!("{}", patient(sync.add_author_key(&key_text(&key)?)).await?),
        AuthorKey::Rotate { old, new, author } => {
            let signer = signer(&old)?;
            let key = key_text(&new)?;
            ensure!(!author.trim().is_empty(), "name the author id that the repository shows");
            let signature = wprs_format::sign_rotation(&signer, author.trim(), &key)?;
            println!("{}", patient(sync.rotate_author_key(&signer.fingerprint, &key, &signature)).await?)
        }
        AuthorKey::Remove { fingerprint } => patient(sync.remove_author_key(&fingerprint)).await?,
    }
    Ok(())
}

pub async fn uploads(client: &Client, json: bool, cancel: Option<String>) -> Result<()> {
    if let Some(id) = cancel {
        return patient(client.sync.cancel_upload(&id)).await;
    }
    let found: Value = serde_json::from_str(&patient(client.sync.list_uploads()).await?)?;
    if json {
        println!("{found}");
        return Ok(());
    }
    for upload in found["uploads"].as_array().into_iter().flatten() {
        let text = |k: &str| upload[k].as_str().unwrap_or("-").to_owned();
        let errors = upload["errors"].as_array().map(|e| e.len()).unwrap_or(0);
        println!("{} {} {} errors={errors}", text("id"), text("state"), upload["created_at"]);
    }
    Ok(())
}
