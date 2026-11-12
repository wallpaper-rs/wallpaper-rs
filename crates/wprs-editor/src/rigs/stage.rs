use serde_json::{Value, json};

pub fn staged(look: &Value) -> Option<Value> {
    let quality = look.get("quality").filter(|q| q.is_string());
    let restart = look.get("restart").filter(|r| r.is_u64());
    let media = look.get("media").filter(|m| m.is_object());
    let fps = look.get("fps").filter(|f| f.is_u64());
    let mut staged = json!({ "quality": quality, "restart": restart });
    media.into_iter().for_each(|m| staged["media"] = m.clone());
    fps.into_iter().for_each(|f| staged["fps"] = f.clone());
    let flags = ["battery", "calm", "shift"].map(|k| look[k] == true);
    ["battery", "calm", "shift"]
        .iter()
        .zip(flags)
        .filter(|(_, on)| *on)
        .for_each(|(k, _)| staged[*k] = json!(true));
    (quality.is_some() || restart.is_some() || media.is_some() || fps.is_some() || flags.contains(&true))
        .then_some(staged)
}

pub fn preview(scene: &mut Value, look: &Value) {
    if look["bloom"] == false {
        scene["canvas"].as_object_mut().into_iter().for_each(|c| drop(c.remove("bloom")));
    }
    crate::lamps::preview(scene);
    crate::stage3d::preview(scene);
    if let Some(what) = look.get(crate::sounds::LISTEN).filter(|w| !w.is_null()) {
        crate::sounds::listen(scene, what);
    }
    if crate::seats::lens::whole(scene) {
        drop(scene["canvas"].as_object_mut().map(|c| c.remove("camera")));
    }
    if let Some(alone) = look["template"].as_str().and_then(|t| scene["templates"].get(t)).cloned() {
        scene["layers"] = json!([alone]);
        drop(scene.as_object_mut().map(|s| s.remove("scripts")));
    }
    let Some(id) = look["layer"].as_str() else { return };
    fn find<'a>(list: &'a mut Value, id: &str) -> Option<&'a mut Value> {
        for l in list.as_array_mut()?.iter_mut() {
            if l["id"] == id {
                return Some(l);
            }
            if let Some(found) = l.get_mut("layers").and_then(|c| find(c, id)) {
                return Some(found);
            }
        }
        None
    }
    let found = scene.get_mut("layers").and_then(|list| find(list, id));
    let Some(layer) = found.filter(|l| l["type"] == "mesh" || l["type"] == "model") else { return };
    if look["still"] == true
        && let Some(map) = layer.as_object_mut()
    {
        map.remove("physics");
    }
    if let Some(clip) = look["clip"].as_str() {
        layer["animations"] = json!([{ "clip": clip }]);
    }
    if let Some(shapes) = look["shapes"].as_object().filter(|s| !s.is_empty()) {
        let mut set = layer["shapes"].as_object().cloned().unwrap_or_default();
        set.extend(shapes.iter().filter(|(_, v)| v.is_number()).map(|(k, v)| (k.clone(), v.clone())));
        layer["shapes"] = json!(set);
    }
}
