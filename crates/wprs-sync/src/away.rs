use crate::{Local, Replica, View, kind};
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};
use wprs_core::model::{Away, AwayProfile};

type Record<'a> = (&'a String, View, &'a str);

fn views<'a>(replica: &'a Replica, prefix: &'a str) -> Vec<Record<'a>> {
    let named = replica.bases.iter().filter(move |(key, _)| key.starts_with(prefix));
    named.filter_map(|(key, (_, doc))| Some((key, doc.view()?, kind(doc)))).collect()
}

fn entries(view: &View) -> impl Iterator<Item = &str> {
    let entries = view.iter().filter(|(field, _)| field.starts_with("e:"));
    entries.filter_map(|(_, entry)| entry.get("i")?.as_str())
}

fn screens<'a>(view: &'a View, kind: &str) -> impl Iterator<Item = (&'a String, &'a str)> {
    let outputs = view.get("outputs").and_then(Value::as_object).filter(|_| kind == "profile");
    let outputs = outputs.into_iter().flatten();
    outputs.filter_map(|(output, section)| Some((output, section.get("item")?.as_str()?)))
}

fn presets<'a>((_, view, kind): &'a Record<'_>) -> Option<&'a str> {
    view.get("item")?.as_str().filter(|_| *kind == "preset")
}

pub fn away(
    replica: &Replica,
    local: &Local,
    links: &BTreeMap<String, String>,
    offered: &HashSet<String>,
    added: &HashSet<String>,
) -> Vec<Away> {
    let (library, playlists, settings) =
        (views(replica, "library/"), views(replica, "playlists/"), views(replica, "settings/"));
    let item = |(key, ..): &Record<'_>| key.trim_start_matches("library/").to_owned();
    let titles: BTreeMap<String, &str> = library
        .iter()
        .map(|r| (item(r), r.1.get("title").and_then(Value::as_str).unwrap_or_default()))
        .collect();
    let stored: HashSet<String> = library.iter().filter(|r| r.1.contains_key("blob")).map(item).collect();
    let repos: BTreeMap<String, &str> =
        library.iter().filter_map(|r| Some((item(r), r.1.get("repo")?.as_str()?))).collect();
    let named = playlists.iter().flat_map(|(_, view, _)| entries(view));
    let shown = settings.iter().flat_map(|(_, view, kind)| screens(view, kind).map(|(_, item)| item));
    let asked = settings.iter().filter_map(presets);
    let wanted = titles.keys().map(String::as_str).chain(named).chain(shown).chain(asked);
    let mut found: BTreeMap<String, Away> = BTreeMap::new();
    for key in
        wanted.filter(|k| !local.items.contains_key(*k) && !stored.contains(*k) && !offered.contains(*k))
    {
        let title = titles.get(key).copied().unwrap_or_default().to_owned();
        let repo = repos.get(key).filter(|repo| !added.contains(**repo)).map(|repo| (*repo).to_owned());
        found.entry(key.to_owned()).or_insert_with(|| Away {
            key: key.to_owned(),
            title,
            playlists: Vec::new(),
            profiles: Vec::new(),
            repo,
        });
    }
    for (key, view, _) in &playlists {
        for item in entries(view) {
            let Some(away) = found.get_mut(item) else { continue };
            let uid = links.get(*key).filter(|uid| !away.playlists.contains(uid));
            away.playlists.extend(uid.cloned());
        }
    }
    for (key, view, kind) in &settings {
        for (output, item) in screens(view, kind) {
            let on = links.get(*key).zip(found.get_mut(item));
            on.into_iter().for_each(|(profile, away)| {
                away.profiles.push(AwayProfile { profile: profile.clone(), output: output.clone() });
            });
        }
    }
    let mut found: Vec<Away> = found.into_values().collect();
    found.sort_by(|a, b| a.title.cmp(&b.title).then(a.key.cmp(&b.key)));
    found
}
