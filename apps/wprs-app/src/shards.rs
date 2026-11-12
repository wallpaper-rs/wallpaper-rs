use crate::backend::qobject::{Backend, reap_deleted};
use cxx_qt_lib::QString;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub const CLAIMS: &str = "WPRS_APP_CHECK_CLAIMS";
pub const SHARDS: &str = "WPRS_APP_CHECK_SHARDS";
const MARK: &str = "wprs-check ";
const MOST: usize = 3;

pub fn count() -> usize {
    let cores = std::thread::available_parallelism().map_or(1, usize::from);
    let wanted = std::env::var(SHARDS).ok().and_then(|n| n.parse::<usize>().ok());
    wanted.unwrap_or(cores / 4).clamp(1, MOST)
}

pub fn claim(unit: &str) -> bool {
    let dir = std::env::var_os(CLAIMS);
    let Some(taken) = claim_in(dir.as_deref().map(Path::new), unit) else { return true };
    eprintln!("{MARK}{} {unit}", if taken { "ran" } else { "seen" });
    taken
}

pub fn claim_in(dir: Option<&Path>, unit: &str) -> Option<bool> {
    Some(std::fs::create_dir(dir?.join(unit)).is_ok())
}

#[derive(Default)]
pub struct Census {
    ran: BTreeMap<String, usize>,
    seen: BTreeSet<String>,
}

impl Census {
    pub fn note(&mut self, line: &str) {
        let Some(rest) = line.strip_prefix(MARK) else { return };
        match rest.split_once(' ') {
            Some(("ran", unit)) => {
                *self.ran.entry(unit.to_owned()).or_default() += 1;
                self.seen.insert(unit.to_owned());
            }
            Some(("seen", unit)) => {
                self.seen.insert(unit.to_owned());
            }
            _ => {}
        }
    }

    pub fn problems(&self) -> Vec<String> {
        let skipped = self.seen.iter().filter(|unit| !self.ran.contains_key(*unit));
        let twice = self.ran.iter().filter(|(_, n)| **n > 1).map(|(unit, _)| unit);
        let skipped = skipped.map(|unit| format!("no process ran the check unit {unit}"));
        skipped.chain(twice.map(|unit| format!("two processes ran the check unit {unit}"))).collect()
    }

    pub fn units(&self) -> usize {
        self.seen.len()
    }
}

impl Backend {
    pub fn claim(&self, unit: &QString) -> bool {
        claim(&unit.to_string())
    }

    pub fn reap(&self) {
        reap_deleted();
    }
}
