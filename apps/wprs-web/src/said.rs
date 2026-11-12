use std::time::Instant;

const BURST: f64 = 50.0;
const PER_SECOND: f64 = 10.0;

#[derive(Default)]
pub struct Said {
    tokens: f64,
    at: Option<Instant>,
}

impl Said {
    pub fn allow(&mut self) -> bool {
        self.allow_at(Instant::now())
    }

    fn allow_at(&mut self, now: Instant) -> bool {
        let earned = self.at.map_or(BURST, |at| now.duration_since(at).as_secs_f64() * PER_SECOND);
        self.tokens = (self.tokens + earned).min(BURST);
        self.at = Some(now);
        if self.tokens < 1.0 {
            return false;
        }
        self.tokens -= 1.0;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn a_flood_is_cut_but_later_lines_show() {
        let mut said = Said::default();
        let start = Instant::now();
        let shown = (0..500).filter(|_| said.allow_at(start)).count();
        assert_eq!(shown, 50);
        assert!(!said.allow_at(start));
        assert!(said.allow_at(start + Duration::from_millis(150)));
        let later = start + Duration::from_secs(600);
        assert!((0..10).all(|_| said.allow_at(later)));
    }

    #[test]
    fn a_steady_stream_is_never_cut() {
        let mut said = Said::default();
        let start = Instant::now();
        assert!((0..2000).all(|i| said.allow_at(start + Duration::from_millis(i * 100))));
    }
}
