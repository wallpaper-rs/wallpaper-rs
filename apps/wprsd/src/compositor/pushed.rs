use super::{Command, Compositor};

impl Compositor {
    pub fn press(&self, output: &str, id: &str) {
        self.send(Command::Press(output.into(), id.into()))
    }

    pub fn fire(&self, output: &str, trigger: &str) {
        self.send(Command::Fire(output.into(), trigger.into()))
    }
}
