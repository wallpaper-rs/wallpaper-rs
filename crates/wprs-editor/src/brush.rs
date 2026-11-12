pub fn falloff(d: f32, soft: f32) -> f32 {
    let hard = 1.0 - soft;
    match d {
        d if d >= 1.0 => 0.0,
        d if d <= hard => 1.0,
        d => {
            let s = (d - hard) / soft.max(1e-6);
            1.0 - s * s * (3.0 - 2.0 * s)
        }
    }
}

pub fn nearest(stroke: &[[f32; 2]], p: [f32; 2], radius: f32) -> f32 {
    let r = radius.max(1e-3);
    stroke.iter().map(|s| (s[0] - p[0]).hypot(s[1] - p[1]) / r).fold(1.0, f32::min)
}
