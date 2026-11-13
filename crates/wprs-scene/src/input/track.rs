use super::MediaState;
use serde::Deserialize;

const MAX_TEXT: usize = 512;
const MAX_SECONDS: f32 = 1e7;

#[derive(Deserialize, Default)]
#[serde(untagged)]
enum Words {
    #[default]
    None,
    One(String),
    Many(Vec<String>),
}

#[derive(Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct MadeTrack {
    title: String,
    artist: Words,
    album: String,
    album_artist: Words,
    subtitle: String,
    genres: Words,
    playing: bool,
    position: f32,
    duration: f32,
    rate: Option<f32>,
    pub cover: Option<String>,
}

fn text(name: &str, words: Words) -> Result<String, String> {
    let joined = match words {
        Words::None => String::new(),
        Words::One(one) => one,
        Words::Many(many) => many.join(", "),
    };
    let fits = joined.chars().count() <= MAX_TEXT && !joined.chars().any(char::is_control);
    match fits {
        true => Ok(joined),
        false => Err(format!("media.{name} holds at most {MAX_TEXT} characters and no control characters")),
    }
}

fn seconds(name: &str, value: f32) -> Result<f32, String> {
    let fits = value.is_finite() && (0.0..=MAX_SECONDS).contains(&value);
    match fits {
        true => Ok(value),
        false => Err(format!("media.{name} goes from 0 to {MAX_SECONDS} seconds, not {value}")),
    }
}

impl MadeTrack {
    pub fn read(value: serde_json::Value) -> Result<MadeTrack, String> {
        serde_json::from_value(value).map_err(|e| format!("the simulated media player: {e}"))
    }

    pub fn state(self, cover: Option<(u64, [[f32; 4]; 5])>) -> Result<MediaState, String> {
        let rate = self.rate.unwrap_or(1.0);
        if !(rate.is_finite() && (-100.0..=100.0).contains(&rate)) {
            return Err("media.rate goes from -100 to 100".into());
        }
        let duration = seconds("duration", self.duration)?;
        let position = seconds("position", self.position)?;
        Ok(MediaState {
            enabled: true,
            track: 1,
            cover: cover.map_or(0, |c| c.0),
            title: text("title", Words::One(self.title))?,
            artist: text("artist", self.artist)?,
            album: text("album", Words::One(self.album))?,
            album_artist: text("album_artist", self.album_artist)?,
            subtitle: text("subtitle", Words::One(self.subtitle))?,
            genres: text("genres", self.genres)?,
            kind: "music".into(),
            playing: self.playing,
            position: if duration > 0.0 { position.min(duration) } else { position },
            duration,
            rate,
            colors: cover.map(|c| c.1),
        })
    }
}

impl MediaState {
    pub fn at(&self, seconds: f64) -> MediaState {
        let moved = match self.playing {
            true => self.position + (seconds * f64::from(self.rate)) as f32,
            false => self.position,
        };
        let top = if self.duration > 0.0 { self.duration } else { f32::MAX };
        MediaState { position: moved.clamp(0.0, top), ..self.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_made_up_track_becomes_a_state_that_moves_only_while_it_plays() {
        let track = MadeTrack::read(json!({ "title": "A", "artist": ["x", "y"], "playing": true,
            "position": 10, "duration": 12, "rate": 2.0 }))
        .unwrap();
        let state = track.state(None).unwrap();
        assert_eq!((state.artist.as_str(), state.track, state.enabled), ("x, y", 1, true));
        assert_eq!(state.at(0.5).position, 11.0);
        assert_eq!(state.at(30.0).position, 12.0);
        let paused = MediaState { playing: false, ..state };
        assert_eq!(paused.at(30.0).position, 10.0);
    }

    #[test]
    fn a_track_with_bad_values_is_refused_with_the_field_named() {
        let refused = |v: serde_json::Value| MadeTrack::read(v).and_then(|t| t.state(None)).unwrap_err();
        assert!(refused(json!({ "rate": 500 })).contains("media.rate"));
        assert!(refused(json!({ "position": -1 })).contains("media.position"));
        assert!(refused(json!({ "title": "a\u{7}" })).contains("control"));
        assert!(refused(json!({ "lyrics": 1 })).contains("unknown field"));
    }
}
