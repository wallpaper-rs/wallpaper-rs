mod account;
mod admin;
mod approval;
mod assets;
mod auth;
mod authors;
mod backup;
mod cases;
mod catalog;
mod certs;
mod checks;
mod config;
mod creds;
mod db;
mod device;
mod discovery;
mod door;
mod entry;
mod error;
mod featuring;
mod files;
mod frame;
mod health;
mod hiding;
mod http;
mod icons;
mod invite;
mod jobs;
pub mod lan;
mod lang;
mod layout;
mod legal;
mod mail;
mod mailbox;
mod me;
mod mod_case;
mod mod_form;
mod mod_more;
mod mod_newer;
mod mod_page;
mod moderation;
mod net;
#[cfg(feature = "oidc")]
mod oidc;
mod own;
mod pages;
mod people;
mod policies;
pub mod qr;
mod records;
mod repo;
mod result;
mod review;
mod served;
pub mod settings;
mod setup;
#[cfg(all(test, feature = "oidc"))]
mod signon;
#[cfg(feature = "oidc")]
mod sso;
mod sync;
#[cfg(test)]
mod tests;
mod ui;
mod units;
mod upload;
mod validate;
mod visibility;
mod votes;
mod web;

pub use backup::{backup, restore};
pub use config::Config;
pub use lang::missed_texts;
pub use records::SYNC_FORMAT;
pub use repo::{import, init, status};
pub use validate::check;

use axum::serve::{Listener, ListenerExt};
use std::fs::DirBuilder;
use std::net::SocketAddr;
use std::os::unix::fs::DirBuilderExt;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify, Semaphore};

type St = axum::extract::State<Arc<App>>;
type Router = axum::Router<Arc<App>>;
type Reply = error::Res<axum::Json<serde_json::Value>>;

pub const VERSION: &str = match option_env!("WPRS_VERSION") {
    Some(v) if !v.is_empty() => v,
    _ => concat!(env!("CARGO_PKG_VERSION"), "-local"),
};

pub struct App {
    pub cfg: Config,
    db: db::Db,
    repo: repo::Repo,
    limits: net::Limiter,
    hub: mailbox::Hub,
    clients: net::ClientLog,
    validations: Semaphore,
    hashing: Semaphore,
    dirty: Notify,
    mail: mail::Post,
    deciding: Mutex<()>,
    pieces: files::Pieces,
    settings: std::sync::RwLock<settings::Settings>,
    door: door::Door,
    held: std::sync::Mutex<setup::Held>,
    #[cfg(feature = "oidc")]
    oidc: oidc::Oidc,
}

impl App {
    pub fn open(cfg: Config) -> anyhow::Result<Arc<App>> {
        for dir in ["db", "blobs", "thumbs", "held", "tmp", "keys", "repo", "logs", "backups"] {
            DirBuilder::new().recursive(true).mode(0o700).create(cfg.data.join(dir))?;
        }
        let db = db::Db::open(&cfg.data.join("db/wprs.sqlite"))?;
        let settings = db.read(|c| settings::load(c, &cfg))?;
        Ok(Arc::new(App {
            door: door::Door::new(&cfg, &settings.network)?,
            settings: settings.into(),
            held: Default::default(),
            db,
            repo: repo::Repo::new(&cfg.data),
            limits: net::Limiter::new(cfg.rate_limits),
            hub: mailbox::Hub::default(),
            clients: net::ClientLog::new(&cfg),
            validations: Semaphore::new(2),
            hashing: Semaphore::new(4),
            dirty: Notify::new(),
            mail: mail::Post::default(),
            deciding: Mutex::new(()),
            pieces: files::Pieces::default(),
            #[cfg(feature = "oidc")]
            oidc: oidc::Oidc::default(),
            cfg,
        }))
    }

    pub fn router(self: &Arc<App>) -> axum::Router {
        http::router(self.clone())
    }

    pub async fn serve<L: Listener<Addr = SocketAddr>>(self: Arc<App>, listener: L) -> anyhow::Result<()> {
        self.announce().await?;
        jobs::spawn(self.clone());
        mail::spawn(self.clone());
        let service = self.router().into_make_service_with_connect_info::<SocketAddr>();
        let listener = listener.tap_io(|_| {});
        Ok(axum::serve(listener, service).with_graceful_shutdown(shutdown()).await?)
    }

    pub async fn publish(self: &Arc<App>) -> anyhow::Result<Option<u64>> {
        let published = jobs::republish(self, true).await.map_err(|e| anyhow::anyhow!(e.detail))?;
        Ok(published.map(|p| p.version))
    }
}

async fn shutdown() {
    let mut term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).ok();
    let term = async move {
        match term.as_mut() {
            Some(signal) => drop(signal.recv().await),
            None => std::future::pending().await,
        }
    };
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        () = term => {}
    }
}
