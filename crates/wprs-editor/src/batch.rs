use crate::actions::Action;
use crate::history::Step;
use crate::ops::Op;
use crate::project::Project;
use anyhow::{Result, ensure};
use wprs_core::message::said;

fn folded(steps: Vec<Step>) -> Option<Step> {
    let at = steps.last()?.at;
    let mut label: Vec<&str> = Vec::new();
    for step in &steps {
        if !label.contains(&step.label.as_str()) {
            label.push(&step.label);
        }
    }
    let label = label.join(", ");
    let undo = steps.iter().rev().flat_map(|s| s.undo.iter().cloned()).collect();
    let redo = steps.iter().flat_map(|s| s.redo.iter().cloned()).collect();
    let size = steps.iter().map(|s| s.size).sum();
    Some(Step { label, undo, redo, at, size, gesture: false })
}

impl Project {
    pub fn act_all(&mut self, actions: &[Action]) -> Result<Vec<Op>> {
        ensure!(
            !actions.iter().any(|a| matches!(a, Action::Undo {} | Action::Redo {})),
            said("editor.no_undo_in_step", "undo and redo cannot be part of a step with other actions", &[])
        );
        let (start, undone) = (self.history.done.len(), std::mem::take(&mut self.history.undone));
        let mut ops = Vec::new();
        for action in actions {
            match self.act(action) {
                Ok(done) => ops.extend(done),
                Err(e) => {
                    while self.history.done.len() > start && self.history.undo(&mut self.doc).is_ok() {}
                    self.history.undone = undone;
                    return Err(e);
                }
            }
        }
        if let Some(step) = folded(self.history.done.split_off(start)) {
            self.history.done.push(step);
            self.history.undone.clear();
            if self.history.saved.is_some_and(|s| s > self.history.done.len()) {
                self.history.saved = None;
            }
        } else {
            self.history.undone = undone;
        }
        Ok(ops)
    }
}
