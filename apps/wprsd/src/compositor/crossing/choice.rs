use super::super::Command;
use super::super::output::{Output, Shown};
use super::super::status::Passage;
use super::{Passing, Pipe};
use std::sync::{Arc, mpsc};
use wprs_core::TransitionKind;
use wprs_render::Renderer;
use wprs_scene::{Quality, Ready, Way};

pub(in crate::compositor) fn choose(
    old: &Shown,
    new: &Shown,
    kind: TransitionKind,
    reduced: bool,
) -> (Option<Passing>, Passage) {
    match kind {
        TransitionKind::None => return (None, Passage { plays: "none", why: "chosen" }),
        TransitionKind::Fade => return (None, Passage { plays: "fade", why: "chosen" }),
        TransitionKind::Own => {}
    }
    let found: Vec<_> =
        [(old, Way::Out), (new, Way::In)].into_iter().filter_map(|(s, w)| own(s, w)).collect();
    let refused = found.iter().any(Result::is_err);
    let steps: Vec<_> = found.into_iter().flatten().collect();
    let why = match () {
        _ if steps.is_empty() && refused => "fallen",
        _ if steps.is_empty() => "plain",
        _ if reduced => "reduced",
        _ => {
            let pipes: Vec<Pipe> = Vec::new();
            let passing = Passing { steps, pipes, built: None, background: new.background };
            return (Some(passing), Passage { plays: "own", why: "own" });
        }
    };
    (None, Passage { plays: "fade", why })
}

fn own(shown: &Shown, way: Way) -> Option<Result<(Arc<Ready>, Quality), ()>> {
    let scene = shown.scene.as_ref()?;
    match scene.spec.pack.scene.passages.way(way)? {
        Ok(ready) => Some(Ok((ready.clone(), scene.drawn_at()))),
        Err(why) => {
            tracing::warn!("the transition of the scene was left out and the plain fade plays: {why}");
            Some(Err(()))
        }
    }
}

impl Output {
    pub(in crate::compositor) fn begin(
        &mut self,
        passing: Option<Passing>,
        told: Passage,
        transition: bool,
        (renderer, tx): (&Renderer, &mpsc::Sender<Command>),
    ) {
        let mut passing = passing.filter(|_| self.fade.is_some());
        self.told = match self.fade {
            Some(_) => told,
            None if transition => Passage::CUT,
            None => Passage { plays: "none", why: "chosen" },
        };
        let tx = tx.clone();
        let wake: Arc<dyn Fn() + Send + Sync> = Arc::new(move || drop(tx.send(Command::Wake)));
        if let Some(Err(why)) = passing.as_mut().map(|p| p.begin(renderer, &wake)) {
            tracing::warn!("the transition of the scene was left out and the plain fade plays: {why}");
            (passing, self.told) = (None, Passage::FALLEN);
        }
        self.passing = passing;
    }
}
