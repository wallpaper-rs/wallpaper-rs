use super::super::item::{Item, Rect, scale_of};
use super::super::paint::{Painter, Target};
use super::super::{Op, Src};
use wprs_scene::{Affine, Content, Lean, Quality};

const MIRROR: f32 = 24.0;
const STEP: f32 = 64.0;

fn plane(lean: Lean, origin: [f32; 2], (canvas, distance): ([f32; 2], f32), screen: Affine) -> [f32; 4] {
    let center = canvas.map(|v| v / 2.0);
    let k = scale_of(screen);
    let at = |q: [f32; 2]| {
        let (s, w) = (lean.apply(q), lean.depth(q).max(1e-3));
        let flat = [0, 1].map(|i| center[i] + (s[i] - center[i]) * w);
        let px = screen.apply(flat);
        [px[0], px[1], k * distance * (1.0 - w)]
    };
    let [o, x, y] = [origin, [origin[0] + STEP, origin[1]], [origin[0], origin[1] + STEP]].map(at);
    let (u, v) = ([0, 1, 2].map(|i| x[i] - o[i]), [0, 1, 2].map(|i| y[i] - o[i]));
    let c = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    let long = c.iter().map(|v| v * v).sum::<f32>().sqrt();
    if long < 1e-9 {
        return [0.0; 4];
    }
    let sign = if c[2] < 0.0 { -1.0 } else { 1.0 };
    let n = c.map(|v| v * sign / long);
    if n[2] < 1e-3 {
        return [0.0; 4];
    }
    [n[0], n[1], n[2], o[2] + (n[0] * o[0] + n[1] * o[1]) / n[2]]
}

impl Painter<'_, '_, '_> {
    pub(crate) fn lit(&self, i: usize) -> bool {
        let d = &self.frame.draws[i];
        d.lit.is_some()
            && matches!(
                d.content,
                Content::Image { .. }
                    | Content::Video { .. }
                    | Content::Text { .. }
                    | Content::Mesh { .. }
                    | Content::Model { .. }
            )
    }

    pub(crate) fn baking(&self, i: usize) -> bool {
        self.frame.quality == Quality::Q0 && self.lit(i)
    }

    pub(crate) fn shifting(&self, range: std::ops::Range<usize>) -> bool {
        let glow = &self.pool.glow;
        (glow.moved || glow.live) && self.frame.draws[range].iter().any(|d| d.lit.is_some())
    }

    fn map_of(&mut self, id: Option<&str>) -> Option<(Src, [f32; 2])> {
        let images = self.images;
        let found = id.and_then(|id| self.moving(id).or_else(|| images.get(id).map(|i| &i.0)))?;
        Some((self.ext(found), found.crop))
    }

    fn base_crop(&self, i: usize) -> [f32; 2] {
        match &self.frame.draws[i].content {
            Content::Image { asset, .. } | Content::Video { asset, .. } => {
                self.picture(asset).map_or([1.0; 2], |image| image.crop)
            }
            _ => [1.0; 2],
        }
    }

    pub(crate) fn light(
        &mut self,
        i: usize,
        entry: &'static str,
        item: &mut Item,
        (m, bounds): (Affine, Rect),
        target: Target,
    ) -> Option<(&'static str, [Src; 8], Src)> {
        let base = if entry == "fs_picture" { self.base_crop(i) } else { [1.0; 2] };
        let entry = match entry {
            "fs_picture" => "fs_lit_picture",
            "fs_texture" => "fs_lit_texture",
            "fs_text" => "fs_lit_text",
            _ => return None,
        };
        let l = self.frame.draws[i].lit.filter(|_| self.lit(i))?;
        item.lit = [l.strength, l.roughness, l.reflection, l.emissive];
        let [a, b, c, d, ..] = m.0;
        let (x, y) = (a.hypot(b).max(1e-6), c.hypot(d).max(1e-6));
        item.lit_basis = [a / x, b / x, c / y, d / y];
        item.lit_at = [target.rect.x, target.rect.y, 1.0, MIRROR * scale_of(self.screen)];
        let d = &self.frame.draws[i];
        let view = (self.frame.canvas, self.frame.distance);
        item.lit_plane = d.lean.map_or([0.0; 4], |l| plane(l, d.world.apply([0.0; 2]), view, self.screen));
        let found = [l.normal, l.roughness_map, l.emissive_map].map(|id| self.map_of(id));
        let mut maps = [Src::White; 8];
        maps[4..].copy_from_slice(&self.gobos());
        let mut scales = [[1.0; 2]; 4];
        for (k, map) in found.into_iter().enumerate() {
            item.lit_maps[k] = f32::from(u8::from(map.is_some()));
            let (src, crop) = map.unwrap_or((Src::White, base));
            (maps[k], scales[k]) = (src, [crop[0] / base[0], crop[1] / base[1]]);
        }
        let mut behind = Src::Clear;
        if l.reflection > 0.0 {
            match self.map_of(l.env) {
                Some((env, crop)) => (maps[3], item.lit_maps[3], scales[3]) = (env, 1.0, crop),
                None => {
                    let (x, y) = (bounds.x - target.rect.x, bounds.y - target.rect.y);
                    if let Some(back) = self.acquire(bounds.bucket().size()) {
                        let at = [0, 0];
                        let (origin, size) = ([x as u32, y as u32], bounds.size());
                        self.ops.push(Op::Copy { from: target.src, to: back, origin, size, at });
                        let (w, h) = self.size_of(back);
                        item.lit_env = [-x / w as f32, -y / h as f32, 1.0 / w as f32, 1.0 / h as f32];
                        (maps[3], item.lit_maps[3], behind) = (back, 2.0, back);
                    }
                }
            }
        }
        let [[a, b], [c, d], [e, f], [g, h]] = scales;
        (item.lit_scale, item.lit_more) = ([a, b, c, d], [e, f, g, h]);
        Some((entry, maps, behind))
    }
}
