use crate::sowed::pair;
use crate::told::{fill, told};
use serde_json::json;

pub(crate) fn indexed(text: &str) -> Result<(usize, [f64; 2]), String> {
    let (k, at) = text.split_once('=').ok_or_else(|| fill(told("%1 is not K=X,Y"), &[&text]))?;
    let k = k
        .trim()
        .trim_start_matches('p')
        .parse()
        .map_err(|_| fill(told("%1 is not a control point"), &[&k]))?;
    Ok((k, pair(at)?))
}

pub(crate) fn takes(text: &str) -> Result<wprs_editor::actions::Takes, String> {
    use wprs_editor::actions::Takes;
    match text {
        "off" => Ok(Takes::Off),
        "all" => Ok(Takes::All),
        "alpha" => Ok(Takes::Alpha),
        _ => Err(fill(told("%1 is not off, all or alpha"), &[&text])),
    }
}

pub(crate) fn mode(text: &str) -> Result<wprs_scene::Blend, String> {
    serde_json::from_value(json!(text)).map_err(|_| fill(told("%1 is not a blend mode"), &[&text]))
}
