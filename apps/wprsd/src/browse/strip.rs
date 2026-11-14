use super::Session;
use crate::daemon::Daemon;
use wprs_core::TransitionKind;
use wprs_core::model::{BrowsePlays, BrowseSource, ItemSummary};

const AHEAD: i32 = 2;

fn scene(item: Option<&ItemSummary>) -> bool {
    item.and_then(|i| i.package.as_ref()).is_some_and(|p| p.kind == "scene")
}

impl Daemon {
    pub(super) fn item_summary(&self, uid: &str) -> Option<ItemSummary> {
        let item = self.store.read(|lib| lib.item(uid)).ok()?;
        self.summary(item).ok()
    }

    pub(super) fn ahead(&self, output: &str, session: &Session) -> Vec<ItemSummary> {
        let uids: Vec<String> = match session.source {
            BrowseSource::Playlist => {
                (1..=AHEAD).filter_map(|n| self.peek(output, session.steps + n).ok()?.item).collect()
            }
            _ => (1..=AHEAD)
                .filter_map(|n| {
                    let at = super::list::moved(session.index, session.items.len(), n)?;
                    session.items.get(at).cloned()
                })
                .collect(),
        };
        let mut seen = vec![session.candidate.clone().unwrap_or_default()];
        let mut ahead = Vec::new();
        for uid in uids {
            if !seen.contains(&uid) {
                ahead.extend(self.item_summary(&uid));
                seen.push(uid);
            }
        }
        ahead
    }

    pub(super) fn plays(
        &self,
        output: &str,
        session: &Session,
        candidate: Option<&ItemSummary>,
    ) -> BrowsePlays {
        let st = self.state();
        let config = &st.config;
        let (kind, why) = match session.transition.as_str() {
            "fade" => (TransitionKind::Fade, "chosen"),
            "none" => (TransitionKind::None, "chosen"),
            _ if config.own_transition(output).is_some() => (config.transition(output), "screen"),
            _ => (config.transition(output), "general"),
        };
        let kind = match kind {
            TransitionKind::Own if !scene(candidate) => TransitionKind::Fade,
            kind => kind,
        };
        let calm = self.compositor.calmed();
        let (kind, why) = match config.transition.less_motion.reduces(calm) {
            true if kind != TransitionKind::None => {
                (TransitionKind::Fade, if calm { "calm" } else { "still" })
            }
            _ => (kind, why),
        };
        let duration_ms = if kind == TransitionKind::Fade { config.transition.duration_ms } else { 0 };
        BrowsePlays { kind: kind.as_str().into(), why: why.into(), duration_ms }
    }
}
