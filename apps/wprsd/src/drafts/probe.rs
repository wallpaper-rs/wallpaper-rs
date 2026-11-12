use serde_json::Value;

pub(crate) type Probe = ([f32; 2], [f32; 2]);

fn pair(value: &Value, low: f64, high: f64) -> Option<[f32; 2]> {
    let at = |i: usize| value.get(i)?.as_f64().filter(|v| v.is_finite()).map(|v| v.clamp(low, high) as f32);
    Some([at(0)?, at(1)?])
}

pub(crate) fn probed(doc: &Value) -> Option<Probe> {
    let probe = doc.pointer("/scene/ext/editor/probe")?;
    let pointer = pair(&probe["pointer"], 0.0, 1.0);
    let tilt = pair(&probe["tilt"], -1.0, 1.0);
    (pointer.is_some() || tilt.is_some()).then(|| (pointer.unwrap_or([0.5; 2]), tilt.unwrap_or([0.0; 2])))
}

#[cfg(test)]
mod tests {
    use super::probed;
    use serde_json::json;

    #[test]
    fn a_draft_with_a_probe_gives_its_pointer_and_tilt_inside_their_ranges() {
        let doc = |probe| json!({ "scene": { "ext": { "editor": { "probe": probe } } } });
        assert_eq!(
            probed(&doc(json!({ "pointer": [0.2, 0.8], "tilt": [-0.5, 0.5] }))),
            Some(([0.2, 0.8], [-0.5, 0.5]))
        );
        assert_eq!(probed(&doc(json!({ "tilt": [3, -3] }))), Some(([0.5, 0.5], [1.0, -1.0])));
        assert_eq!(probed(&doc(json!({ "pointer": [2, -1] }))), Some(([1.0, 0.0], [0.0, 0.0])));
        assert_eq!(probed(&doc(json!({}))), None);
        assert_eq!(probed(&json!({ "scene": {} })), None);
    }

    #[test]
    fn the_probe_the_editor_canvas_action_writes_is_read_as_it_was_set() {
        let written = json!({ "scene": { "ext": { "editor": { "probe": { "pointer": [0.5, 0.5], "tilt": [0.0, 1.0] } } } } });
        assert_eq!(probed(&written), Some(([0.5, 0.5], [0.0, 1.0])));
    }
}
