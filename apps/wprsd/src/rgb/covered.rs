use super::{Ask, LIGHTS};
use crate::compositor::lock;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;
use wprs_core::rgb::Covered;

static COVERED: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());
static WANTED: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());
static RESTING: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());
static WAITING: Mutex<BTreeMap<String, &'static str>> = Mutex::new(BTreeMap::new());

fn mark(set: &Mutex<BTreeSet<String>>, source: &str, on: bool) {
    let mut set = lock(set);
    let changed = if on { set.insert(source.into()) } else { set.remove(source) };
    drop(set);
    if let Some(l) = LIGHTS.get().filter(|_| changed) {
        l.asks.send(Ask::Covered).ok();
    }
}

pub(crate) fn cover(source: &str, covered: bool) {
    mark(&COVERED, source, covered);
}

pub(crate) fn want(source: &str, wanted: bool) {
    mark(&WANTED, source, wanted);
}

pub(crate) fn rest(source: &str, resting: bool) {
    mark(&RESTING, source, resting);
}

pub(crate) fn note(source: &str, why: Option<&'static str>) {
    let mut waiting = lock(&WAITING);
    match why {
        Some(why) if waiting.get(source) != Some(&why) => drop(waiting.insert(source.into(), why)),
        None if waiting.contains_key(source) => drop(waiting.remove(source)),
        _ => {}
    }
}

pub(crate) fn waiting(asleep: &'static str) -> BTreeMap<String, &'static str> {
    let waiting = lock(&WAITING);
    waiting.iter().map(|(k, v)| (k.clone(), if *v == "asleep" { asleep } else { v })).collect()
}

pub(crate) fn resting(source: &str) -> bool {
    lock(&RESTING).contains(source)
}

pub(crate) fn covered(source: &str) -> bool {
    lock(&COVERED).contains(source)
}

fn mode() -> Option<Covered> {
    LIGHTS.get().map(|l| lock(&l.status).rgb.when_covered)
}

pub(crate) fn wanted() -> bool {
    !lock(&WANTED).is_empty()
}

pub(crate) fn follows() -> bool {
    let receiving = LIGHTS.get().is_some_and(|l| lock(&l.status).receiving());
    super::enabled() && mode() == Some(Covered::Follow) && receiving
}

pub(crate) fn paused(source: &str) -> bool {
    mode().is_none_or(|m| m == Covered::Pause) && covered(source)
}

#[cfg(test)]
mod tests {
    #[test]
    fn covered_lights_say_why_they_wait_with_the_reason_the_session_sleeps() {
        super::note("TEST-WHY", Some("asleep"));
        super::note("TEST-WHY-IMAGE", Some("colorless"));
        let waiting = super::waiting("locked");
        assert_eq!(
            (waiting.get("TEST-WHY"), waiting.get("TEST-WHY-IMAGE")),
            (Some(&"locked"), Some(&"colorless"))
        );
        super::note("TEST-WHY", None);
        super::note("TEST-WHY-IMAGE", None);
        assert!(
            !super::waiting("paused").keys().any(|k| k.starts_with("TEST-WHY")),
            "a reason outlived its output"
        );
    }
}
