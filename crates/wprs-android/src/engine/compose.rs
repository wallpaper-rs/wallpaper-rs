use super::fade::{FILM, GONE, Gone, direct};
use super::passage::{Now, pass};
use super::{Engine, FRAMES};
use crate::power::{Power, delay_ms};
use crate::relief::{Look as Seen, layer};
use crate::{blend, gpu, settings};
use std::sync::atomic::Ordering::Relaxed;
use wprs_core::Fit;
use wprs_render::RenderError;

impl Engine {
    pub(super) fn compose(&mut self, nanos: i64, fps: u32, power: Power) -> Result<(u32, i64), RenderError> {
        let reels = self.reels();
        let Some((gpu, surface)) = self.target.as_mut() else { return Ok((0, 0)) };
        let (r, film) = (&gpu.renderer, self.film.as_deref());
        let (headroom, relit) = match (film, &self.stage) {
            (Some(film), _) => crate::hdr::light(r, surface, power.charging, crate::media::bright(film))?,
            (None, None) => (1.0, crate::hdr::dark(r, surface)?),
            (None, Some(_)) => (1.0, false),
        };
        self.dirty |= relit;
        self.clips.take(r, (gpu.epoch, headroom));
        let new = reels.iter().any(|(_, path)| self.clips.fresh(path));
        let since = match self.still {
            Some(at) if !film.is_some_and(|film| self.clips.fresh(film)) => at,
            _ => nanos,
        };
        self.still = Some(since);
        let frames = self.clips.shown(&reels);
        let reel = |id: &str| frames.iter().find(|(key, _)| *key == id).map(|(_, image)| *image);
        let size = self.size;
        let (fit, background, flip, grade) =
            (self.look.fit, self.look.background, self.look.flip, self.look.grade);
        let look = Seen { fit, background, pan: self.shown, crop: self.crop, flip, grade };
        let stretched = Seen { fit: Fit::Stretch, crop: None, ..look };
        if let Some(stage) = self.stage.as_mut() {
            let r = &gpu.renderer;
            let tier = crate::power::tier(r.software(), r.compute()).min(r.tier());
            (stage.reports, stage.measured) =
                ((!self.preview).then_some(usize::from(self.lock)), self.measured);
            stage.render(r, (size, nanos, fps), (power, tier), &frames, true)?;
        }
        let image = self.image.as_ref().map(|(key, _)| key.item.as_str());
        let shows = Now { film: self.film.as_ref(), stage: self.stage.as_ref(), image };
        let own = (self.look.own, self.look.background);
        let built = pass(self.fading.as_mut(), shows, own, &gpu.renderer, power);
        let (now, now_look) = match (&self.film, &self.stage, &self.image) {
            (Some(_), ..) => (reel(FILM), look),
            (_, Some(stage), _) => (stage.image(), stretched),
            (.., Some((_, image))) => (Some(image.as_ref()), look),
            _ => (None, look),
        };
        let had = match self.fading.as_ref().map(|f| &f.gone) {
            Some(Gone::Picture(..)) => true,
            Some(Gone::Scene(stage)) => stage.image().is_some(),
            Some(Gone::Film(..)) => reel(GONE).is_some(),
            None => false,
        };
        let passage = self.fading.as_ref().and_then(|f| f.passage.as_ref()?.duration());
        let duration = passage.map_or(self.look.fade.as_nanos() as f32, |s| s * 1e9).max(1.0);
        let progress = self.fading.as_mut().map_or(1.0, |fading| {
            let asked = *fading.asked.get_or_insert(nanos);
            let ready = built && blend::begins(had, now.is_some(), nanos - asked);
            let start = fading.start.or(ready.then_some(nanos));
            fading.start = start;
            start.map_or(0.0, |start| ((nanos - start) as f32 / duration).clamp(0.0, 1.0))
        });
        let (fading, waiting) =
            (self.fading.is_some(), self.fading.as_ref().is_some_and(|f| f.start.is_none()));
        let (gone, passage) = match self.fading.as_mut() {
            Some(f) => (Some(&f.gone), f.passage.as_mut()),
            None => (None, None),
        };
        let (was, was_look) = match gone {
            Some(Gone::Picture(image, (fit, crop))) => {
                (Some(image.as_ref()), Seen { fit: *fit, crop: *crop, ..look })
            }
            Some(Gone::Scene(stage)) => (stage.image(), stretched),
            Some(Gone::Film(_, (fit, background))) => {
                (reel(GONE), Seen { fit: *fit, background: *background, crop: None, ..look })
            }
            None => (None, look),
        };
        let mut layers = match fading {
            true => [layer(was, 1.0, &was_look, size, None), layer(now, progress, &now_look, size, None)],
            false => [layer(now, 1.0, &now_look, size, None), layer(None, 0.0, &now_look, size, None)],
        };
        let shown = &mut layers[usize::from(fading)];
        if self.film.is_some() && self.reel.framed() {
            shown.uv = shown.uv.map(|uv| uv.framed(self.reel.zoom, self.reel.pan));
        }
        let mut failed = false;
        if !waiting && (now.is_some() || was.is_some()) && (new || self.dirty || fading) {
            let crossed = match passage.and_then(|p| p.passing()).filter(|_| was.is_some() && now.is_some()) {
                Some(passing) => match passing.frame(&gpu.renderer, (layers[0], layers[1]), progress, size) {
                    Ok(crossed) => Some([crossed, layer(None, 0.0, &now_look, size, None)]),
                    Err(why) => {
                        log::warn!("the transition of the scene stopped and the plain fade plays: {why}");
                        (failed, self.dirty) = (true, true);
                        None
                    }
                },
                None => None,
            };
            let drawn = crossed.as_ref().unwrap_or(&layers);
            self.dirty = !gpu.renderer.present_layers(surface, size, drawn, || {})?;
            if !self.dirty {
                self.presented = nanos;
                FRAMES.fetch_add(1, Relaxed);
            }
        }
        if let Some(passage) = self.fading.as_mut().and_then(|f| f.passage.as_mut()).filter(|_| failed) {
            passage.drop_passing();
        }
        if self.fading.is_some() && (fps == 0 || progress >= 1.0) {
            self.fading = None;
            let flip = self.look.flip;
            self.handover = self.film.is_some()
                && self.reel.direct()
                && direct(&settings::Look { flip, ..settings::look(self.lock) });
            gpu::trim();
        }
        let ended = blend::rests(self.reel.once, now.is_some(), self.over, nanos - since);
        let busy = self.fading.is_some() || (self.film.is_some() && power.moving() && !ended);
        if !busy || fps == 0 {
            self.still = None;
        }
        Ok(if busy && fps > 0 { (fps, delay_ms(fps)) } else { (0, 0) })
    }
}
