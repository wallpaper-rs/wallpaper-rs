use super::SceneView;
use wgpu::TextureFormat;
use wprs_scene::Frame;

const PAST: &str = "this device lights at most 8 lights; the lights past the 8th are not drawn";
const FLAT: &str = "this device has no blendable half-float target, so the particles do not refract";

impl SceneView {
    pub(super) fn note_dropped(&mut self, frame: &Frame<'_>, cap: usize) {
        let flat = self.state_format != TextureFormat::Rgba16Float && frame.quality.bend().is_some();
        let swarms = self.swarms.as_ref().filter(|_| flat);
        let now = || {
            let past = frame.lamps.iter().skip(cap).map(|l| (l.layer.id.as_str(), PAST));
            past.chain(swarms.into_iter().flat_map(|s| s.bent()).map(|id| (id, FLAT)))
        };
        if now().eq(self.dropped.iter().map(|(id, why)| (id.as_str(), *why))) {
            return;
        }
        let fresh = now().map(|(id, why)| (id.to_owned(), why)).collect();
        self.dropped = fresh;
    }
}
