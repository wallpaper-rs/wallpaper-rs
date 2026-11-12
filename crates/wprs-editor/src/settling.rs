use crate::meta::sync;
use crate::project::Project;
use anyhow::Result;

impl Project {
    pub fn settle(&mut self) -> Result<bool> {
        let follow = sync(&self.doc, &Default::default(), Some(&self.dir));
        if follow.is_empty() || self.dirty() || self.pending.is_some() {
            return Ok(false);
        }
        self.record("sync", follow)?;
        self.save().map(|_| true)
    }
}
