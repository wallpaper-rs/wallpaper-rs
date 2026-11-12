use serde_json::{Value, json};
use wprs_core::message::said;
use wprs_scene::{AudioChannel, BANDS, Band, Binding, Gain, Level, Reduce, SPLITS, center, levels};

const DISPLAYS: [usize; 3] = [16, 32, 64];

fn channel(reading: &Value, key: &str, side: usize) -> [f32; BANDS] {
    let list = reading[key][side].as_array();
    std::array::from_fn(|i| list.and_then(|l| l.get(i)).and_then(Value::as_f64).unwrap_or(0.0) as f32)
}

fn fold(levels: &[f32; BANDS], of: usize, reduce: Reduce) -> Vec<f32> {
    let step = BANDS / of;
    levels
        .chunks(step)
        .map(|c| match reduce {
            Reduce::Max => c.iter().copied().fold(0.0, f32::max),
            Reduce::Mean => c.iter().sum::<f32>() / c.len() as f32,
        })
        .collect()
}

fn span(source: &Binding<f32>) -> Option<(usize, usize)> {
    let of = source.of.unwrap_or(BANDS as u32).clamp(1, BANDS as u32) as usize;
    let group = |lo: u32, hi: u32| {
        let a = (lo as usize * BANDS / of).min(BANDS - 1);
        (a, ((hi.max(lo) as usize + 1) * BANDS / of).clamp(a + 1, BANDS))
    };
    match source.audio? {
        Band::Index(i) => Some(group(i, i)),
        Band::Range([lo, hi]) => Some(group(lo, hi)),
        Band::Named(Level::Bass) => Some((0, below(SPLITS[0]))),
        Band::Named(Level::Mid) => Some((below(SPLITS[0]), below(SPLITS[1]))),
        Band::Named(Level::Treble) => Some((below(SPLITS[1]), BANDS)),
        Band::Named(_) => None,
    }
}

fn below(limit: f32) -> usize {
    (0..BANDS).filter(|b| center(*b) < limit).count()
}

pub fn query(doc: &Value, q: &Value) -> Value {
    let of = q["of"].as_u64().map_or(BANDS, |n| n as usize);
    if !DISPLAYS.contains(&of) {
        return json!({ "error": said("editor.meter_of", "bands are counted out of 16, 32 or 64", &[]).to_string() });
    }
    let source: Binding<f32> = match serde_json::from_value(q["source"].clone()) {
        Ok(source) => source,
        Err(e) => return json!({ "error": e.to_string() }),
    };
    let reading = &q["reading"];
    let listening = reading["listening"] == true && reading["sound"] != false;
    let mut out = json!({
        "granted": reading["granted"] != false,
        "listening": listening,
        "of": of,
        "marked": span(&source).map(|(a, z)| [a * of / BANDS, (z * of).div_ceil(BANDS) - 1]),
    });
    if !listening {
        return out;
    }
    let fixed = doc["scene"]["audio"]["gain"] == json!(Gain::Fixed);
    let key = if fixed { "loud" } else { "raw" };
    let (left, right) = (channel(reading, key, 0), channel(reading, key, 1));
    let average: [f32; BANDS] = std::array::from_fn(|i| (left[i] + right[i]) / 2.0);
    let side = match source.channel.unwrap_or_default() {
        AudioChannel::Average => &average,
        AudioChannel::Left => &left,
        AudioChannel::Right => &right,
    };
    let [bass, mid, treble] = levels(&average);
    let loudness = reading["loudness"].as_f64().unwrap_or(0.0) as f32;
    let beat = f32::from(reading["beat"] == true);
    let value = span(&source).map_or_else(
        || if matches!(source.audio, Some(Band::Named(Level::Beat))) { beat } else { loudness },
        |(a, z)| {
            let named = match source.audio {
                Some(Band::Named(Level::Bass)) => Some(bass),
                Some(Band::Named(Level::Mid)) => Some(mid),
                Some(Band::Named(Level::Treble)) => Some(treble),
                _ => None,
            };
            let picked = &side[a..z];
            named.unwrap_or_else(|| match source.reduce.unwrap_or_default() {
                Reduce::Max => picked.iter().copied().fold(0.0, f32::max),
                Reduce::Mean => picked.iter().sum::<f32>() / picked.len() as f32,
            })
        },
    );
    out["bands"] = json!(fold(side, of, source.reduce.unwrap_or_default()));
    out["raw"] = json!(value);
    out["value"] = json!(source.modify(value));
    out["levels"] = json!({ "bass": bass, "mid": mid, "treble": treble, "level": loudness, "beat": beat });
    out["silent"] = json!(reading["silent"] == true);
    out
}
