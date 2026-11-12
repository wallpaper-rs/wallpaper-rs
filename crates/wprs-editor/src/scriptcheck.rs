use crate::findings::{Problem, problem};
use wprs_core::message::said;
use wprs_scene::Scene;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptFault {
    pub index: usize,
    pub path: String,
    pub line: Option<u32>,
    pub message: String,
    pub missing: bool,
}

impl ScriptFault {
    pub fn said(&self) -> wprs_core::message::Message {
        match (self.missing, self.line) {
            (true, _) => said(
                "editor.script_missing",
                "%1 is listed in scripts but is not in the folder",
                &[&self.path],
            ),
            (false, Some(line)) => {
                said("editor.script_syntax_at", "%1:%2: %3", &[&self.path, &line, &self.message])
            }
            (false, None) => said("editor.script_syntax", "%1: %2", &[&self.path, &self.message]),
        }
    }
}

pub fn faults(scene: &Scene, read: impl Fn(&str) -> Option<String>) -> Vec<ScriptFault> {
    let mut out = Vec::new();
    let mut sources = Vec::new();
    let mut owners = Vec::new();
    for (index, script) in scene.scripts.iter().enumerate() {
        match read(&script.path) {
            Some(text) => {
                sources.push((script.path.clone(), text));
                owners.push(index);
            }
            None => out.push(ScriptFault {
                index,
                path: script.path.clone(),
                line: None,
                message: String::new(),
                missing: true,
            }),
        }
    }
    for (at, fault) in wprs_script::syntax(&sources, &scene.first_interface()) {
        let index = owners[at];
        let path = scene.scripts[index].path.clone();
        out.push(ScriptFault { index, path, line: fault.line, message: fault.message, missing: false });
    }
    out.sort_by_key(|f| f.index);
    out
}

pub fn problems(scene: &Scene, dir: &std::path::Path) -> Vec<Problem> {
    let read = |path: &str| std::fs::read_to_string(dir.join(path)).ok();
    let found = faults(scene, read);
    found.iter().map(|f| problem(format!("/scene/scripts/{}/path", f.index), f.said())).collect()
}
