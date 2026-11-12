use super::Stage3d;
use super::lamps::Lamp;
use crate::eye::{Eye, unit3};
use crate::light::Beam;

fn turn(eye: &Eye, [x, y, z]: [f32; 3]) -> [f32; 3] {
    let (r, u, f) = eye.basis();
    unit3(std::array::from_fn(|i| r[i] * x - u[i] * y - f[i] * z))
}

pub fn lamp3d(stage: &Stage3d, eye: &Eye, lamp: &Lamp) -> ([f32; 3], [f32; 3]) {
    let at = lamp.space.unwrap_or_else(|| {
        let local = stage.world.inverse().apply(lamp.at());
        let px = [local[0] + stage.origin[0] * stage.size[0], local[1] + stage.origin[1] * stage.size[1]];
        eye.place(px, lamp.z, stage.size)
    });
    let dir = match (lamp.aim, lamp.beam) {
        (Some(aim), _) => unit3(std::array::from_fn(|i| aim[i] - at[i])),
        (None, Beam::Tube) => turn(eye, [lamp.world.0[0], lamp.world.0[1], 0.0]),
        _ => turn(eye, lamp.heading()),
    };
    (at, dir)
}
