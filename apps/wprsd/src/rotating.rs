use crate::daemon::Daemon;
use crate::error::{Result, invalid};
use crate::jobs::now;
use crate::playlists::set_rule;
use crate::scan::local;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use wprs_library::Rules;

#[cfg(test)]
mod tests;

const ROTATION: &str = "Rotation";

pub(crate) fn follows(dir: &Path) -> Result<String> {
    let text = dir.to_string_lossy();
    if text.contains('"') {
        return Err(invalid(format!("{text} has a quote in its name and cannot be used as a folder")));
    }
    Ok(format!("folder:\"{text}\" -kind:package"))
}

fn taken(names: &[String], name: &str) -> String {
    let mut chosen = name.to_owned();
    let mut number = 1;
    while names.iter().any(|n| n.eq_ignore_ascii_case(&chosen)) {
        number += 1;
        chosen = format!("{name} {number}");
    }
    chosen
}

impl Daemon {
    fn folder_known(self: &Arc<Self>, path: &str) -> Result<PathBuf> {
        let given = local(path)?;
        let dir = std::fs::canonicalize(&given).ok().filter(|dir| dir.is_dir());
        let dir = dir.ok_or_else(|| invalid(format!("{path} is not an existing folder")))?;
        let sources = self.store.read(|lib| lib.sources())?;
        let covered = sources.iter().any(|s| s.path == dir || (s.recursive && dir.starts_with(&s.path)));
        if !covered {
            let uid = self.register_source(path, true).or_else(|_| self.register_source(path, false))?;
            self.store.connect()?.scan(&uid, now())?;
            self.library_changed(Vec::new());
            self.analyze_job();
        }
        Ok(dir)
    }

    pub fn add_folder(self: &Arc<Self>, playlist: &str, path: &str, follow: bool) -> Result<u32> {
        let dir = self.folder_known(path)?;
        let query = follows(&dir)?;
        let info = self.store.read(|lib| lib.playlist(playlist))?;
        let held = || self.store.read(|lib| lib.playlist_items(&info.uid, now()));
        if follow {
            let same = info.query.as_deref() == Some(query.as_str());
            if !same && (info.query.is_some() || !held()?.is_empty()) {
                return Err(invalid(
                    "a playlist that follows a folder holds nothing else: make a new playlist",
                ));
            }
            self.set_query(playlist, &query)?;
        } else {
            if info.query.is_some() {
                return Err(invalid("this playlist follows a search: make a list playlist to add a folder"));
            }
            let found = self.store.read(|lib| lib.search(&query, now(), 0, u32::MAX))?;
            let uids: Vec<&str> = found.iter().map(|item| item.uid.as_str()).collect();
            self.edit_entries(playlist, |lib, uid| lib.add_to_playlist(uid, &uids))?;
        }
        self.store.read(|lib| lib.count(&query, now()))
    }

    pub fn rotate_playlist(
        self: &Arc<Self>,
        name: &str,
        uids: &[String],
        (folder, follow): (&str, bool),
        every: &str,
        output: &str,
    ) -> Result<String> {
        if uids.is_empty() && folder.is_empty() {
            return Err(invalid("choose the pictures or the folder to rotate"));
        }
        if follow && folder.is_empty() {
            return Err(invalid("only a folder can be followed"));
        }
        if follow && !uids.is_empty() {
            return Err(invalid("a playlist that follows a folder holds nothing else"));
        }
        if set_rule(&Rules::default(), "interval", every)?.interval.is_zero() {
            return Err(invalid("give a time longer than zero to change the picture"));
        }
        let wanted = match (name.trim(), Path::new(folder).file_name()) {
            ("", Some(last)) => last.to_string_lossy().into_owned(),
            ("", None) => ROTATION.to_owned(),
            (name, _) => name.to_owned(),
        };
        let names: Vec<String> =
            self.store.read(|lib| lib.playlists())?.into_iter().map(|p| p.name).collect();
        let uid = self.create_playlist(&taken(&names, &wanted), "")?;
        let filled = || -> Result<()> {
            let refs: Vec<&str> = uids.iter().map(String::as_str).collect();
            if !refs.is_empty() {
                self.edit_entries(&uid, |lib, key| lib.add_to_playlist(key, &refs))?;
            }
            if !folder.is_empty() {
                self.add_folder(&uid, folder, follow)?;
            }
            self.set_playlist_rule(&uid, "interval", every)?;
            self.start(&uid, output)
        };
        filled().inspect_err(|_| drop(self.delete_playlist(&uid)))?;
        Ok(uid)
    }
}
