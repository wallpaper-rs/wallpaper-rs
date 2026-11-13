use super::Session;
use crate::daemon::Daemon;
use crate::error::{Error, Result, failed};
use crate::jobs::now;
use wprs_core::model::BrowseSource;
use wprs_library::Item;

const MAX_LIBRARY: u32 = 2000;

pub fn moved(index: Option<usize>, len: usize, steps: i32) -> Option<usize> {
    let len = i64::try_from(len).ok().filter(|l| *l > 0)?;
    let steps = i64::from(steps);
    let at = match index {
        Some(i) => i as i64 + steps,
        None if steps == 0 => return None,
        None if steps > 0 => steps - 1,
        None => len + steps,
    };
    Some(at.rem_euclid(len) as usize)
}

fn browsable(item: &Item) -> bool {
    let kind = item.package.as_ref().map(|p| p.kind.as_str());
    !item.missing && item.meta.error.is_none() && kind != Some("assets")
}

impl Daemon {
    pub(super) fn travel(&self, output: &str, session: &mut Session, steps: i32) -> Result<()> {
        if session.source == BrowseSource::Playlist {
            let peeked = self.peek(output, session.steps + steps)?;
            session.steps += steps;
            session.playlist = Some((peeked.playlist, peeked.name));
            (session.candidate, session.position, session.count) =
                (peeked.item, peeked.position, peeked.count);
            return Ok(());
        }
        if session.items.is_empty() {
            return Err(failed("the library has no wallpaper to show"));
        }
        if let Some(index) = moved(session.index, session.items.len(), steps) {
            session.index = Some(index);
            session.candidate = Some(session.items[index].clone());
        }
        session.steps += steps;
        (session.position, session.count) =
            (session.index.unwrap_or_default() as u32, session.items.len() as u32);
        Ok(())
    }

    pub(super) fn browsable(&self) -> Result<Vec<String>> {
        let hide = self.state().config.library.hide_mature;
        let query = wprs_library::shown("", hide);
        let items = self.store.read(|lib| lib.search(&query, now(), 0, MAX_LIBRARY))?;
        Ok(items.into_iter().filter(browsable).map(|i| i.uid).collect())
    }

    pub(super) fn shown_item(&self, output: &str) -> Option<String> {
        let (playing, image) = {
            let st = self.state();
            let name = match output {
                "" => st.outputs.keys().filter(|o| !crate::drafts::virtual_output(o)).min()?.clone(),
                name => name.to_owned(),
            };
            let playing = st.playing.get(&name).filter(|_| st.config.resolve(&name).playlist.is_some());
            (playing.and_then(|p| p.item.clone()), st.config.resolve(&name).image)
        };
        if playing.is_some() {
            return playing;
        }
        let path = image?.to_str()?.to_owned();
        self.store.read(|lib| Ok::<_, Error>(lib.resolve_item(&path).ok())).ok().flatten()
    }

    pub(super) fn targets(&self, output: &str) -> Vec<String> {
        if !output.is_empty() {
            return vec![output.to_owned()];
        }
        let st = self.state();
        st.outputs.keys().filter(|o| !crate::drafts::virtual_output(o)).cloned().collect()
    }
}
