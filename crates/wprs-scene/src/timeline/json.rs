use super::keys::{Key, sample};
use crate::Color;
use crate::animate::Animate;
use crate::value::{Param, Vec2, Vec3};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value as Json;

fn sampled<T: Animate + DeserializeOwned>(keys: &[Json], t: f32) -> Option<T> {
    let keys: Vec<Key<T>> = serde_json::from_value(Json::Array(keys.to_vec())).ok()?;
    (!keys.is_empty()).then(|| sample(&keys, t))
}

fn written<T: Serialize>(v: T) -> Option<Json> {
    Some(rounded(serde_json::to_value(v).ok()?))
}

fn rounded(v: Json) -> Json {
    match v {
        Json::Number(n) => n.as_f64().map_or(Json::Number(n), |x| Json::from((x * 1e6).round() / 1e6)),
        Json::Array(list) => Json::Array(list.into_iter().map(rounded).collect()),
        Json::Object(map) => Json::Object(map.into_iter().map(|(k, v)| (k, rounded(v))).collect()),
        other => other,
    }
}

fn numbers(v: &Json) -> Option<usize> {
    v.as_array().filter(|list| list.iter().all(Json::is_number)).map(Vec::len)
}

pub fn sample_json(keys: &[Json], t: f64) -> Option<Json> {
    let values = || keys.iter().map(|k| &k[1]);
    let t = t as f32;
    let hex = values().any(|v| v.as_str().is_some_and(|s| Color::parse(s).is_some()));
    let lists = values().filter(|v| v.is_array()).map(numbers).collect::<Vec<_>>();
    let width = match lists.first() {
        Some(Some(n)) if lists.iter().all(|w| *w == Some(*n)) => Some(*n),
        Some(_) => None,
        None => Some(1),
    };
    match (hex, width) {
        (true, _) => sampled::<Color>(keys, t).map(|c| Json::from(c.hex())),
        (false, Some(1)) if values().all(Json::is_number) => written(sampled::<f32>(keys, t)?),
        (false, Some(2)) => written(sampled::<Vec2>(keys, t)?),
        (false, Some(3)) => written(sampled::<Vec3>(keys, t)?),
        _ => written(sampled::<Param>(keys, t)?),
    }
}
