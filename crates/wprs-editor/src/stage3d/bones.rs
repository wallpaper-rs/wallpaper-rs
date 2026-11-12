use super::gizmo::{Seen, px, world};
use serde_json::{Value, json};
use wprs_scene::{Placed, Runtime};

pub(super) fn bones(rt: &Runtime, seen: &Seen, p: &Placed, doc: &Value, mut out: Value) -> Value {
    let Some(rig) = p.model.rig.as_ref() else { return out };
    let Some(puppet) = rt.puppet(&p.layer.id) else { return out };
    let worlds = puppet.joint_worlds();
    let head = |n: usize| worlds.get(n).map(|w| world(&p.matrix, [w[3], w[7], w[11]]));
    let mut list = Vec::new();
    for (n, j) in rig.joints.iter().enumerate().filter(|(_, j)| !j.point) {
        let Some(at) = head(n).and_then(|h| seen.at(h)) else { continue };
        let up = j.parent.and_then(|q| head(q as usize)).and_then(|h| seen.at(h));
        list.push(json!({ "name": j.name, "at": px(at), "from": up.map(px) }));
    }
    out["bones"] = json!(list);
    let pointer = out["pointer"].as_str().unwrap_or_default().to_owned();
    let raw = doc.pointer(&pointer).cloned().unwrap_or_default();
    out["swing"] = json!(super::swing::arcs(rt, seen, p, &raw));
    out["colliders"] = json!(super::hulls::colliders(seen, p, (rig, worlds), &raw));
    out["slides"] = json!(super::hulls::slides(seen, p, worlds, rig, &raw));
    out["reach"] = json!(super::hulls::reach(seen, &raw));
    out["clips"] =
        json!(raw["clips"].as_array().into_iter().flatten().map(|c| c["id"].clone()).collect::<Vec<_>>());
    let path = puppet.course(p.layer, 2.0);
    out["path"] =
        json!(path.iter().filter_map(|q| seen.at(world(&p.matrix, *q)).map(px)).collect::<Vec<_>>());
    out
}
