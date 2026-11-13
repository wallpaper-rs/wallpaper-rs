use super::known;
use crate::{Error, said, settings, store};
use serde_json::{Value, json};
use std::collections::HashSet;
use wprs_library::{Item, Library, Rules};

pub(crate) fn remove(lib: &mut Library, uids: &[String], now: i64) -> Result<Value, Error> {
    let refs = known(lib, uids);
    let hashes = lib.hashed()?.into_iter().filter(|(i, _)| refs.contains(&i.uid.as_str()));
    let removed: Vec<String> = hashes.map(|(_, hash)| wprs_sync::removed_blob(&hash)).collect();
    let items = lib.remove_items(&refs)?;
    crate::keeping::forget(lib, &items)?;
    crate::state::forget(removed, now * 1000)?;
    items.iter().try_for_each(store::forget)?;
    items.iter().for_each(crate::converted::forget);
    let gone =
        |o: &wprs_core::config::OutputConfig| items.iter().any(|item| o.image.as_ref() == Some(&item.path));
    let config = settings::config();
    if gone(&config.default) || config.outputs.values().any(gone) {
        settings::update(|config| {
            let sections = std::iter::once(&mut config.default).chain(config.outputs.values_mut());
            sections.filter(|o| gone(o)).for_each(|o| o.image = None);
            config.outputs.retain(|_, o| *o != Default::default());
            Ok(())
        })?;
    }
    Ok(Value::Null)
}

pub(super) fn playlists(lib: &Library, now: i64) -> Result<Value, Error> {
    let (config, away) = (settings::config(), crate::state::read(|s| s.away.clone()));
    let on = |uid: &str, screen: &str| config.resolve(screen).playlist.as_deref() == Some(uid);
    let listed = lib.playlists()?.into_iter().map(|p| {
        let count = lib.playlist_items(&p.uid, now)?.len();
        let (home, lock) = (on(&p.uid, ""), on(&p.uid, crate::choice::LOCK));
        let away: Vec<&str> = away.iter().filter(|a| a.playlists.contains(&p.uid)).map(|a| a.title.as_str()).collect();
        Ok(json!({ "count": count, "current": home || lock, "home": home, "lock": lock, "playlist": p, "away": away }))
    });
    Ok(Value::Array(listed.collect::<Result<_, Error>>()?))
}

pub(super) fn add(lib: &mut Library, playlist: &str, items: &[String]) -> Result<Value, Error> {
    let listed = |item: &str| lib.item_playlists(item).is_ok_and(|p| p.iter().any(|p| p.uid == playlist));
    let mut seen = HashSet::new();
    let fresh: Vec<&str> = known(lib, items).into_iter().filter(|i| seen.insert(*i) && !listed(i)).collect();
    Ok(json!(lib.add_to_playlist(playlist, &fresh)?))
}

pub(super) fn rotate(lib: &mut Library, name: &str, items: &[String], every: &str) -> Result<Value, Error> {
    let rules: Rules = serde_json::from_value(json!({ "interval": every }))?;
    if rules.interval.is_zero() {
        return Err(said("android.rotation_interval", "give a time longer than zero", &[]));
    }
    let names: Vec<String> = lib.playlists()?.into_iter().map(|p| p.name).collect();
    let (mut chosen, mut number) = (name.to_owned(), 1);
    while names.contains(&chosen) {
        number += 1;
        chosen = format!("{name} {number}");
    }
    let uid = lib.create_playlist(&chosen, None, &rules)?;
    lib.add_to_playlist(&uid, &known(lib, items))?;
    crate::choice::pick(Some(crate::choice::Choice::Playlist(uid.clone())));
    Ok(json!(uid))
}

pub(super) fn listed(items: Vec<Item>) -> Result<Value, Error> {
    let listed = items.into_iter().map(|item| {
        let exposes = item.package.as_ref().map(crate::page::exposes);
        let thumb = store::thumb_file(&item.uid, "jpg").ok().filter(|path| path.is_file());
        let mut value = serde_json::to_value(item)?;
        if let Some(exposes) = exposes {
            value["package"]["exposes"] = json!(exposes);
        }
        value["thumb"] = json!(thumb);
        Ok(value)
    });
    listed.collect::<Result<_, Error>>().map(Value::Array)
}

pub(crate) fn drop_playlist(lib: &mut Library, playlist: &str) -> Result<(), Error> {
    lib.remove_playlist(playlist)?;
    let config = settings::config();
    let listed = |o: &wprs_core::config::OutputConfig| o.playlist.as_deref() == Some(playlist);
    if listed(&config.default) || config.outputs.values().any(listed) {
        settings::update(|config| {
            let sections = std::iter::once(&mut config.default).chain(config.outputs.values_mut());
            sections.filter(|o| listed(o)).for_each(|o| o.playlist = None);
            config.outputs.retain(|_, o| *o != Default::default());
            Ok(())
        })?;
    }
    Ok(())
}

pub(super) fn sound(
    lib: &mut Library,
    uids: &[String],
    (muted, volume): (Option<String>, Option<String>),
) -> Result<Value, Error> {
    let parsed = sound_words(muted.as_deref(), volume.as_deref())?;
    crate::own_sound::forget();
    Ok(json!(lib.set_sound_many(&known(lib, uids), parsed.0, parsed.1)?))
}

fn word(text: Option<&str>) -> Option<&str> {
    text.map(str::trim).filter(|t| !t.is_empty())
}

type Words = (Option<Option<bool>>, Option<Option<u8>>);

pub(super) fn sound_words(muted: Option<&str>, volume: Option<&str>) -> Result<Words, Error> {
    let bad = |word: &str| said("android.sound_value", "%1 is not a sound value", &[&word]);
    let muted = match word(muted) {
        None => None,
        Some("none") => Some(None),
        Some(other) => Some(Some(other.parse::<bool>().map_err(|_| bad(other))?)),
    };
    let volume = match word(volume) {
        None => None,
        Some("none") => Some(None),
        Some(other) => Some(Some(other.parse::<u8>().ok().filter(|v| *v <= 100).ok_or_else(|| bad(other))?)),
    };
    Ok((muted, volume))
}

pub(super) fn heard(lib: &Library, path: &str) -> Result<Value, Error> {
    let (muted, volume) = lib.sound_where("path", path)?;
    Ok(json!(crate::own_sound::factor(muted, volume)))
}
