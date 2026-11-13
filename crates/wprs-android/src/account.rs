use crate::said;
use crate::state::{CANCEL, linked, update};
use crate::{Error, cycle, escrow, keys, link, listen, pairing, private, repos, votes};
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Mutex;
use std::sync::atomic::Ordering::Relaxed;
use zeroize::Zeroizing;

pub(crate) const NETWORK: &str = "sync_network";
pub(crate) const LISTEN: &str = "sync_listen";

static BUSY: Mutex<()> = Mutex::new(());

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
enum Request {
    Status,
    Cancel,
    Pause {
        paused: bool,
    },
    Add {
        url: String,
        #[serde(default)]
        fingerprint: Option<String>,
    },
    Link {
        label: String,
        plaintext: bool,
    },
    Connect {
        label: String,
    },
    Syncing {
        on: bool,
        plaintext: bool,
    },
    Wait,
    Join {
        device: String,
    },
    Share {
        device: String,
    },
    Confirm {
        code: String,
    },
    Preview {
        code: String,
    },
    Approve {
        code: String,
        approve: bool,
        #[serde(default)]
        password: String,
    },
    Sync {
        conflict: String,
    },
    Remove,
    Revoke {
        device: String,
        password: String,
    },
    Rotate,
    Escrow {
        passphrase: String,
    },
    Restore {
        passphrase: String,
    },
    Files,
    ShareFiles {
        targets: Vec<String>,
        share: bool,
    },
    Fetch {
        id: String,
    },
    Listen {
        token: u64,
    },
    Hush {
        token: u64,
    },
    Repos,
    Offer {
        repo: String,
        add: bool,
    },
    RemoveRepo {
        repo_id: String,
    },
    Browse {
        query: String,
    },
    Install {
        repo_id: String,
        id: String,
    },
    Thumb {
        spec: String,
    },
    InstallBase {
        uid: String,
    },
    Votes {
        ids: Vec<String>,
    },
    Vote {
        id: String,
        vote: String,
    },
}

use crate::level::status;

pub(crate) fn request(text: &str) -> String {
    let answer = crate::reply::tried(text, crate::updates::request)
        .or_else(|| crate::reply::tried(text, crate::providers::request))
        .or_else(|| crate::reply::tried(text, repos::anchored::request))
        .unwrap_or_else(|| serde_json::from_str(text).map_err(Error::from).and_then(run));
    answer.unwrap_or_else(|e| e.reply()).to_string()
}

fn run(request: Request) -> Result<Value, Error> {
    match request {
        Request::Status => return Ok(status()),
        Request::Cancel => {
            CANCEL.store(true, Relaxed);
            return Ok(Value::Null);
        }
        Request::Pause { paused } => {
            linked()?;
            return update(|s| s.paused = paused).map(|()| status());
        }
        Request::Hush { token } => return Ok(json!(listen::hush(token))),
        Request::Files => return Ok(private::list()),
        Request::ShareFiles { targets, share } => return private::share(&targets, share),
        Request::Repos => return repos::list(),
        Request::Offer { repo, add } => return repos::confirm(&repo, add),
        Request::RemoveRepo { repo_id } => return repos::remove(&repo_id),
        Request::Browse { query } => return repos::browse(&query),
        Request::Install { repo_id, id } => return repos::install(&repo_id, &id),
        Request::Thumb { spec } => return repos::preview::thumb(&spec),
        Request::InstallBase { uid } => return crate::kin::install_base(&uid),
        Request::Votes { ids } => return votes::counts(&ids),
        Request::Vote { id, vote } => return votes::cast(&id, &vote),
        Request::Listen { token } => {
            let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
            return runtime.block_on(listen::listen(token));
        }
        _ => {}
    }
    let _busy = BUSY.try_lock().map_err(|_| {
        said("android.another_account_task", "another account task is running; try again when it ends", &[])
    })?;
    CANCEL.store(false, Relaxed);
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    runtime.block_on(async {
        match request {
            Request::Add { url, fingerprint } => link::add(&url, fingerprint.as_deref()).await,
            Request::Link { label, plaintext } => link::start(&label, Some(plaintext)).await,
            Request::Connect { label } => link::start(&label, None).await,
            Request::Syncing { on, plaintext } => crate::level::set(on, plaintext).await,
            Request::Wait => link::wait().await,
            Request::Join { device } => pairing::join(&device).await,
            Request::Share { device } => pairing::share(&device).await,
            Request::Confirm { code } => pairing::confirm(&code).await,
            Request::Preview { code } => keys::device_code(&code, "preview", json!({})).await,
            Request::Approve { code, approve, password } => {
                keys::device_code(&code, "approve", json!({"approve": approve, "password": password})).await
            }
            Request::Sync { conflict } => cycle::sync(&conflict).await,
            Request::Remove => link::remove().await.map(|()| status()),
            Request::Revoke { device, password } => crate::revoking::revoke(&device, &password).await,
            Request::Rotate => keys::rotate_now().await,
            Request::Escrow { passphrase } => escrow::set(Zeroizing::new(passphrase)).await,
            Request::Restore { passphrase } => escrow::restore(Zeroizing::new(passphrase)).await,
            Request::Fetch { id } => {
                crate::level::syncing()?;
                let (api, secrets) = crate::state::api()?;
                private::fetch(&api, &secrets, &id).await
            }
            _ => Ok(Value::Null),
        }
    })
}
