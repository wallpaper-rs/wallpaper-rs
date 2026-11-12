use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use wprs_scene::{Affine, Lean, Quality, Runtime, Scene, Seated, Values};

pub(crate) mod boxes;
pub(crate) mod grips;
mod kin;
pub mod lens;
mod snap;

pub use kin::{chain, parents};

#[derive(Debug, Clone, PartialEq)]
pub struct Seat {
    pub id: String,
    pub pointer: String,
    pub kind: String,
    pub frame: Affine,
    pub world: Affine,
    pub size: [f64; 2],
    pub origin: [f64; 2],
    pub lean: Option<Lean>,
    pub parent: Option<String>,
}

fn point(p: [f32; 2]) -> [f64; 2] {
    p.map(f64::from)
}

fn single(p: [f64; 2]) -> [f32; 2] {
    p.map(|v| v as f32)
}

impl Seat {
    pub fn unit(&self) -> Affine {
        let [w, h] = single(self.size);
        let [ox, oy] = single(self.origin);
        self.world.then(Affine::translate([-ox * w, -oy * h])).then(Affine::scale([w, h]))
    }

    pub fn flat(&self, u: [f64; 2]) -> [f64; 2] {
        point(self.unit().apply(single(u)))
    }

    pub fn at(&self, u: [f64; 2]) -> [f64; 2] {
        let flat = self.unit().apply(single(u));
        point(self.lean.map_or(flat, |l| l.apply(flat)))
    }

    pub fn fraction(&self, q: [f64; 2]) -> [f64; 2] {
        let flat = self.lean.map_or(single(q), |l| l.inverse().apply(single(q)));
        point(self.unit().inverse().apply(flat))
    }

    pub fn local(&self, q: [f64; 2]) -> [f64; 2] {
        point(self.frame.inverse().apply(single(q)))
    }

    pub fn canvas(&self, local: [f64; 2]) -> [f64; 2] {
        point(self.frame.apply(single(local)))
    }

    pub fn pivot(&self) -> [f64; 2] {
        point(self.world.apply([0.0; 2]))
    }

    pub fn position(&self) -> [f64; 2] {
        self.local(self.pivot())
    }

    pub fn own(&self) -> Affine {
        let [a, b, c, d, ..] = self.frame.inverse().then(self.world).0;
        Affine([a, b, c, d, 0.0, 0.0])
    }

    pub fn outline(&self) -> [[f64; 2]; 4] {
        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]].map(|u| self.flat(u))
    }

    pub fn corners(&self) -> [[f64; 2]; 4] {
        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]].map(|u| self.at(u))
    }
}

fn parsed(scene: &Value) -> Option<Scene> {
    let mut scene = scene.clone();
    if scene.get("schema").is_none() {
        scene["schema"] = json!(wprs_scene::SCHEMA);
    }
    if scene.pointer("/canvas/size").is_none() {
        scene["canvas"] = json!({ "size": [1920, 1080] });
    }
    if let Ok(found) = Scene::from_json(&scene.to_string()) {
        return Some(found);
    }
    fn keep(list: &mut Value) {
        if let Some(items) = list.as_array_mut() {
            items.retain(|l| serde_json::from_value::<wprs_scene::Layer>(l.clone()).is_ok());
            items.iter_mut().for_each(|l| keep(&mut l["layers"]));
        }
    }
    let mut bare = json!({ "schema": scene["schema"], "canvas": scene["canvas"], "layers": scene["layers"] });
    keep(&mut bare["layers"]);
    Scene::from_json(&bare.to_string()).ok()
}

pub(crate) fn canvas_of(doc: &Value) -> [f64; 2] {
    let size = doc.pointer("/scene/canvas/size").and_then(Value::as_array);
    let at = |i: usize| size.and_then(|s| s.get(i)?.as_f64());
    [at(0).unwrap_or(1920.0), at(1).unwrap_or(1080.0)]
}

pub fn seats(dir: &Path, doc: &Value, t: f64) -> Vec<Seat> {
    let Some(mut scene) = parsed(&doc["scene"]) else { return Vec::new() };
    if lens::whole(&doc["scene"]) {
        scene.canvas.camera = None;
    }
    let canvas = canvas_of(doc);
    let rt = {
        let mut rt = Runtime::new(Arc::new(scene), Values::new());
        rt.advance(t.max(0.0));
        rt
    };
    let view = [0.0, 0.0, canvas[0] as f32, canvas[1] as f32];
    let placed: HashMap<String, Seated> =
        rt.seats(view, Quality::Q3).into_iter().map(|s| (s.id.clone(), s)).collect();
    let mut out: Vec<Seat> = crate::layers::walk(doc)
        .into_iter()
        .filter_map(|(pointer, layer)| {
            let id = layer["id"].as_str()?;
            let found = placed.get(id)?;
            Some(Seat {
                id: id.to_owned(),
                kind: layer["type"].as_str().unwrap_or_default().to_owned(),
                frame: found.frame,
                world: found.world,
                size: boxes::size(dir, doc, layer, canvas, t),
                origin: point(found.origin),
                lean: found.lean,
                parent: layer["parent"].as_str().filter(|p| !p.is_empty()).map(String::from),
                pointer,
            })
        })
        .collect();
    boxes::groups(&mut out);
    out
}

pub fn seat(dir: &Path, doc: &Value, id: &str, t: f64) -> Option<Seat> {
    seats(dir, doc, t).into_iter().find(|s| s.id == id)
}
