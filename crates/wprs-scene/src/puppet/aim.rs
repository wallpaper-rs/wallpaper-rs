use super::Seat;
use crate::eye::{Mat4, placement};
use crate::resolve::Context;
use crate::rigged::math::{Mat, apply, invert, mul};
use crate::{Kind, Layer, Value, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Sight {
    pub to_model: Mat,
    pub eye: crate::eye::Eye,
    pub size: [f32; 2],
}

fn narrow(m: &Mat4) -> Mat {
    std::array::from_fn(|k| m[(k % 4) * 4 + k / 4])
}

pub(crate) fn sight(cx: &Context, layer: &Layer, seat: &Seat, travel: &Mat) -> Option<Sight> {
    let Kind::Model(m) = &layer.kind else { return None };
    let vec = |v: &Option<Value<Vec3>>, or: [f32; 3]| v.as_ref().map_or(or, |v| v.get(cx).0);
    let placed = placement(vec(&m.position, [0.0; 3]), vec(&m.rotation, [0.0; 3]), vec(&m.scale, [1.0; 3]));
    let to_world = mul(&narrow(&placed), travel);
    let (eye, _) = crate::eval::staged::eye(cx);
    let [w, h] = seat.size;
    let eye = crate::eye::Eye { aspect: w / h.max(1e-3), ..eye };
    Some(Sight { to_model: invert(&to_world), eye, size: seat.size })
}

impl Sight {
    pub(crate) fn onto(&self, [bx, by]: [f32; 2], depth_of: [f32; 3]) -> [f32; 2] {
        let (r, u, f) = self.eye.basis();
        let t = (self.eye.fov.clamp(1.0, 179.0).to_radians() * 0.5).tan();
        let [w, h] = self.size;
        let (nx, ny) = (bx / (w * 0.5).max(1e-3), by / (h * 0.5).max(1e-3));
        let dir: [f32; 3] = std::array::from_fn(|i| f[i] + r[i] * nx * t * self.eye.aspect + u[i] * ny * t);
        let world = apply(&invert(&self.to_model), depth_of);
        let d: f32 = (0..3).map(|i| (world[i] - self.eye.at[i]) * f[i]).sum();
        let p: [f32; 3] = std::array::from_fn(|i| self.eye.at[i] + dir[i] * d);
        let [x, y, _] = apply(&self.to_model, p);
        [x, y]
    }
}
