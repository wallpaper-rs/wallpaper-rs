use crate::compositor::lock;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use wprs_core::model::Input;
use wprs_scene::{AudioFrame, BANDS};

const WATCH: Duration = Duration::from_secs(3);
const FRESH: Duration = Duration::from_secs(1);

struct Reading {
    at: Instant,
    raw: [[f32; BANDS]; 2],
    loud: [[f32; BANDS]; 2],
    loudness: f32,
    beat: bool,
    silent: bool,
}

struct Meter {
    watched: BTreeMap<String, Instant>,
    readings: BTreeMap<String, Reading>,
}

static METER: Mutex<Meter> = Mutex::new(Meter { watched: BTreeMap::new(), readings: BTreeMap::new() });

fn round(levels: &[f32; BANDS]) -> Vec<f64> {
    levels.iter().map(|v| (f64::from(*v) * 1000.0).round() / 1000.0).collect()
}

pub(crate) fn metered(output: &str, frame: &AudioFrame) {
    let mut guard = lock(&METER);
    let Meter { watched, readings } = &mut *guard;
    watched.retain(|_, at| at.elapsed() < WATCH);
    readings.retain(|name, _| watched.contains_key(name));
    if !watched.contains_key(output) {
        return;
    }
    let reading = Reading {
        at: Instant::now(),
        raw: frame.raw,
        loud: frame.loud,
        loudness: frame.loudness,
        beat: frame.beat,
        silent: frame.silent,
    };
    readings.insert(output.to_owned(), reading);
}

pub(crate) fn bands(output: &str) -> Value {
    let capturing = super::hub().is_some_and(|hub| lock(&hub.control).capturing);
    reading(output, super::allows(output, Input::Audio), capturing)
}

fn reading(output: &str, granted: bool, capturing: bool) -> Value {
    let mut meter = lock(&METER);
    meter.watched.insert(output.to_owned(), Instant::now());
    let found = meter.readings.get(output).filter(|r| r.at.elapsed() < FRESH);
    let listening = granted && capturing;
    let Some(r) = found.filter(|_| granted) else {
        return json!({ "granted": granted, "listening": listening, "sound": false, "bands": BANDS });
    };
    json!({
        "granted": true,
        "listening": listening,
        "sound": !r.silent,
        "bands": BANDS,
        "raw": [round(&r.raw[0]), round(&r.raw[1])],
        "loud": [round(&r.loud[0]), round(&r.loud[1])],
        "loudness": (f64::from(r.loudness) * 1000.0).round() / 1000.0,
        "beat": r.beat,
        "silent": r.silent,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_is_kept_for_an_output_nobody_watches() {
        let frame = AudioFrame { loudness: 0.5, silent: false, ..AudioFrame::default() };
        metered("preview:quiet", &frame);
        assert!(!lock(&METER).readings.contains_key("preview:quiet"));
        assert_eq!(reading("preview:quiet", true, true)["sound"], false);
        metered("preview:quiet", &frame);
        let seen = reading("preview:quiet", true, true);
        assert_eq!((seen["sound"].clone(), seen["loudness"].clone()), (json!(true), json!(0.5)));
        assert_eq!(reading("preview:quiet", false, true)["raw"], Value::Null);
        let idle = reading("preview:quiet", true, false);
        assert_eq!((idle["listening"].clone(), idle["sound"].clone()), (json!(false), json!(true)));
    }

    #[test]
    fn an_open_capture_with_nothing_arriving_is_listening_without_sound() {
        let quiet = reading("preview:none", true, true);
        assert_eq!((quiet["listening"].clone(), quiet["sound"].clone()), (json!(true), json!(false)));
    }
}
