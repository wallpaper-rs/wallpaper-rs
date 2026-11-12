use anyhow::Result;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

#[derive(Default)]
pub(super) struct Files {
    pub(super) used: Vec<String>,
    made: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
}

impl Files {
    pub(super) fn undo(&self) {
        self.made.iter().for_each(|f| drop(std::fs::remove_file(f)));
        self.dirs.iter().rev().for_each(|d| drop(std::fs::remove_dir(d)));
    }
}

fn placed(dir: &Path, from: &Path, path: &str, files: &mut Files) -> Result<String> {
    wprs_format::check_path(path)?;
    let bytes = std::fs::read(from.join(path))?;
    let (stem, ext) = path.rsplit_once('.').map_or((path, ""), |(s, e)| (s, e));
    for n in 1..10_000 {
        let name = if n == 1 { path.to_owned() } else { format!("{stem}_{n}.{ext}") };
        let target = dir.join(&name);
        let same = std::fs::read(&target).ok().map(|old| old == bytes);
        match same {
            Some(true) if !files.used.contains(&name) => return Ok(name),
            None if !files.used.contains(&name) => {
                let parent = target.parent().unwrap_or(dir);
                let mut absent: Vec<PathBuf> =
                    parent.ancestors().take_while(|a| !a.exists()).map(Path::to_path_buf).collect();
                absent.reverse();
                std::fs::create_dir_all(parent)?;
                files.dirs.extend(absent);
                std::fs::write(&target, &bytes)?;
                files.made.push(target);
                files.used.push(name.clone());
                return Ok(name);
            }
            _ => {}
        }
    }
    Ok(path.to_owned())
}

pub(super) fn moved(dir: &Path, from: &Path, v: &mut Value, keys: &[&str], files: &mut Files) -> Result<()> {
    for key in keys {
        if let Some(path) = v[*key].as_str().map(String::from) {
            v[*key] = json!(placed(dir, from, &path, files)?);
        }
    }
    Ok(())
}
