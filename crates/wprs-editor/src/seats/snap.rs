use super::Seat;

fn bounds(points: &[[f64; 2]]) -> [[f64; 3]; 2] {
    [0, 1].map(|i| {
        let lo = points.iter().map(|p| p[i]).fold(f64::MAX, f64::min);
        let hi = points.iter().map(|p| p[i]).fold(f64::MIN, f64::max);
        [lo, (lo + hi) / 2.0, hi]
    })
}

fn follows(seats: &[Seat], me: &Seat, other: &Seat) -> bool {
    let mut at = other.parent.as_deref();
    for _ in 0..=wprs_scene::MAX_DEPTH {
        match at {
            Some(id) if id == me.id => return true,
            Some(id) => at = seats.iter().find(|s| s.id == id).and_then(|s| s.parent.as_deref()),
            None => return false,
        }
    }
    false
}

pub fn lines(seats: &[Seat], me: &Seat, canvas: [f64; 2]) -> [Vec<f64>; 2] {
    let inside = format!("{}/", me.pointer);
    let mut out = [0, 1].map(|i| vec![0.0, canvas[i] / 2.0, canvas[i]]);
    let others = seats.iter().filter(|s| {
        s.id != me.id && !s.pointer.starts_with(&inside) && !follows(seats, me, s) && s.size != [0.0; 2]
    });
    for other in others {
        let b = bounds(&other.corners());
        (0..2).for_each(|i| out[i].extend(b[i]));
    }
    out
}

pub fn snapped(seats: &[Seat], me: &Seat, to: [f64; 2], tolerance: f64, canvas: [f64; 2]) -> [f64; 2] {
    let pivot = me.pivot();
    let shift = [to[0] - pivot[0], to[1] - pivot[1]];
    let moved: Vec<[f64; 2]> = me.corners().iter().map(|c| [c[0] + shift[0], c[1] + shift[1]]).collect();
    let edges = bounds(&moved);
    let found = lines(seats, me, canvas);
    [0, 1].map(|i| {
        let pulls = found[i].iter().flat_map(|line| edges[i].iter().map(move |e| line - e));
        let best = pulls.filter(|d| d.abs() <= tolerance).min_by(|a, b| a.abs().total_cmp(&b.abs()));
        to[i] + best.unwrap_or(0.0)
    })
}
