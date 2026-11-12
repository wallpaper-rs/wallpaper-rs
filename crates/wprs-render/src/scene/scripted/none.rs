use super::super::SceneAssets;
use std::path::PathBuf;
use wprs_scene::{Runtime, Scene};

pub type ScriptHost = (PathBuf, Vec<String>);

pub fn serve_script_host() {}

pub(crate) struct Scripted {
    pub(crate) console: Vec<String>,
    pub(crate) stopped: Vec<String>,
}

impl Scripted {
    pub(crate) fn start(
        _: &Scene,
        _: &SceneAssets,
        _: &Runtime,
        _: (&str, Option<&ScriptHost>),
    ) -> Option<Scripted> {
        None
    }

    pub(crate) fn warm(&mut self, _: &mut Runtime, _: ([f32; 4], [u32; 2], u32, u8)) {}

    pub(crate) fn step(&mut self, _: &mut Runtime, _: ([f32; 4], [u32; 2], u32, u8)) {}
}
