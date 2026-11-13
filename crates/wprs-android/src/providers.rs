use crate::account::NETWORK;
use crate::{Error, library, said, settings, store};
use serde::Deserialize;
use serde_json::{Value, json};

mod build;
mod kinds;
mod options;
mod run;
mod saved;
mod tidy;

use kinds::{Options, hosts, kind, schema, secret_of};
use options::effective;
use saved::{Entry, find, held, load, save};

pub(crate) use tidy::tag;

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub(crate) enum Request {
    Providers,
    ProviderAdd {
        kind: String,
        #[serde(default)]
        options: Options,
        #[serde(default)]
        secret: String,
        #[serde(default)]
        on: bool,
    },
    ProviderSet {
        id: String,
        key: String,
        value: String,
    },
    ProviderSecret {
        id: String,
        secret: String,
    },
    ProviderOn {
        id: String,
        on: bool,
    },
    ProviderRemove {
        id: String,
        purge: bool,
    },
    ProviderSync {
        id: String,
        force: bool,
    },
}

pub(crate) fn request(request: Request) -> Result<Value, Error> {
    match request {
        Request::Providers => listed(),
        Request::ProviderAdd { kind, options, secret, on } => add(&kind, &options, &secret, on),
        Request::ProviderSet { id, key, value } => set(&id, &key, &value),
        Request::ProviderSecret { id, secret } => {
            let _held = held();
            find(&mut load(), &id)?;
            saved::keep(&id, secret.trim()).map(|()| Value::Null)
        }
        Request::ProviderOn { id, on } => switch(&id, on),
        Request::ProviderRemove { id, purge } => remove(&id, purge),
        Request::ProviderSync { id, force } => run::sync(&id, force),
    }
}

fn plain(entry: &Entry) -> bool {
    ["url", "server"]
        .iter()
        .filter_map(|key| entry.options.get(*key))
        .any(|v| v.trim().starts_with("http://"))
}

fn shown(entry: &Entry, now: i64) -> Result<Value, Error> {
    let known = kind(&entry.kind)?;
    let count = library::with(|lib| Ok(lib.count(&format!("tag:{}", tag(&entry.id)), now)?))?;
    let secret = secret_of(known).map(|(name, required)| json!({"name": name, "required": required, "held": saved::secret(&entry.id).is_some()}));
    Ok(
        json!({"id": entry.id, "kind": entry.kind, "options": entry.options, "on": entry.on, "last_sync": entry.last_sync,
        "last_error": entry.last_error, "hosts": hosts(known, &entry.options), "count": count, "secret": secret, "plain": plain(entry),
        "credits": entry.credits.iter().map(|c| json!({"uid": c.uid, "title": c.title, "attribution": c.attribution,
            "license": c.license, "link": c.link})).collect::<Vec<_>>()}),
    )
}

fn listed() -> Result<Value, Error> {
    let now = library::now();
    let entries: Result<Vec<Value>, Error> = load().iter().map(|e| shown(e, now)).collect();
    Ok(json!({"kinds": schema(), "providers": entries?, "network": settings::chosen(NETWORK)}))
}

fn fresh_id(entries: &[Entry], kind: &str) -> String {
    let taken = |id: &str| entries.iter().any(|e| e.id == id);
    let named = (1..).map(|n| if n == 1 { kind.to_owned() } else { format!("{kind}-{n}") });
    named.into_iter().find(|id| !taken(id)).unwrap_or_default()
}

fn add(name: &str, given: &Options, secret: &str, on: bool) -> Result<Value, Error> {
    let known = kind(name)?;
    let options = effective(known, given)?;
    if secret_of(known).is_some_and(|(_, required)| required) && secret.trim().is_empty() {
        return Err(said("android.provider_secret", "%1 needs its key or password first", &[&name]));
    }
    let _held = held();
    let mut entries = load();
    let id = fresh_id(&entries, name);
    entries.push(Entry { id: id.clone(), kind: name.to_owned(), options, on, ..Entry::default() });
    save(&entries)?;
    saved::keep(&id, secret.trim())?;
    Ok(json!(id))
}

fn set(id: &str, key: &str, value: &str) -> Result<Value, Error> {
    let _held = held();
    let mut entries = load();
    let entry = find(&mut entries, id)?;
    let known = kind(&entry.kind)?;
    let before = hosts(known, &entry.options);
    let mut given = entry.options.clone();
    given.insert(key.to_owned(), value.to_owned());
    entry.options = effective(known, &given)?;
    if hosts(known, &entry.options) != before {
        entry.on = false;
        saved::keep(id, "")?;
    }
    save(&entries)?;
    Ok(Value::Null)
}

fn switch(id: &str, on: bool) -> Result<Value, Error> {
    let _held = held();
    let mut entries = load();
    find(&mut entries, id)?.on = on;
    save(&entries)?;
    Ok(Value::Null)
}

fn remove(id: &str, purge: bool) -> Result<Value, Error> {
    let _held = held();
    let mut entries = load();
    find(&mut entries, id)?;
    entries.retain(|e| e.id != id);
    save(&entries)?;
    saved::keep(id, "")?;
    if purge {
        let now = library::now();
        library::with(|lib| {
            let mine = lib.search(&format!("tag:{}", tag(id)), now, 0, u32::MAX)?;
            let uids: Vec<String> = mine.into_iter().map(|i| i.uid).collect();
            library::drop_items(lib, &uids, now).map(drop)
        })?;
    }
    store::changed();
    Ok(Value::Null)
}
