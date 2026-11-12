use crate::daemon::{Daemon, output_name};
use crate::error::invalid;
use anyhow::{Context, Result, ensure};
use std::path::Path;
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

mod again;
pub(crate) mod media;
mod pack;
mod page;
mod probe;

pub(crate) use page::Drawn;

const LIFE: Duration = Duration::from_secs(60);
const MAX_DOCUMENT: usize = 9 << 20;
const DECODED: u64 = 512 << 20;
const PREVIEW: &str = "preview:";

pub fn virtual_output(name: &str) -> bool {
    name.starts_with(PREVIEW)
}

pub(crate) fn previews(
    status: &std::collections::BTreeMap<String, crate::compositor::OutputStatus>,
    drafted: impl Fn(&str) -> bool,
) -> std::collections::BTreeMap<String, wprs_core::model::PreviewState> {
    let shown = status.iter().filter(|(name, s)| {
        (virtual_output(name) || drafted(name)) && (s.error.is_some() || !s.console.is_empty())
    });
    let state = |s: &crate::compositor::OutputStatus| wprs_core::model::PreviewState {
        error: s.error.clone(),
        console: s.console.clone(),
    };
    shown.map(|(name, s)| (name.clone(), state(s))).collect()
}

pub(crate) fn layers<'a>(
    config: &'a wprs_core::Config,
    tried: Option<&'a crate::sceneprops::Stored>,
    output: &str,
    package: &str,
) -> Vec<&'a crate::sceneprops::Stored> {
    tried.map_or_else(|| config.scene_values(output, package), |t| vec![t])
}

pub(crate) fn forget_previews(config: &mut wprs_core::Config) -> bool {
    let before = config.outputs.len();
    config.outputs.retain(|name, _| !virtual_output(name));
    config.outputs.len() != before
}

pub(crate) fn save_config(config: &wprs_core::Config, path: &Path) -> Result<(), wprs_core::ConfigError> {
    let mut kept = config.clone();
    forget_previews(&mut kept);
    kept.save(path)
}

pub(crate) struct Draft {
    pub(crate) pack: Drawn,
    pub(crate) hold: Option<f64>,
    pub(crate) values: Option<crate::sceneprops::Stored>,
    document: String,
    trusted: bool,
    at: Instant,
}

fn kind(path: &str) -> String {
    Path::new(path).extension().and_then(|e| e.to_str()).unwrap_or_default().to_ascii_lowercase()
}

impl Daemon {
    pub fn preview_scene(
        self: &Arc<Self>,
        output: &str,
        dir: &Path,
        (document, trusted): (&str, bool),
        time: f64,
    ) -> Result<()> {
        let outputs: Vec<String> = match output.trim() {
            "" => self.state().outputs.keys().filter(|o| !virtual_output(o)).cloned().collect(),
            name => vec![output_name(name)?],
        };
        ensure!(document.len() <= MAX_DOCUMENT, invalid("the scene is too large"));
        let (document, simulated) = media::split(document);
        let (document, tried) = media::values(document);
        let document = document.as_str();
        let simulated = simulated.map(|m| media::frame(dir, m).map(Arc::new)).transpose()?;
        let hold =
            (time >= 0.0 && time.is_finite()).then_some(time.min(f64::from(wprs_scene::MAX_POSTER_TIME)));
        let pack = match document.trim() {
            "" => None,
            _ => {
                let dir =
                    std::fs::canonicalize(dir).with_context(|| format!("cannot find {}", dir.display()))?;
                let web = page::wanted(document);
                let found = self
                    .state()
                    .drafts
                    .values()
                    .find(|d| {
                        let same =
                            d.document == document || (!web && again::restarted(&d.document, document));
                        same && d.pack.path() == dir && d.trusted == trusted
                    })
                    .map(|d| d.pack.clone());
                let made = |this: &Arc<Self>| match web {
                    true => this.draft_page(&dir, document).map(|p| Drawn::Page(Arc::new(p))),
                    false => this.draft_pack(&dir, document, trusted).map(|p| Drawn::Scene(Arc::new(p))),
                };
                Some(found.map_or_else(|| made(self), Ok)?)
            }
        };
        let (started, again) = {
            let mut st = self.state();
            let (mut started, mut again) = (false, Vec::new());
            for output in &outputs {
                match &pack {
                    Some(pack) => {
                        let old = st.drafts.get(output);
                        let kept =
                            old.is_some_and(|d| d.pack.same(pack) && d.hold == hold && d.values == tried);
                        if kept && old.is_some_and(|d| again::restarted(&d.document, document)) {
                            again.push(output.clone());
                        }
                        let (pack, document, at) = (pack.clone(), document.into(), Instant::now());
                        let draft = Draft { pack, hold, values: tried.clone(), document, trusted, at };
                        started |= st.drafts.insert(output.clone(), draft).is_none();
                    }
                    None => drop(st.drafts.remove(output)),
                }
            }
            (started, again)
        };
        let simulated = simulated.filter(|_| pack.is_some());
        outputs.iter().for_each(|o| crate::media::simulate(o, simulated.clone()));
        if started {
            tokio::spawn(expire(Arc::downgrade(self)));
        }
        for output in &outputs {
            match again.contains(output) {
                true => self.compositor.restart_particles(output),
                false => drop(self.configure_output(output)),
            }
        }
        self.compositor.settled();
        Ok(())
    }
}

async fn expire(daemon: Weak<Daemon>) {
    while let Some(d) = daemon.upgrade() {
        let now = Instant::now();
        let (gone, next) = {
            let mut st = d.state();
            let gone: Vec<String> =
                st.drafts.iter().filter(|(_, x)| x.at + LIFE <= now).map(|(o, _)| o.clone()).collect();
            gone.iter().for_each(|o| drop(st.drafts.remove(o)));
            (gone, st.drafts.values().map(|x| x.at + LIFE).min())
        };
        gone.iter().for_each(|o| crate::media::simulate(o, None));
        if !gone.is_empty() {
            let daemon = d.clone();
            tokio::task::spawn_blocking(move || gone.iter().for_each(|o| drop(daemon.configure_output(o))));
        }
        let Some(next) = next else { return };
        drop(d);
        tokio::time::sleep_until(next.into()).await;
    }
}

#[cfg(test)]
mod tests;
