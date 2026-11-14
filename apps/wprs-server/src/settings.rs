use crate::config::{Publish, Registration};
use crate::error::Res;
use crate::{App, Config};
use rusqlite::{Connection, OptionalExtension, params};
use std::collections::HashMap;
use std::sync::PoisonError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Network {
    Home,
    Own { cert: String, key: String },
}

impl Network {
    pub fn served(&self) -> bool {
        match self {
            Network::Home => true,
            Network::Own { cert, .. } => !cert.is_empty(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    Closed,
    Invite,
    Open,
}

impl Access {
    pub const ALL: [Access; 3] = [Access::Closed, Access::Invite, Access::Open];

    pub fn value(self) -> &'static str {
        match self {
            Access::Closed => "closed",
            Access::Invite => "invite",
            Access::Open => "open",
        }
    }

    pub fn parse(value: &str) -> Option<Access> {
        Access::ALL.into_iter().find(|a| a.value() == value)
    }

    pub fn publish(self) -> Publish {
        if self == Access::Open { Publish::Trusted } else { Publish::All }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    pub registration: Registration,
    pub auto_publish: Publish,
    pub network: Network,
    pub network_chosen: bool,
    pub advertise: bool,
    pub done: bool,
    pub key_pending: bool,
    pub admin_handle: Option<String>,
    pub repo_name: Option<String>,
}

impl Settings {
    pub fn access(&self) -> Access {
        match self.registration {
            Registration::Closed => Access::Closed,
            Registration::Invite => Access::Invite,
            Registration::Open => Access::Open,
        }
    }
}

fn registration(value: &str) -> Option<Registration> {
    Some(match value {
        "closed" => Registration::Closed,
        "invite" => Registration::Invite,
        "open" => Registration::Open,
        _ => return None,
    })
}

pub fn parse_publish(value: &str) -> Option<Publish> {
    Some(match value {
        "never" => Publish::Never,
        "trusted" => Publish::Trusted,
        "all" => Publish::All,
        _ => return None,
    })
}

pub fn publish_value(p: Publish) -> &'static str {
    match p {
        Publish::Never => "never",
        Publish::Trusted => "trusted",
        Publish::All => "all",
    }
}

const HANDLE: &str =
    "SELECT handle FROM users WHERE id = (SELECT CAST(value AS INTEGER) FROM settings WHERE key = 'admin')";

pub fn load(conn: &Connection, cfg: &Config) -> anyhow::Result<Settings> {
    let mut statement = conn.prepare("SELECT key, value FROM settings")?;
    let stored: HashMap<String, String> =
        statement.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect::<Result<_, _>>()?;
    let get = |key: &str| stored.get(key).map(String::as_str);
    let network = match get("network") {
        Some("home") => Some(Network::Home),
        Some(_) => Some(Network::Own {
            cert: get("cert_path").unwrap_or_default().into(),
            key: get("key_path").unwrap_or_default().into(),
        }),
        None => None,
    };
    let started =
        if cfg.self_signed { Network::Home } else { Network::Own { cert: "".into(), key: "".into() } };
    Ok(Settings {
        registration: get("registration").and_then(registration).unwrap_or(cfg.registration),
        auto_publish: get("auto_publish").and_then(parse_publish).unwrap_or(cfg.auto_publish),
        advertise: match &network {
            Some(n) => *n == Network::Home,
            None => cfg.advertise,
        },
        network_chosen: network.is_some(),
        network: network.unwrap_or(started),
        done: get("setup") == Some("done"),
        key_pending: get("root_key") == Some("pending"),
        repo_name: get("repo_name").map(str::to_owned),
        admin_handle: conn.query_row(HANDLE, [], |r| r.get(0)).optional()?,
    })
}

pub fn put(conn: &Connection, pairs: &[(&str, &str)]) -> Res<()> {
    let sql = "INSERT INTO settings (key, value) VALUES (?1, ?2) \
               ON CONFLICT (key) DO UPDATE SET value = excluded.value";
    for (key, value) in pairs {
        conn.execute(sql, params![key, value])?;
    }
    Ok(())
}

pub fn drop_key(conn: &Connection, key: &str) -> Res<()> {
    conn.execute("DELETE FROM settings WHERE key = ?1", [key])?;
    Ok(())
}

pub fn access_pairs(access: Access) -> [(&'static str, &'static str); 2] {
    [("registration", access.value()), ("auto_publish", publish_value(access.publish()))]
}

impl App {
    pub fn settings(&self) -> Settings {
        self.settings.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub async fn change(&self, work: impl FnOnce(&Connection) -> Res<()> + Send + 'static) -> Res<Settings> {
        let cfg = self.cfg.clone();
        let fresh = self.db.run(move |c| {
            work(c)?;
            load(c, &cfg).map_err(crate::error::Problem::internal)
        });
        let fresh = fresh.await?;
        *self.settings.write().unwrap_or_else(PoisonError::into_inner) = fresh.clone();
        Ok(fresh)
    }

    pub async fn set_access(&self, access: Access) -> Res<Settings> {
        self.change(move |c| put(c, &access_pairs(access))).await
    }
}
