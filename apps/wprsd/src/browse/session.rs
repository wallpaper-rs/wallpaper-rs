use super::{IDLE, Session, Told, source};
use crate::apply::target_output;
use crate::compositor::lock;
use crate::daemon::Daemon;
use crate::error::{Result, failed, not_found};
use crate::rotation::Act;
use std::time::Instant;
use wprs_core::model::{BrowseEnd, BrowseSource};

impl Daemon {
    pub fn browse_move(&self, output: &str, from: &str, steps: i32) -> Result<Told> {
        let (output, from) = (self.browse_output(&target_output(output)?), source(from)?);
        let mut sessions = lock(&self.rotations.browse.sessions);
        let kept = sessions.remove(&output).filter(|s| from.is_none_or(|f| f == s.source));
        let fresh = kept.is_none();
        let mut session = match kept {
            Some(session) => session,
            None => self.begin(&output, from)?,
        };
        self.travel(&output, &mut session, steps)?;
        session.until = Instant::now() + IDLE;
        let json = self.browse_reply(&output, Some(&session), None)?;
        let serial = fresh.then_some(session.serial);
        sessions.insert(output.clone(), session);
        Ok(Told { output, json, fresh: serial })
    }

    pub fn browse_apply(&self, output: &str, transition: &str) -> Result<Told> {
        let output = self.browse_output(&target_output(output)?);
        let mut sessions = lock(&self.rotations.browse.sessions);
        let session = sessions.get_mut(&output).ok_or_else(|| nothing(&output))?;
        let word = if transition.trim().is_empty() { session.transition.clone() } else { transition.into() };
        let held = self.transition_of(&word)?;
        session.until = Instant::now() + IDLE;
        let mut session = session.clone();
        let outputs = self.targets(&output);
        match session.source {
            BrowseSource::Playlist => {
                let steps = session.steps;
                let before = session.candidate.clone();
                self.travel(&output, &mut session, 0)?;
                if session.candidate != before {
                    sessions.insert(output.clone(), session);
                    return Err(failed(
                        "the playlist moved on while browsing; look at the new candidate first",
                    ));
                }
                held.inspect(|d| self.rotations.browse.hold(&outputs, *d));
                if steps != 0 {
                    self.show(Act::Step(std::slice::from_ref(&output), steps));
                }
            }
            _ => {
                let uid =
                    session.candidate.clone().ok_or_else(|| failed("there is no candidate to apply yet"))?;
                held.inspect(|d| self.rotations.browse.hold(&outputs, *d));
                self.apply(&uid, &output, "", "")?;
            }
        }
        sessions.remove(&output);
        drop(sessions);
        let json = self.browse_reply(&output, None, Some(BrowseEnd::Applied))?;
        Ok(Told { output, json, fresh: None })
    }

    pub fn browse_cancel(&self, output: &str) -> Result<Told> {
        let output = self.browse_output(&target_output(output)?);
        let ended = lock(&self.rotations.browse.sessions).remove(&output).map(|_| BrowseEnd::Cancelled);
        let json = self.browse_reply(&output, None, ended)?;
        Ok(Told { output, json, fresh: None })
    }

    pub fn get_browse(&self, output: &str) -> Result<String> {
        let output = self.browse_output(&target_output(output)?);
        let session = lock(&self.rotations.browse.sessions).get(&output).cloned();
        self.browse_reply(&output, session.as_ref(), None)
    }

    pub fn browse_transition(&self, output: &str, transition: &str) -> Result<Told> {
        let output = self.browse_output(&target_output(output)?);
        self.transition_of(transition)?;
        let mut sessions = lock(&self.rotations.browse.sessions);
        let session = sessions.get_mut(&output).ok_or_else(|| nothing(&output))?;
        (session.transition, session.until) = (transition.trim().into(), Instant::now() + IDLE);
        let json = self.browse_reply(&output, Some(session), None)?;
        Ok(Told { output, json, fresh: None })
    }

    fn begin(&self, output: &str, from: Option<BrowseSource>) -> Result<Session> {
        let playing = !output.is_empty() && self.state().assigned().iter().any(|(o, ..)| o == output);
        let source = from.unwrap_or(if playing { BrowseSource::Playlist } else { BrowseSource::Library });
        if source == BrowseSource::Playlist && !playing {
            let screen = if output.is_empty() { "every screen" } else { output };
            return Err(not_found(format!("{screen} shows no playlist; give a screen with one")));
        }
        let shown = self.shown_item(output);
        let items = match source {
            BrowseSource::Library => self.browsable()?,
            _ => Vec::new(),
        };
        let index = shown.as_ref().and_then(|s| items.iter().position(|u| u == s));
        Ok(Session {
            source,
            playlist: None,
            items,
            index,
            steps: 0,
            candidate: None,
            shown,
            position: 0,
            count: 0,
            transition: String::new(),
            until: Instant::now() + IDLE,
            serial: self.rotations.browse.next_serial(),
        })
    }
}

fn nothing(output: &str) -> crate::error::Error {
    let screen = if output.is_empty() { "every screen" } else { output };
    not_found(format!("nothing is being browsed on {screen}"))
}
