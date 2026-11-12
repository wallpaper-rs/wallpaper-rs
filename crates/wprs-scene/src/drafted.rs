use crate::Scene;

pub const DEBUG_VIEWS: [&str; 4] = ["", "mask", "matte", "shadows"];

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Drafted {
    pub debug3d: u8,
    pub only: Option<[String; 2]>,
    pub split: f32,
}

impl Drafted {
    pub fn debug_of(&self, layer: &str, material: &str) -> u8 {
        match &self.only {
            Some([l, m]) if l != layer || m != material => 0,
            _ => self.debug3d,
        }
    }
}

impl Scene {
    pub fn drafted(&mut self) {
        let editor = self.ext.get("editor");
        let debug = editor.map(|e| &e["debug3d"]);
        let shown = debug.and_then(|d| d.as_str().or_else(|| d["show"].as_str()));
        let debug3d = DEBUG_VIEWS.iter().position(|k| Some(*k) == shown).unwrap_or(0) as u8;
        let only =
            debug.and_then(|d| Some([d["layer"].as_str()?.to_string(), d["material"].as_str()?.to_string()]));
        let split = editor.and_then(|e| e["tonemap_split"].as_f64()).unwrap_or(0.0).clamp(0.0, 1.0) as f32;
        self.draft = Drafted { debug3d, only: only.filter(|_| debug3d > 0), split };
    }
}
