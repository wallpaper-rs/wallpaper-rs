use crate::action::Edit3d;
use crate::ops::Op;
use anyhow::Result;
use serde_json::Value;
use std::path::Path;

mod bones;
mod camera;
mod carve;
mod carved;
mod carving;
mod combed;
mod cost;
mod debug;
mod edit;
mod gizmo;
mod grab;
mod hulls;
mod info;
mod maps;
mod material;
mod newmat;
mod painted;
mod paths;
mod picked;
mod pose;
mod rings;
mod shadows;
mod spend;
mod sphere;
mod store;
mod surface;
mod swing;
mod thumbs;
mod turns;
mod view;

pub use cost::cost;
pub use gizmo::stage;
pub use info::info;
pub use newmat::TEMPLATE;
pub use store::file;
pub use view::{View, preview};

pub(crate) const QUERIES: [&str; 10] = [
    "model_info",
    "stage3d",
    "materials3d",
    "cost3d",
    "shadows3d",
    "paths3d",
    "material_check",
    "bone3d",
    "bound3d",
    "surface3d",
];

pub(crate) fn act(dir: &Path, doc: &Value, e: &Edit3d) -> Result<Vec<Op>> {
    match e {
        Edit3d::Model(m) => edit::model(dir, doc, m),
        Edit3d::Group { id, into, layers } => edit::group(doc, id, into, layers),
        Edit3d::Space { layer, deep } => edit::space(doc, layer, *deep),
        Edit3d::DepthTest { layer, on } => edit::depth_test(doc, layer, *on),
        Edit3d::Material(m) => material::material(dir, doc, m),
        Edit3d::Paint(s) => painted::paint(dir, doc, s),
        Edit3d::Sculpt(s) => carve::sculpt(dir, doc, s),
        Edit3d::Camera(c) => camera::camera_edit(doc, c),
        Edit3d::View(v) => camera::view_edit(dir, doc, v),
        Edit3d::Tones(t) => camera::tones(doc, t),
        Edit3d::Shadows(s) => shadows::edit(doc, s),
        Edit3d::Path(p) => paths::edit(dir, doc, p),
        Edit3d::Grab(g) => grab::grab(dir, doc, g),
        Edit3d::Light(l) => grab::place(dir, doc, l),
        Edit3d::NewMaterial { name } => newmat::create(dir, doc, name),
        Edit3d::MaterialCode { name, text } => newmat::write(dir, doc, name, text),
        Edit3d::Key(k) => picked::key_from(dir, doc, k),
        Edit3d::Comb(c) => combed::comb(dir, doc, c),
        Edit3d::Debug { show, layer, material } => debug::debug(dir, doc, show.as_deref(), (layer, material)),
    }
}

pub fn asks(q: &Value) -> bool {
    QUERIES.iter().any(|k| q.get(k).is_some())
}

pub fn model_info(dir: &Path, doc: &Value, q: &Value) -> Value {
    let path = match (q["asset"].as_str(), q["file"].as_str()) {
        (Some(a), _) => store::path(dir, doc, a),
        (None, Some(f)) => Ok(dir.join(f)),
        _ => return Value::Null,
    };
    match path.and_then(|p| file(&p)) {
        Ok((model, head)) => info(&model, &head),
        Err(e) => serde_json::json!({ "error": e.to_string() }),
    }
}

pub fn query(dir: &Path, doc: &Value, id: &str, q: &Value) -> Value {
    let t = |v: &Value| v["time"].as_f64().unwrap_or(0.0);
    match () {
        _ if q.get("model_info").is_some() => model_info(dir, doc, &q["model_info"]),
        _ if q.get("stage3d").is_some() => stage(dir, doc, t(&q["stage3d"])),
        _ if q.get("shadows3d").is_some() => shadows::shadows(dir, doc, t(&q["shadows3d"])),
        _ if q.get("bone3d").is_some() => pose::bone(dir, doc, &q["bone3d"]),
        _ if q.get("bound3d").is_some() => swing::bound(dir, doc, &q["bound3d"]),
        _ if q.get("surface3d").is_some() => surface::surface(dir, doc, &q["surface3d"]),
        _ if q.get("paths3d").is_some() => paths::paths(dir, doc, t(&q["paths3d"])),
        _ if q.get("material_check").is_some() => {
            let c = &q["material_check"];
            newmat::check(dir, doc, c["name"].as_str().unwrap_or_default(), c["text"].as_str())
        }
        _ if q.get("materials3d").is_some() => {
            material::listing(dir, doc, q["materials3d"]["layer"].as_str().unwrap_or(id))
        }
        _ => cost(dir, doc, serde_json::from_value(q["cost3d"]["quality"].clone()).unwrap_or_default()),
    }
}
