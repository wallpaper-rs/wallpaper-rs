pub fn size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let (mut value, mut unit) = (bytes as f64, 0);
    while value >= 1000.0 && unit < UNITS.len() - 1 {
        value /= 1000.0;
        unit += 1;
    }
    match (unit, value < 10.0 && value.fract() >= 0.05) {
        (0, _) => format!("{bytes}\u{a0}B"),
        (_, true) => format!("{value:.1}\u{a0}{}", UNITS[unit]),
        _ => format!("{value:.0}\u{a0}{}", UNITS[unit]),
    }
}

#[cfg(test)]
mod tests {
    use super::size;

    #[test]
    fn sizes_read_in_the_nearest_unit() {
        assert_eq!(size(0), "0\u{a0}B");
        assert_eq!(size(640_000_000), "640\u{a0}MB");
        assert_eq!(size(2_000_000_000), "2\u{a0}GB");
        assert_eq!(size(1_500_000), "1.5\u{a0}MB");
    }
}
