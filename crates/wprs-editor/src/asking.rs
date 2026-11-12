use crate::Project;
use serde_json::{Value, json};

impl Project {
    pub fn ask(&self, layer: &str, q: &Value) -> Value {
        match q.get("history") {
            Some(_) => self.past(),
            None => crate::rigs::query(&self.dir, &self.holding(layer), layer, q),
        }
    }

    fn past(&self) -> Value {
        let (done, undone) = self.history.labels();
        let row = |label: &&str| json!({ "label": label, "text": crate::history::step(label).local() });
        let (undo, redo) = (done.last().map(row), undone.first().map(row));
        json!({ "done": done.iter().map(row).collect::<Vec<_>>(), "undone": undone.iter().map(row).collect::<Vec<_>>(),
            "undo": undo, "redo": redo, "dirty": self.dirty() })
    }
}
