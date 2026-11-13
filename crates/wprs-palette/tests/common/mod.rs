pub const W: usize = 96;
pub const H: usize = 54;

fn noise(seed: u64) -> impl FnMut() -> u8 {
    let mut s = seed;
    move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 24) as u8
    }
}

fn build(mut f: impl FnMut(usize, usize) -> [u8; 3]) -> Vec<u8> {
    let mut out = Vec::with_capacity(W * H * 3);
    for y in 0..H {
        for x in 0..W {
            out.extend(f(x, y));
        }
    }
    out
}

fn mix(a: [u8; 3], b: [u8; 3], t: f64) -> [u8; 3] {
    [0, 1, 2].map(|i| (f64::from(a[i]) + (f64::from(b[i]) - f64::from(a[i])) * t).round() as u8)
}

pub fn pictures() -> Vec<(&'static str, Vec<u8>)> {
    let mut n1 = noise(88172645463325252);
    let mut n2 = noise(0x9e3779b97f4a7c15);
    let mut n3 = noise(0x1234_5678_9abc_def1);
    vec![
        ("flat_blue", build(|_, _| [30, 80, 200])),
        ("flat_red", build(|_, _| [200, 30, 30])),
        ("flat_green", build(|_, _| [40, 170, 60])),
        ("black", build(|_, _| [0, 0, 0])),
        ("white", build(|_, _| [255, 255, 255])),
        ("grey", build(|_, _| [128, 128, 128])),
        ("dark_grey", build(|_, _| [30, 31, 30])),
        ("light", build(|x, y| [235, 230 - (x % 7) as u8, 225 - (y % 5) as u8])),
        ("dark", build(|x, y| [10 + (x / 12) as u8, 14 + (y / 9) as u8, 40 + (x / 4) as u8])),
        ("saturated", build(|x, _| if x < 48 { [255, 0, 80] } else { [0, 220, 255] })),
        ("gradient_h", build(|x, _| mix([20, 10, 90], [250, 160, 40], x as f64 / (W - 1) as f64))),
        ("gradient_v", build(|_, y| mix([5, 5, 5], [240, 240, 255], y as f64 / (H - 1) as f64))),
        (
            "sunset",
            build(|x, y| {
                mix(
                    mix([20, 20, 70], [255, 120, 60], y as f64 / (H - 1) as f64),
                    [255, 220, 150],
                    (x as f64 / W as f64).powi(2),
                )
            }),
        ),
        ("noise", build(|_, _| [n1(), n2(), n3()])),
        ("noise_dark", build(|_, _| [n1() / 6, n2() / 6, n3() / 5])),
        (
            "noise_grey",
            build(|_, _| {
                let v = n1();
                [v, v, v]
            }),
        ),
        (
            "night_city",
            build(
                |x, y| if (x * 7 + y * 3) % 23 == 0 { [255, 200, 90] } else { [8, 12 + (y / 3) as u8, 30] },
            ),
        ),
        ("forest", build(|x, y| mix([15, 60, 30], [90, 140, 70], ((x + 2 * y) % 31) as f64 / 30.0))),
    ]
}
