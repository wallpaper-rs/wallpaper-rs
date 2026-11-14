use super::fade::{Fading, Gone};
use crate::power::Power;
use crate::stage::Stage;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use wprs_render::{Passing, Renderer};
use wprs_scene::{Quality, Ready, Way};

pub(super) struct Passage {
    passing: Option<Passing>,
    toward: Toward,
}

#[derive(PartialEq)]
enum Toward {
    Film(PathBuf),
    Stage(String),
    Image(String),
    Nothing,
}

fn plays(own: bool, power: Power) -> bool {
    crate::blend::passes(own, (power.saver, power.low, power.charging, power.lowest()))
}

fn step(stage: &Stage, way: Way) -> Option<(Arc<Ready>, Quality)> {
    let quality = stage.drawn_at();
    match stage.pack.scene.passages.way(way)? {
        Ok(ready) if ready.plays_at(quality) => Some((ready.clone(), quality)),
        Ok(_) => {
            log::info!(
                "the transition of the scene costs too much for quality {} and the plain fade plays",
                quality as u8
            );
            None
        }
        Err(why) => {
            log::warn!("the transition of the scene was left out and the plain fade plays: {why}");
            None
        }
    }
}

fn steps(gone: &Gone, coming: Option<&Stage>) -> Vec<(Arc<Ready>, Quality)> {
    let out = match gone {
        Gone::Scene(stage) => step(stage, Way::Out),
        _ => None,
    };
    out.into_iter().chain(coming.and_then(|stage| step(stage, Way::In))).collect()
}

#[derive(Clone, Copy)]
pub(super) struct Now<'a> {
    pub(super) film: Option<&'a PathBuf>,
    pub(super) stage: Option<&'a Stage>,
    pub(super) image: Option<&'a str>,
}

impl Now<'_> {
    fn toward(self) -> Toward {
        match (self.film, self.stage, self.image) {
            (Some(path), ..) => Toward::Film(path.clone()),
            (_, Some(stage), _) => Toward::Stage(stage.uid.clone()),
            (.., Some(item)) => Toward::Image(item.to_owned()),
            _ => Toward::Nothing,
        }
    }

    fn holds(self, toward: &Toward) -> bool {
        match (self.film, self.stage, self.image, toward) {
            (Some(path), _, _, Toward::Film(was)) => path == was,
            (None, Some(stage), _, Toward::Stage(was)) => stage.uid == *was,
            (None, None, Some(item), Toward::Image(was)) => item == was,
            (None, None, None, Toward::Nothing) => true,
            _ => false,
        }
    }
}

pub(super) fn pass(
    fading: Option<&mut Fading>,
    now: Now<'_>,
    (own, background): (bool, [u8; 3]),
    renderer: &Renderer,
    power: Power,
) -> bool {
    let Some(fading) = fading else { return true };
    if let Some(passage) = fading.passage.as_mut() {
        if !now.holds(&passage.toward) && passage.passing.take().is_some() {
            log::info!("the wallpaper changed during its transition and the plain fade goes on");
        }
        return passage.ready();
    }
    let coming = now.stage.filter(|_| now.film.is_none());
    let found = if plays(own, power) { steps(&fading.gone, coming) } else { Vec::new() };
    let mut passing = Passing::new(found, background);
    let wake: Arc<dyn Fn() + Send + Sync> = Arc::new(|| {});
    if let Some(Err(why)) = passing.as_mut().map(|p| p.begin(renderer, &wake)) {
        log::warn!("the transition of the scene was left out and the plain fade plays: {why}");
        passing = None;
    }
    fading.passage.insert(Passage { passing, toward: now.toward() }).ready()
}

impl Passage {
    fn ready(&mut self) -> bool {
        match self.passing.as_mut().map(|p| p.ready(Instant::now())) {
            Some(Err(why)) => {
                log::warn!("the transition of the scene was left out and the plain fade plays: {why}");
                self.passing = None;
                true
            }
            Some(Ok(done)) => done,
            None => true,
        }
    }

    pub(super) fn passing(&mut self) -> Option<&mut Passing> {
        self.passing.as_mut()
    }

    pub(super) fn drop_passing(&mut self) {
        self.passing = None;
    }

    pub(super) fn duration(&self) -> Option<f32> {
        self.passing.as_ref().map(Passing::duration)
    }
}
