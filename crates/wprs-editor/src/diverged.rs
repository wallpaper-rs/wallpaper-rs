use crate::project::{Project, read, stamp};
use anyhow::{Result, ensure};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use wprs_core::message::said;

fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        match entry.file_type()?.is_dir() {
            true => copy_tree(&entry.path(), &target)?,
            false => drop(fs::copy(entry.path(), &target)?),
        }
    }
    Ok(())
}

fn walk(root: &Path, prefix: &str, hasher: &mut impl Hasher) {
    let mut found: Vec<_> = fs::read_dir(root.join(prefix)).into_iter().flatten().flatten().collect();
    found.sort_by_key(|e| e.file_name());
    for item in found {
        let name = item.file_name().to_string_lossy().into_owned();
        let path = if prefix.is_empty() { name } else { format!("{prefix}/{name}") };
        match item.metadata() {
            Ok(m) if m.is_dir() => walk(root, &path, hasher),
            Ok(m) => (path, m.len(), m.modified().ok()).hash(hasher),
            Err(_) => path.hash(hasher),
        }
    }
}

impl Project {
    pub fn disk_stamp(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        walk(&self.dir, "", &mut hasher);
        hasher.finish()
    }

    pub fn follow_disk(&mut self) -> Result<bool> {
        match self.diverged() && !self.dirty() && self.pending.is_none() {
            true => self.take_theirs().map(|_| true),
            false => Ok(false),
        }
    }

    pub fn diverged(&self) -> bool {
        read(&self.dir).map(|d| stamp(&d)).unwrap_or_default() != self.base
    }

    pub fn keep_mine(&mut self) -> Result<()> {
        self.ready()?;
        self.base = read(&self.dir).map(|d| stamp(&d)).unwrap_or_default();
        self.save()
    }

    pub fn take_theirs(&mut self) -> Result<()> {
        let disk = read(&self.dir)?;
        self.base = stamp(&disk);
        self.doc = disk;
        self.history = Default::default();
        self.pending = None;
        self.history.mark_saved();
        self.persist()
    }

    pub fn save_elsewhere(&mut self, to: &Path) -> Result<()> {
        self.ready()?;
        let empty = !to.exists() || fs::read_dir(to)?.next().is_none();
        ensure!(empty, said("editor.not_empty_folder", "%1 is not an empty folder", &[&(to.display())]));
        copy_tree(&self.dir, to)?;
        let from = std::mem::replace(&mut self.dir, to.to_owned());
        self.base = read(to).map(|d| stamp(&d)).unwrap_or_default();
        let saved = self.save();
        if saved.is_err() {
            self.dir = from;
        }
        saved
    }
}
