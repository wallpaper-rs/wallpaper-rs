use crate::cli::Wait;
use crate::daemon::job;
use crate::input::{confirm, secret};
use crate::show::{self, bytes, table, text, when};
use crate::told::{fill, told, tr};
use anyhow::{Result, bail};
use futures_util::TryFutureExt;
use wprs_client::Client;
use wprs_core::model::AccountReply;

pub const NAMES: &str = "Item names on the server are keyed hashes: a device that once held the names key \
                         and has the same file can still tell that the account stores it, even after it is \
                         revoked.";
const OWN: &str = told(
    "Approve only a code that your own new device shows on its screen: \
                   someone who sends you a code is trying to get into your account.",
);

const NO_KEYRING: &str = told(
    "No keyring is available to keep the access of this computer. Start or unlock the \
                          system keyring and link again, or keep the secrets in a private file with `wprs-ctl \
                          provider store file` (whoever can read your files could then use them).",
);

pub fn keyring_hint(error: &anyhow::Error) {
    if wprs_client::error_name(error) == Some(wprs_core::dbus::error::KEYRING_UNAVAILABLE) {
        eprintln!("{}", tr(NO_KEYRING));
    }
}

fn state_note(state: String) -> String {
    match state.as_str() {
        "signed_out" => "signed out: the server no longer accepts this computer (it was removed from the \
                         account or the account was deleted); run `wprs-ctl server remove` and link again"
            .into(),
        "connected" => tr(level::CONNECTED),
        _ => state,
    }
}

pub async fn revoke(client: &Client, device: &str, yes: bool, wait: &Wait) -> Result<()> {
    eprintln!(
        "{}",
        tr(told("The device can no longer sync with this account, and the sync key is replaced."))
    );
    if !yes && !confirm(&fill(told("Remove %1 from the account?"), &[&device])).await? {
        bail!("{}", tr(told("nothing was removed")));
    }
    let password = zeroize::Zeroizing::new(secret(&tr(told("Password of the account: ")), false)?);
    job(client, client.sync.revoke_device(device, &password).map_ok(|id| ((), id)), wait).await
}

pub fn status(reply: AccountReply) -> String {
    let Some(a) = reply.account else { return tr(told("no server is added\n")) };
    let encryption = match a.encrypted {
        Some(false) => tr(told("off: the server can read every record")),
        Some(true) => tr(told("end to end")),
        None => "-".into(),
    };
    let device = a.device.map_or("-".into(), |name| format!("{name} ({})", a.label));
    let storage = format!("{} of {}", bytes(a.used_bytes), bytes(a.quota_bytes));
    let comparison = a.pairing.map_or("-".into(), |p| {
        let name = a.devices.iter().find(|d| d.id == p.device).map_or(p.device.clone(), |d| d.name.clone());
        format!("{name} {}", p.code.unwrap_or("waiting".into()))
    });
    let pairs = [
        ("server", a.url),
        ("name", a.name),
        ("account", a.handle.unwrap_or("-".into())),
        ("device", device),
        ("state", state_note(text(&a.state))),
        ("encryption", encryption),
        (
            "certificate",
            a.pin.map_or_else(|| "-".into(), |pin| fill(told("pinned, fingerprint %1"), &[&pin])),
        ),
        ("synced", when(a.last_sync)),
        ("records", a.records.to_string()),
        ("storage", storage),
        ("warning", a.warning.unwrap_or("-".into())),
        ("error", a.last_error.unwrap_or("-".into())),
        (
            "access kept in",
            if a.file_secrets { tr(told("a private file")) } else { tr(told("the system keyring")) },
        ),
        ("link problem", a.link_error.unwrap_or("-".into())),
        ("quarantined", a.quarantined.to_string()),
        (&tr(told("not applied")), a.unapplied.to_string()),
        ("comparison", comparison),
        ("recovery", recovery(text(&a.recovery))),
        ("private files", format!("{} of {}", bytes(a.files_bytes), bytes(a.files_quota_bytes))),
    ];
    let offers = a.offers.into_iter().map(|o| {
        let fingerprint = o.fingerprints.first().map_or("-", |f| &f[..f.len().min(16)]).to_owned();
        (
            "repository to confirm",
            fill(
                told("%1 %2 %3 (fingerprint %4, id %5)"),
                &[&o.name, &o.url, &o.repo_id, &fingerprint, &o.id],
            ),
        )
    });
    let rows = a.devices.into_iter().map(|d| {
        let here = if d.current { tr(told("this computer")) } else { String::new() };
        vec![d.name, d.label.unwrap_or("-".into()), d.platform, when(d.seen_at), here]
    });
    show::lines(pairs) + &show::lines(offers) + &table(&["DEVICE", "NAME", "PLATFORM", "SEEN", ""], rows)
}

fn recovery(state: String) -> String {
    match state.as_str() {
        "stale" => "stale: a device that may know the passphrase was revoked; set a new, different one \
                    with wprs-ctl server escrow"
            .into(),
        _ => state,
    }
}

pub async fn approve(client: &Client, code: &str, yes: bool, wait: &Wait) -> Result<()> {
    let sync = &client.sync;
    let (_, found) = client.run_job(sync.preview_code(code).map_ok(|id| ((), id)), |_, _| {}).await?;
    let found = found.output.unwrap_or_default();
    let field = |key: &str| {
        found[key].as_str().filter(|t| !t.is_empty()).unwrap_or(&tr(told("not given"))).to_owned()
    };
    let requested = when(found["requested_at"].as_i64().map(|ms| ms / 1000));
    eprintln!("{}", tr(OWN));
    print!(
        "{}",
        show::lines([
            ("platform", field("platform")),
            ("software", field("client")),
            ("requested", requested)
        ])
    );
    if !yes && !confirm(&tr(told("Approve this device?"))).await? {
        bail!("{}", tr(told("not approved")));
    }
    let password = zeroize::Zeroizing::new(secret(&tr(told("Password of the account: ")), false)?);
    job(client, sync.approve_device(code, &password).map_ok(|id| ((), id)), wait).await
}

mod level;
mod linking;

pub use level::{Connect, Syncing, connect, syncing};
pub use linking::{join, share};
