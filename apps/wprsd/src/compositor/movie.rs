use super::Movie;

impl Movie {
    pub fn heard(self, factor: f64) -> Movie {
        Movie { volume: (f64::from(self.volume) * factor).round() as u32, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compositor::Reel;

    #[test]
    fn a_screen_lowers_the_own_volume_of_its_movie_and_silences_it_at_zero() {
        let movie =
            Movie { reel: Reel::plain("a.mp4".into()), volume: 80, zoom: 1.0, pan: [0.0; 2], max_fps: 0 };
        assert_eq!(movie.clone().heard(0.5).volume, 40);
        assert_eq!(movie.clone().heard(1.0).volume, 80);
        assert_eq!(movie.heard(0.0).volume, 0);
    }
}
