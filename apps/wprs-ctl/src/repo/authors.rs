use crate::input::secret;
use crate::keys::key_text;
use crate::show::either;
use anyhow::Result;
use clap::Subcommand;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::Method;
use serde_json::json;
use wprs_sync::Api;

const PATH: &str = "/api/v1/me/author-keys";

#[derive(Subcommand, Debug)]
pub enum AuthorCommand {
    #[command(about = "Register a public key as an online author, with a personal token of the upload scope")]
    RegisterAuthor {
        #[arg(help = "Address of the server, such as https://wallpapers.example.org")]
        server: String,
        #[arg(long, help = "Public key file or text")]
        key: String,
        #[arg(long, help = "Print the result as JSON")]
        json: bool,
    },
    #[command(about = "Hand an online author to a new key with the signature pack rotate-key printed")]
    RotateAuthor {
        #[arg(help = "Address of the server, such as https://wallpapers.example.org")]
        server: String,
        #[arg(long, help = "Fingerprint of the key that signed the earlier versions")]
        from: String,
        #[arg(long, help = "Public key file or text of the new key")]
        key: String,
        #[arg(long, help = "Signature file or text that wprs-ctl pack rotate-key printed")]
        signature: String,
        #[arg(long, help = "Print the result as JSON")]
        json: bool,
    },
    #[command(about = "Remove a registered author key; refused while a rotation still names it")]
    DropAuthorKey {
        #[arg(help = "Address of the server, such as https://wallpapers.example.org")]
        server: String,
        #[arg(help = "Fingerprint of the author key")]
        fingerprint: String,
    },
}

async fn api(server: &str) -> Result<Api> {
    let token = match std::env::var("WPRS_SERVER_TOKEN") {
        Ok(token) => token,
        Err(_) => secret("Personal token with the upload scope: ", false)?,
    };
    Ok(Api::new(server)?.with_token(token.trim()))
}

fn shown(reply: &serde_json::Value) -> String {
    format!("fingerprint {}", reply["fingerprint"].as_str().unwrap_or_default())
}

pub async fn run(command: AuthorCommand) -> Result<()> {
    match command {
        AuthorCommand::RegisterAuthor { server, key, json } => {
            let body = json!({"public_key": key_text(&key)?});
            let reply = api(&server).await?.post(PATH, body).await?;
            either(&reply, json, shown(&reply))?
        }
        AuthorCommand::RotateAuthor { server, from, key, signature, json } => {
            let body = json!({"from": from, "public_key": key_text(&key)?, "signature": key_text(&signature)?});
            let reply = api(&server).await?.post(&format!("{PATH}/rotate"), body).await?;
            either(&reply, json, shown(&reply))?
        }
        AuthorCommand::DropAuthorKey { server, fingerprint } => {
            let path = format!("{PATH}/{}", utf8_percent_encode(&fingerprint, NON_ALPHANUMERIC));
            drop(api(&server).await?.call(Method::DELETE, &path, None).await?)
        }
    }
    Ok(())
}
