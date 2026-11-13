use crate::providers::patient;
use crate::told::{fill, told, tr};
use anyhow::Result;
use wprs_client::{Client, parse};
use wprs_core::model::ProvidersReply;

const FILE: &str = told(
    "Secrets (provider passwords and API keys, and the access to the sync server) are kept in a private \
     file in the data folder, not in the system keyring: whoever can read your files can use them.",
);
const NONE: &str = told(
    "No system keyring was found, so secrets cannot be stored. Run `wprs-ctl provider store file` to \
     keep them in a private file instead (whoever can read your files could then use them).",
);
const PLAIN: &str =
    told("%1 uses plain http: its password or key travels unencrypted. Use https if the server allows it.");

pub fn notes(store: &str, keyring_found: bool, plain: &[String]) -> String {
    let mut out = String::new();
    if store == "file" {
        out += &format!("\n{}\n", tr(FILE));
    } else if store == "keyring" && !keyring_found {
        out += &format!("\n{}\n", tr(NONE));
    }
    for id in plain {
        out += &format!("{}\n", fill(PLAIN, &[id]));
    }
    out
}

pub async fn run(client: &Client, store: Option<&str>) -> Result<()> {
    match store {
        Some(store) => {
            if store == "file" {
                eprintln!("{}", tr(FILE));
            }
            patient(client.providers.set_secret_store(store)).await
        }
        None => {
            let reply: ProvidersReply = parse(&client.providers.list().await?)?;
            println!("{}", if reply.store == "file" { "file" } else { "keyring" });
            print!("{}", notes(&reply.store, reply.keyring_found, &[]));
            Ok(())
        }
    }
}
