use crate::{Error, choice, said, settings};
use serde::Deserialize;
use serde_json::{Value, json};
use wprs_core::filter::{Filter, LOOKS, Look, RANGES};

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub(crate) enum Request {
    Filter { lock: bool },
    SetFilter { lock: bool, filter: Option<Value> },
}

pub(crate) fn request(request: Request) -> Result<Value, Error> {
    match request {
        Request::Filter { lock } => Ok(shown(lock)),
        Request::SetFilter { lock, filter } => {
            let chosen = filter.map(read).transpose()?;
            settings::update(|config| {
                match (lock, chosen) {
                    (false, chosen) => config.default.filter = chosen.filter(|f| !f.is_identity()),
                    (true, Some(chosen)) => config.set_filter(choice::LOCK, chosen),
                    (true, None) => drop(config.outputs.get_mut(choice::LOCK).map(|o| o.filter.take())),
                }
                Ok(())
            })?;
            Ok(shown(lock))
        }
    }
}

fn read(value: Value) -> Result<Filter, Error> {
    let filter: Filter = serde_json::from_value(value)
        .map_err(|e| said("android.filter_value", "the filter cannot be read: %1", &[&e]))?;
    if !filter.valid() {
        let ranges = RANGES.iter().map(|(name, lo, hi)| format!("{name} {lo} to {hi}"));
        let ranges = ranges.collect::<Vec<_>>().join(", ");
        return Err(said("android.filter_range", "the filter takes %1", &[&ranges]));
    }
    Ok(filter.sanitized())
}

const SAMPLE: [[u8; 3]; 8] = [
    [120, 170, 230],
    [70, 140, 60],
    [225, 175, 145],
    [245, 245, 245],
    [128, 128, 128],
    [35, 40, 55],
    [200, 60, 50],
    [60, 90, 190],
];
const GAMMA: f32 = 2.2;

fn swatch(look: &Look) -> Vec<u32> {
    let grade = Filter { look: *look, ..Filter::default() }.grade();
    SAMPLE
        .iter()
        .map(|rgb| {
            let linear = rgb.map(|c| (f32::from(c) / 255.0).powf(GAMMA));
            let shown = grade.as_ref().map_or(linear, |g| g.apply(linear));
            let [r, g, b] = shown.map(|c| (c.clamp(0.0, 1.0).powf(1.0 / GAMMA) * 255.0).round() as u32);
            0xff00_0000 | (r << 16) | (g << 8) | b
        })
        .collect()
}

fn shown(lock: bool) -> Value {
    let config = settings::config();
    let own = if lock {
        config.outputs.get(choice::LOCK).is_some_and(|o| o.filter.is_some())
    } else {
        config.default.filter.is_some()
    };
    let filter = config.filter(choice::screen(lock));
    let value = serde_json::to_value(filter).unwrap_or(Value::Null);
    let fields = RANGES.iter().map(|(key, min, max)| {
        let places = if max - min > 100.0 { 0 } else { 2 };
        json!({ "key": key, "min": min, "max": max, "decimals": places, "value": value[key] })
    });
    let looks = LOOKS.iter().map(|look| json!(look));
    let swatches: serde_json::Map<String, Value> = LOOKS
        .iter()
        .map(|look| (json!(look).as_str().unwrap_or_default().to_owned(), json!(swatch(look))))
        .collect();
    json!({
        "filter": value, "own": own, "fields": fields.collect::<Vec<_>>(), "looks": looks.collect::<Vec<_>>(),
        "swatches": swatches,
    })
}
