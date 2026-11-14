use super::Engine;
use super::passage::Passage;
use crate::gpu;
use crate::power::Power;
use crate::settings::{self, Look};
use crate::stage::Stage;
use crate::{blend, choice};
use std::path::PathBuf;
use std::sync::Arc;
use wprs_core::Fit;
use wprs_render::GpuImage;

pub(super) const FILM: &str = "\u{0}film";
pub(super) const GONE: &str = "\u{0}gone";

pub(crate) enum Gone {
    Picture(Arc<GpuImage>, (Fit, Option<[f32; 4]>)),
    Scene(Box<Stage>),
    Film(PathBuf, (Fit, [u8; 3])),
}

pub(crate) struct Fading {
    pub(super) gone: Gone,
    pub(super) start: Option<i64>,
    pub(super) asked: Option<i64>,
    pub(super) passage: Option<Passage>,
}

impl Fading {
    pub(super) fn new(gone: Gone) -> Self {
        Fading { gone, start: None, asked: None, passage: None }
    }
}

pub(super) fn direct(look: &Look) -> bool {
    matches!(look.fit, Fit::Fill | Fit::Stretch) && look.flip == [false; 2] && look.grade.is_none()
}

impl Engine {
    pub(crate) fn video(&mut self) -> Option<String> {
        self.forget_old_gpu();
        let shown = choice::shown_on(self.preview, self.lock).filter(|shown| shown.video);
        let flip = shown.as_ref().map_or([false; 2], |s| s.flip);
        let reel = crate::reel::shown(shown.as_ref());
        self.dirty |= reel != self.reel;
        self.reel = reel;
        let shown = shown.map(|shown| shown.path);
        let current = self.direct.clone().or_else(|| self.film.clone());
        let look = settings::Look { flip, ..settings::look(self.lock) };
        if shown.is_some() && shown == current && look != self.look {
            (self.look, self.dirty) = (look, true);
        } else if shown != current {
            let fps = Power::now().fps(self.preview);
            let film = (look.fit, look.background);
            let gone = match (self.direct.take().or(self.film.take()), shown.is_some()) {
                (Some(path), _) => Some(Gone::Film(path, film)),
                (None, true) => match (self.image.take(), self.stage.take()) {
                    (Some((_, image)), _) => Some(Gone::Picture(image, (self.look.fit, self.crop))),
                    (_, Some(stage)) => stage.image().is_some().then(|| Gone::Scene(Box::new(stage))),
                    _ => None,
                },
                (None, false) => None,
            };
            let fades =
                |gone: &Gone| blend::fades(look.fade, fps, self.hidden, matches!(gone, Gone::Film(..)));
            self.fading = match gone {
                Some(gone) if fades(&gone) => Some(Fading::new(gone)),
                Some(_) => None,
                None => self.fading.take(),
            };
            (self.film, self.look, self.dirty, self.still, self.over) = (shown, look, true, None, false);
        }
        let plain = direct(&settings::Look { flip, ..settings::look(self.lock) }) && self.reel.direct();
        if self.film.is_some() && self.fading.is_none() && plain {
            self.direct = self.film.take();
            (self.target, self.image, self.raised, self.stage) = (None, None, None, None);
            gpu::trim();
        } else if self.direct.is_some() && !plain {
            (self.film, self.dirty) = (self.direct.take(), true);
        }
        if self.video != self.direct.is_some() {
            (self.video, self.checked, self.dirty) = (self.direct.is_some(), (0, 0, (0, 0)), true);
        }
        if self.direct.is_some() {
            self.hint(self.reel.cap());
        }
        self.unreport();
        self.direct.as_ref().map(|path| format!("file://{}", path.display()))
    }

    pub(crate) fn reel(&self) -> Option<String> {
        self.direct.as_ref().or(self.film.as_ref()).map(|_| self.reel.told().to_string())
    }

    pub(crate) fn film(&self) -> Option<String> {
        self.film.as_ref().and_then(|path| path.to_str()).map(String::from)
    }

    pub(super) fn reels(&self) -> Vec<(String, PathBuf)> {
        let own = self.stage.as_ref().map(|stage| stage.videos().to_vec()).unwrap_or_default();
        let gone = match self.fading.as_ref().map(|f| &f.gone) {
            Some(Gone::Film(path, _)) => Some((GONE.to_owned(), path.clone())),
            _ => None,
        };
        own.into_iter().chain(gone).chain(self.film.clone().map(|path| (FILM.to_owned(), path))).collect()
    }
}
